mod client_cache_status;
mod disconnect;
mod handshake;
mod login;
mod network_settings;
mod play_status;
mod request_network_setting;
mod resource_pack_client_response;
mod resource_packs_info;

pub use client_cache_status::ClientCacheStatusPacket;
pub use disconnect::DisconnectPacket;
pub use handshake::{key_exchange, ClientToServerHandshakePacket, ServerToClientHandshakePacket};
pub use login::{login_verify, LoginPacket};
pub use network_settings::{CompressionAlgorithmType, NetworkSettingsPacket};
pub use play_status::PlayStatusPacket;
pub use request_network_setting::RequestNetworkSettingPacket;
pub use resource_packs_info::{BehaviourPackInfo, ResourcePackInfo, ResourcePacksInfoPacket};

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
        impl std::fmt::Display for PacketKind {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{{name:{},id:{}}}", self.get_name(), self.get_id())
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
    ResourcePacksInfoPacket
];

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
