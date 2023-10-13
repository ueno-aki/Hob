use crate::packet_ids;
use anyhow::Result;
use protodef::prelude::*;

#[derive(Debug)]
pub struct RequestNetworkSettingPacket {
    pub client_protocol: i32,
}

impl RequestNetworkSettingPacket {
    pub fn from_buf(buffer: &[u8], offset: usize) -> Result<Self> {
        let client_protocol = buffer.read_i32(offset);
        Ok(RequestNetworkSettingPacket { client_protocol })
    }
}
packet_ids!(
    RequestNetworkSettingPacket,
    193,
    "request_network_settings_packet"
);
