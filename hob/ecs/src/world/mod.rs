use specs::{world, WorldExt};

use self::components::RuntimeIdComponent;

pub mod components;
pub mod resources;

pub fn init_world(world: &mut specs::World, dispatcher: &mut specs::DispatcherBuilder) {
    world.register::<RuntimeIdComponent>();
    world.insert(resources::EntityCountResource::default());
}

pub(crate) fn handle_world(world: &world::World) {}
