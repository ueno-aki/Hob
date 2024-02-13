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
    pub compression_ready: bool,
    ss_key: [u8; 32],
}

impl Encoder {
    pub fn setup_cipher(&mut self, shared_secret: &[u8; 32]) {
        let mut iv: [u8; 16] = [0; 16];
        iv[15] = 2;
        iv[..12].copy_from_slice(&shared_secret[..12]);
        self.cipher = Some(Aes256Ctr::new(shared_secret.into(), &iv.into()));
        self.ss_key.copy_from_slice(shared_secret);
    }
    pub fn encode(&mut self, packet: PacketKind) -> Vec<u8> {
        let mut content = BytesMut::new();
        {
            let mut packet_buf = BytesMut::new();
            packet.encode(&mut packet_buf).unwrap();
            content.put_varint(packet_buf.len() as u64);
            content.put(packet_buf);
        }

        if self.compression_ready {
            let mut compressed = Vec::new();
            if content.len() > 512 {
                compressed.put_u8(0x00);
                self.compress(&mut content);
            } else {
                compressed.put_u8(0xff);
            }
            compressed.extend_from_slice(&content);

            content.clear();
            content.extend_from_slice(&compressed);
        }

        if self.cipher.is_some() {
            self.encrypt(&mut content);
        }
        let mut result = vec![0xfe];
        result.extend_from_slice(&content);
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
