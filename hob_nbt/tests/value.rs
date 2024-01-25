use std::{collections::HashMap, f32::consts::PI};

use hob_nbt::{
    ser::{ByteArray, IntArray, LongArray}, value::Value, LittleEndian, VarInt
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Lists {
    bytes: ByteArray,
    ints: IntArray,
    longs: LongArray,
    list: Vec<Pos>,
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
        list: vec![
            Pos {
                x: 1.1,
                y: 2.2,
                z: 3.3,
            },
            Pos {
                x: PI,
                y: 1.5,
                z: 9.2,
            },
        ],
    };
    let buf = VarInt::to_vec(ins).unwrap();
    let value: Value = VarInt::from_slice(&buf).unwrap();
    assert!(value.is_compound());

    let map:HashMap<String,String> = HashMap::new();
    let buf = LittleEndian::to_vec(map).unwrap();
    let value: Value = LittleEndian::from_slice(&buf).unwrap();
    assert_eq!(*value.as_compound().unwrap(),HashMap::<String,Value>::new())
}
