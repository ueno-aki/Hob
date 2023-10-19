use anyhow::Result;
use flate2::{
    bufread::{DeflateDecoder, DeflateEncoder},
    Compression,
};
use protodef::prelude::*;
use std::io::Read as _;

use crate::protocol::mcpe::packet::PacketKind;

pub fn decode(buffer: &[u8]) -> Result<Vec<Vec<u8>>> {
    let flate = decompress(buffer);
    let mut packets: Vec<Vec<u8>> = Vec::new();
    let mut offset: usize = 0;

    while offset < flate.len() {
        let (value, size) = flate.read_varint(offset)?;
        let mut dec: Vec<u8> = vec![0; value as usize];
        offset += size as usize;
        let edge = offset + value as usize;
        dec.copy_from_slice(&flate[offset..edge]);
        offset += value as usize;
        packets.push(dec);
    }
    Ok(packets)
}
fn decompress(buffer: &[u8]) -> Vec<u8> {
    let mut decoder = DeflateDecoder::new(buffer);
    let mut flate: Vec<u8> = Vec::new();
    match decoder.read_to_end(&mut flate) {
        Ok(_) => flate,
        Err(_) => buffer.to_vec(),
    }
}

pub fn encode(packet: &PacketKind, force_compress: bool) -> Result<Vec<u8>> {
    let mut content: Vec<u8> = Vec::new();
    content.write_varint(packet.id())?;
    packet.read_to_buffer(&mut content)?;
    let mut encoded = Vec::new();
    encoded.write_varint(content.len() as u64)?;
    encoded = [encoded, content].concat();
    Ok(compress(&encoded, force_compress)?)
}
fn compress(buffer: &[u8], force: bool) -> Result<Vec<u8>> {
    if buffer.len() > 512 || force {
        let mut encoder = DeflateEncoder::new(buffer, Compression::new(7));
        let mut flate = Vec::new();
        encoder.read_to_end(&mut flate)?;
        Ok(flate)
    } else {
        Ok(buffer.to_vec())
    }
}
