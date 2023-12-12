use bytes::{BufMut, BytesMut};
use proto_bytes::*;
use serde::{Deserialize, Serialize};

use crate::{types::NBTTypes, StructDeserializer};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct User {
    id: i32,
    name: String,
    pos: Position,
    bytes: Vec<i64>,
    package: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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
    vec.put_i32_le(45);

    vec.put_i8(NBTTypes::LongArray as i8);
    vec.put_cstring("bytes");
    vec.put_i32_le(6);
    vec.put_i64_le(12345679);
    vec.put_i64_le(2345679);
    vec.put_i64_le(345679);
    vec.put_i64_le(-345679);
    vec.put_i64_le(-45679);
    vec.put_i64_le(-5679);

    vec.put_i8(NBTTypes::List as i8);
    vec.put_cstring("package");
    vec.put_i8(NBTTypes::String as i8);
    vec.put_i32_le(4);
    vec.put_cstring("Shut");
    vec.put_cstring("your");
    vec.put_cstring("fuckin'");
    vec.put_cstring("mouth");

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

    let user = User::deserialize(&mut StructDeserializer { input: vec }).unwrap();

    assert_eq!(
        user,
        User {
            id: 45,
            name: "Mark2".to_string(),
            pos: Position { x: 23.1, z: 5.6 },
            bytes: vec![12345679, 2345679, 345679, -345679, -45679, -5679],
            package: vec![
                "Shut".to_string(),
                "your".to_string(),
                "fuckin'".to_string(),
                "mouth".to_string()
            ]
        }
    )
}
