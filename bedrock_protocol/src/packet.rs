use anyhow::Result;
use bytes::BytesMut;
use proto_bytes::*;

mod login;
pub use login::*;
mod play_status;
pub use play_status::*;
mod handshake_s2c;
pub use handshake_s2c::*;
mod handshake_c2s;
pub use handshake_c2s::*;
mod disconnect;
pub use disconnect::*;
mod resource_pack_info;
pub use resource_pack_info::*;
mod resource_pack_stack;
pub use resource_pack_stack::*;
mod resource_pack_response;
pub use resource_pack_response::*;

pub trait Packet {
    fn from_bytes(bytes: &mut BytesMut) -> Result<Self> where Self: Sized;
    fn read_to_bytes(&self,bytes: &mut BytesMut) -> Result<()>;
}

macro_rules! packet_kind {
    ($($kind:ident = $id:expr)+) => {
        pub enum PacketKind {
            $($kind($kind),)*
        }
        impl PacketKind {
            pub fn id(&self) -> usize {
                match self {
                    $(Self::$kind(_) => $id,)*
                }
            }
            pub fn name(&self) -> &str {
                match self {
                    $(Self::$kind(_) => stringify!($kind),)*
                }
            }
            pub fn from_bytes(bytes: &mut BytesMut) ->  Result<Self> {
                let id = bytes.get_varint();
                let packet = match id {
                    $(
                        $id => $kind::from_bytes(bytes)?.into(),
                    )*
                     _ => todo!("packet_id:{}", id),
                };
                Ok(packet)
            }
            pub fn read_to_bytes(&self,bytes: &mut BytesMut) -> Result<()> {
                bytes.put_varint(self.id() as u64);
                match self {
                    $(
                        Self::$kind(v) => v.read_to_bytes(bytes)?,
                    )*
                }
                Ok(())
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
packet_kind!{
    LoginPacket = 1
    PlayStatusPacket = 2
    ServerToClientHandshakePacket = 3
    ClientToServerHandshakePacket = 4
    DisconnectPacket = 5
    ResourcePacksInfoPacket = 6
    ResourcePacksStackPacket = 7
    ResourcePackClientResponsePacket = 8
}