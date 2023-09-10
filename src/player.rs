use crate::components::ClientId;
use crate::protocol::packet::{PacketKind, RequestNetworkSetting};
use crate::protocol::transforms::framer;

use anyhow::{Context, Result};
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use rand::Rng;
use rust_raknet::RaknetSocket;
use sparsey::prelude::*;
use sparsey::world::{Comp, World};
use tokio::sync::mpsc::Sender;
use std::fmt::Display;
use std::sync::Arc;

pub struct Player {
    id: u64,
    socket: RaknetSocket,
    world: Arc<AtomicRefCell<World>>,
    status: PlayerStatus,
}
impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{id:{},status:{:?}}}", self.id, self.status)
    }
}

impl Player {
    pub fn new(socket: RaknetSocket, world: Arc<AtomicRefCell<World>>) -> Self {
        Player {
            socket,
            world,
            id: rand::thread_rng().gen(),
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

    pub async fn listen(&mut self,sender:Sender<i32>) -> Result<()> {
        self.get_world_mut().create((ClientId { id: self.id },));
        while let Ok(buffer) = self.socket.recv().await {
            self.handle(buffer,sender.clone()).await?;
        }
        self.destory_self_entity()?;
        println!("disconnected,{}", self);
        Ok(())
    }
    async fn handle(&mut self, buffer: Vec<u8>,sender:Sender<i32>) -> Result<()> {
        let raw_pkts = framer::decode(buffer)?;
        for pkt in raw_pkts {
            let packet = framer::parse_packet(pkt)?;
            match packet {
                PacketKind::RequestNetworkSetting(pkt) => {
                    if RequestNetworkSetting::is_current_protocol(pkt.client_protocol)? {
                        println!("valid client_protocol");
                        sender.send(123).await?;
                    } else {
                        println!("invalid client_protocol")
                    }
                },
                _ => todo!()
            }
        }
        Ok(())
    }

    fn destory_self_entity(&self) -> Result<()> {
        let mut me: Option<Entity> = None;
        self.get_world().run(|clients: Comp<ClientId>| {
            (&clients).for_each_with_entity(|(e, cl)| {
                if cl.id == self.id {
                    me = Some(e);
                }
            });
        });
        let me = me.context(format!(
            "Failed to get {}'s socket",
            self.socket.peer_addr().unwrap()
        ))?;
        
        self.get_world_mut().destroy(me);
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct PlayerStatus {
    encryption_enabled: bool,
}
