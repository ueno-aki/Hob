use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use anyhow::{Ok, Result};
use log::info;
use server_repaired::{logging, Server};
use tokio::runtime::Builder;

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
        let mut server = Server::create(Arc::clone(&runtime)).await.unwrap();

        info!("Server Created");
        loop {
            if let Some(player) = server.player_registry.recv().await {
                info!("Player Joined {:?}", player.user);
            }
        }
    });

    Ok(())
}
