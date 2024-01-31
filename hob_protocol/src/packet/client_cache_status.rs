use proto_bytes::ConditionalBuf;

use super::Packet;

#[derive(Debug)]
pub struct ClientCacheStatusPacket {
    pub enabled: bool,
}

impl Packet for ClientCacheStatusPacket {
    fn decode(bytes: &mut proto_bytes::BytesMut) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let enabled = bytes.get_bool();
        Ok(ClientCacheStatusPacket { enabled })
    }

    fn encode(&self, _bytes: &mut proto_bytes::BytesMut) -> anyhow::Result<()> {
        todo!()
    }
}
