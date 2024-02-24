use self::player_join::PlayerJoinEvent;
use crate::plugin::Plugin;
use specs::prelude::*;
pub mod player_join;

pub fn init_events(world: &mut specs::World, dispatcher: &mut specs::DispatcherBuilder) {
    world.insert::<Plugin<PlayerJoinEvent>>(Plugin::new());
}

pub(super) fn handle_events(world: &World) {
    player_join::accept_new_player(world);
}
