pub mod constants;
pub mod key_exchange;

use anyhow::Result;
use protodef::prelude::*;

use super::{Packet, PacketKind};

#[derive(Debug)]
pub struct ServerToClientHandshakePacket {
    pub token: String,
}

impl Packet for ServerToClientHandshakePacket {
    fn from_buf(_buffer: &[u8], _offset: usize) -> Result<PacketKind>
    where
        Self: Sized,
    {
        unimplemented!()
    }
    fn read_to_buffer(&self, vec: &mut Vec<u8>) -> Result<()> {
        vec.write_string(&self.token)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct ClientToServerHandshakePacket();

impl Packet for ClientToServerHandshakePacket {
    fn from_buf(_buffer: &[u8], _offset: usize) -> Result<PacketKind> {
        Ok(PacketKind::ClientToServerHandshakePacket(
            ClientToServerHandshakePacket(),
        ))
    }
    fn read_to_buffer(&self, _vec: &mut Vec<u8>) -> Result<()> {
        unimplemented!()
    }
}
