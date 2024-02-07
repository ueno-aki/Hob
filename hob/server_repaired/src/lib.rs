pub mod connection_client;
pub mod initial_handler;
pub mod listener;
pub mod logging;
pub mod player_init;

use anyhow::{anyhow, Result};
use listener::Listener;
use std::{fmt::Debug, sync::Arc};
use tokio::{
    runtime::Runtime,
    sync::mpsc::{self, Receiver},
};

use player_init::PlayerRegistry;

pub struct Server {
    pub runtime: Arc<Runtime>,
    pub player_registry: Receiver<PlayerRegistry>,
}
impl Server {
    pub async fn create(runtime: Arc<Runtime>) -> Result<Self> {
        let (player_registry_tx, player_registry_rx) = mpsc::channel(32);
        Listener::start(Arc::clone(&runtime), player_registry_tx).await?;
        Ok(Server {
            runtime,
            player_registry: player_registry_rx,
        })
    }
}

#[inline]
pub(crate) fn into_anyhow(input: impl Debug) -> anyhow::Error {
    anyhow!("{:?}", input)
}
