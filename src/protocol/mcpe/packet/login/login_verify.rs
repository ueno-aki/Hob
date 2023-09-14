use anyhow::Result;
use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::protocol::mcpe::{packet::login::{errors::LoginErrors, constants::MOJANG_PUBKEY}, crypto::es384::ES384PublicKey};

#[derive(Serialize, Deserialize)]
struct AuthChain {
    chain:Vec<String>
}
#[derive(Deserialize, Serialize)]
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
    pub sandboxId: String
}

pub fn verify_login(chains: &str) -> Result<(String,ExtraUserdata)> {
    let chains = serde_json::from_str::<AuthChain>(chains)?.chain;
    if chains.len() != 3 {
        return Err(LoginErrors::InvalidChainLength(chains.len()).into());
    }
    let mut public_key = ES384PublicKey::decode_header(&chains[0])?.x5u;
    let mut verified = false;
    let mut user_data = None;
    for chain in chains {
        let key = ES384PublicKey::from_der(&base64::decode(&public_key)?)?;
        let (header,claim) = key.verify_token::<LoginIdentityClaim>(&chain)?;
        if header.x5u == MOJANG_PUBKEY {
            verified = true;
        }
        if let Some(data) = claim.extraData {
            user_data = Some(data);
        }
        public_key = claim.identityPublicKey;
    }
    if verified == false {
        return Err(LoginErrors::NotAuthenticated.into())
    }
    match user_data {
        Some(data) => Ok((public_key,data)),
        None => Err(LoginErrors::ExtraUserdataNotFound.into())
    }
}

pub fn verify_client_data(public_key: &str, client: &str) -> Result<Value> {
    let key = ES384PublicKey::from_der(&base64::decode(public_key)?)?;
    match key.verify_token::<Value>(client) {
        Ok((_, claim)) => Ok(claim),
        Err(_) => Err(LoginErrors::WrongSkinData(client.to_owned()).into()),
    }
}