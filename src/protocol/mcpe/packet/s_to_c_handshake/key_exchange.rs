use anyhow::Result;

use crate::protocol::mcpe::crypto::es384::{ES384PrivateKey, ES384PublicKey};
use super::constants::salt;

pub fn shared_secret(peer_pubkey: &str) -> Result<(ES384PrivateKey,[u8;32])>{
    let my_secret = ES384PrivateKey::generate();
    let peer_pubkey = ES384PublicKey::from_der(&base64::decode(peer_pubkey)?)?;
    let ss_key = peer_pubkey.diffie_hellman(&my_secret);
    
    let mut digest = hmac_sha256::Hash::new();
    digest.update(salt);
    digest.update(ss_key);
    let secret_key_bytes = digest.finalize();
    Ok((my_secret,secret_key_bytes))
}