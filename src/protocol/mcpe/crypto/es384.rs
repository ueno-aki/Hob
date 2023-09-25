use crate::{
    protocol::mcpe::crypto::errors::CryptoErrors,
    utils::{decode_nopad_base64, encode_nopad_base64},
};
use anyhow::{anyhow, Result};
use hmac_sha512::sha384;
use p384::{
    ecdsa::{
        signature::{DigestVerifier, RandomizedDigestSigner},
        Signature, SigningKey, VerifyingKey,
    },
    pkcs8::{DecodePublicKey, EncodePublicKey},
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ES384Header {
    pub alg: String,
    pub x5u: String,
}

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
                let header = decode_nopad_base64(header)?;
                Ok(serde_json::from_slice(&header)?)
            }
            None => Err(CryptoErrors::InvalidJWTFormat(token.to_owned()).into()),
        }
    }
    pub fn verify_token<Claim>(&self, token: &str) -> Result<(ES384Header, Claim)>
    where
        Claim: Serialize + DeserializeOwned,
    {
        let mut r_token = token.rsplitn(2, ".");
        let (Some(sig), Some(payload)) = (r_token.next(), r_token.next()) else {
            return Err(CryptoErrors::InvalidJWTFormat(token.to_owned()).into());
        };
        let signature = Signature::try_from(decode_nopad_base64(sig)?.as_ref())?;
        let mut digest = sha384::Hash::new();
        digest.update(payload.as_bytes());
        self.as_ref()
            .verify_digest(digest, &signature)
            .map_err(|_| CryptoErrors::FailedVerification)?;
        let mut r_p = payload.rsplitn(2, ".");
        let (Some(claim), Some(header)) = (r_p.next(), r_p.next()) else {
            return Err(CryptoErrors::InvalidJWTPayload(payload.to_owned()).into());
        };
        Ok((
            serde_json::from_slice(&decode_nopad_base64(header)?)?,
            serde_json::from_slice(&decode_nopad_base64(claim)?)?,
        ))
    }
}

pub struct ES384PrivateKey(SigningKey);
impl AsRef<SigningKey> for ES384PrivateKey {
    fn as_ref(&self) -> &SigningKey {
        &self.0
    }
}
impl ES384PrivateKey {
    pub fn generate() -> Self {
        Self(SigningKey::random(&mut rand::thread_rng()))
    }
    pub fn public_key(&self) -> ES384PublicKey {
        ES384PublicKey(*self.as_ref().verifying_key())
    }

    pub fn sign<Claim>(&self, header: &ES384Header, claim: &Claim) -> Result<String>
    where
        Claim: Serialize + DeserializeOwned,
    {
        let header_json = encode_nopad_base64(serde_json::to_string(&header)?);
        let claim_json = encode_nopad_base64(serde_json::to_string(&claim)?);
        let payload = format!("{}.{}", header_json, claim_json);

        let mut rng = rand::thread_rng();
        let mut digest = sha384::Hash::new();
        digest.update(payload.as_bytes());
        let signature: Signature = self.as_ref().sign_digest_with_rng(&mut rng, digest);

        let token = format!("{}.{}", payload, encode_nopad_base64(signature.to_vec()));
        Ok(token)
    }
}
