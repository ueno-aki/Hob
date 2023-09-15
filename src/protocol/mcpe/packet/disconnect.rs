use anyhow::Result;
use protodef::prelude::*;

use crate::packet_feature;

#[derive(Debug)]
pub struct Disconnect {
    pub hide_disconnect_reason: bool,
    pub message: String,
}

impl Disconnect {
    pub fn read_to_buffer(&self, vec: &mut Vec<u8>) -> Result<()> {
        vec.write_bool(self.hide_disconnect_reason)?;
        vec.write_string(self.message.clone())?;
        Ok(())
    }
}
packet_feature!(Disconnect, 5, "disconnect_packet");
