use p384::ecdh::diffie_hellman;

use super::es384::{ES384PublicKey, ES384PrivateKey};

impl ES384PublicKey {
    pub fn diffie_hellman(&self,peer_secret:ES384PrivateKey) -> Vec<u8> {
        let shared_secret = diffie_hellman(
            peer_secret.as_ref().as_nonzero_scalar(),
            self.as_ref().as_affine()
        );
        let bytes:&[u8] = shared_secret.raw_secret_bytes().as_ref();
        bytes.to_vec()
    }
}