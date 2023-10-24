use aes::{
    cipher::{KeyIvInit, StreamCipher, StreamCipherCoreWrapper},
    Aes256,
};
use anyhow::Result;
use ctr::{flavors::Ctr64BE, CtrCore};
use protodef::prelude::*;
use std::io::Write;

use crate::player::Player;

pub type Cipher = StreamCipherCoreWrapper<CtrCore<Aes256, Ctr64BE>>;
pub type Aes256Ctr64BE = ctr::Ctr64BE<Aes256>;

pub trait Aes256CtrManager {
    fn setup_cipher(&mut self, key: &[u8; 32], iv: &[u8; 16]);
    fn decrypt_or(&mut self, buffer: &mut [u8]);
    fn encrypt_or(&mut self, buffer: &mut Vec<u8>) -> Result<()>;
    fn compute_packet_tag(counter: u64, plain_pkt: &[u8], ss_key: &[u8; 32]) -> Result<Vec<u8>>;
}

impl Aes256CtrManager for Player {
    fn setup_cipher(&mut self, key: &[u8; 32], iv: &[u8; 16]) {
        let mut status = self.get_status_mut();
        status.cipher = Some(Aes256Ctr64BE::new(key.into(), iv.into()));
        status.decipher = Some(Aes256Ctr64BE::new(key.into(), iv.into()));
        status.ss_key = Some(*key);
        status.encryption_enabled = true;
    }
    fn decrypt_or(&mut self, buffer: &mut [u8]) {
        let encryption_enabled = self.get_status().encryption_enabled;
        if encryption_enabled {
            self.get_status_mut()
                .decipher
                .as_mut()
                .unwrap()
                .apply_keystream(buffer);
        }
    }
    fn encrypt_or(&mut self, buffer: &mut Vec<u8>) -> Result<()> {
        let encryption_enabled = self.get_status().encryption_enabled;
        if encryption_enabled {
            let tag = Self::compute_packet_tag(
                self.get_status().send_counter,
                buffer,
                &self.get_status().ss_key.unwrap(),
            )?;
            buffer.write(&tag)?;
            self.get_status_mut()
                .cipher
                .as_mut()
                .unwrap()
                .apply_keystream(buffer);
            self.get_status_mut().send_counter += 1;
        }
        Ok(())
    }
    fn compute_packet_tag(counter: u64, plain_pkt: &[u8], ss_key: &[u8; 32]) -> Result<Vec<u8>> {
        let mut digest = hmac_sha256::Hash::new();
        let mut counter_vec: Vec<u8> = Vec::new();
        counter_vec.write_lu64(counter)?;
        digest.update(&counter_vec);
        digest.update(plain_pkt);
        digest.update(ss_key);
        let result = digest.finalize();
        Ok(result[0..8].to_vec())
    }
}
