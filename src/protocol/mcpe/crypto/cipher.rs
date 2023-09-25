use aes::{
    cipher::{KeyIvInit, StreamCipher, StreamCipherCoreWrapper},
    Aes256,
};
use anyhow::Result;
use ctr::{flavors::Ctr64BE, CtrCore};

use crate::player::Player;

pub type Cipher = StreamCipherCoreWrapper<CtrCore<Aes256, Ctr64BE>>;
pub type Aes256Ctr64BE = ctr::Ctr64BE<Aes256>;

pub trait Aes256CtrManager {
    fn setup_cipher(&mut self, key: &[u8; 32], iv: &[u8; 16]) -> Result<()>;
    fn decrypt_or<'a>(&mut self, buffer: &'a mut [u8]) -> &'a [u8];
    fn encrypt_or(&mut self, buffer: &[u8]) -> Result<Vec<u8>> ;
    fn compute_packet_tag(counter: u64, plain_pkt: &[u8], ss_key: [u8; 32]) -> Result<Vec<u8>>;
}

impl Aes256CtrManager for Player {
    fn setup_cipher(&mut self, key: &[u8; 32], iv: &[u8; 16]) -> Result<()> {
        self.get_status_mut().cipher = Some(Aes256Ctr64BE::new(key.into(), iv.into()));
        self.get_status_mut().decipher = Some(Aes256Ctr64BE::new(key.into(), iv.into()));
        Ok(())
    }
    fn decrypt_or<'a>(&mut self, buffer: &'a mut [u8]) -> &'a [u8] {
        let encryption_enabled = self.get_status().encryption_enabled;
        if encryption_enabled {
            self.get_status_mut()
                .decipher
                .as_mut()
                .unwrap()
                .apply_keystream(buffer);
        }
        buffer
    }
    fn encrypt_or(&mut self, buffer: &[u8]) -> Result<Vec<u8>> {
        let mut result = buffer.to_vec();
        let encryption_enabled = self.get_status().encryption_enabled;
        if encryption_enabled {
            let tag = Self::compute_packet_tag(
                self.get_status().send_counter,
                &result,
                self.get_status().ss_key.unwrap(),
            )?;
            result = [result, tag].concat();
            self.get_status_mut()
                .cipher
                .as_mut()
                .unwrap()
                .apply_keystream(&mut result);
            self.get_status_mut().send_counter += 1;
        }
        Ok(result)
    }
    fn compute_packet_tag(counter: u64, plain_pkt: &[u8], ss_key: [u8; 32]) -> Result<Vec<u8>> {
        let mut digest = hmac_sha256::Hash::new();
        use protodef::prelude::Write;
        let mut counter_vec = Vec::<u8>::new();
        counter_vec.write_lu64(counter)?;
        digest.update(&counter_vec);
        digest.update(plain_pkt);
        digest.update(ss_key);
        let result = digest.finalize();
        Ok(result[0..8].to_vec())
    }
}
