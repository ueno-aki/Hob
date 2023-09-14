use aes::{cipher::StreamCipherCoreWrapper, Aes256};
use ctr::{flavors::Ctr64BE, CtrCore};

pub type Cipher = StreamCipherCoreWrapper<CtrCore<Aes256, Ctr64BE>>;
pub type Aes256Ctr64BE = ctr::Ctr64BE<Aes256>;
