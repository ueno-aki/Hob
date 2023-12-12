use bytes::{Buf, BufMut, BytesMut};
use proto_bytes::*;
use serde::{de, forward_to_deserialize_any, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    pos: Position,
}

#[derive(Debug, Serialize, Deserialize)]
struct Position {
    x: f64,
    z: f64,
}

#[test]
fn lets_go() {
    let mut vec = BytesMut::new();
    
    vec.put_i8(NBTTypes::Compound as i8);
    vec.put_cstring("user");


    vec.put_i8(NBTTypes::Int as i8);
    vec.put_cstring("id");
    vec.put_i32_le(63);

    vec.put_i8(NBTTypes::Compound as i8);
    vec.put_cstring("pos");
    vec.put_i8(NBTTypes::Double as i8);
    vec.put_cstring("x");
    vec.put_f64_le(23.1);
    vec.put_i8(NBTTypes::Double as i8);
    vec.put_cstring("z");
    vec.put_f64_le(5.6);
    vec.put_i8(NBTTypes::Void as i8);

    vec.put_i8(NBTTypes::String as i8);
    vec.put_cstring("name");
    vec.put_cstring("Mark2");


    vec.put_i8(NBTTypes::Void as i8);
    
    let user = User::deserialize(&mut StructDeserializer { input: vec });
    println!("{:?}", user)
}

pub struct StructDeserializer {
    pub input: BytesMut,
}
impl<'de> de::Deserializer<'de> for &mut StructDeserializer {
    type Error = DeserializeError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(DeserializeError::Unsupported("Unsupported Type".to_owned()))
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let id = self.input.get_i8();
        assert_eq!(id,NBTTypes::Compound as i8);
        let _tag = self.input.get_cstring();
        visitor.visit_map(MapX {
            de: &mut *self,
            next_id:0,
            fields
        })
    }


    forward_to_deserialize_any! {
        i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bool
        bytes byte_buf option unit unit_struct newtype_struct seq tuple identifier
        tuple_struct map enum ignored_any
    }
}

struct MapDeserializer<'a> {
    de: &'a mut StructDeserializer,
    id: i8,
    fields:&'static [&'static str]
}
impl<'de, 'a> de::Deserializer<'de> for &mut MapDeserializer<'a> {
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
        let str = self.de.input.get_cstring();
        for v in self.fields.iter() {
            if *v == str {
                return visitor.visit_str(&str);
            }
        }
        Err(DeserializeError::Parse("Not Found TagName".to_owned()))
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        assert_eq!(self.id,NBTTypes::Byte as i8);
        let v = self.de.input.get_i8();
        visitor.visit_i8(v)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        assert_eq!(self.id,NBTTypes::Short as i8);
        let v = self.de.input.get_i16_le();
        visitor.visit_i16(v)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        assert_eq!(self.id,NBTTypes::Int as i8);
        let v = self.de.input.get_i32_le();
        visitor.visit_i32(v)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        assert_eq!(self.id,NBTTypes::Long as i8);
        let v = self.de.input.get_i64_le();
        visitor.visit_i64(v)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        assert_eq!(self.id,NBTTypes::Float as i8);
        let v = self.de.input.get_f32_le();
        visitor.visit_f32(v)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        assert_eq!(self.id,NBTTypes::Double as i8);
        let v = self.de.input.get_f64_le();
        visitor.visit_f64(v)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        assert_eq!(self.id,NBTTypes::String as i8);
        let v = self.de.input.get_cstring();
        visitor.visit_string(v)
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        assert_eq!(self.id,NBTTypes::Compound as i8);
        visitor.visit_map(MapX {
            de: &mut *self.de,
            next_id:0,
            fields
        })
    }

    forward_to_deserialize_any! {
        i128 u8 u16 u32 u64 u128 char str bool
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map enum ignored_any
    }
}

struct MapX<'a> {
    de: &'a mut StructDeserializer,
    next_id:i8,
    fields:&'static [&'static str]
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
        seed.deserialize(&mut MapDeserializer { de: &mut *self.de ,id:self.next_id,fields:self.fields}).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut MapDeserializer { de: &mut *self.de ,id:self.next_id,fields:self.fields})
    }
}

use thiserror::Error;

use crate::types::NBTTypes;
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
