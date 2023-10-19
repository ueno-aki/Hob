use anyhow::Result;
use protodef::prelude::*;

use super::{Packet, PacketKind};

#[derive(Debug)]
pub struct ClientCacheStatusPacket {
    pub enabled: bool,
}

impl Packet for ClientCacheStatusPacket {
    fn from_buf(buffer: &[u8], offset: usize) -> Result<PacketKind> {
        let (enabled, _) = buffer.read_bool(offset)?;
        Ok(PacketKind::ClientCacheStatusPacket(
            ClientCacheStatusPacket { enabled },
        ))
    }
    fn read_to_buffer(&self, _vec: &mut Vec<u8>) -> Result<()> {
        unimplemented!()
    }
}
