use crate::packet_id;
use anyhow::Result;
use protodef::prelude::*;

pub mod constants;
pub mod login_verify;
pub mod errors;

#[derive(Debug)]
pub struct Login {
    pub protocol_version: i32,
    pub identity: String,
    pub client: String,
}

impl Login {
    pub fn from_buf(buffer: Vec<u8>, offset: u64) -> Result<Self> {
        let mut cursor = offset;
        let protocol_version = buffer.read_i32(cursor);
        cursor += 4;
        let (_payload, payload_size) = buffer.read_varint(cursor)?;
        cursor += payload_size;
        let (identity, identity_size) = buffer.read_little_string(cursor)?;
        cursor += identity_size;
        let (client, _client_size) = buffer.read_little_string(cursor)?;
        Ok(Login {
            protocol_version,
            identity,
            client,
        })
    }
}
packet_id!(Login, 1);
