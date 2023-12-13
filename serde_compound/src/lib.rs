use bytes::{Buf, BytesMut};
use proto_bytes::*;
use serde::{de, forward_to_deserialize_any};
use skip::skip_value;
use types::NBTTypes;

mod skip;
#[cfg(test)]
mod test;
pub mod types;

pub fn from_buffer<'a, D>(buf: &'a [u8]) -> Result<D, DeserializeError>
where
    D: de::Deserialize<'a>,
{
    D::deserialize(&mut CompoundDeserializer::new(buf))
}

pub struct CompoundDeserializer {
    pub input: BytesMut,
}
impl CompoundDeserializer {
    pub fn new(buf: &[u8]) -> Self {
        CompoundDeserializer {
            input: BytesMut::from(buf),
        }
    }
}
impl<'de> de::Deserializer<'de> for &mut CompoundDeserializer {
    type Error = DeserializeError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(DeserializeError::Unsupported("Unsupported Type".to_owned()))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let id = self.input.get_i8();
        assert_eq!(id, NBTTypes::Compound as i8);
        let _tag = self.input.get_short_string();
        visitor.visit_map(MapX {
            de: &mut *self,
            next_id: 0,
            fields,
        })
    }

    forward_to_deserialize_any! {
        i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bool
        bytes byte_buf option unit unit_struct newtype_struct seq tuple identifier
        tuple_struct map enum ignored_any
    }
}

struct ValueDeserializer<'a> {
    de: &'a mut CompoundDeserializer,
    id: i8,
}
impl<'de, 'a> de::Deserializer<'de> for &mut ValueDeserializer<'a> {
    type Error = DeserializeError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(DeserializeError::Unsupported("Unsupported Type".to_owned()))
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        assert_eq!(self.id, NBTTypes::Byte as i8);
        let v = self.de.input.get_i8();
        visitor.visit_i8(v)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        assert_eq!(self.id, NBTTypes::Short as i8);
        let v = self.de.input.get_i16_le();
        visitor.visit_i16(v)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        assert_eq!(self.id, NBTTypes::Int as i8);
        let v = self.de.input.get_i32_le();
        visitor.visit_i32(v)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        assert_eq!(self.id, NBTTypes::Long as i8);
        let v = self.de.input.get_i64_le();
        visitor.visit_i64(v)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        assert_eq!(self.id, NBTTypes::Float as i8);
        let v = self.de.input.get_f32_le();
        visitor.visit_f32(v)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        assert_eq!(self.id, NBTTypes::Double as i8);
        let v = self.de.input.get_f64_le();
        visitor.visit_f64(v)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        assert_eq!(self.id, NBTTypes::String as i8);
        let v = self.de.input.get_short_string();
        visitor.visit_string(v)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.id {
            n if n == NBTTypes::ByteArray as i8 => {
                let len = self.de.input.get_i32_le() as isize;
                visitor.visit_seq(SeqX {
                    de: &mut *self.de,
                    id: NBTTypes::Byte as i8,
                    len,
                })
            }
            n if n == NBTTypes::List as i8 => {
                let id = self.de.input.get_i8();
                let len = self.de.input.get_i32_le() as isize;
                visitor.visit_seq(SeqX {
                    de: &mut *self.de,
                    id,
                    len,
                })
            }
            n if n == NBTTypes::IntArray as i8 => {
                let len = self.de.input.get_i32_le() as isize;
                visitor.visit_seq(SeqX {
                    de: &mut *self.de,
                    id: NBTTypes::Int as i8,
                    len,
                })
            }
            n if n == NBTTypes::LongArray as i8 => {
                let len = self.de.input.get_i32_le() as isize;
                visitor.visit_seq(SeqX {
                    de: &mut *self.de,
                    id: NBTTypes::Long as i8,
                    len,
                })
            }
            _ => Err(DeserializeError::Unsupported(
                "Unsupported Seq Type".to_owned(),
            )),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        assert_eq!(self.id, NBTTypes::Compound as i8);
        visitor.visit_map(MapX {
            de: &mut *self.de,
            next_id: 0,
            fields,
        })
    }

    forward_to_deserialize_any! {
        i128 u8 u16 u32 u64 u128 char str bool
        bytes byte_buf option unit unit_struct newtype_struct tuple identifier
        tuple_struct map enum
    }
}

struct IdentifierDeserializer<'a> {
    de: &'a mut CompoundDeserializer,
    fields: &'static [&'static str],
    id: i8,
}
impl<'de, 'a> de::Deserializer<'de> for &mut IdentifierDeserializer<'a> {
    type Error = DeserializeError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(DeserializeError::Unsupported("Unsupported Type".to_owned()))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let str = self.de.input.get_short_string();
        for v in self.fields {
            if *v == str {
                return visitor.visit_str(&str);
            }
        }
        skip_value(NBTTypes::from_i8(self.id).unwrap(), &mut self.de.input);
        visitor.visit_str(&str)
    }

    forward_to_deserialize_any! {
        i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bool
        bytes byte_buf option unit unit_struct newtype_struct seq tuple struct
        tuple_struct map enum ignored_any
    }
}

struct MapX<'a> {
    de: &'a mut CompoundDeserializer,
    next_id: i8,
    fields: &'static [&'static str],
}
impl<'de, 'a> de::MapAccess<'de> for MapX<'a> {
    type Error = DeserializeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        self.next_id = self.de.input.get_i8();
        if self.next_id == NBTTypes::Void as i8 {
            return Ok(None);
        }
        seed.deserialize(&mut IdentifierDeserializer {
            de: &mut *self.de,
            fields: self.fields,
            id: self.next_id,
        })
        .map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut ValueDeserializer {
            de: &mut *self.de,
            id: self.next_id,
        })
    }
}

struct SeqX<'a> {
    de: &'a mut CompoundDeserializer,
    id: i8,
    len: isize,
}
impl<'de, 'a> de::SeqAccess<'de> for SeqX<'a> {
    type Error = DeserializeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.len == 0 {
            return Ok(None);
        }
        self.len -= 1;
        seed.deserialize(&mut ValueDeserializer {
            de: &mut *self.de,
            id: self.id,
        })
        .map(Some)
    }
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeserializeError {
    #[error("Failed to parse {0}")]
    Parse(String),
    #[error("Unsupported: {0}")]
    Unsupported(String),
    #[error("DeserializeError:{0}")]
    Message(String),
}
impl de::Error for DeserializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        DeserializeError::Message(msg.to_string())
    }
}
