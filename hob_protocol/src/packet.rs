use std::fmt;

use anyhow::Result;
use proto_bytes::{BytesMut, ConditionalReader, ConditionalWriter};

mod login;
pub use login::*;
mod play_status;
pub use play_status::*;
mod handshake;
pub use handshake::*;
mod disconnect;
pub use disconnect::*;
mod resource_pack_info;
pub use resource_pack_info::*;
mod resource_pack_stack;
pub use resource_pack_stack::*;
mod resource_pack_response;
pub use resource_pack_response::*;
mod request_network_setting;
pub use request_network_setting::*;
mod network_settings;
pub use network_settings::*;

pub trait Packet {
    fn decode(bytes: &mut BytesMut) -> Result<Self>
    where
        Self: Sized;
    fn encode(&self, bytes: &mut BytesMut) -> Result<()>;
}

macro_rules! packet_kind {
    ($($kind:ident = $id:expr)+) => {
        #[derive(Debug)]
        pub enum PacketKind {
            $($kind($kind),)*
        }
        impl PacketKind {
            #[inline]
            pub fn id(&self) -> usize {
                match self {
                    $(Self::$kind(_) => $id,)*
                }
            }
            #[inline]
            pub fn name(&self) -> &str {
                match self {
                    $(Self::$kind(_) => stringify!($kind),)*
                }
            }
            #[inline]
            pub fn decode(bytes: &mut BytesMut) ->  Result<Self> {
                let id = bytes.get_varint();
                let packet = match id {
                    $(
                        $id => $kind::decode(bytes)?.into(),
                    )*
                     _ => todo!("packet_id:{}", id),
                };
                Ok(packet)
            }
            #[inline]
            pub fn encode(&self,bytes: &mut BytesMut) -> Result<()> {
                bytes.put_varint(self.id() as u64);
                match self {
                    $(
                        Self::$kind(v) => v.encode(bytes)?,
                    )*
                }
                Ok(())
            }
        }
        $(
            impl From<$kind> for PacketKind {
                #[inline]
                fn from(value: $kind) -> Self {
                    Self::$kind(value)
                }
            }
        )*
    };
}
packet_kind! {
    LoginPacket = 1
    PlayStatusPacket = 2
    ServerToClientHandshakePacket = 3
    ClientToServerHandshakePacket = 4
    DisconnectPacket = 5
    ResourcePacksInfoPacket = 6
    ResourcePacksStackPacket = 7
    ResourcePackClientResponsePacket = 8
    NetworkSettingsPacket = 143
    RequestNetworkSettingPacket = 193
}
impl fmt::Display for PacketKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ name:{}, id:{} }}", self.name(), self.id())
    }
}
