use crate::protocol::internal::packet::{CreateClient, DestoryClient, InternalPacketKind};
use crate::protocol::mcpe::crypto::cipher::{Cipher, Aes256Ctr64BE};
use crate::protocol::mcpe::packet::login_verify::{verify_login, verify_skin_data};
use crate::protocol::mcpe::packet::{
    CompressionAlgorithmType, NetworkSettings, PacketKind, PlayStatus, key_exchange, ServerToClientHandshake,
};
use crate::protocol::mcpe::transforms::framer;
use crate::utils::get_option;

use anyhow::{anyhow, Result};
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use rand::Rng;
use rust_raknet::RaknetSocket;
use std::fmt::Display;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

#[derive(Clone)]
pub struct Player {
    id: u64,
    socket: Arc<AtomicRefCell<RaknetSocket>>,
    status: PlayerStatus,
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
    #[inline]
    pub fn get_socket_mut(&self) -> AtomicRefMut<RaknetSocket> {
        self.socket.borrow_mut()
    }
    pub async fn listen(&mut self, tx: Sender<InternalPacketKind>) -> Result<()> {
        let create_cl = CreateClient { client_id: self.id };
        tx.send(create_cl.into()).await?;
        while let Ok(buffer) = self.clone().get_socket().recv().await {
            self.handle(buffer, tx.clone()).await?;
        }
        let destory_cl = DestoryClient { client_id: self.id };
        tx.send(destory_cl.into()).await?;
        println!("disconnected,{}", self);
        Ok(())
    }
    async fn handle(&mut self, buffer: Vec<u8>, tx: Sender<InternalPacketKind>) -> Result<()> {
        let raw_pkts = framer::decode(buffer,&self.status.encryption_enabled,&mut self.status.decipher)?;
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
                    let (secret,token) = key_exchange::shared_secret(&key)?;
                    self.send_packet(ServerToClientHandshake {token}).await?;
                    self.status.encryption_enabled = true;
                    println!("{secret:?}");
                    self.setup_cipher(secret)?;
                },
                _ => todo!(),
            }
        }
        Ok(())
    }
    async fn send_packet<T: Into<PacketKind>>(&self, packet: T) -> Result<()> {
        let packet: PacketKind = packet.into();
        println!("[S=>C]{}", packet);
        let buffer = framer::encode(packet)?;
        self.get_socket()
            .send(
                &[vec![0xfe], buffer].concat(),
                rust_raknet::Reliability::ReliableOrdered,
            )
            .await
            .map_err(|e| anyhow!("FailedToSendPacket:{:?}", e))?;
        Ok(())
    }
    async fn send_network_setting(&self) -> Result<()> {
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

    fn setup_cipher(&mut self,key:[u8;32]) -> Result<()>{
            use aes::cipher::KeyIvInit;
            match (&self.status.cipher,&self.status.decipher) {
            (None,None) => {
                let iv = [key.clone()[0..12].to_vec(),vec![0,0,0,2]].concat();
                let iv:[u8;16] = iv.try_into().unwrap();
                self.status.cipher = Some(Aes256Ctr64BE::new(&key.into(),&iv.into()));
                self.status.decipher = Some(Aes256Ctr64BE::new(&key.into(),&iv.into()));
                Ok(())
            }
            _ => panic!("")
        }
    }
}

#[derive(Default,Clone)]
pub struct PlayerStatus {
    encryption_enabled: bool,
    cipher: Option<Cipher>,
    decipher: Option<Cipher>,
}
