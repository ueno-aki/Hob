use std::f32::consts::PI;

use crate::{
    ser::{ByteArray, IntArray, LongArray}, value::Value, LittleEndian, VarInt
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Lists {
    bytes: ByteArray,
    ints: IntArray,
    longs: LongArray,
    a: Pos,
}
#[derive(Debug, Serialize, Deserialize)]
struct Pos {
    x: f32,
    y: f32,
    z: f32,
}

#[test]
fn value_works() {
    let ins = Lists {
        bytes: ByteArray(vec![0, 0, 0, 2]),
        ints: IntArray(vec![1, 2, 3, 4, 5]),
        longs: LongArray(vec![11, 22, 33, 44, 55]),
        a: Pos { x: 1.0, y: 1.0, z: 1.0 },
    };
    let buf = LittleEndian::to_vec(ins).unwrap();
    println!("{:?}",buf);
    let value: Value = LittleEndian::from_slice(&buf).unwrap();
    println!("{:?}", value);
}
