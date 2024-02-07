use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use anyhow::{Ok, Result};
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
        let server = Server::create(Arc::clone(&runtime)).await?;

        tokio::time::sleep(std::time::Duration::from_millis(10000)).await;
        Ok(())
    })?;
    println!("Hello World");
    Ok(())
}