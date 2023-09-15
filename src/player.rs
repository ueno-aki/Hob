use crate::protocol::internal::packet::{CreateClient, DestoryClient, InternalPacketKind};
use crate::protocol::mcpe::crypto::cipher::{Aes256CtrCipherManager, Cipher};
use crate::protocol::mcpe::packet::login_verify::{verify_login, verify_skin_data};
use crate::protocol::mcpe::packet::{
    key_exchange, CompressionAlgorithmType, Disconnect, NetworkSettings, PacketKind, PlayStatus,
    ServerToClientHandshake,
};
use crate::protocol::mcpe::transforms::framer;
use crate::utils::get_option;

use aes::cipher::StreamCipher;
use anyhow::{anyhow, Result};
use atomic_refcell::{AtomicRef, AtomicRefCell};
use rand::Rng;
use rust_raknet::RaknetSocket;
use std::fmt::Display;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

#[derive(Clone)]
pub struct Player {
    pub id: u64,
    pub socket: Arc<AtomicRefCell<RaknetSocket>>,
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
        write!(f, "{{id:{}}}", self.id)
    }
}

impl Player {
    pub fn new(socket: RaknetSocket) -> Self {
        Player {
            socket: Arc::new(AtomicRefCell::new(socket)),
            id: rand::thread_rng().gen(),
            status: PlayerStatus::default(),
        }
    }
    #[inline]
    pub fn get_socket(&self) -> AtomicRef<RaknetSocket> {
        self.socket.borrow()
    }
    pub async fn listen(&mut self, tx: Sender<InternalPacketKind>) -> Result<()> {
        let create_cl = CreateClient { client_id: self.id };
        tx.send(create_cl.into()).await?;
        while let Ok(buffer) = self.clone().get_socket().recv().await {
            self.handle(buffer, tx.clone()).await.unwrap();
        }
        let destory_cl = DestoryClient { client_id: self.id };
        tx.send(destory_cl.into()).await?;
        println!("disconnected,{}", self);
        Ok(())
    }
    async fn handle(&mut self, buffer: Vec<u8>, tx: Sender<InternalPacketKind>) -> Result<()> {
        let raw_pkts = framer::decode(self.decrypt_or(buffer[1..].to_vec()))?;
        for pkt in raw_pkts {
            let packet = framer::parse_packet(pkt)?;
            println!("[C=>S]{}", packet);
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
                    let iv: [u8; 16] = [secret.clone()[0..12].to_vec(), vec![0, 0, 0, 2]]
                        .concat()
                        .try_into()
                        .unwrap();
                    self.send_packet(ServerToClientHandshake { token }).await?;
                    self.status.encryption_enabled = true;
                    self.status.ss_key = Some(secret.clone());
                    self.setup_cipher(secret, iv)?;
                }
                PacketKind::ClientToServerHandshake(_) => {
                    self.send_packet(Disconnect {
                        message: "bye".to_owned(),
                        hide_disconnect_reason: false,
                    })
                    .await?;
                }
                _ => todo!(),
            }
        }
        Ok(())
    }
    async fn send_packet<T: Into<PacketKind>>(&mut self, packet: T) -> Result<()> {
        let packet: PacketKind = packet.into();
        println!("[S=>C]{}", packet);
        let bind = framer::encode(packet, self.status.encryption_enabled)?;
        let buffer = self.encrypt_or(bind);
        self.get_socket()
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
            result = [result.clone(), self.compute_packet_tag(result)].concat();
            self.status
                .cipher
                .as_mut()
                .unwrap()
                .apply_keystream(&mut result);
            self.status.send_counter += 1;
        }
        result
    }
    fn compute_packet_tag(&self, plain_pkt: Vec<u8>) -> Vec<u8> {
        let mut digest = hmac_sha256::Hash::new();
        digest.update(self.status.send_counter.to_be_bytes());
        digest.update(plain_pkt);
        digest.update(self.status.ss_key.unwrap());
        let result = digest.finalize();
        result[0..8].to_vec()
    }
}
