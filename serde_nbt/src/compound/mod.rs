use std::marker::PhantomData;

use bytes::{Buf, BytesMut};
use serde::{de, forward_to_deserialize_any};

use crate::{binary_format::BinaryFormat, types::NBTTypes};

pub struct CompoundDeserializer<T>
where
    T: BinaryFormat,
{
    pub input: BytesMut,
    _marker: PhantomData<T>,
}

impl<T> CompoundDeserializer<T>
where
    T: BinaryFormat,
{
    pub fn new(buf: &[u8]) -> Self {
        CompoundDeserializer {
            input: BytesMut::from(buf),
            _marker: PhantomData,
        }
    }
    fn get_byte(&mut self) -> i8 {
        T::byte(&mut self.input)
    }
    fn get_short(&mut self) -> i16 {
        T::short(&mut self.input)
    }
    fn get_int(&mut self) -> i32 {
        T::int(&mut self.input)
    }
    fn get_long(&mut self) -> i64 {
        T::long(&mut self.input)
    }
    fn get_float(&mut self) -> f32 {
        T::float(&mut self.input)
    }
    fn get_double(&mut self) -> f64 {
        T::double(&mut self.input)
    }
    fn get_string(&mut self) -> String {
        T::string(&mut self.input)
    }
    fn eat_value(&mut self, types: NBTTypes) {
        use NBTTypes::*;
        match types {
            Void => {},
            Byte => T::eat_byte(&mut self.input),
            Short => T::eat_short(&mut self.input),
            Int => T::eat_int(&mut self.input),
            Long => T::eat_long(&mut self.input),
            Float => T::eat_float(&mut self.input),
            Double => T::eat_double(&mut self.input),
            ByteArray => {
                let len = self.get_int() as usize;
                self.input.advance(len);
            },
            String => T::eat_string(&mut self.input),
            List => {
                let elem_types = NBTTypes::from_i8(self.get_byte()).unwrap();
                let len = self.get_int() as usize;
                for _ in 0..len {
                    self.eat_value(elem_types.clone())
                }
            }
            Compound => loop {
                let id = self.get_byte();
                if id == 0 {
                    break;
                }
                self.eat_value(String);
                self.eat_value(NBTTypes::from_i8(id).unwrap());
            },
            IntArray => {
                let len = self.get_int() as usize;
                self.input.advance(len * 4);
            }
            LongArray => {
                let len = self.get_int() as usize;
                self.input.advance(len * 8);
            }
        }
    }
    
}

impl<'de,T> de::Deserializer<'de> for &mut CompoundDeserializer<T> 
where
    T: BinaryFormat
{
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
        let id = self.get_byte();
        let _tag = self.get_string();
        assert_eq!(id, NBTTypes::Compound as i8);
        visitor.visit_map(MapX {
            de: &mut *self,
            next_id: NBTTypes::Void,
            fields,
        })
    }

    forward_to_deserialize_any! {
        i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bool
        bytes byte_buf option unit unit_struct newtype_struct seq tuple identifier
        tuple_struct map enum ignored_any
    }
}

struct ValueDeserializer<'a,T> 
where
    T: BinaryFormat
{
    de: &'a mut CompoundDeserializer<T>,
    id: NBTTypes,
}
impl<'de, 'a, T> de::Deserializer<'de> for &mut ValueDeserializer<'a,T> 
where
    T: BinaryFormat
{
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
        assert_eq!(self.id, NBTTypes::Byte);
        let v = self.de.get_byte();
        visitor.visit_i8(v)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        assert_eq!(self.id, NBTTypes::Short);
        let v = self.de.get_short();
        visitor.visit_i16(v)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        assert_eq!(self.id, NBTTypes::Int);
        let v = self.de.get_int();
        visitor.visit_i32(v)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        assert_eq!(self.id, NBTTypes::Long);
        let v = self.de.get_long();
        visitor.visit_i64(v)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        assert_eq!(self.id, NBTTypes::Float);
        let v = self.de.get_float();
        visitor.visit_f32(v)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        assert_eq!(self.id, NBTTypes::Double);
        let v = self.de.get_double();
        visitor.visit_f64(v)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        assert_eq!(self.id, NBTTypes::String);
        let v = self.de.get_string();
        visitor.visit_string(v)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.id {
            NBTTypes::List => {
                let id = NBTTypes::from_i8(self.de.get_byte()).unwrap();
                let len = self.de.get_int() as usize;
                visitor.visit_seq(SeqX {
                    de: &mut *self.de,
                    id,
                    len,
                })
            }
            NBTTypes::ByteArray => {
                let len = self.de.get_int() as usize;
                visitor.visit_seq(NumSeqX {
                    input: &mut self.de.input,
                    len,
                })
            }
            NBTTypes::IntArray => {
                let len = self.de.get_int() as usize;
                visitor.visit_seq(NumSeqX {
                    input: &mut self.de.input,
                    len,
                })
            }
            NBTTypes::LongArray => {
                let len = self.de.get_int() as usize;
                visitor.visit_seq(NumSeqX {
                    input: &mut self.de.input,
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
        assert_eq!(self.id, NBTTypes::Compound);
        visitor.visit_map(MapX {
            de: &mut *self.de,
            next_id: NBTTypes::Void,
            fields,
        })
    }

    forward_to_deserialize_any! {
        i128 u8 u16 u32 u64 u128 char str bool
        bytes byte_buf option unit unit_struct newtype_struct tuple identifier
        tuple_struct map enum
    }
}

struct IdentifierDeserializer<'a,T> 
where
    T: BinaryFormat
{
    de: &'a mut CompoundDeserializer<T>,
    fields: &'static [&'static str],
    id: NBTTypes,
}
impl<'de, 'a,T> de::Deserializer<'de> for &mut IdentifierDeserializer<'a,T> 
where
    T: BinaryFormat
{
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
        let str = self.de.get_string();
        for v in self.fields {
            if *v == str {
                return visitor.visit_str(&str);
            }
        }
        self.de.eat_value(self.id);
        visitor.visit_str(&str)
    }

    forward_to_deserialize_any! {
        i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bool
        bytes byte_buf option unit unit_struct newtype_struct seq tuple struct
        tuple_struct map enum ignored_any
    }
}

struct MapX<'a,T> 
where
    T: BinaryFormat
{
    de: &'a mut CompoundDeserializer<T>,
    next_id: NBTTypes,
    fields: &'static [&'static str],
}
impl<'de, 'a,T> de::MapAccess<'de> for MapX<'a,T> 
where
    T: BinaryFormat
{
    type Error = DeserializeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        self.next_id = NBTTypes::from_i8(self.de.get_byte()).unwrap();
        match self.next_id {
            NBTTypes::Void => Ok(None),
            _ => {
                seed.deserialize(&mut IdentifierDeserializer {
                    de: &mut *self.de,
                    fields: self.fields,
                    id: self.next_id,
                })
                .map(Some)  
            }
        }
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

struct SeqX<'a,T> 
where
    T: BinaryFormat
{
    de: &'a mut CompoundDeserializer<T>,
    id: NBTTypes,
    len: usize,
}
impl<'de, 'a, T> de::SeqAccess<'de> for SeqX<'a,T> 
where
    T: BinaryFormat
{
    type Error = DeserializeError;

    fn next_element_seed<E>(&mut self, seed: E) -> Result<Option<E::Value>, Self::Error>
    where
        E: de::DeserializeSeed<'de>,
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

struct NumSeqX<'a> {
    input: &'a mut BytesMut,
    len: usize,
}
impl<'de, 'a> de::SeqAccess<'de> for NumSeqX<'a> {
    type Error = DeserializeError;

    fn next_element_seed<E>(&mut self, seed: E) -> Result<Option<E::Value>, Self::Error>
    where
        E: de::DeserializeSeed<'de>,
    {
        struct NumArrayDeserializer<'a> {
            input: &'a mut BytesMut,
        }
        impl<'de,'a> de::Deserializer<'de> for &mut NumArrayDeserializer<'a> {
            type Error = DeserializeError;
            fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
            where
                V: de::Visitor<'de>,
            {
                Err(DeserializeError::Unsupported("Unsupported Type".to_owned()))
            }

            fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
                where
                    V: de::Visitor<'de> {
                let v = self.input.get_i8();
                visitor.visit_i8(v)
            }
            fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
                where
                    V: de::Visitor<'de> {
                let v = self.input.get_i32_le();
                visitor.visit_i32(v)
            }
            fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
                where
                    V: de::Visitor<'de> {
                let v = self.input.get_i64_le();
                visitor.visit_i64(v)
            }
            
            forward_to_deserialize_any! {
                i16 i128 u8 u16 u32 u64 u128 f32 f64 char str string bool
                bytes byte_buf option unit unit_struct newtype_struct struct seq tuple identifier
                tuple_struct map enum ignored_any
            }
        }

        if self.len == 0 {
            return Ok(None);
        }
        self.len -= 1;
        seed.deserialize(&mut NumArrayDeserializer {
            input: self.input,
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
