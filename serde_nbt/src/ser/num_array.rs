use serde::{
    ser::{Serialize, SerializeTupleStruct},
    Deserialize,
};

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct ByteArray(pub Vec<i8>);

impl Serialize for ByteArray {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut tuple_struct =
            serializer.serialize_tuple_struct("__nbt_byte_array", self.0.len())?;
        for i in self.0.iter() {
            tuple_struct.serialize_field(i)?;
        }
        tuple_struct.end()
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct IntArray(pub Vec<i32>);

impl Serialize for IntArray {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut tuple_struct =
            serializer.serialize_tuple_struct("__nbt_int_array", self.0.len())?;
        for i in self.0.iter() {
            tuple_struct.serialize_field(i)?;
        }
        tuple_struct.end()
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct LongArray(pub Vec<i64>);

impl Serialize for LongArray {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut tuple_struct =
            serializer.serialize_tuple_struct("__nbt_long_array", self.0.len())?;
        for i in self.0.iter() {
            tuple_struct.serialize_field(i)?;
        }
        tuple_struct.end()
    }
}
