use std::sync::Arc;

use anyhow::{anyhow, Result};
use hob_protocol::{
    decode::Decoder,
    encode::Encoder,
    packet::{
        network_settings::{CompressionAlgorithmType, NetworkSettingsPacket},
        PacketKind,
    },
};
use proto_bytes::BytesMut;
use rust_raknet::RaknetSocket;

pub struct Client {
    pub socket: Arc<RaknetSocket>,
    pub encoder: Encoder,
    pub decoder: Decoder,
}

impl Client {
    pub fn new(socket: RaknetSocket) -> Self {
        Client {
            socket: Arc::new(socket),
            encoder: Encoder::default(),
            decoder: Decoder::default(),
        }
    }
    pub async fn listen(&mut self) -> Result<()> {
        loop {
            let buf = self.socket.recv().await.map_err(|e| anyhow!("{:?}", e))?;
            let mut bytes = BytesMut::from(buf.as_slice());
            for packet in self.decoder.decode(&mut bytes)? {
                self.handle(packet).await?;
            }
        }
    }
    pub async fn handle(&mut self, packet: PacketKind) -> Result<()> {
        println!("(CtoS) {}", packet);
        match packet {
            PacketKind::RequestNetworkSetting(v) => {
                let network_setting = NetworkSettingsPacket {
                    compression_threshold: 512,
                    compression_algorithm: CompressionAlgorithmType::Deflate,
                    client_throttle: false,
                    client_throttle_threshold: 0,
                    client_throttle_scalar: 0.0,
                };
                self.encoder.compression_threshold = 512;
                self.send_packet(network_setting).await?;
            }
            PacketKind::Login(v) => {
                println!("{}",v.identity)
            }
            _ => {}
        }
        Ok(())
    }
    pub async fn send_packet<T: Into<PacketKind>>(&mut self, packet: T) -> Result<()> {
        let packet: PacketKind = packet.into();
        println!("(StoC) {}", packet);
        let buffer = self.encoder.encode(packet);
        self.socket
            .send(&buffer, rust_raknet::Reliability::Reliable)
            .await
            .map_err(|e| anyhow!("{:?}", e))?;
        Ok(())
    }
    pub async fn close(&mut self) -> Result<()> {
        self.socket.close().await.map_err(|e| anyhow!("{:?}", e))?;
        println!("disconnect");
        Ok(())
    }
}
