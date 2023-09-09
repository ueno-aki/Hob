mod request_network_setting;
pub mod Packets {
    pub use super::request_network_setting::RequestNetworkSetting;
    pub enum PacketKind {
        RequestNetworkSetting(RequestNetworkSetting)
    }
    
}

#[macro_export]
macro_rules! packet_id {
    ($t:ty,$e:expr) => {
        impl $t {
            pub fn get_id(&self) -> u64 {
                $e
            }
        }
    };
}