use std::sync::Arc;

use crate::{
    ecs::{
        components::{DeviceOS, PlayerName, Position, RunTimeID},
        resources::EntityCount,
    },
    player::Player,
    utils::get_option,
};
use anyhow::Result;
use atomic_refcell::AtomicRefCell;
use rust_raknet::RaknetListener;
use specs::{Builder, World, WorldExt};

pub struct Server {
    world: Arc<AtomicRefCell<World>>,
}

impl Server {
    pub fn new() -> Self {
        let mut world = World::new();
        world.register::<Position>();
        world.register::<DeviceOS>();
        world.register::<PlayerName>();
        world.register::<RunTimeID>();
        world.insert(EntityCount::default());
        Server {
            world: Arc::new(AtomicRefCell::new(world)),
        }
    }

    pub async fn launch(&mut self) -> Result<()> {
        println!("Server Started");
        let addr = format!("0.0.0.0:{}", get_option("port")?);
        let mut listener = RaknetListener::bind(&addr.parse()?).await.unwrap();

        listener.set_full_motd(Self::load_motd()?).unwrap();
        listener.listen().await;
        let world = self.world.clone();
        tokio::spawn(async move {
            while let Ok(socket) = listener.accept().await {
                let world = world.clone();
                tokio::spawn(async move {
                    let count = Self::increment_entity_count(world.clone());
                    let entity = world
                        .try_borrow_mut()
                        .unwrap()
                        .create_entity()
                        .with(RunTimeID { id: count })
                        .build();
                    let mut player = Player::new(socket, entity, world);
                    player.listen().await.unwrap();
                });
            }
        })
        .await
        .unwrap();
        Ok(())
    }

    fn increment_entity_count(world: Arc<AtomicRefCell<World>>) -> u64 {
        let binding = world.try_borrow().unwrap();
        let mut entity_count_res = binding.write_resource::<EntityCount>();
        entity_count_res.count += 1;
        entity_count_res.count
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
