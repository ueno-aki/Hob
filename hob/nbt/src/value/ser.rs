use serde::{ser::SerializeMap, Serialize};

use crate::ser::{ByteArray, IntArray, LongArray};

use super::Value;

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Value::Byte(n) => serializer.serialize_i8(*n),
            Value::Short(n) => serializer.serialize_i16(*n),
            Value::Int(n) => serializer.serialize_i32(*n),
            Value::Long(n) => serializer.serialize_i64(*n),
            Value::Float(n) => serializer.serialize_f32(*n),
            Value::Double(n) => serializer.serialize_f64(*n),
            Value::String(s) => serializer.serialize_str(s),
            Value::List(v) => v.serialize(serializer),
            Value::Compound(m) => {
                let mut map = serializer.serialize_map(None)?;
                for (key, v) in m {
                    map.serialize_entry(key, v)?;
                }
                map.end()
            }
            Value::ByteArray(v) => ByteArray(v.clone()).serialize(serializer),
            Value::IntArray(v) => IntArray(v.clone()).serialize(serializer),
            Value::LongArray(v) => LongArray(v.clone()).serialize(serializer),
        }
    }
}
