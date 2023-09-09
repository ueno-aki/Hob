use crate::components::ClientId;
use crate::protocol::transforms::framer;

use anyhow::{Context, Result};
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use rand::Rng;
use rust_raknet::RaknetSocket;
use sparsey::prelude::*;
use sparsey::world::{Comp, World};
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

    pub async fn listen(&mut self) -> Result<()> {
        self.get_world_mut().create((ClientId { id: self.id },));
        loop {
            match self.socket.recv().await {
                Ok(buffer) => {
                    self.handle(buffer).await?;
                }
                Err(e) => {
                    println!("RakNetError:{:?}", e);
                    break;
                }
            }
        }
        self.destory_self_entity()?;
        println!("disconnected,{}", self);
        Ok(())
    }
    async fn handle(&mut self, buffer: Vec<u8>) -> Result<()> {
        let decoded = framer::decode(buffer)?;
        println!("{:?}", decoded);
        Ok(())
    }
    fn destory_self_entity(&self) -> Result<()> {
        let mut me: Option<Entity> = None;
        self.get_world_mut().run(|clients: Comp<ClientId>| {
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
