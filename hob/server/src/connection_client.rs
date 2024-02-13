use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use hob_protocol::{decode::Decoder, encode::Encoder, packet::PacketKind};
use log::debug;
use proto_bytes::BytesMut;
use rust_raknet::RaknetSocket;
use tokio::{
    runtime::Runtime,
    sync::mpsc::{self, Receiver, Sender},
};

use crate::{
    initial_handler::{login_process, LoginResult},
    into_anyhow,
    player_registry::PlayerRegistry,
};

pub struct ConnectionClient {
    pub reader: Reader,
    pub writer: Writer,
    pub address: SocketAddr,
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
        let (packet_from_client_tx, packet_from_client_rx) = mpsc::channel(32);
        let reader = Reader::new(socket.clone(), packet_from_client_tx);
        let writer = Writer::new(socket.clone(), packet_to_client_rx);

        Self {
            reader,
            writer,
            address:socket.peer_addr().unwrap(),
            packet_from_client: packet_from_client_rx,
            packet_to_client: packet_to_client_tx,
            player_registry,
            runtime,
        }
    }
    pub fn start(mut self) {
        let runtime_ref = Arc::clone(&self.runtime);
        runtime_ref.spawn(async move {
            let result = login_process(&mut self).await;
            if result.is_err() {
                debug!("login failed: {:?}", result);
                return;
            }
            self.proceed(result.unwrap()).await;
        });
    }

    pub async fn proceed(self, result: LoginResult) {
        let Self {
            reader,
            writer,
            address,
            packet_from_client,
            packet_to_client,
            player_registry,
            runtime,
        } = self;

        match result {
            LoginResult::Success(skin, userdata) => {
                let player = PlayerRegistry {
                    skin,
                    user: userdata,
                    address,
                    packet_from_client,
                    packet_to_client,
                };
                player_registry.send(player).await.unwrap();
                Self::split(reader, writer, runtime);
            }
            LoginResult::Failed(e) => {
                debug!("login failed: {:?}", e);
            }
        }
    }

    pub fn split(reader: Reader, writer: Writer, runtime: Arc<Runtime>) {
        let reader = runtime.spawn(async move { reader.run().await });
        let writer = runtime.spawn(async move { writer.run().await });
        runtime.spawn(async move {
            tokio::select! {
                e = reader => {
                    debug!("reader finished: {:?}", e);
                },
                e = writer => {
                    debug!("writer finished {:?}", e);
                },
            }
        });
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
        self.reader.decoder.compression_ready = true;
        self.writer.encoder.compression_ready = true;
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
        loop {
            if let Some(v) = self.packet_to_client.recv().await {
                self.write(v).await?;
            }
        }
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
