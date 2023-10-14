use anyhow::Result;
use protodef::prelude::*;

use super::{Packet, PacketKind};

#[derive(Debug)]
pub struct NetworkSettingsPacket {
    pub compression_threshold: u16,
    pub compression_algorithm: CompressionAlgorithmType,
    pub client_throttle: bool,
    pub client_throttle_threshold: u8,
    pub client_throttle_scalar: f32,
}

impl Packet for NetworkSettingsPacket {
    fn read_to_buffer(&self, vec: &mut Vec<u8>) -> Result<()> {
        vec.write_u16(self.compression_threshold)?;
        vec.write_u16(self.compression_algorithm.clone() as u16)?;
        vec.write_bool(self.client_throttle)?;
        vec.write_u8(self.client_throttle_threshold)?;
        vec.write_lf32(self.client_throttle_scalar)?;
        Ok(())
    }
    fn from_buf(_buffer: &[u8], _offset: usize) -> Result<PacketKind>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub enum CompressionAlgorithmType {
    Deflate,
    Snappy,
}
