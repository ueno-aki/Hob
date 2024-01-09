pub mod de;
mod macros;
pub mod nbt_tag;
pub mod ser;

use crate::de::{error::DeserializeError, Deserializer};
use ser::{error::SerializeError, Serializer};
use serde::{de::Deserialize, Serialize};

pub struct BigEndian;
pub struct LittleEndian;
pub struct VarInt;

macro_rules! impl_buffer {
    ($($f:ty),*) => {
        $(
            impl $f {
                pub fn from_slice<'a,D>(buf:&'a [u8]) -> Result<D, DeserializeError>
                where
                    D:Deserialize<'a>
                {
                    let mut deserializer = Deserializer::<$f>::from_slice(buf);
                    D::deserialize(&mut deserializer)
                }
                pub fn to_vec<S>(v:S) -> Result<Vec<u8>,SerializeError>
                where S:Serialize
                {
                    let mut serializer = Serializer::<$f>::default();
                    v.serialize(&mut serializer)?;
                    Ok(serializer.output.to_vec())
                }
            }
        )*
    };
}
impl_buffer!(BigEndian, LittleEndian, VarInt);
