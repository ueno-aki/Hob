use aes::{cipher::{StreamCipherCoreWrapper,KeyIvInit}, Aes256};
use anyhow::Result;
use ctr::{flavors::Ctr64BE, CtrCore};

use crate::player::Player;

use super::errors::CryptoErrors;

pub type Cipher = StreamCipherCoreWrapper<CtrCore<Aes256, Ctr64BE>>;
pub type Aes256Ctr64BE = ctr::Ctr64BE<Aes256>;

pub trait Aes256CtrCipherManager {
    fn setup_cipher(&mut self,key:[u8;32],iv:[u8;16])->Result<()>;
}

impl Aes256CtrCipherManager for Player {
    fn setup_cipher(&mut self,key:[u8;32],iv:[u8;16])->Result<()> {
        match (&self.status.cipher, &self.status.decipher) {
            (None, None) => {
                self.status.cipher = Some(Aes256Ctr64BE::new(&key.into(), &iv.into()));
                self.status.decipher = Some(Aes256Ctr64BE::new(&key.into(), &iv.into()));
                Ok(())
            }
            _ => Err(CryptoErrors::AlreadyCipherExists().into()),
        }
    }
}