pub mod player;
pub mod world;
pub use specs::WorldExt;

use hob_server::Server;
use player::{handle_player, init_player};
use specs::prelude::*;
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
        Game { world, dispatcher: dispatcher.build() }
    }
    pub fn handle(&mut self) {
        self.dispatcher.dispatch(&self.world);
        handle_player(&mut self.world);
        handle_world(&mut self.world);
        self.world.maintain();
    }
}