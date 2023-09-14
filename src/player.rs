use crate::protocol::internal::packet::{CreateClient, DestoryClient, InternalPacketKind};
use crate::protocol::mcpe::packet::login_verify::verify_auth;
use crate::protocol::mcpe::packet::{
    CompressionAlgorithmType, NetworkSettings, PacketKind, PlayStatus,
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
        write!(f, "{{id:{},status:{:?}}}", self.id, self.status)
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
        let raw_pkts = framer::decode(buffer)?;
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
                PacketKind::Login(pkt) => verify_auth(&pkt.identity)?,
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
}

#[derive(Debug, Default, Clone)]
pub struct PlayerStatus {
    encryption_enabled: bool,
}
