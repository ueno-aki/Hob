use proto_bytes::{BufMut, ConditionalWriter};

use super::Packet;

#[derive(Debug)]
pub struct NetworkSettingsPacket {
    pub compression_threshold: u16,
    pub compression_algorithm: CompressionAlgorithmType,
    pub client_throttle: bool,
    pub client_throttle_threshold: u8,
    pub client_throttle_scalar: f32,
}

impl Packet for NetworkSettingsPacket {
    fn decode(_bytes: &mut proto_bytes::BytesMut) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn encode(&self, bytes: &mut proto_bytes::BytesMut) -> anyhow::Result<()> {
        bytes.put_u16(self.compression_threshold);
        bytes.put_u16(self.compression_algorithm.clone() as u16);
        bytes.put_bool(self.client_throttle);
        bytes.put_u8(self.client_throttle_threshold);
        bytes.put_f32_le(self.client_throttle_scalar);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum CompressionAlgorithmType {
    Deflate,
    Snappy,
}
