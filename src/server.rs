use crate::{
    components::{ClientId, Position},
    player::Player,
    utils::get_option, protocol::internal::packet::InternalPacketKind,
};
use anyhow::{anyhow, Result};
use atomic_refcell::AtomicRefCell;
use rust_raknet::RaknetListener;
use sparsey::prelude::*;
use tokio::sync::mpsc;
use std::sync::Arc;

pub struct Server {
    world: Arc<AtomicRefCell<World>>,
}

impl Server {
    pub fn new() -> Self {
        let mut world = World::default();
        world.register::<ClientId>();
        world.register::<Position>();
        Server {
            world: Arc::new(AtomicRefCell::new(world)),
        }
    }

    pub async fn launch(&mut self) -> Result<()> {
        println!("Server Started");
        let mut listener = RaknetListener::bind(&"0.0.0.0:19132".parse().unwrap())
            .await
            .map_err(|_| anyhow!("Failed to bind RaknetListener"))?;

        listener
            .set_full_motd(Self::load_motd()?)
            .map_err(|_| anyhow!("Failed to set full motd"))?;
        listener.listen().await;
        let (tx,mut rx) = mpsc::channel(32);
        tokio::spawn(async move {
            while let Some(v) = rx.recv().await {
                println!("receive::{:?}",v)
            }
        });
        while let Ok(socket) = listener.accept().await {
            let tx_c = tx.clone();
            let w_clone = self.world.clone();
            tokio::spawn(async move {
                let mut player = Player::new(socket, w_clone);
                player.listen(tx_c).await.unwrap();
            });
        }
        Ok(())
    }
    fn load_motd() -> Result<String> {
        let motd = format!(
            "MCPE;{};{};{};{};{};{};{};Survival;1;{}",
            get_option("server_name")?,
            get_option("protocol")?,
            get_option("version")?,
            100,
            get_option("max_connection")?,
            rand::random::<u64>(),
            get_option("description")?,
            get_option("port")?
        );
        Ok(motd)
    }
}
