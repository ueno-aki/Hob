use crate::packet_ids;
use anyhow::{Result, anyhow};
use protodef::prelude::*;

#[derive(Debug)]
pub struct ResourcePackClientResponse {
    response_status:ResponseStatus,
    resourcepack_ids:Vec<String>,
}

// impl ResourcePackClientResponse {
//     pub fn from_buf(buffer: Vec<u8>, offset: u64)  -> Result<Self> {
//         let st = ResponseStatus::try_from(0).unwrap();    
//     }
// }

#[derive(Debug)]
pub enum ResponseStatus {
    None,
    Refused,
    SendPacks,
    HaveAllPacks,
    Completed
}
impl ResponseStatus {
    pub fn from_usize(value:usize) -> Result<Self>{
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
    ResourcePackClientResponse,
    8,
    "resource_pack_client_response_packet"
);
