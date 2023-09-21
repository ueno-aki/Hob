use std::sync::Arc;

use crate::{
    components::{DeviceOS, PlayerName, Position},
    player::Player,
    utils::get_option,
};
use anyhow::{anyhow, Result};
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use rust_raknet::RaknetListener;
use sparsey::prelude::*;

pub struct Server {
    world: Arc<AtomicRefCell<World>>,
}

impl Server {
    #[inline]
    fn get_world(&self) -> AtomicRef<World> {
        self.world.borrow()
    }
    #[inline]
    fn get_world_mut(&self) -> AtomicRefMut<World> {
        self.world.borrow_mut()
    }

    pub fn new() -> Self {
        let mut world = World::default();
        world.register::<Position>();
        world.register::<DeviceOS>();
        world.register::<PlayerName>();
        Server {
            world: Arc::new(AtomicRefCell::new(world)),
        }
    }

    pub async fn launch(&mut self) -> Result<()> {
        println!("Server Started");
        let addr = format!("0.0.0.0:{}", get_option("port")?);
        let mut listener = RaknetListener::bind(&addr.parse()?)
            .await
            .map_err(|_| anyhow!("Failed to bind RaknetListener"))?;

        listener
            .set_full_motd(Self::load_motd()?)
            .map_err(|_| anyhow!("Failed to set full motd"))?;
        listener.listen().await;
        
        let world = self.world.clone();
        tokio::spawn(async move {
            let world = world.clone();
            while let Ok(socket) = listener.accept().await {
                let world = world.clone();
                tokio::spawn(async move {
                    let entity = world.borrow_mut().create(());
                    let mut player = Player::new(socket, entity, world);
                    player.listen().await.unwrap();
                });
            }
        }).await.unwrap();
        Ok(())
    }

    fn load_motd() -> Result<String> {
        let motd = format!(
            "MCPE;{};{};{};{};{};{};{};Survival;1;{}",
            get_option("server_name")?,
            get_option("protocol")?,
            get_option("version")?,
            0,
            get_option("max_connection")?,
            rand::random::<u64>(),
            get_option("description")?,
            get_option("port")?
        );
        Ok(motd)
    }
}
