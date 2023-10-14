use anyhow::Result;
use protodef::prelude::*;

use super::{Packet, PacketKind};

#[derive(Debug)]
pub struct DisconnectPacket {
    pub hide_disconnect_reason: bool,
    pub message: String,
}

impl Packet for DisconnectPacket {
    fn from_buf(_buffer: &[u8], _offset: usize) -> Result<PacketKind> {
        unimplemented!()
    }
    fn read_to_buffer(&self, vec: &mut Vec<u8>) -> Result<()> {
        vec.write_bool(self.hide_disconnect_reason)?;
        vec.write_string(&self.message)?;
        Ok(())
    }
}
