pub mod binary_format;
pub mod error;

use bytes::{Buf, BytesMut};
use serde::{de, forward_to_deserialize_any};
use std::marker::PhantomData;
use self::{binary_format::BinaryFormat, error::DeserializeError};
use crate::nbt_types::NBTTypes;

macro_rules! cmp_type {
    ($x:expr , $y:expr) => {
        if $x == $y {
            Ok(())
        } else {
            Err(DeserializeError::MissMatch($x, $y))
        }
    };
}

pub struct Deserializer<T>
where
    T: BinaryFormat,
{
    pub input: BytesMut,
    _marker: PhantomData<T>,
}

impl<T> Deserializer<T>
where
    T: BinaryFormat,
{
    pub fn new(buf: &[u8]) -> Self {
        Deserializer {
            input: BytesMut::from(buf),
            _marker: PhantomData,
        }
    }
    #[inline]
    fn get_byte(&mut self) -> i8 {
        T::byte(&mut self.input)
    }
    #[inline]
    fn get_short(&mut self) -> i16 {
        T::short(&mut self.input)
    }
    #[inline]
    fn get_int(&mut self) -> i32 {
        T::int(&mut self.input)
    }
    #[inline]
    fn get_long(&mut self) -> i64 {
        T::long(&mut self.input)
    }
    #[inline]
    fn get_float(&mut self) -> f32 {
        T::float(&mut self.input)
    }
    #[inline]
    fn get_double(&mut self) -> f64 {
        T::double(&mut self.input)
    }
    #[inline]
    fn get_string(&mut self) -> String {
        T::string(&mut self.input)
    }
    fn eat_value(&mut self, types: NBTTypes) {
        use NBTTypes::*;
        match types {
            Void => {}
            Byte => T::eat_byte(&mut self.input),
            Short => T::eat_short(&mut self.input),
            Int => T::eat_int(&mut self.input),
            Long => T::eat_long(&mut self.input),
            Float => T::eat_float(&mut self.input),
            Double => T::eat_double(&mut self.input),
            ByteArray => {
                let len = self.get_int() as usize;
                self.input.advance(len);
            }
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

impl<'de, T> de::Deserializer<'de> for &mut Deserializer<T>
where
    T: BinaryFormat,
{
    type Error = DeserializeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let id = NBTTypes::from_i8(self.get_byte()).unwrap();
        let _tag = self.get_string();
        let var = &mut Variant {
            de: &mut *self,
            types: id,
        };
        var.deserialize_any(visitor)
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
        let id = NBTTypes::from_i8(self.get_byte()).unwrap();
        let _tag = self.get_string();
        cmp_type!(id, NBTTypes::Compound).and(visitor.visit_map(MapX {
            de: &mut *self,
            next_types: NBTTypes::Void,
            fields,
        }))
    }

    forward_to_deserialize_any! {
        i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bool
        bytes byte_buf option unit unit_struct newtype_struct seq tuple identifier
        tuple_struct map enum ignored_any
    }
}

struct Variant<'a, T>
where
    T: BinaryFormat,
{
    de: &'a mut Deserializer<T>,
    types: NBTTypes,
}
impl<'de, 'a, T> de::Deserializer<'de> for &mut Variant<'a, T>
where
    T: BinaryFormat,
{
    type Error = DeserializeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        use NBTTypes::*;
        match self.types {
            Byte => visitor.visit_i8(self.de.get_byte()),
            Short => visitor.visit_i16(self.de.get_short()),
            Int => visitor.visit_i32(self.de.get_int()),
            Long => visitor.visit_i64(self.de.get_long()),
            Float => visitor.visit_f32(self.de.get_float()),
            Double => visitor.visit_f64(self.de.get_double()),
            String => visitor.visit_string(self.de.get_string()),
            List => {
                let id = NBTTypes::from_i8(self.de.get_byte()).unwrap();
                let len = self.de.get_int();
                visitor.visit_seq(SeqX {
                    de: &mut *self.de,
                    types: id,
                    len: len as usize,
                })
            }
            ByteArray | IntArray | LongArray => {
                let len = self.de.get_int();
                visitor.visit_seq(NumSeqX {
                    input: &mut self.de.input,
                    types: self.types,
                    len: len as usize,
                })
            }
            Compound | Void => Err(DeserializeError::Message("Parse Error".into())),
        }
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
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
        cmp_type!(self.types, NBTTypes::Compound).and(visitor.visit_map(MapX {
            de: &mut *self.de,
            next_types: NBTTypes::Void,
            fields,
        }))
    }

    forward_to_deserialize_any! {
        i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bool
        bytes byte_buf option unit unit_struct newtype_struct tuple seq identifier
        tuple_struct map enum
    }
}

struct MapKey<'a, T>
where
    T: BinaryFormat,
{
    de: &'a mut Deserializer<T>,
    fields: &'static [&'static str],
    types: NBTTypes,
}
impl<'de, 'a, T> de::Deserializer<'de> for &mut MapKey<'a, T>
where
    T: BinaryFormat,
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
        self.de.eat_value(self.types);
        visitor.visit_str(&str)
    }

    forward_to_deserialize_any! {
        i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bool
        bytes byte_buf option unit unit_struct newtype_struct seq tuple struct
        tuple_struct map enum ignored_any
    }
}

struct MapX<'a, T>
where
    T: BinaryFormat,
{
    de: &'a mut Deserializer<T>,
    next_types: NBTTypes,
    fields: &'static [&'static str],
}
impl<'de, 'a, T> de::MapAccess<'de> for MapX<'a, T>
where
    T: BinaryFormat,
{
    type Error = DeserializeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        self.next_types = NBTTypes::from_i8(self.de.get_byte()).unwrap();
        match self.next_types {
            NBTTypes::Void => Ok(None),
            _ => seed
                .deserialize(&mut MapKey {
                    de: &mut *self.de,
                    fields: self.fields,
                    types: self.next_types,
                })
                .map(Some),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut Variant {
            de: &mut *self.de,
            types: self.next_types,
        })
    }
}

struct SeqX<'a, T>
where
    T: BinaryFormat,
{
    de: &'a mut Deserializer<T>,
    types: NBTTypes,
    len: usize,
}
impl<'de, 'a, T> de::SeqAccess<'de> for SeqX<'a, T>
where
    T: BinaryFormat,
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
        seed.deserialize(&mut Variant {
            de: &mut *self.de,
            types: self.types,
        })
        .map(Some)
    }
}

struct NumSeqX<'a> {
    input: &'a mut BytesMut,
    types: NBTTypes,
    len: usize,
}
impl<'de, 'a> de::SeqAccess<'de> for NumSeqX<'a> {
    type Error = DeserializeError;

    fn next_element_seed<E>(&mut self, seed: E) -> Result<Option<E::Value>, Self::Error>
    where
        E: de::DeserializeSeed<'de>,
    {
        struct NumArrayDeserializer<'a> {
            types: NBTTypes,
            input: &'a mut BytesMut,
        }
        impl<'de, 'a> de::Deserializer<'de> for &mut NumArrayDeserializer<'a> {
            type Error = DeserializeError;
            fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
            where
                V: de::Visitor<'de>,
            {
                use NBTTypes::*;
                match self.types {
                    ByteArray => visitor.visit_i8(self.input.get_i8()),
                    IntArray => visitor.visit_i32(self.input.get_i32_le()),
                    LongArray => visitor.visit_i64(self.input.get_i64_le()),
                    _ => Err(DeserializeError::Message("Parse Error".into())),
                }
            }

            forward_to_deserialize_any! {
                i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bool
                bytes byte_buf option unit unit_struct newtype_struct struct seq tuple identifier
                tuple_struct map enum ignored_any
            }
        }

        if self.len == 0 {
            return Ok(None);
        }
        self.len -= 1;
        seed.deserialize(&mut NumArrayDeserializer {
            types: self.types,
            input: self.input,
        })
        .map(Some)
    }
}
