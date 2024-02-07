use std::sync::Arc;

use anyhow::{anyhow, Result};
use flume::{Receiver, Sender};
use hob_protocol::packet::PacketKind;
use log::debug;
use rust_raknet::RaknetSocket;
use tokio::runtime::Runtime;

use crate::player_init::NewPlayer;

pub struct Worker {
    reader: Reader,
    writer: Writer,
    runtime: Arc<Runtime>,
    received_packet: Receiver<PacketKind>,
    packet_to_send: Sender<PacketKind>,
    new_players: Sender<NewPlayer>,
}

impl Worker {
    pub fn new(
        stream: RaknetSocket,
        runtime: Arc<Runtime>,
        new_players: Sender<NewPlayer>,
    ) -> Self {
        let stream = Arc::new(stream);
        let (received_packets_tx, received_packets_rx) = flume::bounded(32);
        let (packets_to_send_tx, packets_to_send_rx) = flume::unbounded();

        let reader = Reader::new(Arc::clone(&stream), received_packets_tx);
        let writer = Writer::new(stream, packets_to_send_rx);

        Self {
            reader,
            writer,
            runtime,
            received_packet: received_packets_rx,
            packet_to_send: packets_to_send_tx,
            new_players,
        }
    }

    pub fn start(self) {
        tokio::task::spawn(async move {
            self.run().await;
        });
    }

    pub async fn run(self) {
        self.new_players
            .send_async(NewPlayer {
                received_packet: self.received_packet.clone(),
                packet_to_send: self.packet_to_send.clone(),
            })
            .await
            .unwrap();
        self.split();
    }

    fn split(self) {
        let Self {
            reader,
            writer,
            runtime,
            ..
        } = self;
        let reader = runtime.spawn(async move { reader.run().await });
        let writer = runtime.spawn(async move { writer.run().await });
        runtime.spawn(async move {
            tokio::select! {
                _ = reader => {
                    log::debug!("lost connection")
                }
                _ = writer => {
                    log::debug!("lost connection")
                }
            };
            //
        });
    }
}

pub struct Reader {
    stream: Arc<RaknetSocket>,
    received_packets: Sender<PacketKind>,
}
impl Reader {
    pub fn new(stream: Arc<RaknetSocket>, received_packets: Sender<PacketKind>) -> Self {
        Self {
            stream,
            received_packets,
        }
    }
    pub async fn run(mut self) -> Result<()> {
        loop {
            let packet = self.read().await?;
            let result = self.received_packets.send_async(packet).await;
            if result.is_err() {
                return Ok(());
            }
        }
    }
    pub async fn read(&mut self) -> Result<PacketKind> {
        let buffer = self.stream.recv().await.map_err(|e| anyhow!("{e:?}"))?;
        Ok(PacketKind::Unknown)
        // loop {
        //     //if let Some(v) = try_next_packet{ return Ok(v) }
        //   recv
        //     //decode into Packets...
        // }
    }
}

pub struct Writer {
    stream: Arc<RaknetSocket>,
    packet_to_send: Receiver<PacketKind>,
}
impl Writer {
    pub fn new(stream: Arc<RaknetSocket>, packet_to_send: Receiver<PacketKind>) -> Self {
        Self {
            stream,
            packet_to_send,
        }
    }
    pub async fn run(mut self) -> Result<()> {
        while let Ok(v) = self.packet_to_send.recv_async().await {
            self.write(v).await?;
        }
        Ok(())
    }
    pub async fn write(&mut self, packet: PacketKind) -> Result<()> {
        // self.stream.send(, r)
        debug!("Send Packet {}", packet);
        Ok(())
    }
}
