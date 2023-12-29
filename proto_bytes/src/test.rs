use bytes::{Buf, BufMut, BytesMut};

use crate::*;

#[test]
fn bytes_works() {
    let mut bytes = BytesMut::new();
    bytes.put_u8(9);
    bytes.put(vec![0xfe, 0xa].as_ref());
    bytes.put_u32_le(0xffeeddcc);

    let b = bytes.get_u32();
    assert_eq!(b, 0x09fe0acc);
    assert_eq!(bytes.to_vec(), vec![0xdd, 0xee, 0xff]);
}

#[test]
fn varint_works() {
    let mut bytes = BytesMut::new();
    let size = bytes.put_varint(0b10111111100000001111111);

    assert_eq!(
        bytes.to_vec(),
        vec![0b11111111, 0b10000000, 0b11111111, 0b10]
    );
    assert_eq!(size, 4);
}

#[test]
fn conditional_string_works() {
    let mut bytes = BytesMut::new();
    let size = bytes.put_string_varint("abcdefg");

    assert_eq!(bytes.as_ref(), b"\x07abcdefg");
    assert_eq!(size, 8);

    let str = bytes.get_string_varint();
    assert_eq!(str, "abcdefg");
    assert_eq!(bytes.as_ref(), b"")
}