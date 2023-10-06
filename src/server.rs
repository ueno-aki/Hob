use std::sync::Arc;

use crate::{
    ecs::{components::{DeviceOS, PlayerName, Position, RunTimeID}, resources::EntityCount},
    player::Player,
    utils::get_option,
};
use anyhow::Result;
use atomic_refcell::AtomicRefCell;
use rust_raknet::RaknetListener;
use specs::{World, WorldExt, Builder};

pub struct Server{
    world: Arc<AtomicRefCell<World>>
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
            world: Arc::new(AtomicRefCell::new(world))
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
            let world = world.clone();
            while let Ok(socket) = listener.accept().await {
                let world_c = world.clone();
                let binding = world.borrow();
                let mut entity_count_res = binding.write_resource::<EntityCount>();
                entity_count_res.count += 1;
                let count = entity_count_res.count;
                tokio::spawn(async move {
                    let entity = world_c.borrow_mut().create_entity().with(RunTimeID{id:count}).build();
                    let mut player = Player::new(socket, entity, world_c);
                    player.listen().await.unwrap();
                });
            }
        })
        .await
        .unwrap();
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
