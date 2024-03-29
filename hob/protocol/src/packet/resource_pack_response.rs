use from_num::from_num;
use proto_bytes::{Buf, ConditionalBuf};

use super::Packet;

#[derive(Debug)]
pub struct ResourcePackClientResponsePacket {
    pub response_status: ResponseStatus,
    pub resourcepack_ids: Vec<String>,
}

impl Packet for ResourcePackClientResponsePacket {
    fn decode(bytes: &mut proto_bytes::BytesMut) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let response_status = ResponseStatus::from_u8(bytes.get_u8())?;
        let len = bytes.get_i16_le();
        let mut resourcepack_ids = Vec::new();
        for _ in 0..len {
            let id = bytes.get_string_varint();
            resourcepack_ids.push(id);
        }
        Ok(ResourcePackClientResponsePacket {
            response_status,
            resourcepack_ids,
        })
    }

    fn encode(&self, _bytes: &mut proto_bytes::BytesMut) -> anyhow::Result<()> {
        todo!()
    }
}

#[derive(Debug)]
#[from_num(u8)]
pub enum ResponseStatus {
    None,
    Refused,
    SendPacks,
    HaveAllPacks,
    Completed,
}
