use hob_protocol::packet::PacketKind;
use specs::Component;
use tokio::sync::mpsc::{error::TryRecvError, Receiver, Sender};

pub struct DisplayNameComponent(pub String);
impl Component for DisplayNameComponent {
    type Storage = specs::VecStorage<Self>;
}

pub struct ConnectionStreamComponent {
    name: String,
    pub packet_from_client: Receiver<PacketKind>,
    pub packet_to_client: Sender<PacketKind>,
}
impl ConnectionStreamComponent {
    pub fn new(
        packet_from_client: Receiver<PacketKind>,
        packet_to_client: Sender<PacketKind>,
        name: &str,
    ) -> Self {
        ConnectionStreamComponent {
            packet_from_client,
            packet_to_client,
            name: name.to_owned(),
        }
    }
    pub fn send_packet(&mut self, packet: impl Into<PacketKind>) {
        let packet = packet.into();
        log::debug!("[{}] Send packet: {}", self.name, packet);
        if let Err(e) = self.packet_to_client.try_send(packet) {
            log::error!("Error sending packet: {:?}", e);
        }
    }
    pub fn try_recv_packet(&mut self) -> Result<PacketKind, TryRecvError> {
        match self.packet_from_client.try_recv() {
            Ok(packet) => {
                log::debug!("[{}] Received  packet: {}", self.name, packet);
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
                Err(e) => return Err(e),
            }
        }
        Ok(packets)
    }
}
impl Component for ConnectionStreamComponent {
    type Storage = specs::VecStorage<Self>;
}
