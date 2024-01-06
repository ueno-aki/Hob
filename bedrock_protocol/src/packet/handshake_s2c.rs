use proto_bytes::ConditionalWriter;

use super::Packet;

#[derive(Debug)]
pub struct ServerToClientHandshakePacket {
    pub token: String,
}

impl Packet for ServerToClientHandshakePacket {
    fn from_bytes(bytes: &mut bytes::BytesMut) -> anyhow::Result<Self> {
        todo!()
    }

    #[inline]
    fn read_to_bytes(&self,bytes: &mut bytes::BytesMut) -> anyhow::Result<()> {
        bytes.put_string_varint(&self.token);
        Ok(())
    }
}
