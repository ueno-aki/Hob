use super::Packet;

#[derive(Debug)]
pub struct ClientToServerHandshakePacket;

impl Packet for ClientToServerHandshakePacket {
    #[inline]
    fn from_bytes(_bytes: &mut bytes::BytesMut) -> anyhow::Result<Self> {
        Ok(ClientToServerHandshakePacket)
    }

    fn read_to_bytes(&self,_bytes: &mut bytes::BytesMut) -> anyhow::Result<()> {
        todo!()
    }
}