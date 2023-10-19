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
pub mod start_game;

use anyhow::Result;
use client_cache_status::ClientCacheStatusPacket;
use disconnect::DisconnectPacket;
use handshake::{ClientToServerHandshakePacket, ServerToClientHandshakePacket};
use login::LoginPacket;
use network_settings::NetworkSettingsPacket;
use play_status::PlayStatusPacket;
use protodef::prelude::*;
use request_network_setting::RequestNetworkSettingPacket;
use resource_pack_client_response::ResourcePackClientResponsePacket;
use resource_pack_stack::ResourcePacksStackPacket;
use resource_packs_info::ResourcePacksInfoPacket;

pub trait Packet {
    fn from_buf(buffer: &[u8], offset: usize) -> Result<PacketKind>;
    fn read_to_buffer(&self, vec: &mut Vec<u8>) -> Result<()>;
}

macro_rules! packet_kind {
    ($($kind:ident => $id:expr),+) => {
        #[derive(Debug)]
        pub enum PacketKind {
            $($kind($kind),)*
        }
        impl PacketKind {
            pub fn from_buf(buffer: &[u8], offset: usize) -> Result<Self> {
                let (buf_id, id_size) = buffer.read_varint(offset)?;
                let packet = match buf_id {
                    $(
                        $id => $kind::from_buf(buffer, id_size)?,
                    )*
                     _ => todo!("packet_id:{}", buf_id),
                };
                Ok(packet)
            }
            pub fn read_to_buffer(&self, vec: &mut Vec<u8>) -> Result<()> {
                match self {
                    $(
                        Self::$kind(v) => v.read_to_buffer(vec)?,
                    )*
                }
                Ok(())
            }
            pub fn id(&self) -> u64 {
                match self {
                    $(Self::$kind(_) => $id,)*
                }
            }
            pub fn name(&self) -> &str {
                match self {
                    $(Self::$kind(_) => stringify!($kind),)*
                }
            }
        }
        $(
            impl From<$kind> for PacketKind {
                fn from(value: $kind) -> Self {
                    Self::$kind(value)
                }
            }
        )*
    };
}
packet_kind![
    LoginPacket => 1,
    PlayStatusPacket => 2,
    ServerToClientHandshakePacket => 3,
    ClientToServerHandshakePacket => 4,
    DisconnectPacket => 5,
    ResourcePacksInfoPacket => 6,
    ResourcePacksStackPacket => 7,
    ResourcePackClientResponsePacket => 8,
    ClientCacheStatusPacket => 129,
    NetworkSettingsPacket => 143,
    RequestNetworkSettingPacket => 193
];
impl std::fmt::Display for PacketKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{name:{},id:{}}}", self.name(), self.id())
    }
}
