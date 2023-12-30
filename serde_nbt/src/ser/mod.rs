use bytes::BytesMut;
use serde::{ser, Serialize};
use std::marker::PhantomData;

use crate::de::binary_format::BinaryFormat;

use self::error::SerializeError;

pub mod error;

pub struct Serializer<B>
where
    B: BinaryFormat,
{
    pub output: BytesMut,
    _marker: PhantomData<B>,
}

impl<'a, B> ser::Serializer for &'a mut Serializer<B>
where
    B: BinaryFormat,
{
    type Ok = ();

    type Error = SerializeError;

    type SerializeStruct = &'a mut Compound<'a,B>;
    type SerializeMap = &'a mut Compound<'a,B>;
    type SerializeSeq = Self;
    type SerializeTuple = ser::Impossible<(), Self::Error>;
    type SerializeTupleStruct = ser::Impossible<(), Self::Error>;
    type SerializeTupleVariant = ser::Impossible<(), Self::Error>;
    type SerializeStructVariant = ser::Impossible<(), Self::Error>;

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }
    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }


    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unsupported("Unsupported Type".into()))
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unsupported("Unsupported Type".into()))
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unsupported("Unsupported Type".into()))
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unsupported("Unsupported Type".into()))
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unsupported("Unsupported Type".into()))
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unsupported("Unsupported Type".into()))
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unsupported("Unsupported Type".into()))
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unsupported("Unsupported Type".into()))
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unsupported("Unsupported Type".into()))
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unsupported("Unsupported Type".into()))
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unsupported("Unsupported Type".into()))
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unsupported("Unsupported Type".into()))
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unsupported("Unsupported Type".into()))
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unsupported("Unsupported Type".into()))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unsupported("Unsupported Type".into()))
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(SerializeError::Unsupported("Unsupported Type".into()))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unsupported("Unsupported Type".into()))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unsupported("Unsupported Type".into()))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unsupported("Unsupported Type".into()))
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(SerializeError::Unsupported("Unsupported Type".into()))
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(SerializeError::Unsupported("Unsupported Type".into()))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(SerializeError::Unsupported("Unsupported Type".into()))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(SerializeError::Unsupported("Unsupported Type".into()))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(SerializeError::Unsupported("Unsupported Type".into()))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(SerializeError::Unsupported("Unsupported Type".into()))
    }
}

pub struct Compound <'a, B>
where
    B: BinaryFormat,
{
    ser:&'a mut Serializer<B>
}

impl<'a, B> ser::SerializeStruct for &'a mut Compound<'a,B>
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
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, B> ser::SerializeMap for &'a mut Compound<'a,B>
where
    B: BinaryFormat,
{
    type Ok = ();

    type Error = SerializeError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }
    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, B> ser::SerializeSeq for &'a mut Serializer<B>
where
    B: BinaryFormat,
{
    type Ok = ();

    type Error = SerializeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}
