use crate::utils::get_option;
use crate::{
    components::DeviceOS,
    protocol::mcpe::{
        crypto::cipher::{Aes256CtrCipherManager, Cipher},
        packet::{
            key_exchange,
            login_verify::{verify_login, verify_skin_data},
            CompressionAlgorithmType, NetworkSettings, PacketKind, PlayStatus,
            ServerToClientHandshake,
        },
        transforms::framer,
    },
};

use aes::cipher::StreamCipher;
use anyhow::{anyhow, Result};
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use rust_raknet::RaknetSocket;
use sparsey::world::World;
use sparsey::{query::Query, storage::Entity, world::Comp};
use std::fmt::Display;
use std::sync::Arc;

#[derive(Clone)]
pub struct Player {
    pub entity: Entity,
    world: Arc<AtomicRefCell<World>>,
    socket: Arc<AtomicRefCell<RaknetSocket>>,
    pub status: PlayerStatus,
}

#[derive(Default, Clone)]
pub struct PlayerStatus {
    pub encryption_enabled: bool,
    pub send_counter: u64,
    pub ss_key: Option<[u8; 32]>,
    pub cipher: Option<Cipher>,
    pub decipher: Option<Cipher>,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{id:{}}}",
            self.socket.borrow().peer_addr().unwrap().to_string()
        )
    }
}

impl Player {
    pub fn new(socket: RaknetSocket, entity: Entity, world: Arc<AtomicRefCell<World>>) -> Self {
        Player {
            socket: Arc::new(AtomicRefCell::new(socket)),
            world,
            entity,
            status: PlayerStatus::default(),
        }
    }
    #[inline]
    fn get_world(&self) -> AtomicRef<World> {
        self.world.borrow()
    }
    #[inline]
    fn get_world_mut(&self) -> AtomicRefMut<World> {
        self.world.borrow_mut()
    }
    pub async fn listen(&mut self) -> Result<()> {
        let socket = self.socket.clone();
        while let Ok(buffer) = socket.borrow().recv().await {
            let pkts = framer::decode(self.decrypt_or(buffer[1..].to_vec()))?;
            for pkt in pkts {
                let packet = framer::parse_packet(pkt)?;
                println!("[C=>S]{}", packet);
                self.handle(packet).await.unwrap();
            }
        }
        self.get_world_mut().destroy(self.entity);
        println!("disconnected,{}", self);
        Ok(())
    }
    async fn handle(&mut self, packet: PacketKind) -> Result<()> {
        match packet {
            PacketKind::RequestNetworkSetting(pkt) => {
                let current_p = get_option("protocol")?.parse::<i32>()?;
                match pkt.client_protocol {
                    x if x > current_p => self.send_packet(PlayStatus::FailedSpawn).await?,
                    x if x < current_p => self.send_packet(PlayStatus::FailedClient).await?,
                    _ => self.send_network_setting().await?,
                };
            }
            PacketKind::Login(pkt) => {
                let (key, data) = verify_login(&pkt.identity)?;
                let skin_data = verify_skin_data(&key, &pkt.client)?;
                let (secret, token) = key_exchange::shared_secret(&key)?;
                self.send_packet(ServerToClientHandshake { token }).await?;
                let iv: [u8; 16] = [secret[0..12].to_vec(), vec![0, 0, 0, 2]]
                    .concat()
                    .try_into()
                    .unwrap();
                self.status.encryption_enabled = true;
                self.status.ss_key = Some(secret.clone());
                self.setup_cipher(secret, iv)?;
                self.get_world_mut()
                    .insert(self.entity, (DeviceOS::from(skin_data.DeviceOS),));
            }
            PacketKind::ClientToServerHandshake(_) => {
                self.send_packet(PlayStatus::LoginSuccess).await?;
                self.get_world()
                    .run(|os: Comp<DeviceOS>| (&os).for_each(|os| println!("{:?}", os)))
            }
            PacketKind::ClientCacheStatus(_) => {}
            _ => todo!(),
        }
        Ok(())
    }
    pub async fn send_packet<T: Into<PacketKind>>(&mut self, packet: T) -> Result<()> {
        let packet: PacketKind = packet.into();
        println!("[S=>C]{}", packet);
        let bind = framer::encode(packet, self.status.encryption_enabled)?;
        let buffer = self.encrypt_or(bind);
        self.socket
            .borrow()
            .send(
                &[vec![0xfe], buffer].concat(),
                rust_raknet::Reliability::Reliable,
            )
            .await
            .map_err(|e| anyhow!("FailedToSendPacket:{:?}", e))?;
        Ok(())
    }
    async fn send_network_setting(&mut self) -> Result<()> {
        let network_setting = NetworkSettings {
            compression_threshold: 512,
            compression_algorithm: CompressionAlgorithmType::Deflate,
            client_throttle: false,
            client_throttle_threshold: 0,
            client_throttle_scalar: 0.0,
        };
        self.send_packet(network_setting).await?;
        Ok(())
    }
    fn decrypt_or(&mut self, buffer: Vec<u8>) -> Vec<u8> {
        let mut result = buffer;
        if self.status.encryption_enabled {
            self.status
                .decipher
                .as_mut()
                .unwrap()
                .apply_keystream(&mut result);
        }
        result
    }
    fn encrypt_or(&mut self, buffer: Vec<u8>) -> Vec<u8> {
        let mut result = buffer;
        if self.status.encryption_enabled {
            let tag = self.compute_packet_tag(&result);
            result = [result, tag].concat();
            self.status
                .cipher
                .as_mut()
                .unwrap()
                .apply_keystream(&mut result);
            self.status.send_counter += 1;
        }
        result
    }
    fn compute_packet_tag(&self, plain_pkt: &[u8]) -> Vec<u8> {
        let mut digest = hmac_sha256::Hash::new();
        digest.update(self.status.send_counter.to_be_bytes());
        digest.update(plain_pkt);
        digest.update(self.status.ss_key.unwrap());
        let result = digest.finalize();
        result[0..8].to_vec()
    }
}
