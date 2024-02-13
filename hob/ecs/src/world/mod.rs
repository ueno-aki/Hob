use specs::{world, WorldExt};

use self::components::RuntimeIdComponent;

pub mod components;
pub mod resources;
pub mod systems;

pub fn init_world(world: &mut specs::World, dispatcher: &mut specs::DispatcherBuilder) {
    world.register::<RuntimeIdComponent>();
    world.insert(resources::EntityCountResource::default());
    dispatcher.add(systems::AcceptNewPlayer, "accept_new_player", &[]);
}

pub(crate) fn handle_world(world: &mut world::World) {}
