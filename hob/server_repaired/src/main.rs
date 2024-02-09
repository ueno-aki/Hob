use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use anyhow::Result;
use hob_protocol::packet::{
    play_status::PlayStatusPacket, resource_pack_info::ResourcePacksInfoPacket,
    resource_pack_response::ResponseStatus, PacketKind,
};
use log::{info, warn};
use server_repaired::{client::Client, logging, Server};
use specs::prelude::*;
use tokio::{runtime::Builder, sync::mpsc::error::TryRecvError, time::Instant};

pub const TPS: u32 = 20;
pub const TICK_MILLIS: u32 = 1000 / TPS;
pub const TICK_DURATION: Duration = Duration::from_millis(TICK_MILLIS as u64);

fn main() -> Result<()> {
    logging::setup(log::LevelFilter::Debug);
    let runtime = Arc::new(
        Builder::new_multi_thread()
            .enable_all()
            .thread_name_fn(|| {
                static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
                let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
                format!("hob-pool-{}", id)
            })
            .build()
            .unwrap(),
    );
    runtime.block_on(async {
        let server = Server::create(runtime.clone()).await.unwrap();
        info!("Server Created");

        let mut world = World::new();
        world.register::<Client>();
        world.insert(server);
        world.insert(EntityCount(0));

        let mut dispatcher = DispatcherBuilder::new()
            .with(AcceptNewPlayer, "accept_new_player", &[])
            .with(PacketHandler, "packet_handler", &[])
            .build();
        loop {
            let start = Instant::now();
            dispatcher.dispatch(&world);
            world.maintain();
            let elapsed = start.elapsed();
            if elapsed <= TICK_DURATION {
                tokio::time::sleep(TICK_DURATION - elapsed).await;
            } else {
                warn!("Tick took too long: {:?}", elapsed - TICK_DURATION)
            }
        }
    });
    Ok(())
}

#[derive(Debug, Default)]
struct EntityCount(u64);

struct AcceptNewPlayer;
impl<'a> System<'a> for AcceptNewPlayer {
    type SystemData = (
        WriteExpect<'a, Server>,
        Write<'a, EntityCount>,
        WriteStorage<'a, Client>,
        Entities<'a>,
    );

    fn run(&mut self, (mut server, mut count, mut clients, entities): Self::SystemData) {
        while let Ok(player) = server.player_registry.try_recv() {
            info!(
                "Player connected: {}, xuid:{}",
                player.user.display_name, player.user.xuid
            );
            let client = Client::new(player);
            let entity = entities.create();
            clients.insert(entity, client).unwrap();
            count.0 += 1;
        }
    }
}

struct PacketHandler;
impl<'a> System<'a> for PacketHandler {
    type SystemData = (WriteStorage<'a, Client>, Entities<'a>);

    fn run(&mut self, (mut clients, entities): Self::SystemData) {
        (&mut clients, &entities)
            .par_join()
            .for_each(|(client, ent)| loop {
                match client.try_recv_packet() {
                    Ok(packet) => {
                        handle_packet(client, packet);
                    }
                    Err(TryRecvError::Disconnected) => {
                        entities.delete(ent).unwrap();
                        break;
                    }
                    Err(TryRecvError::Empty) => {
                        break;
                    }
                }
            })
    }
}
fn handle_packet(client: &mut Client, packet: PacketKind) {
    match packet {
        PacketKind::ClientToServerHandshake(_) => {
            client
                .try_send_packet(PlayStatusPacket::LoginSuccess)
                .unwrap();
            let resource_info = ResourcePacksInfoPacket {
                must_accept: false,
                has_scripts: false,
                force_server_packs: false,
                behaviour_packs: vec![],
                texture_packs: vec![],
                resource_pack_links: vec![],
            };
            client.try_send_packet(resource_info).unwrap();
        }
        PacketKind::ResourcePackClientResponse(v) => match v.response_status {
            ResponseStatus::None | ResponseStatus::Refused => {}
            ResponseStatus::SendPacks => {}
            ResponseStatus::HaveAllPacks => {}
            ResponseStatus::Completed => {}
        },
        _ => {}
    }
}
