use flume::{Receiver, Sender};
use hob_protocol::packet::PacketKind;

pub struct NewPlayer {
    pub received_packet: Receiver<PacketKind>,
    pub packet_to_send: Sender<PacketKind>,
}
