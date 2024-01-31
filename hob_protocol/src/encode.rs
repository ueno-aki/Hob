use std::io::Read;

use aes::{
    cipher::{KeyIvInit, StreamCipher},
    Aes256,
};
use flate2::{read::DeflateEncoder, Compression};
use proto_bytes::{BufMut, BytesMut, ConditionalBufMut};

use crate::packet::PacketKind;

type Aes256Ctr = ctr::Ctr64BE<Aes256>;

#[derive(Default)]
pub struct Encoder {
    pub cipher: Option<Aes256Ctr>,
    pub counter: u64,
    pub force_compress: bool,
    ss_key: [u8; 32],
}

impl Encoder {
    pub fn setup_cipher(&mut self, shared_secret: &[u8; 32]) {
        let mut iv: [u8; 16] = [0; 16];
        iv[15] = 2;
        iv[..12].copy_from_slice(&shared_secret[..12]);
        self.cipher = Some(Aes256Ctr::new(
            shared_secret.as_ref().into(),
            iv.as_ref().into(),
        ));
        self.ss_key.copy_from_slice(shared_secret);
    }
    pub fn encode(&mut self, packet: PacketKind) -> Vec<u8> {
        let mut encoded = BytesMut::new();

        let mut content = BytesMut::new();
        packet.encode(&mut content).unwrap();
        encoded.put_varint(content.len() as u64);
        encoded.put(content);

        if self.force_compress {
            self.compress(&mut encoded);
        }
        if self.cipher.is_some() {
            self.encrypt(&mut encoded);
        }
        let mut result = vec![0xfe];
        result.extend_from_slice(&encoded);
        result
    }
    fn compress(&mut self, bytes: &mut BytesMut) {
        let mut encoder = DeflateEncoder::new(bytes.as_ref(), Compression::new(7));
        let mut flate = Vec::new();
        encoder.read_to_end(&mut flate).unwrap();
        bytes.clear();
        bytes.extend_from_slice(&flate);
    }
    fn encrypt(&mut self, bytes: &mut BytesMut) {
        let mut counter_vec: Vec<u8> = Vec::new();
        counter_vec.put_u64_le(self.counter);
        let mut digest = hmac_sha256::Hash::new();
        digest.update(counter_vec);
        digest.update(bytes.as_ref());
        digest.update(self.ss_key);
        bytes.put_slice(&digest.finalize()[..8]);
        self.cipher
            .as_mut()
            .unwrap()
            .apply_keystream(bytes.as_mut());
        self.counter += 1;
    }
}
