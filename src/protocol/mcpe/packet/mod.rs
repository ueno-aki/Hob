mod login;
mod network_settings;
mod play_status;
mod request_network_setting;

pub use login::{Login,login_verify};
pub use network_settings::{CompressionAlgorithmType, NetworkSettings};
pub use play_status::PlayStatus;
pub use request_network_setting::RequestNetworkSetting;
#[macro_export]
macro_rules! packet_id {
    ($t:ty,$e:expr) => {
        impl $t {
            pub fn get_id(&self) -> u64 {
                $e
            }
            pub fn id() -> u64 {
                $e
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
}

use std::fmt::Display;
impl Display for PacketKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{id:{},{:?}}}", self.get_id(), self)
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
        }
    };
}
packet_impls!(Login, PlayStatus, NetworkSettings, RequestNetworkSetting);
