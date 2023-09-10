use crate::{packet_id, utils::get_option};
use anyhow::Result;
use protodef::prelude::*;

#[derive(Debug)]
pub struct RequestNetworkSetting {
    pub client_protocol: i32,
}

impl RequestNetworkSetting {
    pub fn from_buf(buffer: Vec<u8>, offset: u64) -> Result<Self> {
        let client_protocol = buffer.read_i32(offset);
        Ok(RequestNetworkSetting { client_protocol })
    }
}
packet_id!(RequestNetworkSetting, 193);
