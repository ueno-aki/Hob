use self::{packet_recv::PacketRecvEvent, player_join::PlayerJoinEvent};
use crate::plugin::Plugin;
use specs::prelude::*;
pub mod packet_recv;
pub mod player_join;

pub fn init_events(world: &mut specs::World, dispatcher: &mut specs::DispatcherBuilder) {
    world.insert::<Plugin<PlayerJoinEvent>>(Plugin::new());
    world.insert::<Plugin<PacketRecvEvent>>(Plugin::new());
}

pub(super) fn handle_events(world: &World) {
    player_join::accept_new_player(world);
    packet_recv::recv_packet(world);
}
