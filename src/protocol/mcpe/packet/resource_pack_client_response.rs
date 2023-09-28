use crate::packet_ids;
use anyhow::{Result, anyhow};
use protodef::prelude::*;

#[derive(Debug)]
pub struct ResourcePackClientResponsePacket {
    response_status:ResponseStatus,
    resourcepack_ids:Vec<String>,
}

impl ResourcePackClientResponsePacket {
    pub fn from_buf(buffer: Vec<u8>, offset: usize)  -> Result<Self> {
        let mut cursor = offset;
        let response_status = ResponseStatus::from_u8(buffer.read_u8(cursor))?;
        cursor += 1;
        let mut ids_length = buffer.read_li16(cursor);
        cursor += 2;
        let mut resourcepack_ids = Vec::new();
        while ids_length > 0 {
            let (id,id_size) = buffer.read_string(cursor)?;
            resourcepack_ids.push(id);
            cursor += id_size;
            ids_length -= 1;
        }
        Ok(Self { response_status, resourcepack_ids })
    }
}

#[derive(Debug)]
pub enum ResponseStatus {
    None,
    Refused,
    SendPacks,
    HaveAllPacks,
    Completed
}
impl ResponseStatus {
    pub fn from_u8(value:u8) -> Result<Self>{
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Refused),
            2 => Ok(Self::SendPacks),
            3 => Ok(Self::HaveAllPacks),
            4 => Ok(Self::Completed),
            _ => Err(anyhow!("Failed to convert into ResponseStatus"))
        }
    }
}

packet_ids!(
    ResourcePackClientResponsePacket,
    8,
    "resource_pack_client_response_packet"
);
