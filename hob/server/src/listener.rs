use std::sync::Arc;

use anyhow::{anyhow, Result};
use flume::Sender;
use rust_raknet::{RaknetListener, RaknetSocket};
use tokio::runtime::Runtime;

use crate::{connection_worker::Worker, player_init::NewPlayer};

pub struct Listener {
    listener: RaknetListener,
    new_players: Sender<NewPlayer>,
    runtime: Arc<Runtime>,
}
impl Listener {
    pub async fn start(runtime: Arc<Runtime>, new_players: Sender<NewPlayer>) -> Result<()> {
        let mut listener = RaknetListener::bind(&"0.0.0.0:19132".parse()?)
            .await
            .map_err(|e| anyhow!("{e:?}"))?;
        listener.listen().await;
        let listener = Listener {
            listener,
            runtime: Arc::clone(&runtime),
            new_players,
        };
        runtime.spawn(async move {
            listener.run().await;
        });
        Ok(())
    }
    async fn run(mut self) {
        loop {
            if let Ok(stream) = self.listener.accept().await {
                self.accept(stream).await;
            }
        }
    }

    async fn accept(&mut self, stream: RaknetSocket) {
        let worker = Worker::new(stream, Arc::clone(&self.runtime), self.new_players.clone());
        worker.start();
    }
}
