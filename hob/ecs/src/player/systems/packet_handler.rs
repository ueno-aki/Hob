use hob_protocol::packet::{
    play_status::PlayStatusPacket, resource_pack_info::ResourcePacksInfoPacket,
    resource_pack_response::ResponseStatus, resource_pack_stack::ResourcePacksStackPacket,
    PacketKind,
};
use log::info;
use specs::prelude::*;
use tokio::sync::mpsc::error::TryRecvError;

use crate::{
    player::components::{ConnectionStreamComponent, DisplayNameComponent},
    world::components::EntityRuntimeIdComponent,
};

pub(crate) fn handle_packet(world: &mut World) {
    let mut conns = world.write_storage::<ConnectionStreamComponent>();
    let display = world.read_storage::<DisplayNameComponent>();
    let entities = world.entities();
    (&mut conns, &display, &entities)
        .par_join()
        .for_each(|(conn, display, ent)| {
            let packets = conn.try_recv_many_packets(32);
            match packets {
                Ok(packets) => {
                    for packet in packets {
                        match_packets(conn, packet, world, ent);
                    }
                }
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => {
                    info!("Client disconnected: {}", display.0);
                    entities.delete(ent).unwrap();
                }
            }
        });
}

fn match_packets(
    conn: &mut ConnectionStreamComponent,
    packet: PacketKind,
    world: &World,
    ent: Entity,
) {
    match packet {
        PacketKind::ClientToServerHandshake(_) => {
            conn.send_packet(PlayStatusPacket::LoginSuccess);
            let resource_info = ResourcePacksInfoPacket::default();
            conn.send_packet(resource_info);
        }

        PacketKind::ResourcePackClientResponse(v) => match v.response_status {
            ResponseStatus::None | ResponseStatus::Refused => {}
            ResponseStatus::SendPacks => {}
            ResponseStatus::HaveAllPacks => {
                let mut res_stack = ResourcePacksStackPacket::default();
                res_stack.add_experiment("gametest", true);
                conn.send_packet(res_stack);
            }
            ResponseStatus::Completed => {
                let runtime_id = world.read_component::<EntityRuntimeIdComponent>();
                let runtime_id = runtime_id.get(ent).unwrap();
                log::debug!(
                    "Resource pack response completed for entity {}",
                    runtime_id.0
                );
            }
        },
        _ => {}
    }
}
