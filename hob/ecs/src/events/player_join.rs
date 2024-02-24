use hob_protocol::packet::login::ExtraUserdata;
use hob_server::{player_registry::PlayerRegistry, Server};
use log::info;
use specs::prelude::*;

use crate::{
    player::components::{
        connection::{ConnectionAddressComponent, ConnectionStreamComponent},
        DisplayNameComponent, XUIDComponent,
    },
    plugin::Plugin,
    world::{components::RuntimeIdComponent, resources::EntityCountResource},
};

pub struct PlayerJoinEvent {
    pub entity: Entity,
    pub user: ExtraUserdata,
}
impl PlayerJoinEvent {
    pub fn new(entity: Entity, user: ExtraUserdata) -> Self {
        PlayerJoinEvent { entity, user }
    }
}

pub(super) fn accept_new_player(world: &World) {
    let mut server = world.write_resource::<Server>();
    let mut count = world.write_resource::<EntityCountResource>();
    let mut join_ev = world.write_resource::<Plugin<PlayerJoinEvent>>();
    let updater = world.read_resource::<LazyUpdate>();
    let entities = world.entities();
    for PlayerRegistry {
        address,
        packet_from_client,
        packet_to_client,
        user,
        ..
    } in server.accept_players(32)
    {
        let entity = updater.create_entity(&entities).build();
        let ev = PlayerJoinEvent::new(entity, user);
        if join_ev.run(&ev, world) {
            entities.delete(entity).unwrap();
            continue;
        }

        let ExtraUserdata {
            xuid, display_name, ..
        } = ev.user;
        count.0 += 1;
        info!("Player connected: {display_name}, xuid:{xuid}");
        updater.insert(
            entity,
            ConnectionStreamComponent::new(packet_from_client, packet_to_client, &display_name),
        );
        updater.insert(entity, ConnectionAddressComponent(address));
        updater.insert(entity, DisplayNameComponent(display_name));
        updater.insert(entity, XUIDComponent(xuid));
        updater.insert(entity, RuntimeIdComponent(count.0));
    }
}
