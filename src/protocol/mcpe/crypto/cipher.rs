use aes::{
    cipher::{KeyIvInit, StreamCipherCoreWrapper},
    Aes256,
};
use anyhow::Result;
use ctr::{flavors::Ctr64BE, CtrCore};

use crate::player::Player;

use super::errors::CryptoErrors;

pub type Cipher = StreamCipherCoreWrapper<CtrCore<Aes256, Ctr64BE>>;
pub type Aes256Ctr64BE = ctr::Ctr64BE<Aes256>;

pub trait Aes256CtrCipherManager {
    fn setup_cipher(&mut self, key: &[u8; 32], iv: &[u8; 16]) -> Result<()>;
}

impl Aes256CtrCipherManager for Player {
    fn setup_cipher(&mut self, key: &[u8; 32], iv: &[u8; 16]) -> Result<()> {
        self.get_status_mut().cipher = Some(Aes256Ctr64BE::new(key.into(), iv.into()));
        self.get_status_mut().decipher = Some(Aes256Ctr64BE::new(key.into(), iv.into()));
        Ok(())
    }
}
