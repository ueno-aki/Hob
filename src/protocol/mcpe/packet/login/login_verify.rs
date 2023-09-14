use anyhow::Result;
use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::protocol::mcpe::{packet::login::{errors::LoginErrors, constants::MOJANG_PUBKEY}, crypto::es384::ES384PublicKey};

#[derive(Serialize, Deserialize)]
struct AuthChain {
    chain:Vec<String>
}

pub fn verify_auth(chains: &str) -> Result<()> {
    let chains = serde_json::from_str::<AuthChain>(chains)?.chain;
    if chains.len() != 3 {
        return Err(LoginErrors::InvalidChainLength(chains.len()).into());
    }
    let mut pubkey = ES384PublicKey::decode_header(&chains[0])?.x5u;
    let mut verified = false;
    for chain in chains {
        let key = ES384PublicKey::from_der(&base64::decode(&pubkey)?)?;
        let (header,claim) = key.verify_token::<Value>(&chain)?;
        if header.x5u == MOJANG_PUBKEY {
            verified = true;
        }
        println!("header{:?},claim:{}",header,claim);
    }
    if verified == false {
        return Err(LoginErrors::NotAuthenticated.into());
    }
    unreachable!()
}