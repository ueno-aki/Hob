use std::io::Read;

use aes::{
    cipher::{KeyIvInit, StreamCipher},
    Aes256,
};
use anyhow::{ensure, Result};
use flate2::read::DeflateDecoder;
use proto_bytes::{Buf, BufMut, BytesMut, ConditionalReader};

use crate::packet::PacketKind;

type Aes256Ctr = ctr::Ctr64BE<Aes256>;

#[derive(Default)]
pub struct Decoder {
    pub cipher: Option<Aes256Ctr>,
    pub counter: u64,
    ss_key: [u8; 32],
}

impl Decoder {
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
    pub fn decode(&mut self, bytes: &mut BytesMut) -> Result<Vec<PacketKind>> {
        ensure!(
            bytes.get_u8() == 0xfe,
            "invalid packet header,expected 0xfe"
        );
        if self.cipher.is_some() {
            self.decrypt(bytes)?;
        }
        self.decompress(bytes);
        let mut packets = Vec::new();
        while !bytes.is_empty() {
            let size = bytes.get_varint();
            let expected = bytes.len() - size as usize;
            packets.push(PacketKind::decode(bytes)?);
            ensure!(bytes.len() == expected, "Invalid packet size");
        }
        Ok(packets)
    }
    fn decompress(&mut self, bytes: &mut BytesMut) {
        let mut decoder = DeflateDecoder::new(bytes.as_ref());
        let mut flate = Vec::new();
        if decoder.read_to_end(&mut flate).is_ok() {
            bytes.clear();
            bytes.extend_from_slice(&flate);
        }
    }
    fn decrypt(&mut self, bytes: &mut BytesMut) -> Result<()> {
        self.cipher
            .as_mut()
            .unwrap()
            .apply_keystream(bytes.as_mut());
        self.verify(bytes)?;
        bytes.truncate(bytes.len() - 8);
        self.counter += 1;
        Ok(())
    }
    fn verify(&mut self, bytes: &mut BytesMut) -> Result<()> {
        ensure!(
            bytes.len() > 8,
            "encrypted packet must be at least 8 bytes long"
        );
        let plain_text = &bytes[..bytes.len() - 8];
        let tag = &bytes[bytes.len() - 8..];

        let mut counter_vec: Vec<u8> = Vec::new();
        counter_vec.put_u64_le(self.counter);
        let mut digest = hmac_sha256::Hash::new();
        digest.update(counter_vec);
        digest.update(plain_text);
        digest.update(self.ss_key);
        ensure!(
            &digest.finalize()[..8] == tag,
            "packet verification faiure:{:x?}",
            bytes.as_ref()
        );
        Ok(())
    }
}
