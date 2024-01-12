use anyhow::{Result, ensure, anyhow};
use base64::prelude::*;
use proto_bytes::{Buf, BytesMut, ConditionalReader};
use serde::Deserialize;

use crate::jwt::ES384PublicKey;

use super::Packet;

#[derive(Deserialize)]
pub struct IdentityClaim {
    #[serde(rename(deserialize = "extraData"))]
    extra_data: Option<ExtraUserdata>,
    #[serde(rename(deserialize = "identityPublicKey"))]
    identity_public_key: String,
}

#[derive(Deserialize)]
pub struct ExtraUserdata {
    #[serde(rename(deserialize = "XUID"))]
    pub xuid: String,
    pub identity: String,
    #[serde(rename(deserialize = "displayName"))]
    pub display_name: String,
    #[serde(rename(deserialize = "titleId"))]
    pub title_id: String,
    #[serde(rename(deserialize = "sandboxId"))]
    pub sandbox_id: String,
}

pub fn verify_login(identity:&str) -> Result<(String, ExtraUserdata)> {
    const MOJANG_PUBKEY: &str = "MHYwEAYHKoZIzj0CAQYFK4EEACIDYgAECRXueJeTDqNRRgJi/vlRufByu/2G0i2Ebt6YMar5QX/R0DIIyrJMcUpruK4QveTfJSTp3Shlq4Gk34cD/4GUWwkv0DVuzeuB+tXija7HBxii03NHDbPAD0AKnLr2wdAp";
    #[derive(Deserialize)]
    struct AuthChain {
        chain: Vec<String>,
    }
    let chain = serde_json::from_str::<AuthChain>(identity)?.chain;
    ensure!(chain.len() == 3,"InvalidChainLength:{}",identity);
    let mut verified = false;
    let mut user_data = None;
    let mut next_pubkey = None;
    for token in chain.iter() {
        let header = ES384PublicKey::decode_header(token)?;
        let key_der = match next_pubkey {
            Some(v) => BASE64_URL_SAFE_NO_PAD.decode(&v)?,
            None => BASE64_URL_SAFE_NO_PAD.decode(&header.x5u)?,
        };
        let key = ES384PublicKey::from_der(&key_der)?;
        let claim = key.verify_token::<IdentityClaim>(token)?;
        if header.x5u == MOJANG_PUBKEY {
            verified = true;
        }
        if claim.extra_data.is_some() {
            user_data = claim.extra_data;
        }
        next_pubkey = Some(claim.identity_public_key);
    }
    ensure!(verified,"NotAuthenticated");
    match user_data {
        Some(data) => Ok((next_pubkey.unwrap(),data)),
        None => Err(anyhow!("ExtraUserdataNotFound"))
    }
}

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
