use crate::protocol::internal::packet::{CreateClient, InternalPacketKind, DestoryClient};
use crate::protocol::mcpe::packet::{PacketKind, RequestNetworkSetting};
use crate::protocol::mcpe::transforms::framer;

use anyhow::Result;
use rand::Rng;
use rust_raknet::RaknetSocket;
use tokio::sync::mpsc::Sender;
use std::fmt::Display;

pub struct Player {
    id: u64,
    socket: RaknetSocket,
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
            socket,
            id: rand::thread_rng().gen(),
            status: PlayerStatus::default(),
        }
    }
    pub async fn listen(&mut self,sender:Sender<InternalPacketKind>) -> Result<()> {
        let create_cl = CreateClient {client_id:self.id};
        sender.send(create_cl.into()).await?;
        while let Ok(buffer) = self.socket.recv().await {
            self.handle(buffer,sender.clone()).await?;
        }
        let destory_cl = DestoryClient {client_id:self.id};
        sender.send(destory_cl.into()).await?;
        println!("disconnected,{}", self);
        Ok(())
    }
    async fn handle(&mut self, buffer: Vec<u8>,sender:Sender<InternalPacketKind>) -> Result<()> {
        let raw_pkts = framer::decode(buffer)?;
        for pkt in raw_pkts {
            let packet = framer::parse_packet(pkt)?;
            match packet {
                PacketKind::RequestNetworkSetting(pkt) => {
                    if RequestNetworkSetting::is_current_protocol(pkt.client_protocol)? {
                        println!("valid client_protocol");
                    } else {
                        println!("invalid client_protocol")
                    }
                },
                _ => todo!()
            }
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct PlayerStatus {
    encryption_enabled: bool,
}
