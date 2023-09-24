use crate::packet_feature;
use anyhow::Result;
use protodef::prelude::*;

#[derive(Debug)]
pub struct RequestNetworkSettingPacket {
    pub client_protocol: i32,
}

impl RequestNetworkSettingPacket {
    pub fn from_buf(buffer: Vec<u8>, offset: u64) -> Result<Self> {
        let client_protocol = buffer.read_i32(offset);
        Ok(RequestNetworkSettingPacket { client_protocol })
    }
}
packet_feature!(
    RequestNetworkSettingPacket,
    193,
    "request_network_settings_packet"
);
