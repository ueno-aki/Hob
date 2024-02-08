use std::sync::Arc;

use anyhow::Result;
use rust_raknet::{RaknetListener, RaknetSocket};
use tokio::{runtime::Runtime, sync::mpsc::Sender};

use crate::{connection_client::ConnectionClient, into_anyhow, player_init::PlayerRegistry};

pub struct Listener {
    listener: RaknetListener,
    player_registry: Sender<PlayerRegistry>,
    runtime: Arc<Runtime>,
}
impl Listener {
    pub async fn start(
        runtime: Arc<Runtime>,
        player_registry: Sender<PlayerRegistry>,
    ) -> Result<()> {
        let mut listener = RaknetListener::bind(&"0.0.0.0:19132".parse()?)
            .await
            .map_err(into_anyhow)?;
        listener.listen().await;

        let listener = Listener {
            listener,
            player_registry,
            runtime: Arc::clone(&runtime),
        };
        runtime.spawn(async move {
            listener.run().await;
        });
        Ok(())
    }
    async fn run(mut self) {
        loop {
            if let Ok(socket) = self.listener.accept().await {
                self.accept(socket).await;
            }
        }
    }
    async fn accept(&mut self, socket: RaknetSocket) {
        let connection =
            ConnectionClient::new(socket, self.player_registry.clone(), self.runtime.clone());
        connection.start();
    }
}
