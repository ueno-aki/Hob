mod client_cache_status;
mod disconnect;
mod handshake;
mod login;
mod network_settings;
mod play_status;
mod request_network_setting;
mod resource_packs_info;

pub use client_cache_status::ClientCacheStatusPacket;
pub use disconnect::DisconnectPacket;
pub use handshake::{key_exchange, ClientToServerHandshakePacket, ServerToClientHandshakePacket};
pub use login::{login_verify, LoginPacket};
pub use network_settings::{CompressionAlgorithmType, NetworkSettingsPacket};
pub use play_status::PlayStatusPacket;
pub use request_network_setting::RequestNetworkSettingPacket;
pub use resource_packs_info::{BehaviourPackInfo, ResourcePackInfo, ResourcePacksInfoPacket};

#[macro_export]
macro_rules! packet_feature {
    ($t:ty,$id:expr,$name:expr) => {
        impl $t {
            pub fn get_id(&self) -> u64 {
                $id
            }
            pub fn id() -> u64 {
                $id
            }
            pub fn name(&self) -> String {
                $name.to_owned()
            }
        }
    };
}

#[derive(Debug)]
pub enum PacketKind {
    LoginPacket(LoginPacket),
    PlayStatusPacket(PlayStatusPacket),
    ServerToClientHandshakePacket(ServerToClientHandshakePacket),
    ClientToServerHandshakePacket(ClientToServerHandshakePacket),
    DisconnectPacket(DisconnectPacket),
    ClientCacheStatusPacket(ClientCacheStatusPacket),
    NetworkSettingsPacket(NetworkSettingsPacket),
    RequestNetworkSettingPacket(RequestNetworkSettingPacket),
    ResourcePacksInfoPacket(ResourcePacksInfoPacket),
}

impl std::fmt::Display for PacketKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{name:{},id:{}}}", self.get_name(), self.get_id())
    }
}

macro_rules! packet_impls {
    ($($t:ident),*) => {
        $(
            impl From<$t> for PacketKind {
                fn from(value: $t) -> Self {
                    PacketKind::$t(value)
                }
            }
            impl From<PacketKind> for $t {
                fn from(value: PacketKind) -> Self {
                    match value {
                        PacketKind::$t(kind) => kind,
                        _ => panic!("Invalid PacketKind")
                    }
                }
            }
        )*
        impl PacketKind {
            pub fn get_id(&self) -> u64{
                match self {
                    $(PacketKind::$t(kind) => kind.get_id(),)*
                }
            }
            pub fn get_name(&self) -> String{
                match self {
                    $(PacketKind::$t(kind) => kind.name(),)*
                }
            }
        }
    };
}
packet_impls![
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
