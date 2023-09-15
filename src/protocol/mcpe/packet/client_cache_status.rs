use crate::packet_feature;
use anyhow::Result;
use protodef::prelude::*;

#[derive(Debug)]
pub struct ClientCacheStatus {
    pub enabled: bool,
}

impl ClientCacheStatus {
    pub fn from_buf(buffer: Vec<u8>, offset: u64) -> Result<Self> {
        let (enabled, _) = buffer.read_bool(offset)?;
        Ok(ClientCacheStatus { enabled })
    }
}
packet_feature!(ClientCacheStatus, 129, "client_cache_status_packet");
