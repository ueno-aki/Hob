use std::sync::Arc;

use crate::{listener::Listener, player_init::NewPlayer};
use anyhow::Result;
use flume::Receiver;
use tokio::runtime::Runtime;

pub struct Server {
    pub runtime: Arc<Runtime>,
    pub new_players: Receiver<NewPlayer>,
}

impl Server {
    pub async fn init(runtime: Arc<Runtime>) -> Result<Self> {
        let (tx, rx) = flume::bounded(4);
        Listener::start(Arc::clone(&runtime), tx).await?;
        log::info!("Server created");
        Ok(Self {
            runtime,
            new_players: rx,
        })
    }
}
