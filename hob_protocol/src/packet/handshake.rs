use anyhow::Result;
use base64::prelude::*;
use proto_bytes::ConditionalWriter;
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};

use crate::jwt::{ES384Header, ES384PrivateKey, ES384PublicKey};

use super::Packet;

pub fn shared_secret(peer_pubkey_der: &str) -> Result<([u8; 32], String)> {
    let my_secret = ES384PrivateKey::generate();
    let peer_pubkey = ES384PublicKey::from_der(&BASE64_URL_SAFE_NO_PAD.decode(peer_pubkey_der)?)?;
    let shared_secret = peer_pubkey.diffie_hellman(&my_secret);

    let mut digest = hmac_sha256::Hash::new();
    let salt = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    digest.update(&salt);
    digest.update(shared_secret.raw_secret_bytes());
    let ss_key = digest.finalize();

    let my_x509 = BASE64_URL_SAFE_NO_PAD.encode(my_secret.public_key().to_der()?);
    let header = ES384Header {
        alg: "ES384".to_owned(),
        x5u: my_x509.clone(),
    };
    let claim = HandshakeClaim {
        salt: BASE64_URL_SAFE_NO_PAD.encode(salt),
        signed_token: my_x509,
    };
    let token = my_secret.sign(&header, &claim)?;
    Ok((ss_key, token))
}

#[derive(Debug, Serialize, Deserialize)]
struct HandshakeClaim {
    salt: String,
    #[serde(rename(serialize = "signedToken", deserialize = "signedToken"))]
    signed_token: String,
}

#[derive(Debug)]
pub struct ServerToClientHandshakePacket {
    pub token: String,
}

impl Packet for ServerToClientHandshakePacket {
    fn decode(bytes: &mut proto_bytes::BytesMut) -> anyhow::Result<Self> {
        todo!()
    }

    #[inline]
    fn encode(&self, bytes: &mut proto_bytes::BytesMut) -> anyhow::Result<()> {
        bytes.put_string_varint(&self.token);
        Ok(())
    }
}

#[derive(Debug)]
pub struct ClientToServerHandshakePacket;

impl Packet for ClientToServerHandshakePacket {
    #[inline]
    fn decode(_bytes: &mut proto_bytes::BytesMut) -> anyhow::Result<Self> {
        Ok(ClientToServerHandshakePacket)
    }

    fn encode(&self, _bytes: &mut proto_bytes::BytesMut) -> anyhow::Result<()> {
        todo!()
    }
}
