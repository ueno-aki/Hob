pub mod de;
pub mod nbt_tag;
pub mod ser;
#[cfg(test)]
mod test;

use crate::de::{error::DeserializeError, Deserializer};
use serde::de::Deserialize;

pub struct BigEndian;
pub struct LittleEndian;
pub struct VarInt;

macro_rules! impl_from_buf {
    ($($f:ty),*) => {
        $(
            impl $f {
                pub fn from_buffer<'a,D>(buf:&[u8]) -> Result<D, DeserializeError>
                where
                    D:Deserialize<'a>
                {
                    let mut deserializer = Deserializer::<$f>::new(buf);
                    D::deserialize(&mut deserializer)
                }
            }
        )*
    };
}
impl_from_buf!(BigEndian, LittleEndian, VarInt);
