use hob_protocol::packet::PacketKind;
use log::info;
use specs::prelude::*;
use tokio::sync::mpsc::error::TryRecvError;

use crate::{
    player::components::connection::ConnectionStreamComponent,
    plugin::Plugin,
    world::components::RuntimeIdComponent,
};

pub struct PacketRecvEvent {
    pub entity: Entity,
    pub packet: PacketKind,
}
impl PacketRecvEvent {
    pub fn new(entity: Entity, packet: PacketKind) -> Self {
        PacketRecvEvent { entity, packet }
    }
}

pub(super) fn recv_packet(world: &World) {
    let mut conns = world.write_storage::<ConnectionStreamComponent>();
    let mut packet_ev = world.write_resource::<Plugin<PacketRecvEvent>>();
    let entities = world.entities();
    let mut evs:Vec<PacketRecvEvent> = Vec::new();
    (&mut conns, &entities)
        .join()
        .for_each(|(conn, ent)| {
            let packets = conn.try_recv_many_packets(32);
            if let Err(e) = packets {
                if e == TryRecvError::Disconnected {
                    info!("Client disconnected: {}", conn.name);
                    entities.delete(ent).unwrap();
                }
                return;
            }
            for packet in packets.unwrap() {
                let ev = PacketRecvEvent::new(ent, packet);
                evs.push(ev);
            }
        });
    drop(conns);

    for ev in evs {
        if packet_ev.run(&ev, world) {
            continue;
        }
        handle_packets(ev.packet, world, ev.entity)
    }
}

fn handle_packets(
    packet: PacketKind,
    world: &World,
    ent: Entity,
) {
    use hob_protocol::packet::{
        play_status::PlayStatusPacket, resource_pack_info::ResourcePacksInfoPacket,
        resource_pack_response::ResponseStatus, resource_pack_stack::ResourcePacksStackPacket,
    };
    let mut conns = world.write_storage::<ConnectionStreamComponent>();
    let conn = conns.get_mut(ent).unwrap();
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
                let runtime_id = world.read_component::<RuntimeIdComponent>();
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
