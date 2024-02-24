pub mod events;
pub mod player;
pub mod plugin;
pub mod world;

use plugin::{Plugin, PluginSys};
pub use specs::prelude::*;

use events::{handle_events, init_events};
use hob_server::Server;
use player::{handle_player, init_player};
use world::{handle_world, init_world};

pub struct Game {
    world: World,
    dispatcher: Dispatcher<'static, 'static>,
}
impl Game {
    pub fn new(server: Server) -> Self {
        let mut world = World::new();
        world.insert(server);
        let mut dispatcher = DispatcherBuilder::new();
        init_player(&mut world, &mut dispatcher);
        init_world(&mut world, &mut dispatcher);
        init_events(&mut world, &mut dispatcher);
        Game {
            world,
            dispatcher: dispatcher.build(),
        }
    }
    pub fn handle(&mut self) {
        self.dispatcher.dispatch(&self.world);
        handle_player(&self.world);
        handle_world(&self.world);
        handle_events(&self.world);
        self.world.maintain();
    }

    pub fn add_plugin<T, E: Send + Sync + 'static>(&mut self, plugin: T)
    where
        T: for<'a> PluginSys<'a, E> + Send + Sync + 'static,
    {
        self.world.write_resource::<Plugin<E>>().add_plugin(plugin);
    }
}
