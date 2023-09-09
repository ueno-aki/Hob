use anyhow::Result;
use flate2::bufread::DeflateDecoder;
use protodef::prelude::*;
use std::io::Read as _;

use crate::protocol::packet::{PacketKind, RequestNetworkSetting};
use super::errors::TransFormError;

pub fn decode(buffer: Vec<u8>) -> Result<Vec<Vec<u8>>> {
    if buffer[0] != 0xfe {
        return Err(TransFormError::ClientUnspecifiedPacket(buffer).into());
    }

    let flate = decompress(buffer[1..].to_vec());
    let mut packets: Vec<Vec<u8>> = Vec::new();
    let mut offset: usize = 0;

    while offset < flate.len() {
        let (value, size) = flate.read_varint(offset as u64)?;
        let mut dec: Vec<u8> = vec![0; value as usize];
        offset += size as usize;
        let edge = offset + value as usize;
        dec.copy_from_slice(&flate[offset..edge]);
        offset += value as usize;
        packets.push(dec);
    }
    Ok(packets)
}
fn decompress(buffer: Vec<u8>) -> Vec<u8> {
    let mut decoder: DeflateDecoder<&[u8]> = DeflateDecoder::new(&buffer[..]);
    let mut flate: Vec<u8> = Vec::new();
    match decoder.read_to_end(&mut flate) {
        Ok(_) => flate,
        Err(_) => buffer,
    }
}
pub fn parse_packet(buffer: Vec<u8>) -> Result<PacketKind>{
    let (name,n_size) = buffer.read_varint(0)?;
    let packet: PacketKind = match name {
        x if x == RequestNetworkSetting::id() => RequestNetworkSetting::from_buf(buffer,n_size)?.into(),
        _ => todo!()
    };
    Ok(packet)
}
