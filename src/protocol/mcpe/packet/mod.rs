mod login;
mod play_status;
mod s_to_c_handshake;
mod network_settings;
mod request_network_setting;

pub use login::{Login,login_verify};
pub use play_status::PlayStatus;
pub use s_to_c_handshake::ServerToClientHandshake;
pub use network_settings::{CompressionAlgorithmType, NetworkSettings};
pub use request_network_setting::RequestNetworkSetting;
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
    Login(Login),
    PlayStatus(PlayStatus),
    NetworkSettings(NetworkSettings),
    RequestNetworkSetting(RequestNetworkSetting),
    ServerToClientHandshake(ServerToClientHandshake)
}

use std::fmt::Display;
impl Display for PacketKind {
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
packet_impls!(Login, PlayStatus, ServerToClientHandshake, NetworkSettings, RequestNetworkSetting);
