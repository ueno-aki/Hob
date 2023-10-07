pub mod client_cache_status;
pub mod disconnect;
pub mod handshake;
pub mod login;
pub mod network_settings;
pub mod play_status;
pub mod request_network_setting;
pub mod resource_pack_client_response;
pub mod resource_pack_stack;
pub mod resource_packs_info;
pub mod start_game_packet;

use client_cache_status::ClientCacheStatusPacket;
use disconnect::DisconnectPacket;
use handshake::{ClientToServerHandshakePacket, ServerToClientHandshakePacket};
use login::LoginPacket;
use network_settings::NetworkSettingsPacket;
use play_status::PlayStatusPacket;
use request_network_setting::RequestNetworkSettingPacket;
use resource_pack_client_response::ResourcePackClientResponsePacket;
use resource_pack_stack::ResourcePacksStackPacket;
use resource_packs_info::ResourcePacksInfoPacket;

macro_rules! packet_kind_enum {
    ($($kind:ident),*) => {
        #[derive(Debug)]
        pub enum PacketKind {
            $($kind($kind),)*
        }
        impl PacketKind {
            pub fn get_id(&self) -> u64{
                match self {
                    $(PacketKind::$kind(v) => v.get_id(),)*
                }
            }
            pub fn get_name(&self) -> &str{
                match self {
                    $(PacketKind::$kind(v) => v.name(),)*
                }
            }
        }
        $(
            impl From<$kind> for PacketKind {
                fn from(value: $kind) -> Self {
                    PacketKind::$kind(value)
                }
            }
        )*
    };
}

packet_kind_enum![
    LoginPacket,
    PlayStatusPacket,
    ServerToClientHandshakePacket,
    ClientToServerHandshakePacket,
    DisconnectPacket,
    ClientCacheStatusPacket,
    NetworkSettingsPacket,
    RequestNetworkSettingPacket,
    ResourcePacksInfoPacket,
    ResourcePackClientResponsePacket,
    ResourcePacksStackPacket
];
impl std::fmt::Display for PacketKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{name:{},id:{}}}", self.get_name(), self.get_id())
    }
}

#[macro_export]
macro_rules! packet_ids {
    ($t:ty,$id:expr,$name:expr) => {
        impl $t {
            pub fn get_id(&self) -> u64 {
                $id
            }
            pub fn id() -> u64 {
                $id
            }
            pub fn name(&self) -> &str {
                $name
            }
        }
    };
}
