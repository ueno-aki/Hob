pub mod player;
pub mod world;
pub use specs::WorldExt;

use hob_server::Server;
use player::{handle_player, init_player};
use specs::prelude::*;
use world::{handle_world, init_world};

pub fn init_game(server: Server) -> (World, Dispatcher<'static, 'static>) {
    let mut world = World::new();
    world.insert(server);
    let mut dispatcher = DispatcherBuilder::new();
    init_player(&mut world, &mut dispatcher);
    init_world(&mut world, &mut dispatcher);
    (world, dispatcher.build())
}

// nodependency system call for the game
pub fn handle_game(world: &mut World) {
    handle_player(world);
    handle_world(world);
}