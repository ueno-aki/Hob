use hob_protocol::packet::login::ExtraUserdata;
use hob_server::{player_registry::PlayerRegistry, Server};
use log::info;
use specs::prelude::*;

use crate::{
    player::components::{ConnectionStreamComponent, DisplayNameComponent, XUIDComponent},
    world::components::EntityRuntimeIdComponent,
};

use super::resources::EntityCountResource;

pub struct AcceptNewPlayer;

impl<'a> System<'a> for AcceptNewPlayer {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, Server>,
        Write<'a, EntityCountResource>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (entities, mut server, mut count, updater): Self::SystemData) {
        for PlayerRegistry {
            packet_from_client,
            packet_to_client,
            user,
            ..
        } in server.accept_players(32)
        {
            let ExtraUserdata {
                display_name, xuid, ..
            } = user;
            info!("Player connected: {display_name}, xuid:{xuid}");
            let entity = entities.create();
            count.0 += 1;
            updater.insert(
                entity,
                ConnectionStreamComponent::new(packet_from_client, packet_to_client, &display_name),
            );
            updater.insert(entity, EntityRuntimeIdComponent(count.0));
            updater.insert(entity, XUIDComponent(xuid));
            updater.insert(entity, DisplayNameComponent(display_name));
        }
    }
}
