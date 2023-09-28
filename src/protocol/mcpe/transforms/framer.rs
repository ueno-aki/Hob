use anyhow::Result;
use flate2::{
    bufread::{DeflateDecoder, DeflateEncoder},
    Compression,
};
use protodef::prelude::*;
use std::io::Read as _;

use crate::protocol::mcpe::packet::{
    ClientCacheStatusPacket, ClientToServerHandshakePacket, LoginPacket, PacketKind,
    RequestNetworkSettingPacket,
};

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
pub fn parse_packet(buffer: Vec<u8>) -> Result<PacketKind> {
    let (name, n_size) = buffer.read_varint(0)?;
    let packet: PacketKind = match name {
        x if x == LoginPacket::id() => LoginPacket::from_buf(buffer, n_size)?.into(),
        x if x == ClientToServerHandshakePacket::id() => ClientToServerHandshakePacket().into(),
        x if x == RequestNetworkSettingPacket::id() => {
            RequestNetworkSettingPacket::from_buf(buffer, n_size)?.into()
        }
        x if x == ClientCacheStatusPacket::id() => {
            ClientCacheStatusPacket::from_buf(buffer, n_size)?.into()
        }
        _ => todo!("packet_id:{}", name),
    };
    Ok(packet)
}

pub fn encode(packet: PacketKind, force_compress: bool) -> Result<Vec<u8>> {
    let mut content: Vec<u8> = Vec::new();
    content.write_var_int(packet.get_id())?;
    match packet {
        PacketKind::PlayStatusPacket(v) => v.read_to_buffer(&mut content)?,
        PacketKind::ServerToClientHandshakePacket(v) => v.read_to_buffer(&mut content)?,
        PacketKind::DisconnectPacket(v) => v.read_to_buffer(&mut content)?,
        PacketKind::NetworkSettingsPacket(v) => v.read_to_buffer(&mut content)?,
        PacketKind::ResourcePacksInfoPacket(v) => v.read_to_buffer(&mut content)?,
        _ => todo!("packet_id:{}", packet.get_id()),
    };
    let mut result = Vec::new();
    result.write_var_int(content.len() as u64)?;
    result = compress([result, content].concat(), force_compress)?;
    Ok(result)
}
fn compress(buffer: Vec<u8>, force: bool) -> Result<Vec<u8>> {
    if buffer.len() > 512 || force {
        let mut encoder = DeflateEncoder::new(buffer.as_ref(), Compression::new(7));
        let mut flate = Vec::new();
        encoder.read_to_end(&mut flate)?;
        Ok(flate)
    } else {
        Ok(buffer)
    }
}
