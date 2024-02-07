use std::sync::Arc;

use anyhow::Result;
use hob_protocol::{decode::Decoder, encode::Encoder, packet::PacketKind};
use proto_bytes::BytesMut;
use rust_raknet::RaknetSocket;
use tokio::{
    runtime::Runtime,
    sync::mpsc::{self, Receiver, Sender},
};

use crate::{into_anyhow, player_init::PlayerRegistry};

pub struct ConnectionClient {
    pub reader: Reader,
    pub writer: Writer,
    pub packet_from_client: Receiver<PacketKind>,
    pub packet_to_client: Sender<PacketKind>,
    pub player_registry: Sender<PlayerRegistry>,
    pub runtime: Arc<Runtime>,
}
impl ConnectionClient {
    pub fn new(
        socket: RaknetSocket,
        player_registry: Sender<PlayerRegistry>,
        runtime: Arc<Runtime>,
    ) -> Self {
        let socket = Arc::new(socket);
        let (packet_to_client_tx, packet_to_client_rx) = mpsc::channel(32);
        let (packet_from_client_tx, packet_from_client_rx) = mpsc::channel(100);
        let reader = Reader::new(socket.clone(), packet_from_client_tx);
        let writer = Writer::new(socket, packet_to_client_rx);

        Self {
            reader,
            writer,
            packet_from_client: packet_from_client_rx,
            packet_to_client: packet_to_client_tx,
            player_registry,
            runtime,
        }
    }
    pub fn start(self) {
        Arc::clone(&self.runtime).spawn(async move { self });
    }
    pub async fn read(&mut self) -> Result<Vec<PacketKind>> {
        self.reader.read().await
    }
    pub async fn write(&mut self, packet: PacketKind) -> Result<()> {
        self.writer.write(packet).await
    }
    pub fn enable_encryption(&mut self, key: &[u8; 32]) {
        self.reader.decoder.setup_cipher(key);
        self.writer.encoder.setup_cipher(key);
    }
    pub fn enable_compression(&mut self) {
        self.writer.encoder.force_compress = true
    }
}

pub struct Reader {
    socket: Arc<RaknetSocket>,
    decoder: Decoder,
    packet_from_client: Sender<PacketKind>,
}
impl Reader {
    pub fn new(socket: Arc<RaknetSocket>, packet_from_client: Sender<PacketKind>) -> Self {
        Self {
            socket,
            decoder: Decoder::default(),
            packet_from_client,
        }
    }

    pub async fn run(mut self) -> Result<()> {
        loop {
            let packets = self.read().await?;
            for packet in packets {
                self.packet_from_client.send(packet).await?;
            }
        }
    }

    pub async fn read(&mut self) -> Result<Vec<PacketKind>> {
        let buffer = self.socket.recv().await.map_err(into_anyhow)?;
        self.decoder.decode(&mut BytesMut::from(&buffer[..]))
    }
}

pub struct Writer {
    socket: Arc<RaknetSocket>,
    encoder: Encoder,
    packet_to_client: Receiver<PacketKind>,
}
impl Writer {
    pub fn new(socket: Arc<RaknetSocket>, packet_to_client: Receiver<PacketKind>) -> Self {
        Self {
            socket,
            encoder: Encoder::default(),
            packet_to_client,
        }
    }
    pub async fn run(mut self) -> Result<()> {
        while let Some(v) = self.packet_to_client.recv().await {
            self.write(v).await?;
        }
        Ok(())
    }
    pub async fn write(&mut self, packet: PacketKind) -> Result<()> {
        let buffer = self.encoder.encode(packet);
        self.socket
            .send(&buffer, rust_raknet::Reliability::ReliableOrdered)
            .await
            .map_err(into_anyhow)?;
        Ok(())
    }
}
