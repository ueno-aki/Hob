use hmac_sha512::sha384;
use p384::{ecdsa::{VerifyingKey, SigningKey, Signature,signature::DigestVerifier}, pkcs8::{DecodePublicKey, EncodePublicKey, EncodePrivateKey}, NonZeroScalar};
use anyhow::{Result, anyhow, Context};
use serde::{Serialize, de::DeserializeOwned, Deserialize};

use crate::protocol::mcpe::crypto::errors::CryptoErrors;

pub struct ES384PublicKey(VerifyingKey);
impl AsRef<VerifyingKey> for ES384PublicKey {
    fn as_ref(&self) -> &VerifyingKey {
        &self.0
    }
}

impl ES384PublicKey {
    pub fn from_der(bytes: &[u8]) -> Result<Self> {
        Ok(Self(VerifyingKey::from_public_key_der(bytes).map_err(|e|anyhow!("{}",e))?))
    }
    pub fn to_der(&self) -> Result<Vec<u8>> {
        let p384_pubkey = p384::PublicKey::from(self.as_ref());
        Ok(p384_pubkey.to_public_key_der().map_err(|e|anyhow!("{}",e))?.as_ref().to_vec())
    }
    pub fn verify_token<Claim>(&self,token:&str) -> Result<(ES384Header,Claim)>
        where Claim: Serialize + DeserializeOwned
    {
        let mut r_token = token.rsplitn(2, ".");
        let sig = decode_b64_nopad(r_token.next().context("InvalidJWTFormat")?)?;
        let signature = Signature::try_from(sig.as_ref())?;
        let payload = r_token.next().context("InvalidJWTFormat")?;

        let mut digest = sha384::Hash::new();
        digest.update(payload.as_bytes());
        self.as_ref().verify_digest(digest, &signature).map_err(|e|CryptoErrors::FailedVerification(format!("{e:?}")))?;

        use serde_json::from_slice;
        let mut r_p = payload.rsplitn(2, ".");
        let claim = decode_b64_nopad(r_p.next().context("InvalidJWTFormat")?)?;
        let header = decode_b64_nopad(r_p.next().context("InvalidJWTFormat")?)?;
        Ok((from_slice(&header)?, from_slice(&claim)?))
    }
}

#[derive(Serialize,Deserialize)]
pub struct ES384Header {
    pub alg: String,
    pub x5u: String,
}
fn decode_b64_nopad(str: &str) ->Result<Vec<u8>> {
    let decoded = base64::decode_config(str, base64::URL_SAFE_NO_PAD)?;
    Ok(decoded)
}

pub struct ES384SecretKey(SigningKey);
impl AsRef<SigningKey> for ES384SecretKey {
    fn as_ref(&self) -> &SigningKey {
        &self.0
    }
}
impl ES384SecretKey {
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
            .map_err(|e|anyhow!("{}",e))?
            .to_string()
        )
    }
}