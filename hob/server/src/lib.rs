pub mod connection_client;
pub mod initial_handler;
pub mod listener;
pub mod logging;
pub mod player_registry;

use anyhow::{anyhow, Result};
use listener::Listener;
use std::{fmt::Debug, sync::Arc};
use tokio::{
    runtime::Runtime,
    sync::mpsc::{self, Receiver},
};

use player_registry::PlayerRegistry;

#[derive(Debug)]
pub struct Server {
    pub player_registry: Receiver<PlayerRegistry>,
}
impl Server {
    pub async fn create(runtime: Arc<Runtime>) -> Result<Self> {
        let (player_registry_tx, player_registry_rx) = mpsc::channel(32);
        Listener::start(Arc::clone(&runtime), player_registry_tx).await?;
        Ok(Server {
            player_registry: player_registry_rx,
        })
    }
    pub fn accept_players(&mut self, max: usize) -> Vec<PlayerRegistry> {
        let mut players = Vec::with_capacity(max);
        for _ in 0..max {
            match self.player_registry.try_recv() {
                Ok(player) => players.push(player),
                Err(mpsc::error::TryRecvError::Empty) => break,
                Err(e) => {
                    log::error!("Error receiving player: {:?}", e);
                    break;
                }
            }
        }
        players
    }
}

#[inline]
pub(crate) fn into_anyhow(input: impl Debug) -> anyhow::Error {
    anyhow!("{:?}", input)
}
