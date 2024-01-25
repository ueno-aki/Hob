mod binary_format;
pub mod error;

use self::{binary_format::BinaryFormat, error::DeserializeError};
use crate::{ensure_nbt, nbt_tag::NBTTag};
use proto_bytes::{BytesMut, ConditionalReader};
use serde::{de, forward_to_deserialize_any};
use std::marker::PhantomData;

pub struct Deserializer<B>
where
    B: BinaryFormat,
{
    pub input: BytesMut,
    _marker: PhantomData<fn() -> B>,
}

impl<B> Deserializer<B>
where
    B: BinaryFormat,
{
    pub fn from_slice(buf: &[u8]) -> Self {
        Deserializer {
            input: BytesMut::from(buf),
            _marker: PhantomData,
        }
    }
    fn eat_value(&mut self, types: NBTTag) {
        use NBTTag::*;
        match types {
            Void => {}
            Byte => B::eat_byte(&mut self.input),
            Short => B::eat_short(&mut self.input),
            Int => B::eat_int(&mut self.input),
            Long => B::eat_long(&mut self.input),
            Float => B::eat_float(&mut self.input),
            Double => B::eat_double(&mut self.input),
            ByteArray => B::eat_byte_array(&mut self.input),
            String => B::eat_string(&mut self.input),
            List => {
                let elem_types = NBTTag::from_i8(B::get_byte(&mut self.input)).unwrap();
                let len = B::get_int(&mut self.input);
                for _ in 0..len {
                    self.eat_value(elem_types)
                }
            }
            Compound => loop {
                let id = B::get_byte(&mut self.input);
                if id == 0 {
                    break;
                }
                self.eat_value(String);
                self.eat_value(NBTTag::from_i8(id).unwrap());
            },
            IntArray => B::eat_int_array(&mut self.input),
            LongArray => B::eat_long_array(&mut self.input),
        }
    }
}

impl<'de, B> de::Deserializer<'de> for &mut Deserializer<B>
where
    B: BinaryFormat,
{
    type Error = DeserializeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let tag = NBTTag::from_i8(B::get_byte(&mut self.input)).unwrap();
        let _ = B::get_string(&mut self.input);
        match tag {
            NBTTag::Compound => {
                let variant = &mut Variant {
                    de: &mut *self,
                    tag: NBTTag::Compound,
                };
                variant.deserialize_map(visitor)
            }
            NBTTag::List => {
                let variant = &mut Variant {
                    de: &mut *self,
                    tag: NBTTag::List,
                };
                variant.deserialize_seq(visitor)
            }
            _ => Err(DeserializeError::Unsupported("Unsupported NBTTag,".into())),
        }
    }

    forward_to_deserialize_any! {
        i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bool
        bytes byte_buf option struct unit unit_struct newtype_struct tuple seq identifier
        tuple_struct map enum ignored_any
    }
}

struct Variant<'a, B>
where
    B: BinaryFormat,
{
    de: &'a mut Deserializer<B>,
    tag: NBTTag,
}
impl<'de, 'a, B> de::Deserializer<'de> for &mut Variant<'a, B>
where
    B: BinaryFormat,
{
    type Error = DeserializeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        use NBTTag::*;
        match self.tag {
            Byte => self.deserialize_i8(visitor),
            Short => self.deserialize_i16(visitor),
            Int => self.deserialize_i32(visitor),
            Long => self.deserialize_i64(visitor),
            Float => self.deserialize_f32(visitor),
            Double => self.deserialize_f64(visitor),
            String => self.deserialize_string(visitor),
            ByteArray | IntArray | LongArray | List => self.deserialize_seq(visitor),
            Compound => self.deserialize_map(visitor),
            _ => Err(DeserializeError::Unsupported(
                "Unsupported NBTTag".into(),
            ))
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        ensure_nbt!(
            self.tag == NBTTag::Byte,
            "Expected a Tag_Byte, found {:?}",
            self.tag
        );
        visitor.visit_i8(B::get_byte(&mut self.de.input))
    }
    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        ensure_nbt!(
            self.tag == NBTTag::Short,
            "Expected a Tag_Short, found {:?}",
            self.tag
        );
        visitor.visit_i16(B::get_short(&mut self.de.input))
    }
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        ensure_nbt!(
            self.tag == NBTTag::Int,
            "Expected a Tag_Int, found {:?}",
            self.tag
        );
        visitor.visit_i32(B::get_int(&mut self.de.input))
    }
    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        ensure_nbt!(
            self.tag == NBTTag::Long,
            "Expected a Tag_Long, found {:?}",
            self.tag
        );
        visitor.visit_i64(B::get_long(&mut self.de.input))
    }
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        ensure_nbt!(
            self.tag == NBTTag::Float,
            "Expected a Tag_Float, found {:?}",
            self.tag
        );
        visitor.visit_f32(B::get_float(&mut self.de.input))
    }
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        ensure_nbt!(
            self.tag == NBTTag::Double,
            "Expected a Tag_Double, found {:?}",
            self.tag
        );
        visitor.visit_f64(B::get_double(&mut self.de.input))
    }
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        ensure_nbt!(
            self.tag == NBTTag::String,
            "Expected a Tag_String, found {:?}",
            self.tag
        );
        visitor.visit_string(B::get_string(&mut self.de.input))
    }
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        use NBTTag::*;
        match self.tag {
            List => {
                let elem_tag = NBTTag::from_i8(B::get_byte(&mut self.de.input)).unwrap();
                let len = B::get_int(&mut self.de.input);
                visitor.visit_seq(SeqX {
                    de: &mut *self.de,
                    tag: elem_tag,
                    len: len as usize,
                })
            }
            ByteArray | IntArray | LongArray => {
                let len = B::get_int(&mut self.de.input);
                visitor.visit_seq(NumSeqX {
                    de: &mut *self.de,
                    tag: self.tag,
                    len: len as usize,
                })
            }
            _ =>  Err(DeserializeError::Unsupported("Unsupported seq NBTTag,expected a Tag_List,Tag_ByteArray,Tag_IntArray and Tag_LongArray.".into()))
        }
    }
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        ensure_nbt!(
            self.tag == NBTTag::Compound,
            "Expected a Tag_Compound, found {:?}",
            self.tag
        );
        visitor.visit_map(MapX {
            de: &mut *self.de,
            next_tag: NBTTag::Void,
        })
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de.eat_value(self.tag);
        visitor.visit_unit()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        ensure_nbt!(
            self.tag == NBTTag::Byte,
            "Expected a Tag_Byte, found {:?}",
            self.tag
        );
        visitor.visit_bool(self.de.input.get_bool())
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de> {
        ensure_nbt!(
            self.tag == NBTTag::String,
            "Expected a Tag_String, found {:?}",
            self.tag
        );
        visitor.visit_string(B::get_string(&mut self.de.input))
    }

    fn deserialize_tuple_struct<V>(
            self,
            _name: &'static str,
            _len: usize,
            _visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de> {
        Err(DeserializeError::Unsupported(
            "Unsupported Variant's Type".into(),
        ))
    }

    forward_to_deserialize_any! {
        i128 u8 u16 u32 u64 u128 char str
        bytes byte_buf option unit unit_struct tuple
        enum
    }
}

struct MapX<'a, B>
where
    B: BinaryFormat,
{
    de: &'a mut Deserializer<B>,
    next_tag: NBTTag,
}
impl<'de, 'a, B> de::MapAccess<'de> for MapX<'a, B>
where
    B: BinaryFormat,
{
    type Error = DeserializeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        self.next_tag = NBTTag::from_i8(B::get_byte(&mut self.de.input)).unwrap();
        match self.next_tag {
            NBTTag::Void => Ok(None),
            _ => seed
                .deserialize(&mut Variant {
                    de: &mut *self.de,
                    tag: NBTTag::String,
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
            tag: self.next_tag,
        })
    }
}

struct SeqX<'a, B>
where
    B: BinaryFormat,
{
    de: &'a mut Deserializer<B>,
    tag: NBTTag,
    len: usize,
}
impl<'de, 'a, B> de::SeqAccess<'de> for SeqX<'a, B>
where
    B: BinaryFormat,
{
    type Error = DeserializeError;

    fn next_element_seed<E>(&mut self, seed: E) -> Result<Option<E::Value>, Self::Error>
    where
        E: de::DeserializeSeed<'de>,
    {
        if self.len == 0 {
            return Ok(None);
        }
        match seed.deserialize(&mut Variant {
            de: &mut *self.de,
            tag: self.tag,
        }) {
            v @ Ok(..) => {
                self.len -= 1;
                v.map(Some)
            }
            Err(e) => Err(e),
        }
    }
}

struct NumSeqX<'a, B>
where
    B: BinaryFormat,
{
    de: &'a mut Deserializer<B>,
    tag: NBTTag,
    len: usize,
}
impl<'de, 'a, B> de::SeqAccess<'de> for NumSeqX<'a, B>
where
    B: BinaryFormat,
{
    type Error = DeserializeError;

    fn next_element_seed<E>(&mut self, seed: E) -> Result<Option<E::Value>, Self::Error>
    where
        E: de::DeserializeSeed<'de>,
    {
        struct NumSeqDeserializer<'a, T>
        where
            T: BinaryFormat,
        {
            de: &'a mut Deserializer<T>,
            tag: NBTTag,
        }
        impl<'de, 'a, T> de::Deserializer<'de> for &mut NumSeqDeserializer<'a, T>
        where
            T: BinaryFormat,
        {
            type Error = DeserializeError;
            fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
            where
                V: de::Visitor<'de>,
            {
                use NBTTag::*;
                match self.tag {
                    ByteArray => visitor.visit_i8(T::get_byte_array_elem(&mut self.de.input)),
                    IntArray => visitor.visit_i32(T::get_int_array_elem(&mut self.de.input)),
                    LongArray => visitor.visit_i64(T::get_long_array_elem(&mut self.de.input)),
                    _ => Err(DeserializeError::Message("Parse Error".into())),
                }
            }

            ///## for Value's Num Array
            fn deserialize_tuple_struct<V>(
                self,
                _name: &'static str,
                len: usize,
                visitor: V,
            ) -> Result<V::Value, Self::Error>
            where
                V: de::Visitor<'de>,
            {
                visitor.visit_seq(NumSeqElememt {
                    de: &mut *self.de,
                    len,
                    tag: self.tag,
                })
            }
            forward_to_deserialize_any! {
                i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bool
                bytes byte_buf option unit unit_struct newtype_struct struct seq tuple identifier
                map enum ignored_any
            }
        }

        if self.len == 0 {
            return Ok(None);
        }
        match seed.deserialize(&mut NumSeqDeserializer {
            de: &mut *self.de,
            tag: self.tag,
        }) {
            v @ Ok(..) => {
                self.len -= 1;
                v.map(Some)
            }
            Err(e) => Err(e),
        }
    }
}

///## for Value's Num Array
struct NumSeqElememt<'a, B>
where
    B: BinaryFormat,
{
    de: &'a mut Deserializer<B>,
    len: usize,
    tag: NBTTag,
}
impl<'de, 'a, B> de::SeqAccess<'de> for NumSeqElememt<'a, B>
where
    B: BinaryFormat,
{
    type Error = DeserializeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        struct ValueNumDeserializer<'a, T>
        where
            T: BinaryFormat,
        {
            de: &'a mut Deserializer<T>,
            flagged: bool,
            types: NBTTag,
        }
        impl<'de, 'a, T> de::Deserializer<'de> for &mut ValueNumDeserializer<'a, T>
        where
            T: BinaryFormat,
        {
            type Error = DeserializeError;
            fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
            where
                V: de::Visitor<'de>,
            {
                use NBTTag::*;
                if self.flagged {
                    match self.types {
                        ByteArray => {
                            visitor.visit_i64(T::get_byte_array_elem(&mut self.de.input) as i64)
                        }
                        IntArray => {
                            visitor.visit_i64(T::get_int_array_elem(&mut self.de.input) as i64)
                        }
                        LongArray => visitor.visit_i64(T::get_long_array_elem(&mut self.de.input)),
                        _ => Err(DeserializeError::Message("Parse Error".into())),
                    }
                } else {
                    match self.types {
                        ByteArray => visitor.visit_i8(0x7),
                        IntArray => visitor.visit_i8(0xB),
                        LongArray => visitor.visit_i8(0xC),
                        _ => Err(DeserializeError::Message("Parse Error".into())),
                    }
                }
            }
            forward_to_deserialize_any! {
                i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bool
                bytes byte_buf option unit unit_struct newtype_struct struct seq tuple identifier
                map enum ignored_any tuple_struct
            }
        }

        let result = match self.len {
            0 => Ok(None),
            1 => seed
                .deserialize(&mut ValueNumDeserializer {
                    de: &mut *self.de,
                    flagged: true,
                    types: self.tag,
                })
                .map(Some),
            2 => seed
                .deserialize(&mut ValueNumDeserializer {
                    de: &mut *self.de,
                    flagged: false,
                    types: self.tag,
                })
                .map(Some),
            _ => Err(DeserializeError::Message("Parse Error".into())),
        };
        self.len -= 1;
        result
    }
}
