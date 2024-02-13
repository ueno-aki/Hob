pub mod components;
pub mod systems;

use specs::{world, WorldExt};

use self::{
    components::{ConnectionStreamComponent, DisplayNameComponent},
    systems::PacketHandler,
};

pub fn init_player(world: &mut world::World, dispatcher: &mut specs::DispatcherBuilder) {
    world.register::<DisplayNameComponent>();
    world.register::<ConnectionStreamComponent>();
    dispatcher.add(PacketHandler, "packet_handler", &[]);
}
