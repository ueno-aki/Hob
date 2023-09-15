use p384::ecdh::{diffie_hellman, SharedSecret};

use super::es384::{ES384PrivateKey, ES384PublicKey};

pub trait DiffieHellman {
    fn diffie_hellman(&self, peer_secret: &ES384PrivateKey) -> SharedSecret;
}

impl DiffieHellman for ES384PublicKey {
    fn diffie_hellman(&self, peer_secret: &ES384PrivateKey) -> SharedSecret {
        let shared_secret = diffie_hellman(
            peer_secret.as_ref().as_nonzero_scalar(),
            self.as_ref().as_affine(),
        );
        shared_secret
    }
}
