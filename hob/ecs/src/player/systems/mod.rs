use hob_protocol::packet::{
    play_status::PlayStatusPacket, resource_pack_info::ResourcePacksInfoPacket,
    resource_pack_response::ResponseStatus, resource_pack_stack::ResourcePacksStackPacket,
    PacketKind,
};
use log::info;
use specs::prelude::*;
use tokio::sync::mpsc::error::TryRecvError;

use super::components::{ConnectionStreamComponent, DisplayNameComponent};

pub struct PacketHandler;

impl<'a> System<'a> for PacketHandler {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, ConnectionStreamComponent>,
        ReadStorage<'a, DisplayNameComponent>,
    );

    fn run(&mut self, (entities, mut conns, display): Self::SystemData) {
        (&mut conns, &display, &entities)
            .par_join()
            .for_each(|(conn, display, ent)| {
                let packets = conn.try_recv_many_packets(100);
                match packets {
                    Ok(packets) => {
                        for packet in packets {
                            handle_packet(conn, packet);
                        }
                    }
                    Err(TryRecvError::Empty) => {}
                    Err(TryRecvError::Disconnected) => {
                        info!("Client disconnected: {}", display.0);
                        entities.delete(ent).unwrap();
                    }
                }
            })
    }
}

fn handle_packet(conn: &mut ConnectionStreamComponent, packet: PacketKind) {
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
            ResponseStatus::Completed => {}
        },
        _ => {}
    }
}
