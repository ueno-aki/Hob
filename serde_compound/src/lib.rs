use serde::{de,Serialize, Deserialize, forward_to_deserialize_any};

#[derive(Debug,Serialize,Deserialize)]
struct User {
    id:i32,
    name:String
}

#[test]
fn lets_go() {
    let mut vec = Vec::new();
    vec.push(String::from("id"));
    vec.push(String::from("name"));
    let user = User::deserialize(&mut StructDeserializer{fields:vec,cursor:0});
    println!("{:?}",user)
}

pub struct StructDeserializer {
    pub fields: Vec<String>,
    pub cursor:usize,
}
impl<'de> de::Deserializer<'de> for &mut StructDeserializer {
    type Error = DeserializeError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        _visitor.visit_unit()
    }
    
    fn deserialize_struct<V>(
            self,
            name: &'static str,
            fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de> {
            visitor.visit_map(MapX{de:&mut *self})
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de> {
        let str = self.fields[self.cursor].clone();
        self.cursor += 1;
        visitor.visit_str(&str)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de> {
        println!("i32");
        visitor.visit_i32(16)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de> {
        println!("str");
        visitor.visit_string(String::from("aiueo"))
    }

    forward_to_deserialize_any! {
        i8 i16 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str bool
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map enum ignored_any
    }
}

struct MapX<'a> {
    de:&'a mut StructDeserializer,
}
impl<'de,'a> de::MapAccess<'de> for MapX<'a> {
    type Error = DeserializeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: de::DeserializeSeed<'de> {
        if self.de.cursor == self.de.fields.len() {
            return Ok(None);
        }
        seed.deserialize(&mut *self.de).map(Some)
    }
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
        where
            V: de::DeserializeSeed<'de> {
        seed.deserialize(&mut *self.de)
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
