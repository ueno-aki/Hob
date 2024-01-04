#![allow(non_snake_case, dead_code)]
use serde::{Deserialize, Serialize};
use serde_nbt::{ser::num_array::ByteArray, BigEndian};
use std::collections::{HashMap, HashSet};

// https://wiki.vg/NBT#Examples

#[test]
fn big_endian_works() {
    let vec = include_bytes!("./hello_world.nbt").to_vec();
    let value: HashMap<String, String> = BigEndian::from_slice(&vec).unwrap();
    let mut v = HashMap::new();
    v.insert("name".into(), "Bananrama".into());
    assert_eq!(value, v)
}

#[test]
fn big_endian_ser_de_() {
    let level: Level = BigEndian::from_slice(&include_bytes!("./bigtest.nbt").to_vec()).unwrap();
    let vec = BigEndian::to_vec(&level).unwrap();
    let a_level: Level = BigEndian::from_slice(&vec).unwrap();
    assert_eq!(level, a_level)
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct Level {
    #[serde(rename(
        serialize = "nested compound test",
        deserialize = "nested compound test"
    ))]
    nestedCompoundTest: HashMap<String, Provider>,
    intTest: i32,
    byteTest: i8,
    stringTest: String,
    #[serde(rename(serialize = "listTest (long)", deserialize = "listTest (long)"))]
    listTestLong: Vec<i64>,
    doubleTest: f32,
    floatTest: f64,
    #[serde(rename(serialize = "listTest (compound)", deserialize = "listTest (compound)"))]
    listTestCompound: HashSet<Issuer>,
    #[serde(rename(
        serialize = "byteArrayTest (the first 1000 values of (n*n*255+n*7)%100, starting with n=0 (0, 62, 34, 16, 8, ...))",
        deserialize = "byteArrayTest (the first 1000 values of (n*n*255+n*7)%100, starting with n=0 (0, 62, 34, 16, 8, ...))"
    ))]
    byteArrayTest: ByteArray,
    shortTest: i16,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Provider {
    name: String,
    value: f32,
}

#[derive(Deserialize, Serialize, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Issuer {
    #[serde(rename(serialize = "created-on", deserialize = "created-on"))]
    createdOn: i64,
    name: String,
}
