use crate::components::{DeviceOS, PlayerName};
use crate::protocol::mcpe::packet::ResourcePackInfo;
use crate::protocol::mcpe::{
    crypto::cipher::{Aes256CtrManager, Cipher},
    packet::{
        key_exchange,
        login_verify::{verify_login, verify_skin_data},
        CompressionAlgorithmType, LoginPacket, NetworkSettingsPacket, PacketKind, PlayStatusPacket,
        ResourcePacksInfoPacket, ServerToClientHandshakePacket,
    },
    transforms::framer,
};
use crate::utils::get_option;

use anyhow::{anyhow, Result};
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use rust_raknet::RaknetSocket;
use sparsey::storage::Entity;
use sparsey::world::World;
use std::fmt::Display;
use std::sync::Arc;

#[derive(Clone)]
pub struct Player {
    pub entity: Entity,
    world: Arc<AtomicRefCell<World>>,
    pub socket: Arc<RaknetSocket>,
    pub status: Arc<AtomicRefCell<PlayerStatus>>,
}

#[derive(Default)]
pub struct PlayerStatus {
    pub encryption_enabled: bool,
    pub send_counter: u64,
    pub ss_key: Option<[u8; 32]>,
    pub cipher: Option<Cipher>,
    pub decipher: Option<Cipher>,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.get_world().borrow::<PlayerName>().get(self.entity) {
            Some(v) => write!(f, "{}", v.user_name),
            None => write!(f, "{:?}", self.socket.peer_addr()),
        }
    }
}

impl Player {
    pub fn new(socket: RaknetSocket, entity: Entity, world: Arc<AtomicRefCell<World>>) -> Self {
        Player {
            socket: Arc::new(socket),
            world,
            entity,
            status: Arc::new(AtomicRefCell::new(Default::default())),
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
    #[inline]
    pub fn get_status(&self) -> AtomicRef<PlayerStatus> {
        self.status.borrow()
    }
    #[inline]
    pub fn get_status_mut(&self) -> AtomicRefMut<PlayerStatus> {
        self.status.borrow_mut()
    }
    pub async fn listen(&mut self) -> Result<()> {
        let socket = self.socket.clone();
        while let Ok(buffer) = socket.recv().await {
            let mut buffer = buffer[1..].to_vec();
            for pkt in framer::decode(self.decrypt_or(&mut buffer))? {
                let packet = framer::parse_packet(pkt)?;
                println!("[C=>S]{}", packet);
                self.handle(&packet).await.unwrap();
            }
        }
        println!("disconnected,{}", self);
        self.get_world_mut().destroy(self.entity);
        Ok(())
    }
    async fn handle(&mut self, packet: &PacketKind) -> Result<()> {
        match packet {
            PacketKind::RequestNetworkSettingPacket(pkt) => {
                let current_p = get_option("protocol")?.parse::<i32>()?;
                match pkt.client_protocol {
                    x if x > current_p => self.send_packet(PlayStatusPacket::FailedSpawn).await?,
                    x if x < current_p => self.send_packet(PlayStatusPacket::FailedClient).await?,
                    _ => self.send_network_setting().await?,
                };
            }
            PacketKind::LoginPacket(pkt) => self.login_handshake(pkt).await?,
            PacketKind::ClientToServerHandshakePacket(_) => {
                self.send_packet(PlayStatusPacket::LoginSuccess).await?;
                let re = ResourcePackInfo {
                    uuid:"2b02525d-d224-4872-a2a6-0c85ab138761".to_owned(),
                    version:"1.0.0".to_owned(),
                    size:3140000,
                    encryption_key:"".to_owned(),
                    sub_pack_name:"".to_owned(),
                    content_identity:"my_resource".to_owned(),
                    scripting:false,
                    rtx_enabled:false
                };
                let resource_info = ResourcePacksInfoPacket {
                    must_accept: true,
                    scripting: false,
                    force_server_packs: false,
                    behaviour_pack_infos: vec![],
                    resource_pack_infos: vec![re],
                    resource_pack_links: vec![],
                };
                self.send_packet(resource_info).await?;
            }
            PacketKind::ClientCacheStatusPacket(_) => {}
            PacketKind::ResourcePackClientResponsePacket(v) => {
                println!("{:?}",v);
            }
            _ => todo!(),
        }
        Ok(())
    }
    pub async fn send_packet<T: Into<PacketKind>>(&mut self, packet: T) -> Result<()> {
        let packet: PacketKind = packet.into();
        println!("[S=>C]{}", packet);
        let bind = framer::encode(packet, self.get_status().encryption_enabled)?;
        let buffer = self.encrypt_or(&bind)?;
        self.socket
            .send(
                &[vec![0xfe], buffer].concat(),
                rust_raknet::Reliability::Reliable,
            )
            .await
            .map_err(|e| anyhow!("FailedToSendPacket:{:?}", e))?;
        Ok(())
    }
    async fn send_network_setting(&mut self) -> Result<()> {
        let network_setting = NetworkSettingsPacket {
            compression_threshold: 512,
            compression_algorithm: CompressionAlgorithmType::Deflate,
            client_throttle: false,
            client_throttle_threshold: 0,
            client_throttle_scalar: 0.0,
        };
        self.send_packet(network_setting).await?;
        Ok(())
    }
    async fn login_handshake(&mut self, login: &LoginPacket) -> Result<()> {
        let (key, data) = verify_login(&login.identity)?;
        let skin_data = verify_skin_data(&key, &login.client)?;

        let (secret, token) = key_exchange::shared_secret(&key)?;
        let iv: [u8; 16] = [secret[0..12].to_vec(), vec![0, 0, 0, 2]]
            .concat()
            .try_into()
            .unwrap();

        self.send_packet(ServerToClientHandshakePacket { token })
            .await?;

        self.get_status_mut().encryption_enabled = true;
        self.get_status_mut().ss_key = Some(secret.clone());
        self.setup_cipher(&secret, &iv)?;

        self.get_world_mut().insert(
            self.entity,
            (
                DeviceOS::from(skin_data.DeviceOS),
                PlayerName {
                    xuid: data.XUID,
                    client_uuid: data.identity,
                    user_name: data.displayName,
                },
            ),
        );
        Ok(())
    }
}
