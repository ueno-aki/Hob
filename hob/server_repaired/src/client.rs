use anyhow::Result;
use hob_protocol::packet::{login::SkinData, PacketKind};
use log::debug;
use specs::{Component, VecStorage};
use tokio::sync::mpsc::{
    error::{TryRecvError, TrySendError},
    Receiver, Sender,
};

use crate::player_init::PlayerRegistry;

#[derive(Debug)]
pub struct Client {
    pub skin: Box<SkinData>,
    pub name: String,
    packet_from_client: Receiver<PacketKind>,
    packet_to_client: Sender<PacketKind>,
}

impl Component for Client {
    type Storage = VecStorage<Self>;
}

impl Client {
    pub fn new(registry: PlayerRegistry) -> Self {
        let PlayerRegistry {
            skin,
            user,
            packet_from_client,
            packet_to_client,
        } = registry;
        Self {
            skin,
            name: user.display_name,
            packet_from_client,
            packet_to_client,
        }
    }
    pub fn try_send_packet(
        &self,
        packet: impl Into<PacketKind>,
    ) -> Result<(), TrySendError<PacketKind>> {
        let packet = packet.into();
        debug!("[{}] Send packet: {}", self.name, packet);
        self.packet_to_client.try_send(packet)
    }
    pub fn try_recv_packet(&mut self) -> Result<PacketKind, TryRecvError> {
        match self.packet_from_client.try_recv() {
            Ok(packet) => {
                debug!("[{}] Received  packet: {}", self.name, packet);
                Ok(packet)
            }
            e @ Err(_) => e,
        }
    }
    pub fn try_recv_many_packets(&mut self, max: usize) -> Result<Vec<PacketKind>, TryRecvError> {
        let mut packets = Vec::with_capacity(max);
        for _ in 0..max {
            match self.try_recv_packet() {
                Ok(packet) => packets.push(packet),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => return Err(TryRecvError::Disconnected),
            }
        }
        Ok(packets)
    }
}
