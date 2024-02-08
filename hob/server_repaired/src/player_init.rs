use hob_protocol::packet::{
    login::{ExtraUserdata, SkinData},
    PacketKind,
};
use tokio::sync::mpsc::{Receiver, Sender};

pub struct PlayerRegistry {
    pub skin: Box<SkinData>,
    pub user: ExtraUserdata,
    pub packet_from_client: Receiver<PacketKind>,
    pub packet_to_client: Sender<PacketKind>,
}
