pub mod client_cache_status;
pub mod disconnect;
pub mod handshake;
pub mod login;
pub mod network_settings;
pub mod play_status;
pub mod request_network_setting;
pub mod resource_pack_info;
pub mod resource_pack_response;
pub mod resource_pack_stack;
pub mod start_game;

use client_cache_status::*;
use disconnect::*;
use handshake::*;
use login::*;
use network_settings::*;
use play_status::*;
use request_network_setting::*;
use resource_pack_info::*;
use resource_pack_response::*;
use resource_pack_stack::*;
use start_game::*;

use crate::packet_kind;

pub trait Packet {
    fn decode(bytes: &mut proto_bytes::BytesMut) -> anyhow::Result<Self>
    where
        Self: Sized;
    fn encode(&self, bytes: &mut proto_bytes::BytesMut) -> anyhow::Result<()>;
}

packet_kind! {
    Login = 1
    PlayStatus = 2
    ServerToClientHandshake = 3
    ClientToServerHandshake = 4
    Disconnect = 5
    ResourcePacksInfo = 6
    ResourcePacksStack = 7
    ResourcePackClientResponse = 8
    StartGame = 0xB
    ClientCacheStatus = 0x81
    NetworkSettings = 0x8F
    RequestNetworkSetting = 0xC1
}
impl std::fmt::Display for PacketKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ name:{}, id:{} }}", self.name(), self.id())
    }
}
