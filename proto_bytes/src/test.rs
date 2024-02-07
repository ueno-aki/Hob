use bytes::BytesMut;

use crate::*;

#[test]
fn varint_works() {
    let mut bytes = BytesMut::new();
    let size = bytes.put_varint(0b10111111100000001111111);

    assert_eq!(
        bytes.to_vec(),
        vec![0b11111111, 0b10000000, 0b11111111, 0b10]
    );
    assert_eq!(size, 4);
    assert_eq!(bytes.get_varint(), 0b10111111100000001111111)
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
