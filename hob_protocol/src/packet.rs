use std::fmt;

use anyhow::Result;
use proto_bytes::{BytesMut, ConditionalReader, ConditionalWriter};

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

pub trait Packet {
    fn decode(bytes: &mut BytesMut) -> Result<Self>
    where
        Self: Sized;
    fn encode(&self, bytes: &mut BytesMut) -> Result<()>;
}

macro_rules! packet_kind {
    ($($kind:ident = $id:expr)+) => {
        paste::paste! {
            #[derive(Debug)]
            pub enum PacketKind {
                $($kind( [<$kind Packet>] ),)*
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
                            $id => [<$kind Packet>]::decode(bytes)?.into(),
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
                impl From< [<$kind Packet>] > for PacketKind {
                    #[inline]
                    fn from(value: [<$kind Packet>]) -> Self {
                        Self::$kind(value)
                    }
                }
            )*
        }
    };
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
    ClientCacheStatus = 129
    NetworkSettings = 143
    RequestNetworkSetting = 193
}
impl fmt::Display for PacketKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ name:{}, id:{} }}", self.name(), self.id())
    }
}
