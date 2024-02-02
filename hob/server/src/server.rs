use std::sync::Arc;

use rust_raknet::{RaknetListener, RaknetSocket};
use specs::{World, WorldExt};
use tokio::runtime::Runtime;
use crate::client::Client;

pub struct Server {
    world: Arc<World>,
    runtime: Arc<Runtime>
}
impl Server {
    pub fn new(runtime:Arc<Runtime>) -> Self {
        let world = World::new();
        Server { world: Arc::new(world),runtime}
    }
    pub async fn listen(&self) {
        let mut listener = RaknetListener::bind(&"0.0.0.0:19132".parse().unwrap())
            .await
            .unwrap();
        listener.listen().await;
        while let Ok(socket) = listener.accept().await {
            let runtime = Arc::clone(&self.runtime);
            self.runtime.spawn(async move {
                let mut client = Client::new(socket,runtime);
                if let Err(e) = client.listen().await {
                    println!("{:?}", e);
                }
            });
        }
    }
}

pub struct Listener {
    listener:RaknetListener,
    runtime: Arc<Runtime>
}
impl Listener {
    pub async fn start(runtime: Arc<Runtime>) {
        let mut listener = RaknetListener::bind(&"0.0.0.0:19132".parse().unwrap())
            .await
            .unwrap();
        listener.listen().await;
        let listener = Listener { listener, runtime: Arc::clone(&runtime) };
        runtime.spawn(async move {
            listener.run().await;
        });
    }
    async fn run(mut self) {
        loop {
            if let Ok(stream) = self.listener.accept().await {
                
            }
        }
    }

    fn accept(&mut self,stream:RaknetSocket) {
        let worker = Worker::new(stream, Arc::clone(&self.runtime));
    }
}
pub struct Worker {
    stream:RaknetSocket,
    runtime:Arc<Runtime>
}
impl Worker {
    pub fn new(stream:RaknetSocket, runtime:Arc<Runtime>) -> Self {
        Self { stream, runtime }
    }
}