use proto_bytes::{Buf, BytesMut, ConditionalReader};

use super::Packet;

#[derive(Debug)]
pub struct LoginPacket {
    pub protocol_version: i32,
    pub identity: String,
    pub client: String,
}
impl Packet for LoginPacket {
    #[inline]
    fn decode(bytes: &mut BytesMut) -> anyhow::Result<Self> {
        let protocol_version = bytes.get_i32();
        let _ = bytes.get_varint();
        let identity = bytes.get_string_lu32();
        let client = bytes.get_string_lu32();
        Ok(LoginPacket {
            protocol_version,
            identity,
            client,
        })
    }

    fn encode(&self, bytes: &mut BytesMut) -> anyhow::Result<()> {
        todo!()
    }
}
