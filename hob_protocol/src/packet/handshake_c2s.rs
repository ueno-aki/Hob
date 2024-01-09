use super::Packet;

#[derive(Debug)]
pub struct ClientToServerHandshakePacket;

impl Packet for ClientToServerHandshakePacket {
    #[inline]
    fn decode(_bytes: &mut proto_bytes::BytesMut) -> anyhow::Result<Self> {
        Ok(ClientToServerHandshakePacket)
    }

    fn encode(&self, _bytes: &mut proto_bytes::BytesMut) -> anyhow::Result<()> {
        todo!()
    }
}
