use anyhow::{anyhow, Result};
use hmac_sha512::sha384;
use p384::{
    ecdsa::{signature::DigestVerifier, Signature, SigningKey, VerifyingKey},
    pkcs8::{DecodePublicKey, EncodePrivateKey, EncodePublicKey},
    NonZeroScalar,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::protocol::mcpe::crypto::errors::CryptoErrors;

pub struct ES384PublicKey(VerifyingKey);
impl AsRef<VerifyingKey> for ES384PublicKey {
    fn as_ref(&self) -> &VerifyingKey {
        &self.0
    }
}

impl ES384PublicKey {
    pub fn from_der(bytes: &[u8]) -> Result<Self> {
        Ok(Self(
            VerifyingKey::from_public_key_der(bytes).map_err(|e| anyhow!("{}", e))?,
        ))
    }
    pub fn to_der(&self) -> Result<Vec<u8>> {
        let p384_pubkey = p384::PublicKey::from(self.as_ref());
        Ok(p384_pubkey
            .to_public_key_der()
            .map_err(|e| anyhow!("{}", e))?
            .as_ref()
            .to_vec())
    }
    pub fn decode_header(token: &str) -> Result<ES384Header> {
        match token.split(".").next() {
            Some(header) => {
                let header = decode_b64_nopad(header)?;
                Ok(serde_json::from_slice(&header)?)
            }
            None => Err(CryptoErrors::InvalidJWTFormat(token.to_owned()).into())
        }
    }
    pub fn verify_token<Claim>(&self, token: &str) -> Result<(ES384Header, Claim)>
    where
        Claim: Serialize + DeserializeOwned,
    {
        let mut r_token = token.rsplitn(2, ".");
        if let (Some(sig), Some(payload)) = (r_token.next(), r_token.next()) {
            let signature = Signature::try_from(decode_b64_nopad(sig)?.as_ref())?;
            let mut digest = sha384::Hash::new();
            digest.update(payload.as_bytes());
            self.as_ref()
                .verify_digest(digest, &signature)
                .map_err(|_| CryptoErrors::FailedVerification)?;

            let mut r_p = payload.rsplitn(2, ".");
            if let (Some(claim), Some(header)) = (r_p.next(), r_p.next()) {
                let claim = decode_b64_nopad(claim)?;
                let header = decode_b64_nopad(header)?;
                Ok((serde_json::from_slice(&header)?, serde_json::from_slice(&claim)?))
            } else {
                Err(CryptoErrors::InvalidJWTPayload(payload.to_owned()).into())
            }
        } else {
            Err(CryptoErrors::InvalidJWTFormat(token.to_owned()).into())
        }
    }
}

#[derive(Debug,Serialize, Deserialize)]
pub struct ES384Header {
    pub alg: String,
    pub x5u: String,
}
fn decode_b64_nopad(str: &str) -> Result<Vec<u8>> {
    let decoded = base64::decode_config(str, base64::URL_SAFE_NO_PAD)?;
    Ok(decoded)
}

pub struct ES384PrivateKey(SigningKey);
impl AsRef<SigningKey> for ES384PrivateKey {
    fn as_ref(&self) -> &SigningKey {
        &self.0
    }
}
impl ES384PrivateKey {
    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();
        Self(SigningKey::random(&mut rng))
    }
    pub fn public_key(&self) -> ES384PublicKey {
        ES384PublicKey(*self.0.verifying_key())
    }
    pub fn to_pem(&self) -> Result<String> {
        let scalar = NonZeroScalar::from_repr(self.0.to_bytes()).unwrap();
        let p384_sk = p384::SecretKey::from(scalar);
        Ok(p384_sk
            .to_pkcs8_pem(Default::default())
            .map_err(|e| anyhow!("{}", e))?
            .to_string())
    }
}
