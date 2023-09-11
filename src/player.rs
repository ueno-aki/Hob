use crate::protocol::internal::packet::{CreateClient, InternalPacketKind, DestoryClient};
use crate::protocol::mcpe::packet::{PacketKind, RequestNetworkSetting, PlayStatus};
use crate::protocol::mcpe::transforms::framer;
use crate::utils::get_option;

use anyhow::{Result, anyhow};
use atomic_refcell::{AtomicRefCell, AtomicRef, AtomicRefMut};
use rand::Rng;
use rust_raknet::RaknetSocket;
use tokio::sync::mpsc::Sender;
use std::fmt::Display;
use std::sync::Arc;

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
            socket:Arc::new(AtomicRefCell::new(socket)),
            id: rand::thread_rng().gen(),
            status: PlayerStatus::default(),
        }
    }
    #[inline]
    pub fn get_socket(&self) ->AtomicRef<RaknetSocket>{
        self.socket.borrow()
    }
    #[inline]
    pub fn get_socket_mut(&self) ->AtomicRefMut<RaknetSocket>{
        self.socket.borrow_mut()
    }
    pub async fn listen(&mut self,tx:Sender<InternalPacketKind>) -> Result<()> {
        let create_cl = CreateClient {client_id:self.id};
        tx.send(create_cl.into()).await?;
        while let Ok(buffer) = self.clone().get_socket().recv().await {
            self.handle(buffer,tx.clone()).await?;
        }
        let destory_cl = DestoryClient {client_id:self.id};
        tx.send(destory_cl.into()).await?;
        println!("disconnected,{}", self);
        Ok(())
    }
    async fn handle(&mut self, buffer: Vec<u8>,tx:Sender<InternalPacketKind>) -> Result<()> {
        let raw_pkts = framer::decode(buffer)?;
        for pkt in raw_pkts {
            let packet = framer::parse_packet(pkt)?;
            match packet {
                PacketKind::RequestNetworkSetting(pkt) => {
                    let current_p = get_option("protocol")?.parse::<i32>()?;
                    match pkt.client_protocol {
                        x if x > current_p => self.send_packet(PlayStatus::FailedSpawn.into()).await?,
                        x if x < current_p => self.send_packet(PlayStatus::FailedClient.into()).await?,
                        _ => todo!()
                    };
                },
                _ => todo!()
            }
        }
        Ok(())
    }
    #[inline]
    async fn send_packet(&self,packet:PacketKind) -> Result<()>{
        let buffer = framer::encode(packet)?;
        self.get_socket()
            .send(&[vec![0xfe],buffer].concat(), rust_raknet::Reliability::ReliableOrdered)
            .await
            .map_err(|e|anyhow!("{:?}",e))?;
        Ok(())
    }
}

#[derive(Debug, Default,Clone)]
pub struct PlayerStatus {
    encryption_enabled: bool,
}
