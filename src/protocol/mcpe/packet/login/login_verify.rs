#![allow(non_snake_case)]
use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::protocol::mcpe::{
    crypto::es384::ES384PublicKey,
    packet::login::{constants::MOJANG_PUBKEY, errors::LoginErrors},
};

#[derive(Serialize, Deserialize)]
struct AuthChain {
    chain: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LoginIdentityClaim {
    extraData: Option<ExtraUserdata>,
    identityPublicKey: String,
}

#[derive(Deserialize, Serialize)]
pub struct ExtraUserdata {
    pub XUID: String,
    pub identity: String,
    pub displayName: String,
    pub titleId: String,
    pub sandboxId: String,
}

pub fn verify_login(chains: &str) -> Result<(String, ExtraUserdata)> {
    let chains = serde_json::from_str::<AuthChain>(chains)?.chain;
    if chains.len() != 3 {
        return Err(LoginErrors::InvalidChainLength(chains.len()).into());
    }
    let mut public_key = ES384PublicKey::decode_header(&chains[0])?.x5u;
    let mut verified = false;
    let mut user_data = None;
    for chain in chains {
        let key = ES384PublicKey::from_der(&base64::decode(&public_key)?)?;
        let (header, claim) = key.verify_token::<LoginIdentityClaim>(&chain)?;
        if header.x5u == MOJANG_PUBKEY {
            verified = true;
        }
        if let Some(data) = claim.extraData {
            user_data = Some(data);
        }
        public_key = claim.identityPublicKey;
    }
    if verified == false {
        return Err(LoginErrors::NotAuthenticated.into());
    }
    match user_data {
        Some(data) => Ok((public_key, data)),
        None => Err(LoginErrors::ExtraUserdataNotFound.into()),
    }
}

pub fn verify_skin_data(public_key: &str, client: &str) -> Result<SkinData> {
    let key = ES384PublicKey::from_der(&base64::decode(public_key)?)?;
    match key.verify_token::<SkinData>(client) {
        Ok((_, claim)) => Ok(claim),
        Err(_) => Err(LoginErrors::WrongSkinData(client.to_owned()).into()),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SkinData {
    pub AnimatedImageData: Vec<AnimatedImageDataType>,
    pub ArmSize: String,
    pub CapeData: String,
    pub CapeId: String,
    pub CapeImageHeight: u64,
    pub CapeImageWidth: u64,
    pub CapeOnClassicSkin: bool,
    pub ClientRandomId: u64,
    pub CompatibleWithClientSideChunkGen: bool,
    pub CurrentInputMode: u8,
    pub DefaultInputMode: u8,
    pub DeviceId: String,
    pub DeviceModel: String,
    pub DeviceOS: u8,
    pub GameVersion: String,
    pub GuiScale: i8,
    pub IsEditorMode: bool,
    pub LanguageCode: String,
    pub OverrideSkin: bool,
    pub PersonaPieces: Vec<PersonaPiecesType>,
    pub PersonaSkin: bool,
    pub PieceTintColors: Vec<PieceTintColorsType>,
    pub PlatformOfflineId: String,
    pub PlatformOnlineId: String,
    pub PlayFabId: String,
    pub PremiumSkin: bool,
    pub SelfSignedId: String,
    pub ServerAddress: String,
    pub SkinAnimationData: String,
    pub SkinColor: String,
    pub SkinData: String,
    pub SkinGeometryData: String,
    pub SkinGeometryDataEngineVersion: String,
    pub SkinId: String,
    pub SkinImageHeight: u64,
    pub SkinImageWidth: u64,
    pub SkinResourcePatch: String,
    pub ThirdPartyName: String,
    pub ThirdPartyNameOnly: bool,
    pub TrustedSkin: bool,
    pub UIProfile: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimatedImageDataType {
    pub AnimationExpression: u64,
    pub Frames: f64,
    pub Image: String,
    pub ImageHeight: u64,
    pub ImageWidth: u64,
    pub Type: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PersonaPiecesType {
    pub IsDefault: bool,
    pub PackId: String,
    pub PieceId: String,
    pub PieceType: String,
    pub ProductId: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PieceTintColorsType {
    pub Colors: Vec<String>,
    pub PieceType: String,
}
