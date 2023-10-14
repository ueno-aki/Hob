use anyhow::Result;
use protodef::prelude::*;

use super::{Packet, PacketKind};

#[derive(Debug)]
pub struct RequestNetworkSettingPacket {
    pub client_protocol: i32,
}

impl Packet for RequestNetworkSettingPacket {
    fn from_buf(buffer: &[u8], offset: usize) -> Result<PacketKind>
    where
        Self: Sized,
    {
        let client_protocol = buffer.read_i32(offset);
        Ok(PacketKind::RequestNetworkSettingPacket(
            RequestNetworkSettingPacket { client_protocol },
        ))
    }
    fn read_to_buffer(&self, _vec: &mut Vec<u8>) -> Result<()> {
        unimplemented!()
    }
}
