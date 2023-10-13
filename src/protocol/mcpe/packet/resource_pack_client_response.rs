use crate::packet_ids;
use anyhow::Result;
use from_num::from_num;
use protodef::prelude::*;

#[derive(Debug)]
pub struct ResourcePackClientResponsePacket {
    pub response_status: ResponseStatus,
    pub resourcepack_ids: Vec<String>,
}

impl ResourcePackClientResponsePacket {
    pub fn from_buf(buffer: &[u8], offset: usize) -> Result<Self> {
        let mut cursor = offset;
        let response_status = ResponseStatus::from(buffer.read_u8(cursor));
        cursor += 1;
        let mut ids_length = buffer.read_li16(cursor);
        cursor += 2;
        let mut resourcepack_ids = Vec::new();
        while ids_length > 0 {
            let (id, id_size) = buffer.read_string(cursor)?;
            resourcepack_ids.push(id);
            cursor += id_size;
            ids_length -= 1;
        }
        Ok(Self {
            response_status,
            resourcepack_ids,
        })
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

packet_ids!(
    ResourcePackClientResponsePacket,
    8,
    "resource_pack_client_response_packet"
);
