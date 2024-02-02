// use std::sync::{atomic::{AtomicUsize, Ordering}, Arc};

// use hob_server::server::Server;
// use tokio::runtime::Builder;

fn main() {
    // let runtime = Arc::new(
    //     Builder::new_multi_thread()
    //         .enable_all()
    //         .worker_threads((num_cpus::get() / 4).max(2 /*MIN_RECOMMENDED_TOKIO_THREADS*/))
    //         .thread_name_fn(|| {
    //             static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
    //             let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
    //             format!("hob-pool-{}", id)
    //         })
    //         .build()
    //         .unwrap()
    //     );
    // let server = Server::new(Arc::clone(&runtime));
    // runtime.block_on(async {
    //     println!("Server Started");
    //     server.listen().await;
    // })
}
