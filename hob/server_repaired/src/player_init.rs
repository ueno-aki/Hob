use hob_protocol::packet::PacketKind;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct PlayerRegistry {
    pub packet_from_client: Receiver<PacketKind>,
    pub packet_to_client: Sender<PacketKind>,
}
