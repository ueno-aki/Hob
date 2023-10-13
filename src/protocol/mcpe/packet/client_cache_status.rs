use crate::packet_ids;
use anyhow::Result;
use protodef::prelude::*;

#[derive(Debug)]
pub struct ClientCacheStatusPacket {
    pub enabled: bool,
}

impl ClientCacheStatusPacket {
    pub fn from_buf(buffer: &[u8], offset: usize) -> Result<Self> {
        let (enabled, _) = buffer.read_bool(offset)?;
        Ok(ClientCacheStatusPacket { enabled })
    }
}
packet_ids!(ClientCacheStatusPacket, 129, "client_cache_status_packet");
