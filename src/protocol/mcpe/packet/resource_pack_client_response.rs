use anyhow::Result;
use from_num::from_num;
use protodef::prelude::*;

use super::{Packet, PacketKind};

#[derive(Debug)]
pub struct ResourcePackClientResponsePacket {
    pub response_status: ResponseStatus,
    pub resourcepack_ids: Vec<String>,
}

impl Packet for ResourcePackClientResponsePacket {
    fn from_buf(buffer: &[u8], offset: usize) -> Result<PacketKind> {
        let mut cursor = offset;
        let response_status = ResponseStatus::from_u8(buffer.read_u8(cursor))?;
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
        Ok(PacketKind::ResourcePackClientResponsePacket(Self {
            response_status,
            resourcepack_ids,
        }))
    }
    fn read_to_buffer(&self, _vec: &mut Vec<u8>) -> Result<()> {
        unimplemented!()
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
