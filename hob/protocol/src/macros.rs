#[macro_export]
macro_rules! packet_kind {
    ($($kind:ident = $id:literal)+) => {
        paste::paste! {
            use proto_bytes::{ConditionalBuf, ConditionalBufMut};
            #[derive(Debug)]
            pub enum PacketKind {
                $($kind( [<$kind Packet>] ),)*
                Unknown(u8)
            }
            impl PacketKind {
                #[inline]
                pub fn id(&self) -> u8 {
                    match self {
                        $(Self::$kind(_) => $id,)*
                        Self::Unknown(id) => *id
                    }
                }
                #[inline]
                pub fn name(&self) -> &str {
                    match self {
                        $(Self::$kind(_) => stringify!($kind),)*
                        Self::Unknown(_) => "Unknown"
                    }
                }
                #[inline]
                pub fn decode(bytes: &mut proto_bytes::BytesMut) ->  anyhow::Result<Self> {
                    let id = bytes.get_varint();
                    let packet = match id {
                        $(
                            $id => Self::$kind(Packet::decode(bytes)?),
                        )*
                        _ => Self::Unknown(id as u8),
                    };
                    Ok(packet)
                }
                #[inline]
                pub fn encode(&self,bytes: &mut proto_bytes::BytesMut) -> anyhow::Result<()> {
                    bytes.put_varint(self.id() as u64);
                    match self {
                        $(
                            Self::$kind(v) => Packet::encode(v,bytes)?,
                        )*
                        Self::Unknown(id) => anyhow::bail!("Unknown packet id:{}",id),
                    }
                    Ok(())
                }
            }
            $(
                impl From<[<$kind Packet>]> for PacketKind {
                    fn from(item: [<$kind Packet>]) -> Self {
                        Self::$kind(item)
                    }
                }
            )*
        }
    };
}
