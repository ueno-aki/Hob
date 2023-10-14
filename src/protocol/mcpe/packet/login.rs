use anyhow::Result;
use protodef::prelude::*;

use super::{Packet, PacketKind};

pub mod constants;
pub mod errors;
pub mod login_verify;

#[derive(Debug)]
pub struct LoginPacket {
    pub protocol_version: i32,
    pub identity: String,
    pub client: String,
}

impl Packet for LoginPacket {
    fn from_buf(buffer: &[u8], offset: usize) -> Result<PacketKind>
    where
        Self: Sized,
    {
        let mut cursor = offset;
        let protocol_version = buffer.read_i32(cursor);
        cursor += 4;
        let (_payload, payload_size) = buffer.read_varint(cursor)?;
        cursor += payload_size;
        let (identity, identity_size) = buffer.read_little_string(cursor)?;
        cursor += identity_size;
        let (client, _client_size) = buffer.read_little_string(cursor)?;
        let packet = LoginPacket {
            protocol_version,
            identity,
            client,
        };
        Ok(PacketKind::LoginPacket(packet))
    }
    fn read_to_buffer(&self, _vec: &mut Vec<u8>) -> Result<()> {
        unimplemented!()
    }
}
