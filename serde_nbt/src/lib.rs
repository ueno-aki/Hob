pub mod compound;
pub mod nbt_types;
mod binary_format;
#[cfg(test)]
mod test;

pub use binary_format::{LittleEndian,VarInt};
use compound::{CompoundDeserializer, DeserializeError};
use serde::de;

macro_rules! impl_from_buf {
    ($($f:ty),*) => {
        $(
            impl $f {
                pub fn from_buffer<'a,D>(buf:&[u8]) -> Result<D, DeserializeError>
                where
                    D:de::Deserialize<'a>
                {
                    let mut deserializer = CompoundDeserializer::<$f>::new(buf);
                    D::deserialize(&mut deserializer)
                }
            }
        )*
    };
}
impl_from_buf!(LittleEndian,VarInt);