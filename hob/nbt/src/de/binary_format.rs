use crate::{BigEndian, LittleEndian, VarInt};
use proto_bytes::{Buf, BytesMut, ConditionalBuf};

pub trait BinaryFormat {
    #[inline]
    fn get_byte(buf: &mut BytesMut) -> i8 {
        buf.get_i8()
    }

    fn get_short(buf: &mut BytesMut) -> i16;

    fn get_int(buf: &mut BytesMut) -> i32;

    fn get_long(buf: &mut BytesMut) -> i64;

    fn get_float(buf: &mut BytesMut) -> f32;

    fn get_double(buf: &mut BytesMut) -> f64;

    fn get_string(buf: &mut BytesMut) -> String;

    #[inline]
    fn get_byte_array_elem(buf: &mut BytesMut) -> i8 {
        Self::get_byte(buf)
    }

    fn get_int_array_elem(buf: &mut BytesMut) -> i32;

    fn get_long_array_elem(buf: &mut BytesMut) -> i64;

    #[inline]
    fn eat_byte(buf: &mut BytesMut) {
        buf.advance(1)
    }

    #[inline]
    fn eat_short(buf: &mut BytesMut) {
        buf.advance(2)
    }

    fn eat_int(buf: &mut BytesMut);

    fn eat_long(buf: &mut BytesMut);

    #[inline]
    fn eat_float(buf: &mut BytesMut) {
        buf.advance(4)
    }

    #[inline]
    fn eat_double(buf: &mut BytesMut) {
        buf.advance(8)
    }

    fn eat_string(buf: &mut BytesMut);

    #[inline]
    fn eat_byte_array(buf: &mut BytesMut) {
        let len = Self::get_int(buf);
        buf.advance(len as usize);
    }

    #[inline]
    fn eat_int_array(buf: &mut BytesMut) {
        let len = Self::get_int(buf);
        buf.advance(4 * len as usize);
    }

    #[inline]
    fn eat_long_array(buf: &mut BytesMut) {
        let len = Self::get_int(buf);
        buf.advance(8 * len as usize);
    }
}

impl BinaryFormat for BigEndian {
    #[inline]
    fn get_short(buf: &mut BytesMut) -> i16 {
        buf.get_i16()
    }

    #[inline]
    fn get_int(buf: &mut BytesMut) -> i32 {
        buf.get_i32()
    }

    #[inline]
    fn get_long(buf: &mut BytesMut) -> i64 {
        buf.get_i64()
    }

    #[inline]
    fn get_float(buf: &mut BytesMut) -> f32 {
        buf.get_f32()
    }

    #[inline]
    fn get_double(buf: &mut BytesMut) -> f64 {
        buf.get_f64()
    }

    #[inline]
    fn get_string(buf: &mut BytesMut) -> String {
        let len = buf.get_u16();
        let bytes = buf.copy_to_bytes(len as usize).to_vec();
        cesu8::from_java_cesu8(&bytes).unwrap().to_string()
    }

    #[inline]
    fn get_int_array_elem(buf: &mut BytesMut) -> i32 {
        Self::get_int(buf)
    }

    #[inline]
    fn get_long_array_elem(buf: &mut BytesMut) -> i64 {
        Self::get_long(buf)
    }

    #[inline]
    fn eat_int(buf: &mut BytesMut) {
        buf.advance(4)
    }

    #[inline]
    fn eat_long(buf: &mut BytesMut) {
        buf.advance(8)
    }

    #[inline]
    fn eat_string(buf: &mut BytesMut) {
        let len = buf.get_u16();
        buf.advance(len as usize)
    }
}

impl BinaryFormat for LittleEndian {
    #[inline]
    fn get_short(buf: &mut BytesMut) -> i16 {
        buf.get_i16_le()
    }

    #[inline]
    fn get_int(buf: &mut BytesMut) -> i32 {
        buf.get_i32_le()
    }

    #[inline]
    fn get_long(buf: &mut BytesMut) -> i64 {
        buf.get_i64_le()
    }

    #[inline]
    fn get_float(buf: &mut BytesMut) -> f32 {
        buf.get_f32_le()
    }

    #[inline]
    fn get_double(buf: &mut BytesMut) -> f64 {
        buf.get_f64_le()
    }

    #[inline]
    fn get_string(buf: &mut BytesMut) -> String {
        buf.get_string_lu16()
    }

    #[inline]
    fn get_int_array_elem(buf: &mut BytesMut) -> i32 {
        Self::get_int(buf)
    }

    #[inline]
    fn get_long_array_elem(buf: &mut BytesMut) -> i64 {
        Self::get_long(buf)
    }

    #[inline]
    fn eat_int(buf: &mut BytesMut) {
        buf.advance(4)
    }

    #[inline]
    fn eat_long(buf: &mut BytesMut) {
        buf.advance(8)
    }

    #[inline]
    fn eat_string(buf: &mut BytesMut) {
        let len = buf.get_u16_le();
        buf.advance(len as usize)
    }
}

impl BinaryFormat for VarInt {
    #[inline]
    fn get_short(buf: &mut BytesMut) -> i16 {
        buf.get_i16_le()
    }

    #[inline]
    fn get_int(buf: &mut BytesMut) -> i32 {
        buf.get_zigzag32()
    }

    #[inline]
    fn get_long(buf: &mut BytesMut) -> i64 {
        buf.get_zigzag64()
    }

    #[inline]
    fn get_float(buf: &mut BytesMut) -> f32 {
        buf.get_f32_le()
    }

    #[inline]
    fn get_double(buf: &mut BytesMut) -> f64 {
        buf.get_f64_le()
    }

    #[inline]
    fn get_string(buf: &mut BytesMut) -> String {
        buf.get_string_varint()
    }

    #[inline]
    fn get_int_array_elem(buf: &mut BytesMut) -> i32 {
        buf.get_i32_le()
    }

    #[inline]
    fn get_long_array_elem(buf: &mut BytesMut) -> i64 {
        buf.get_i64_le()
    }

    #[inline]
    fn eat_int(buf: &mut BytesMut) {
        buf.get_varint();
    }

    #[inline]
    fn eat_long(buf: &mut BytesMut) {
        buf.get_varint();
    }

    #[inline]
    fn eat_string(buf: &mut BytesMut) {
        let len = buf.get_varint();
        buf.advance(len as usize)
    }
}
