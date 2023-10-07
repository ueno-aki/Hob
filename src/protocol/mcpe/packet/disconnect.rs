use anyhow::Result;
use protodef::prelude::*;

use crate::packet_ids;

#[derive(Debug)]
pub struct DisconnectPacket {
    pub hide_disconnect_reason: bool,
    pub message: String,
}

impl DisconnectPacket {
    pub fn read_to_buffer(&self, vec: &mut Vec<u8>) -> Result<()> {
        vec.write_bool(self.hide_disconnect_reason)?;
        vec.write_string(&self.message)?;
        Ok(())
    }
}
packet_ids!(DisconnectPacket, 5, "disconnect_packet");
