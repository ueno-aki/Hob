use anyhow::{Ok, Result};
use hob_ecs::{events::player_join::PlayerJoinEvent, plugin::PluginSys, Game};
use hob_server::{logging, Server};
use log::info;
use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::{runtime::Builder, time::Instant};

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
        pub const TPS: u32 = 20;
        pub const TICK_MILLIS: u32 = 1000 / TPS;
        pub const TICK_DURATION: Duration = Duration::from_millis(TICK_MILLIS as u64);

        let server = Server::create(Arc::clone(&runtime)).await.unwrap();
        let mut game = Game::new(server);
        game.add_plugin(HelloWorld);
        info!("Server Created");
        loop {
            let start = Instant::now();
            game.handle();
            let elapsed = start.elapsed();
            if elapsed <= TICK_DURATION {
                tokio::time::sleep(TICK_DURATION - elapsed).await;
            } else {
                log::warn!("Tick took too long: {:?}", elapsed - TICK_DURATION)
            }
        }
    });
    Ok(())
}

pub struct HelloWorld;
impl<'a> PluginSys<'a, PlayerJoinEvent> for HelloWorld {
    type SystemData = ();
    fn run(&mut self, event: &'a PlayerJoinEvent, _data: Self::SystemData) -> bool {
        info!("Hello, {}! Welcome to the server!", event.user.display_name);
        false
    }
}
