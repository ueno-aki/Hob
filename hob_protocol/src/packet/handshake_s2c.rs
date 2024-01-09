use proto_bytes::ConditionalWriter;

use super::Packet;

#[derive(Debug)]
pub struct ServerToClientHandshakePacket {
    pub token: String,
}

impl Packet for ServerToClientHandshakePacket {
    fn decode(bytes: &mut proto_bytes::BytesMut) -> anyhow::Result<Self> {
        todo!()
    }

    #[inline]
    fn encode(&self, bytes: &mut proto_bytes::BytesMut) -> anyhow::Result<()> {
        bytes.put_string_varint(&self.token);
        Ok(())
    }
}
