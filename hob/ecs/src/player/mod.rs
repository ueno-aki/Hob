pub mod components;
pub mod systems;

use specs::{world, WorldExt};

use self::{components::{ConnectionStreamComponent, DisplayNameComponent, XUIDComponent}, systems::packet_handler::handle_packet};

pub(crate) fn init_player(world: &mut world::World, dispatcher: &mut specs::DispatcherBuilder) {
    world.register::<DisplayNameComponent>();
    world.register::<XUIDComponent>();
    world.register::<ConnectionStreamComponent>();
}

pub(crate) fn handle_player(world: &mut world::World) {
    handle_packet(world);
}
