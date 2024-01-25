use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug)]
pub enum Value {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(Vec<Value>),
    Compound(HashMap<String, Value>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl<'de> serde::Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(NBTValueVisitor)
    }
}

struct NBTValueVisitor;
impl<'de> serde::de::Visitor<'de> for NBTValueVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("any valid NBT value")
    }
    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::Byte(v))
    }
    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::Short(v))
    }
    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::Int(v))
    }
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::Long(v))
    }
    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::Float(v))
    }
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::Double(v))
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::String(v.to_owned()))
    }
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut compound = HashMap::new();
        while let Some((key, value)) = map.next_entry::<String, Value>()? {
            compound.insert(key, value);
        }
        Ok(Value::Compound(compound))
    }
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        #[derive(Debug,Deserialize)]
        struct NumArray(i8,i64);
        if let Ok(Some(first)) = seq.next_element::<NumArray>() {
            let (tag,first) = (first.0,first.1);
            match tag {
                0x7 => {
                    let mut vec = vec![first as i8];
                    while let Some(value) = seq.next_element::<i8>()? {
                        vec.push(value)
                    }
                    Ok(Value::ByteArray(vec))
                },
                0xB => {
                    let mut vec = vec![first as i32];
                    while let Some(value) = seq.next_element::<i32>()? {
                        vec.push(value)
                    }
                    Ok(Value::IntArray(vec))
                },
                0xC => {
                    let mut vec = vec![first];
                    while let Some(value) = seq.next_element::<i64>()? {
                        vec.push(value)
                    }
                    Ok(Value::LongArray(vec))
                },
                _ => panic!("Unknown")
            }
        }else {
            let mut vec = Vec::new();
            while let Some(value) = seq.next_element::<Value>()? {
                vec.push(value)
            }
            Ok(Value::List(vec))
        }
    }
}
