use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

// use hob_server::server::Server;
use tokio::runtime::Builder;

use hob_server::{logging, server::Server};

fn main() {
    logging::setup(logging::LevelFilter::Debug);
    logging::info!("Sever Stated!");
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
        let server = Server::init(Arc::clone(&runtime)).await.unwrap();
    });
    // let server = Server::new(Arc::clone(&runtime));
    // runtime.block_on(async {
    //     println!("Server Started");
    //     server.listen().await;
    // })
}
