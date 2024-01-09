use proto_bytes::BytesMut;
use serde::{ser, Serialize};
use std::marker::PhantomData;

use self::error::SerializeError;
use crate::{nbt_tag::NBTTag, ser::binary_format::BinaryFormat, unimplemented_serealize};

mod binary_format;
pub mod error;
mod num_array;

pub use num_array::*;

pub struct Serializer<B> {
    pub output: BytesMut,
    _marker: PhantomData<B>,
}
impl<B> Serializer<B> {
    fn new() -> Self {
        Serializer {
            output: BytesMut::new(),
            _marker: PhantomData,
        }
    }
}
impl<B> Default for Serializer<B> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, B> ser::Serializer for &'a mut Serializer<B>
where
    B: BinaryFormat,
{
    type Ok = ();

    type Error = SerializeError;

    type SerializeStruct = Compound<'a, B>;
    type SerializeMap = Compound<'a, B>;
    type SerializeSeq = List<'a, B>;
    type SerializeTupleStruct = ser::Impossible<(), Self::Error>;
    type SerializeTuple = ser::Impossible<(), Self::Error>;
    type SerializeTupleVariant = ser::Impossible<(), Self::Error>;
    type SerializeStructVariant = ser::Impossible<(), Self::Error>;

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        B::put_byte(&mut self.output, NBTTag::List as i8);
        B::put_string(&mut self.output, "");
        Ok(List {
            ser: &mut *self,
            tagged: false,
            len: len.unwrap_or(0),
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        B::put_byte(&mut self.output, NBTTag::Compound as i8);
        B::put_string(&mut self.output, "");
        Ok(Compound { ser: &mut *self })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        B::put_byte(&mut self.output, NBTTag::Compound as i8);
        B::put_string(&mut self.output, "");
        Ok(Compound { ser: &mut *self })
    }

    unimplemented_serealize! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str bytes
        none some unit unit_struct unit_variant newtype_struct newtype_variant
        tuple tuple_struct tuple_variant struct_variant
    }
}

pub struct Compound<'a, B>
where
    B: BinaryFormat,
{
    ser: &'a mut Serializer<B>,
}

impl<'a, B> ser::SerializeStruct for Compound<'a, B>
where
    B: BinaryFormat,
{
    type Ok = ();

    type Error = SerializeError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut Tag {
            ser: &mut *self.ser,
            name: Some(key.to_string()),
        })?;
        value.serialize(&mut Variant {
            ser: &mut *self.ser,
        })?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        B::put_byte(&mut self.ser.output, NBTTag::Void as i8);
        Ok(())
    }
}

impl<'a, B> ser::SerializeMap for Compound<'a, B>
where
    B: BinaryFormat,
{
    type Ok = ();

    type Error = SerializeError;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }

    fn serialize_entry<K: ?Sized, V: ?Sized>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<(), Self::Error>
    where
        K: Serialize,
        V: Serialize,
    {
        value.serialize(&mut Tag {
            ser: &mut *self.ser,
            name: None,
        })?;
        key.serialize(&mut Variant {
            ser: &mut *self.ser,
        })?;
        value.serialize(&mut Variant {
            ser: &mut *self.ser,
        })?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        B::put_byte(&mut self.ser.output, NBTTag::Void as i8);
        Ok(())
    }
}

pub struct List<'a, B>
where
    B: BinaryFormat,
{
    ser: &'a mut Serializer<B>,
    tagged: bool,
    len: usize,
}

impl<'a, B> ser::SerializeSeq for List<'a, B>
where
    B: BinaryFormat,
{
    type Ok = ();

    type Error = SerializeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if !self.tagged {
            value.serialize(&mut Tag {
                ser: &mut *self.ser,
                name: None,
            })?;
            B::put_int(&mut self.ser.output, self.len as i32);
            self.tagged = true;
        }
        value.serialize(&mut Variant {
            ser: &mut *self.ser,
        })?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, B> ser::SerializeTupleStruct for List<'a, B>
where
    B: BinaryFormat,
{
    type Ok = ();

    type Error = SerializeError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut Variant {
            ser: &mut *self.ser,
        })?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

struct Tag<'a, B>
where
    B: BinaryFormat,
{
    ser: &'a mut Serializer<B>,
    name: Option<String>,
}

impl<'a, B> ser::Serializer for &'a mut Tag<'a, B>
where
    B: BinaryFormat,
{
    type Ok = ();

    type Error = SerializeError;

    type SerializeStruct = Nil;
    type SerializeMap = Nil;
    type SerializeSeq = Nil;
    type SerializeTupleStruct = Nil;
    type SerializeTuple = ser::Impossible<(), Self::Error>;
    type SerializeTupleVariant = ser::Impossible<(), Self::Error>;
    type SerializeStructVariant = ser::Impossible<(), Self::Error>;

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        B::put_byte(&mut self.ser.output, NBTTag::Byte as i8);
        if let Some(ref v) = self.name {
            B::put_string(&mut self.ser.output, v);
        }
        Ok(())
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        B::put_byte(&mut self.ser.output, NBTTag::Short as i8);
        if let Some(ref v) = self.name {
            B::put_string(&mut self.ser.output, v);
        }
        Ok(())
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        B::put_byte(&mut self.ser.output, NBTTag::Int as i8);
        if let Some(ref v) = self.name {
            B::put_string(&mut self.ser.output, v);
        }
        Ok(())
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        B::put_byte(&mut self.ser.output, NBTTag::Long as i8);
        if let Some(ref v) = self.name {
            B::put_string(&mut self.ser.output, v);
        }
        Ok(())
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        B::put_byte(&mut self.ser.output, NBTTag::Float as i8);
        if let Some(ref v) = self.name {
            B::put_string(&mut self.ser.output, v);
        }
        Ok(())
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        B::put_byte(&mut self.ser.output, NBTTag::Double as i8);
        if let Some(ref v) = self.name {
            B::put_string(&mut self.ser.output, v);
        }
        Ok(())
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        B::put_byte(&mut self.ser.output, NBTTag::String as i8);
        if let Some(ref v) = self.name {
            B::put_string(&mut self.ser.output, v);
        }
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        B::put_byte(&mut self.ser.output, NBTTag::List as i8);
        if let Some(ref v) = self.name {
            B::put_string(&mut self.ser.output, v);
        }
        Ok(Nil)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        B::put_byte(&mut self.ser.output, NBTTag::Compound as i8);
        if let Some(ref v) = self.name {
            B::put_string(&mut self.ser.output, v);
        }
        Ok(Nil)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        B::put_byte(&mut self.ser.output, NBTTag::Compound as i8);
        if let Some(ref v) = self.name {
            B::put_string(&mut self.ser.output, v);
        }
        Ok(Nil)
    }

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        B::put_byte(&mut self.ser.output, NBTTag::Byte as i8);
        if let Some(ref v) = self.name {
            B::put_string(&mut self.ser.output, v);
        }
        Ok(())
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        let tag = match name {
            "__nbt_byte_array" => NBTTag::ByteArray,
            "__nbt_int_array" => NBTTag::IntArray,
            "__nbt_long_array" => NBTTag::LongArray,
            _ => {
                return Err(SerializeError::Unsupported(
                    "Unsupported 'tuple_struct'".into(),
                ))
            }
        };
        B::put_byte(&mut self.ser.output, tag as i8);
        if let Some(ref v) = self.name {
            B::put_string(&mut self.ser.output, v);
        }
        Ok(Nil)
    }

    unimplemented_serealize! {
        u8 u16 u32 u64 char bytes
        none some unit unit_struct unit_variant newtype_struct newtype_variant
        tuple tuple_variant struct_variant
    }
}
struct Variant<'a, B>
where
    B: BinaryFormat,
{
    ser: &'a mut Serializer<B>,
}

impl<'a, B> ser::Serializer for &'a mut Variant<'a, B>
where
    B: BinaryFormat,
{
    type Ok = ();

    type Error = SerializeError;

    type SerializeStruct = Compound<'a, B>;
    type SerializeMap = Compound<'a, B>;
    type SerializeSeq = List<'a, B>;
    type SerializeTupleStruct = List<'a, B>;
    type SerializeTuple = ser::Impossible<(), Self::Error>;
    type SerializeTupleVariant = ser::Impossible<(), Self::Error>;
    type SerializeStructVariant = ser::Impossible<(), Self::Error>;

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        B::put_byte(&mut self.ser.output, v);
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        B::put_short(&mut self.ser.output, v);
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        B::put_int(&mut self.ser.output, v);
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        B::put_long(&mut self.ser.output, v);
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        B::put_float(&mut self.ser.output, v);
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        B::put_double(&mut self.ser.output, v);
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        B::put_string(&mut self.ser.output, v);
        Ok(())
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(List {
            ser: &mut *self.ser,
            tagged: false,
            len: len.unwrap_or(0),
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(Compound {
            ser: &mut *self.ser,
        })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(Compound {
            ser: &mut *self.ser,
        })
    }

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        B::put_byte(&mut self.ser.output, v as i8);
        Ok(())
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        match name {
            "__nbt_byte_array" | "__nbt_int_array" | "__nbt_long_array" => {
                B::put_int(&mut self.ser.output, len as i32);
                Ok(List {
                    ser: &mut *self.ser,
                    tagged: true,
                    len,
                })
            }
            _ => Err(SerializeError::Unsupported(
                "Unsupported 'tuple_struct'".into(),
            )),
        }
    }

    unimplemented_serealize! {
        u8 u16 u32 u64 char bytes
        none some unit unit_struct unit_variant newtype_struct newtype_variant
        tuple tuple_variant struct_variant
    }
}

pub struct Nil;
impl ser::SerializeStruct for Nil {
    type Ok = ();

    type Error = SerializeError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl ser::SerializeMap for Nil {
    type Ok = ();

    type Error = SerializeError;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl ser::SerializeSeq for Nil {
    type Ok = ();

    type Error = SerializeError;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl ser::SerializeTupleStruct for Nil {
    type Ok = ();

    type Error = SerializeError;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
