use hob_protocol::packet::login::ExtraUserdata;
use log::info;
use hob_server::{player_init::PlayerRegistry, Server};
use specs::prelude::*;

use crate::{
    player::components::{ConnectionStreamComponent, DisplayNameComponent},
    world::components::EntityRuntimeIdComponent,
};

use super::resources::EntityCountResource;

pub struct AcceptNewPlayer;

impl<'a> System<'a> for AcceptNewPlayer {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, Server>,
        Write<'a, EntityCountResource>,
        WriteStorage<'a, EntityRuntimeIdComponent>,
        WriteStorage<'a, ConnectionStreamComponent>,
        WriteStorage<'a, DisplayNameComponent>,
    );

    fn run(
        &mut self,
        (entities, mut server, mut count, mut runtime_id, mut conns, mut display): Self::SystemData,
    ) {
        for PlayerRegistry {
            packet_from_client,
            packet_to_client,
            user,
            ..
        } in server.accept_players()
        {
            let ExtraUserdata {
                display_name, xuid, ..
            } = user;
            info!("Player connected: {display_name}, xuid:{xuid}");
            let entity = entities.create();
            count.0 += 1;
            conns
                .insert(
                    entity,
                    ConnectionStreamComponent::new(
                        packet_from_client,
                        packet_to_client,
                        &display_name,
                    ),
                )
                .unwrap();
            runtime_id
                .insert(entity, EntityRuntimeIdComponent(count.0))
                .unwrap();
            display
                .insert(entity, DisplayNameComponent(display_name))
                .unwrap();
        }
    }
}
