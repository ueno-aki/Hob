pub mod key_exchange;
pub mod constants;

use anyhow::Result;
use protodef::prelude::*;
use crate::packet_feature;

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
packet_feature!(ServerToClientHandshake, 3, "server_to_client_handshake_packet");
