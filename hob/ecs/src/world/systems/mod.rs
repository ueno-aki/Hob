use hob_protocol::packet::login::ExtraUserdata;
use hob_server::{player_registry::PlayerRegistry, Server};
use log::info;
use specs::prelude::*;

use crate::{
    player::components::{
        connection::{ConnectionAddressComponent, ConnectionStreamComponent},
        DisplayNameComponent, XUIDComponent,
    },
    world::components::RuntimeIdComponent,
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
            address,
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
            count.0 += 1;
            updater.create_entity(&entities).with(ConnectionStreamComponent::new(
                packet_from_client,
                packet_to_client,
                &display_name,
            ))
            .with(ConnectionAddressComponent(address))
            .with(RuntimeIdComponent(count.0))
            .with(XUIDComponent(xuid))
            .with(DisplayNameComponent(display_name))
            .build();
        }
    }
}
