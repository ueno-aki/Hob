use bytes::{Buf, BufMut, BytesMut};
use proto_bytes::ConditionalReader;

use crate::types::NBTTypes;

pub fn skip_value<'a>(types: NBTTypes, byte: &'a mut BytesMut) {
    use NBTTypes::*;
    match types {
        Byte => byte.advance(1),
        Short => byte.advance(2),
        Int => byte.advance(4),
        Long => byte.advance(8),
        Float => byte.advance(4),
        Double => byte.advance(8),
        ByteArray => {
            let len = byte.get_i32_le();
            byte.advance(len as usize)
        }
        String => {
            let len = byte.get_i16_le();
            byte.advance(len as usize)
        }
        List => {
            let elem_types = NBTTypes::from_i8(byte.get_i8()).unwrap();
            let len = byte.get_i32_le() as usize;
            for _ in 0..len {
                skip_value(elem_types.clone(), byte)
            }
        }
        Compound => loop {
            let id = byte.get_i8();
            if id == NBTTypes::Void as i8 {
                break;
            }
            skip_value(String, byte);
            skip_value(NBTTypes::from_i8(id).unwrap(), byte);
        },
        IntArray => {
            let len = byte.get_i32_le();
            byte.advance((len * 4) as usize)
        }
        LongArray => {
            let len = byte.get_i32_le();
            byte.advance((len * 8) as usize)
        }
        _ => panic!("Unsupported ID"),
    }
}
