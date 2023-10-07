use super::constants::SALT;
use crate::{
    protocol::mcpe::crypto::{
        ecdh::DiffieHellman,
        es384::{ES384Header, ES384PrivateKey, ES384PublicKey},
    },
    utils::{decode_base64, encode_base64},
};
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub fn shared_secret(peer_pubkey: &str) -> Result<([u8; 32], String)> {
    let my_secret = ES384PrivateKey::generate();
    let peer_pubkey = ES384PublicKey::from_der(&decode_base64(peer_pubkey)?)?;
    let ss_key = peer_pubkey.diffie_hellman(&my_secret);

    let mut digest = hmac_sha256::Hash::new();
    digest.update(SALT);
    digest.update(ss_key.raw_secret_bytes());
    let secret_key_bytes = digest.finalize();

    let my_x509 = encode_base64(my_secret.public_key().to_der()?);
    let header = ES384Header {
        alg: "ES384".to_owned(),
        x5u: my_x509.clone(),
    };
    let claim = HandshakeClaim {
        salt: encode_base64(SALT),
        signedToken: my_x509,
    };
    let token = my_secret.sign(&header, &claim)?;
    Ok((secret_key_bytes, token))
}

#[derive(Serialize, Deserialize)]
struct HandshakeClaim {
    salt: String,
    signedToken: String,
}
