use anyhow::{anyhow, bail, ensure, Result};
use base64::prelude::*;
use hmac_sha512::sha384;
use p384::{
    ecdh::{diffie_hellman, SharedSecret},
    ecdsa::{
        signature::{DigestVerifier, RandomizedDigestSigner},
        Signature, SigningKey, VerifyingKey,
    },
    pkcs8::{DecodePublicKey, EncodePublicKey},
    PublicKey,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ES384Header {
    pub alg: String,
    pub x5u: String,
}

pub struct ES384PublicKey(VerifyingKey);
impl ES384PublicKey {
    pub fn from_der(bytes: &[u8]) -> Result<Self> {
        Ok(Self(
            VerifyingKey::from_public_key_der(bytes).map_err(|e| anyhow!("{}", e))?,
        ))
    }
    pub fn to_der(&self) -> Result<Vec<u8>> {
        let p384_pubkey = PublicKey::from(&self.0);
        Ok(p384_pubkey
            .to_public_key_der()
            .map_err(|e| anyhow!("{}", e))?
            .to_vec())
    }
    pub fn decode_header(token: &str) -> Result<ES384Header> {
        match token.split('.').next() {
            Some(header) => {
                let header = BASE64_URL_SAFE_NO_PAD.decode(header)?;
                Ok(serde_json::from_slice(&header)?)
            }
            None => Err(anyhow!("Invalid JWT Format:{}", token)),
        }
    }
    pub fn verify_token<Claim>(&self, token: &str) -> Result<Claim>
    where
        Claim: DeserializeOwned,
    {
        let mut r_token = token.rsplitn(2, '.');
        let (Some(signature), Some(payload)) = (r_token.next(), r_token.next()) else {
            bail!("Invalid JWT Format:{}", token);
        };
        let signature = Signature::try_from(BASE64_URL_SAFE_NO_PAD.decode(signature)?.as_ref())?;
        let mut digest = sha384::Hash::new();
        digest.update(payload);
        ensure!(
            self.0.verify_digest(digest, &signature).is_ok(),
            "JWTFailedVerification"
        );
        let Some(claim) = payload.rsplit('.').next() else {
            bail!("Invalid JWT Format:{}", token);
        };
        Ok(serde_json::from_slice(
            &BASE64_URL_SAFE_NO_PAD.decode(claim)?,
        )?)
    }
    pub fn diffie_hellman(&self, peer_secret: &ES384PrivateKey) -> SharedSecret {
        diffie_hellman(
            peer_secret.0.as_nonzero_scalar(),
            self.0.as_affine(),
        )
    }
}

pub struct ES384PrivateKey(SigningKey);
impl ES384PrivateKey {
    pub fn generate() -> Self {
        Self(SigningKey::random(&mut rand::thread_rng()))
    }
    pub fn public_key(&self) -> ES384PublicKey {
        ES384PublicKey(*self.0.verifying_key())
    }
    pub fn sign<Claim>(&self, header: ES384Header, claim: Claim) -> Result<String>
    where
        Claim: Serialize,
    {
        let header_json = BASE64_URL_SAFE_NO_PAD.encode(serde_json::to_string(&header)?);
        let claim_json = BASE64_URL_SAFE_NO_PAD.encode(serde_json::to_string(&claim)?);
        let payload = format!("{}.{}", header_json, claim_json);

        let mut digest = sha384::Hash::new();
        digest.update(&payload);
        let signature: Signature = self.0.sign_digest_with_rng(&mut rand::thread_rng(), digest);
        let token = format!(
            "{}.{}",
            payload,
            BASE64_URL_SAFE_NO_PAD.encode(signature.to_vec())
        );
        Ok(token)
    }
}
