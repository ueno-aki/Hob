use std::collections::HashSet;

use bytes::{BufMut, BytesMut};
use proto_bytes::*;
use serde::{Deserialize, Serialize};

use crate::{nbt_tag::NBTTag, LittleEndian, VarInt};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

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
fn compound_works() {
    let user: User = LittleEndian::from_buffer(&create_buf_le()).unwrap();
    assert_eq!(
        user,
        User {
            id: 455,
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
    );

    let user: User = VarInt::from_buffer(&create_buf_varint()).unwrap();
    assert_eq!(
        user,
        User {
            id: 455,
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
    );
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
struct Player {
    id: i8,
    name: String,
}

#[test]
fn list_works() {
    let mut buf = BytesMut::new();
    buf.put_i8(NBTTag::List as i8);
    buf.put_string_varint("players");
    buf.put_i8(NBTTag::Compound as i8);
    buf.put_zigzag32(3);
    //
    buf.put_i8(NBTTag::Byte as i8);
    buf.put_string_varint("id");
    buf.put_i8(23);

    buf.put_i8(NBTTag::String as i8);
    buf.put_string_varint("name");
    buf.put_string_varint("Tom");

    buf.put_i8(NBTTag::Void as i8);
    //
    //
    buf.put_i8(NBTTag::Byte as i8);
    buf.put_string_varint("id");
    buf.put_i8(16);

    buf.put_i8(NBTTag::String as i8);
    buf.put_string_varint("name");
    buf.put_string_varint("Bob");

    buf.put_i8(NBTTag::Void as i8);
    //
    //
    buf.put_i8(NBTTag::Byte as i8);
    buf.put_string_varint("id");
    buf.put_i8(32);

    buf.put_i8(NBTTag::String as i8);
    buf.put_string_varint("name");
    buf.put_string_varint("Alex");

    buf.put_i8(NBTTag::Void as i8);
    //
    let list: HashSet<Player> = VarInt::from_buffer(&buf.to_vec()).unwrap();
    assert_eq!(
        list,
        HashSet::from([
            Player {
                id: 23,
                name: "Tom".into()
            },
            Player {
                id: 16,
                name: "Bob".into()
            },
            Player {
                id: 32,
                name: "Alex".into()
            }
        ])
    )
}

fn create_buf_le() -> Vec<u8> {
    let mut vec = BytesMut::new();
    vec.put_i8(NBTTag::Compound as i8);
    vec.put_string_short("");

    vec.put_i8(NBTTag::Int as i8);
    vec.put_string_short("id");
    vec.put_i32_le(455);

    vec.put_i8(NBTTag::LongArray as i8);
    vec.put_string_short("bytes");
    vec.put_i32_le(6);
    vec.put_i64_le(12345679);
    vec.put_i64_le(2345679);
    vec.put_i64_le(345679);
    vec.put_i64_le(-345679);
    vec.put_i64_le(-45679);
    vec.put_i64_le(-5679);

    vec.put_i8(NBTTag::List as i8);
    vec.put_string_short("package");
    vec.put_i8(NBTTag::String as i8);
    vec.put_i32_le(4);
    vec.put_string_short("Shut");
    vec.put_string_short("your");
    vec.put_string_short("fuckin'");
    vec.put_string_short("mouth");

    //dummy
    vec.put_i8(NBTTag::List as i8);
    vec.put_string_short("dummy");
    vec.put_i8(NBTTag::String as i8);
    vec.put_i32_le(4);
    vec.put_string_short("Shut");
    vec.put_string_short("your");
    vec.put_string_short("fuckin'");
    vec.put_string_short("mouth");

    vec.put_i8(NBTTag::Compound as i8);
    vec.put_string_short("dummmy");
    vec.put_i8(NBTTag::Double as i8);
    vec.put_string_short("x");
    vec.put_f64_le(23.1);
    vec.put_i8(NBTTag::Double as i8);
    vec.put_string_short("z");
    vec.put_f64_le(5.6);
    vec.put_i8(NBTTag::Void as i8);

    vec.put_i8(NBTTag::String as i8);
    vec.put_string_short("dummmydummmy");
    vec.put_string_short("Mark2");
    //

    vec.put_i8(NBTTag::Compound as i8);
    vec.put_string_short("pos");
    vec.put_i8(NBTTag::Double as i8);
    vec.put_string_short("x");
    vec.put_f64_le(23.1);
    vec.put_i8(NBTTag::Double as i8);
    vec.put_string_short("z");
    vec.put_f64_le(5.6);
    vec.put_i8(NBTTag::Void as i8);

    vec.put_i8(NBTTag::String as i8);
    vec.put_string_short("name");
    vec.put_string_short("Mark2");

    vec.put_i8(NBTTag::Void as i8);
    vec.to_vec()
}

fn create_buf_varint() -> Vec<u8> {
    let mut vec = BytesMut::new();
    vec.put_i8(NBTTag::Compound as i8);
    vec.put_string_varint("user");

    vec.put_i8(NBTTag::Int as i8);
    vec.put_string_varint("id");
    vec.put_zigzag32(455);

    vec.put_i8(NBTTag::LongArray as i8);
    vec.put_string_varint("bytes");
    vec.put_zigzag32(6);
    vec.put_i64_le(12345679);
    vec.put_i64_le(2345679);
    vec.put_i64_le(345679);
    vec.put_i64_le(-345679);
    vec.put_i64_le(-45679);
    vec.put_i64_le(-5679);

    vec.put_i8(NBTTag::List as i8);
    vec.put_string_varint("package");
    vec.put_i8(NBTTag::String as i8);
    vec.put_zigzag32(4);
    vec.put_string_varint("Shut");
    vec.put_string_varint("your");
    vec.put_string_varint("fuckin'");
    vec.put_string_varint("mouth");

    //dummy
    vec.put_i8(NBTTag::List as i8);
    vec.put_string_varint("dummy");
    vec.put_i8(NBTTag::String as i8);
    vec.put_zigzag32(4);
    vec.put_string_varint("Shut");
    vec.put_string_varint("your");
    vec.put_string_varint("fuckin'");
    vec.put_string_varint("mouth");

    vec.put_i8(NBTTag::Compound as i8);
    vec.put_string_varint("dummmy");
    vec.put_i8(NBTTag::Double as i8);
    vec.put_string_varint("x");
    vec.put_f64_le(23.1);
    vec.put_i8(NBTTag::Double as i8);
    vec.put_string_varint("z");
    vec.put_f64_le(5.6);
    vec.put_i8(NBTTag::Void as i8);

    vec.put_i8(NBTTag::String as i8);
    vec.put_string_varint("dummmydummmy");
    vec.put_string_varint("Mark2");
    //

    vec.put_i8(NBTTag::Compound as i8);
    vec.put_string_varint("pos");
    vec.put_i8(NBTTag::Double as i8);
    vec.put_string_varint("x");
    vec.put_f64_le(23.1);
    vec.put_i8(NBTTag::Double as i8);
    vec.put_string_varint("z");
    vec.put_f64_le(5.6);
    vec.put_i8(NBTTag::Void as i8);

    vec.put_i8(NBTTag::String as i8);
    vec.put_string_varint("name");
    vec.put_string_varint("Mark2");

    vec.put_i8(NBTTag::Void as i8);
    vec.to_vec()
}
