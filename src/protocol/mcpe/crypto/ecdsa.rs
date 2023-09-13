use p384::{ecdsa::{VerifyingKey, SigningKey}, pkcs8::{DecodePublicKey, EncodePublicKey, EncodePrivateKey}, NonZeroScalar};
use anyhow::{Result, anyhow};

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