pub mod components;

use specs::{world, WorldExt};

use self::components::{
    connection::{ConnectionAddressComponent, ConnectionStreamComponent},
    DisplayNameComponent, XUIDComponent,
};

pub(crate) fn init_player(world: &mut world::World, dispatcher: &mut specs::DispatcherBuilder) {
    world.register::<DisplayNameComponent>();
    world.register::<XUIDComponent>();
    world.register::<ConnectionStreamComponent>();
    world.register::<ConnectionAddressComponent>();
}

pub(crate) fn handle_player(world: &world::World) {}
