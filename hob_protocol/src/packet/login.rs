use anyhow::{anyhow, ensure, Result};
use base64::prelude::*;
use proto_bytes::{Buf, BytesMut, ConditionalReader};
use serde::Deserialize;

use crate::jwt::ES384PublicKey;

use super::Packet;

#[derive(Debug, Deserialize)]
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

pub fn verify_login(identity: &str) -> Result<(String, ExtraUserdata)> {
    const MOJANG_PUBKEY: &str = "MHYwEAYHKoZIzj0CAQYFK4EEACIDYgAECRXueJeTDqNRRgJi/vlRufByu/2G0i2Ebt6YMar5QX/R0DIIyrJMcUpruK4QveTfJSTp3Shlq4Gk34cD/4GUWwkv0DVuzeuB+tXija7HBxii03NHDbPAD0AKnLr2wdAp";
    #[derive(Deserialize)]
    struct AuthChain {
        chain: Vec<String>,
    }
    #[derive(Deserialize)]
    struct IdentityClaim {
        #[serde(rename(deserialize = "extraData"))]
        extra_data: Option<ExtraUserdata>,
        #[serde(rename(deserialize = "identityPublicKey"))]
        identity_public_key: String,
    }

    let chain = serde_json::from_str::<AuthChain>(identity)?.chain;
    ensure!(chain.len() == 3, "InvalidChainLength:{}", identity);
    let mut verified = false;
    let mut user_data = None;
    let mut next_pubkey = None;
    for token in chain.iter() {
        let header = ES384PublicKey::decode_header(token)?;
        let key_der = match next_pubkey {
            Some(v) => BASE64_STANDARD.decode(v)?,
            None => BASE64_STANDARD.decode(&header.x5u)?,
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
    ensure!(verified, "NotAuthenticated");
    match user_data {
        Some(data) => Ok((next_pubkey.unwrap(), data)),
        None => Err(anyhow!("ExtraUserdataNotFound")),
    }
}

pub fn verify_skin(public_key: &str, client: &str) -> Result<SkinData> {
    let key = ES384PublicKey::from_der(&BASE64_STANDARD.decode(public_key)?)?;
    key.verify_token(client)
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SkinData {
    pub animated_image_data: Vec<AnimatedImageDataType>,
    pub arm_size: String,
    pub cape_data: String,
    pub cape_id: String,
    pub cape_image_height: u64,
    pub cape_image_width: u64,
    pub cape_on_classic_skin: bool,
    pub client_random_id: u64,
    pub compatible_with_client_side_chunk_gen: bool,
    pub current_input_mode: u8,
    pub default_input_mode: u8,
    pub device_id: String,
    pub device_model: String,
    #[serde(rename(deserialize = "DeviceOS"))]
    pub device_os: u8,
    pub game_version: String,
    pub gui_scale: i8,
    pub is_editor_mode: bool,
    pub language_code: String,
    pub override_skin: bool,
    pub persona_pieces: Vec<PersonaPiecesType>,
    pub persona_skin: bool,
    pub piece_tint_colors: Vec<PieceTintColorsType>,
    pub platform_offline_id: String,
    pub platform_online_id: String,
    pub play_fab_id: String,
    pub premium_skin: bool,
    pub self_signed_id: String,
    pub server_address: String,
    pub skin_animation_data: String,
    pub skin_color: String,
    pub skin_data: String,
    pub skin_geometry_data: String,
    pub skin_geometry_data_engine_version: String,
    pub skin_id: String,
    pub skin_image_height: u64,
    pub skin_image_width: u64,
    pub skin_resource_patch: String,
    pub third_party_name: String,
    pub third_party_name_only: bool,
    pub trusted_skin: bool,
    #[serde(rename(deserialize = "UIProfile"))]
    pub uiprofile: u8,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AnimatedImageDataType {
    pub animation_expression: u64,
    pub frames: f64,
    pub image: String,
    pub image_height: u64,
    pub image_width: u64,
    #[serde(rename(deserialize = "Type"))]
    pub t_ype: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PersonaPiecesType {
    pub is_default: bool,
    pub pack_id: String,
    pub piece_id: String,
    pub piece_type: String,
    pub product_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PieceTintColorsType {
    pub colors: Vec<String>,
    pub piece_type: String,
}
