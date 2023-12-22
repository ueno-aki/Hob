use bytes::{BufMut, BytesMut};
use proto_bytes::*;
use serde::{Deserialize, Serialize};

use crate::{nbt_types::NBTTypes, LittleEndian, VarInt};

#[test]
fn num_works() {
    let mut buf = BytesMut::new();
    buf.put_i8(NBTTypes::Float as i8);
    buf.put_short_string("");
    buf.put_f32_le(3.1415);

    let var: f32 = LittleEndian::from_buffer(&buf.to_vec()).unwrap();
    assert_eq!(var, 3.1415);

    let mut buf = BytesMut::new();
    buf.put_i8(NBTTypes::Int as i8);
    buf.put_cstring("");
    buf.put_zigzag32(11451);

    let var: i32 = VarInt::from_buffer(&buf.to_vec()).unwrap();
    assert_eq!(var, 11451);
}

#[test]
fn string_works() {
    let mut buf = BytesMut::new();
    buf.put_i8(NBTTypes::String as i8);
    buf.put_short_string("greet");
    buf.put_short_string("hello");

    let var: String = LittleEndian::from_buffer(&buf.to_vec()).unwrap();
    assert_eq!(var, String::from("hello"));

    let mut buf = BytesMut::new();
    buf.put_i8(NBTTypes::String as i8);
    buf.put_cstring("greet");
    buf.put_cstring("hello");

    let var: String = VarInt::from_buffer(&buf.to_vec()).unwrap();
    assert_eq!(var, String::from("hello"));
}

#[test]
fn num_seq_works() {
    let mut buf = BytesMut::new();
    buf.put_i8(NBTTypes::IntArray as i8);
    buf.put_short_string("arr");
    buf.put_i32_le(3);
    buf.put_i32_le(191);
    buf.put_i32_le(-981);
    buf.put_i32_le(0);

    let var: Vec<i64> = LittleEndian::from_buffer(&buf.to_vec()).unwrap();
    assert_eq!(var, vec![191, -981, 0]);

    let mut buf = BytesMut::new();
    buf.put_i8(NBTTypes::LongArray as i8);
    buf.put_cstring("p");
    buf.put_zigzag32(3);
    buf.put_i64_le(82589933);
    buf.put_i64_le(-77232917);
    buf.put_i64_le(74207281);

    let var: Vec<i64> = VarInt::from_buffer(&buf.to_vec()).unwrap();
    assert_eq!(var, vec![82589933, -77232917, 74207281]);
}

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
    )
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Player {
    id:i8,
    pos:Location
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Location {
    x: f64,
    z: f64,
}

#[test]
fn list_works() {
    let mut buf = BytesMut::new();
    buf.put_i8(NBTTypes::List as i8);
    buf.put_short_string("lorem");
    buf.put_i8(NBTTypes::String as i8);
    buf.put_i32_le(3);
    buf.put_short_string("Lorem ipsum  sit amet");
    buf.put_short_string("consectetur adipiscing elit");
    buf.put_short_string("sed do eiusmod tempor incididunt ut labore et dolore magna aliqua");

    let var: Vec<String> = LittleEndian::from_buffer(&buf.to_vec()).unwrap();
    let v: Vec<String> = vec![
        "Lorem ipsum  sit amet".into(),
        "consectetur adipiscing elit".into(),
        "sed do eiusmod tempor incididunt ut labore et dolore magna aliqua".into(),
    ];
    assert_eq!(var, v);

    let mut buf = BytesMut::new();
    buf.put_i8(NBTTypes::List as i8);
    buf.put_cstring("players");
    buf.put_i8(NBTTypes::Compound as i8);
    buf.put_zigzag32(3);
    //
    buf.put_i8(NBTTypes::Byte as i8);
    buf.put_cstring("id");
    buf.put_i8(23);

    buf.put_i8(NBTTypes::Compound as i8);
    buf.put_cstring("pos");
    buf.put_i8(NBTTypes::Double as i8);
    buf.put_cstring("x");
    buf.put_f64_le(5.6);
    buf.put_i8(NBTTypes::Double as i8);
    buf.put_cstring("z");
    buf.put_f64_le(5.6);
    buf.put_i8(NBTTypes::Void as i8);

    buf.put_i8(NBTTypes::Void as i8);
    //
    //
    buf.put_i8(NBTTypes::Byte as i8);
    buf.put_cstring("id");
    buf.put_i8(16);
    buf.put_i8(NBTTypes::Compound as i8);
    buf.put_cstring("pos");

    buf.put_i8(NBTTypes::Double as i8);
    buf.put_cstring("x");
    buf.put_f64_le(0.0);
    buf.put_i8(NBTTypes::Double as i8);
    buf.put_cstring("z");
    buf.put_f64_le(0.0);
    buf.put_i8(NBTTypes::Void as i8);

    buf.put_i8(NBTTypes::Void as i8);
    //
    //
    buf.put_i8(NBTTypes::Byte as i8);
    buf.put_cstring("id");
    buf.put_i8(56);
    buf.put_i8(NBTTypes::Compound as i8);
    buf.put_cstring("pos");

    buf.put_i8(NBTTypes::Double as i8);
    buf.put_cstring("x");
    buf.put_f64_le(-5.6);
    buf.put_i8(NBTTypes::Double as i8);
    buf.put_cstring("z");
    buf.put_f64_le(-5.6);
    buf.put_i8(NBTTypes::Void as i8);

    buf.put_i8(NBTTypes::Void as i8);
    //
    
    let var: Vec<Player> = VarInt::from_buffer(&buf.to_vec()).unwrap();
    let v: Vec<Player> = vec![
        Player {
            id:23,
            pos:Location {
                x:5.6,
                z:5.6
            }
        },
        Player {
            id:16,
            pos:Location {
                x:0.0,
                z:0.0
            }
        },
        Player {
            id:56,
            pos:Location {
                x:-5.6,
                z:-5.6
            }
        }
    ];
    assert_eq!(var, v);
}


fn create_buf_le() -> Vec<u8> {
    let mut vec = BytesMut::new();
    vec.put_i8(NBTTypes::Compound as i8);
    vec.put_short_string("");

    vec.put_i8(NBTTypes::Int as i8);
    vec.put_short_string("id");
    vec.put_i32_le(455);

    vec.put_i8(NBTTypes::LongArray as i8);
    vec.put_short_string("bytes");
    vec.put_i32_le(6);
    vec.put_i64_le(12345679);
    vec.put_i64_le(2345679);
    vec.put_i64_le(345679);
    vec.put_i64_le(-345679);
    vec.put_i64_le(-45679);
    vec.put_i64_le(-5679);

    vec.put_i8(NBTTypes::List as i8);
    vec.put_short_string("package");
    vec.put_i8(NBTTypes::String as i8);
    vec.put_i32_le(4);
    vec.put_short_string("Shut");
    vec.put_short_string("your");
    vec.put_short_string("fuckin'");
    vec.put_short_string("mouth");

    //dummy
    vec.put_i8(NBTTypes::List as i8);
    vec.put_short_string("dummy");
    vec.put_i8(NBTTypes::String as i8);
    vec.put_i32_le(4);
    vec.put_short_string("Shut");
    vec.put_short_string("your");
    vec.put_short_string("fuckin'");
    vec.put_short_string("mouth");

    vec.put_i8(NBTTypes::Compound as i8);
    vec.put_short_string("dummmy");
    vec.put_i8(NBTTypes::Double as i8);
    vec.put_short_string("x");
    vec.put_f64_le(23.1);
    vec.put_i8(NBTTypes::Double as i8);
    vec.put_short_string("z");
    vec.put_f64_le(5.6);
    vec.put_i8(NBTTypes::Void as i8);

    vec.put_i8(NBTTypes::String as i8);
    vec.put_short_string("dummmydummmy");
    vec.put_short_string("Mark2");
    //

    vec.put_i8(NBTTypes::Compound as i8);
    vec.put_short_string("pos");
    vec.put_i8(NBTTypes::Double as i8);
    vec.put_short_string("x");
    vec.put_f64_le(23.1);
    vec.put_i8(NBTTypes::Double as i8);
    vec.put_short_string("z");
    vec.put_f64_le(5.6);
    vec.put_i8(NBTTypes::Void as i8);

    vec.put_i8(NBTTypes::String as i8);
    vec.put_short_string("name");
    vec.put_short_string("Mark2");

    vec.put_i8(NBTTypes::Void as i8);
    vec.to_vec()
}

fn create_buf_varint() -> Vec<u8> {
    let mut vec = BytesMut::new();
    vec.put_i8(NBTTypes::Compound as i8);
    vec.put_cstring("user");

    vec.put_i8(NBTTypes::Int as i8);
    vec.put_cstring("id");
    vec.put_zigzag32(455);

    vec.put_i8(NBTTypes::LongArray as i8);
    vec.put_cstring("bytes");
    vec.put_zigzag32(6);
    vec.put_i64_le(12345679);
    vec.put_i64_le(2345679);
    vec.put_i64_le(345679);
    vec.put_i64_le(-345679);
    vec.put_i64_le(-45679);
    vec.put_i64_le(-5679);

    vec.put_i8(NBTTypes::List as i8);
    vec.put_cstring("package");
    vec.put_i8(NBTTypes::String as i8);
    vec.put_zigzag32(4);
    vec.put_cstring("Shut");
    vec.put_cstring("your");
    vec.put_cstring("fuckin'");
    vec.put_cstring("mouth");

    //dummy
    vec.put_i8(NBTTypes::List as i8);
    vec.put_cstring("dummy");
    vec.put_i8(NBTTypes::String as i8);
    vec.put_zigzag32(4);
    vec.put_cstring("Shut");
    vec.put_cstring("your");
    vec.put_cstring("fuckin'");
    vec.put_cstring("mouth");

    vec.put_i8(NBTTypes::Compound as i8);
    vec.put_cstring("dummmy");
    vec.put_i8(NBTTypes::Double as i8);
    vec.put_cstring("x");
    vec.put_f64_le(23.1);
    vec.put_i8(NBTTypes::Double as i8);
    vec.put_cstring("z");
    vec.put_f64_le(5.6);
    vec.put_i8(NBTTypes::Void as i8);

    vec.put_i8(NBTTypes::String as i8);
    vec.put_cstring("dummmydummmy");
    vec.put_cstring("Mark2");
    //

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
    vec.to_vec()
}
