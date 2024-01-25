use proto_bytes::Buf;

use super::Packet;

#[derive(Debug)]
pub struct RequestNetworkSettingPacket {
    pub client_protocol: i32,
}

impl Packet for RequestNetworkSettingPacket {
    fn decode(bytes: &mut proto_bytes::BytesMut) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let client_protocol = bytes.get_i32();
        Ok(RequestNetworkSettingPacket { client_protocol })
    }

    fn encode(&self, _bytes: &mut proto_bytes::BytesMut) -> anyhow::Result<()> {
        todo!()
    }
}
