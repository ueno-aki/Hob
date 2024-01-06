use bytes::{Buf,BytesMut};
use proto_bytes::ConditionalReader;

use super::Packet;

#[derive(Debug)]
pub struct LoginPacket {
    pub protocol_version: i32,
    pub identity: String,
    pub client: String,
}
impl Packet for LoginPacket {
    #[inline]
    fn from_bytes(bytes: &mut BytesMut) -> anyhow::Result<Self> {
        let protocol_version = bytes.get_i32();
        let _payload = bytes.get_varint();
        let identity = bytes.get_string_lu32();
        let client = bytes.get_string_lu32();
        Ok(LoginPacket {protocol_version,identity,client})
    }

    fn read_to_bytes(&self,bytes: &mut BytesMut) -> anyhow::Result<()> {
        todo!()
    }
}