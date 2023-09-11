mod play_status;
mod network_settings;
mod request_network_setting;
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

pub use play_status::PlayStatus;
pub use network_settings::{NetworkSettings,CompressionAlgorithmType};
pub use request_network_setting::RequestNetworkSetting;
#[derive(Debug)]
pub enum PacketKind {
    PlayStatus(PlayStatus),
    NetworkSettings(NetworkSettings),
    RequestNetworkSetting(RequestNetworkSetting),
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
packet_impls!(PlayStatus,NetworkSettings,RequestNetworkSetting);
