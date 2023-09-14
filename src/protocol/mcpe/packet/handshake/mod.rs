pub mod constants;
pub mod key_exchange;

use crate::packet_feature;
use anyhow::Result;
use protodef::prelude::*;

#[derive(Debug)]
pub struct ServerToClientHandshake {
    pub token: String,
}

impl ServerToClientHandshake {
    pub fn read_to_buffer(&self, vec: &mut Vec<u8>) -> Result<()> {
        vec.write_string(self.token.clone())?;
        Ok(())
    }
}
packet_feature!(
    ServerToClientHandshake,
    3,
    "server_to_client_handshake_packet"
);

#[derive(Debug)]
pub struct ClientToServerHandshake();

packet_feature!(
    ClientToServerHandshake,
    4,
    "client_to_server_handshake_packet"
);
