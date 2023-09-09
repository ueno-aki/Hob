use crate::{
    components::{ClientId, Position},
    player::Player,
};
use anyhow::{anyhow, Context, Result};
use atomic_refcell::AtomicRefCell;
use rust_raknet::RaknetListener;
use sparsey::prelude::*;
use std::fs;
use std::sync::Arc;
use yaml_rust::{Yaml, YamlLoader};

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
        while let Ok(socket) = listener.accept().await {
            let w_clone = self.world.clone();
            tokio::spawn(async move {
                let mut player = Player::new(socket, w_clone);
                player.listen().await.unwrap();
            });
        }
        Ok(())
    }
    fn load_motd() -> Result<String> {
        let file = fs::read_to_string("./.server_properties")?;
        let options = &YamlLoader::load_from_str(&file)?[0];
        let motd = format!(
            "MCPE;{};{};{};{};{};{};{};Survival;1;{}",
            Self::get_option(options, "server_name")?,
            Self::get_option(options, "protocol")?,
            Self::get_option(options, "version")?,
            100,
            Self::get_option(options, "max_connection")?,
            rand::random::<u64>(),
            Self::get_option(options, "description")?,
            Self::get_option(options, "port")?
        );
        Ok(motd)
    }
    fn get_option(yaml: &Yaml, name: &str) -> Result<String> {
        Ok(yaml[name]
            .as_str()
            .context(format!("Failed to get option:{}", name))?
            .to_owned())
    }
}
