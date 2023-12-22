use crate::{LittleEndian, VarInt};
use bytes::{Buf, BytesMut};
use proto_bytes::ConditionalReader;

pub trait BinaryFormat {
    fn byte(buf: &mut BytesMut) -> i8;
    fn short(buf: &mut BytesMut) -> i16;
    fn int(buf: &mut BytesMut) -> i32;
    fn long(buf: &mut BytesMut) -> i64;
    fn float(buf: &mut BytesMut) -> f32;
    fn double(buf: &mut BytesMut) -> f64;
    fn string(buf: &mut BytesMut) -> String;

    fn eat_byte(buf: &mut BytesMut);
    fn eat_short(buf: &mut BytesMut);
    fn eat_int(buf: &mut BytesMut);
    fn eat_long(buf: &mut BytesMut);
    fn eat_float(buf: &mut BytesMut);
    fn eat_double(buf: &mut BytesMut);
    fn eat_string(buf: &mut BytesMut);
}

impl BinaryFormat for LittleEndian {
    #[inline]
    fn byte(buf: &mut BytesMut) -> i8 {
        buf.get_i8()
    }

    #[inline]
    fn short(buf: &mut BytesMut) -> i16 {
        buf.get_i16_le()
    }

    #[inline]
    fn int(buf: &mut BytesMut) -> i32 {
        buf.get_i32_le()
    }

    #[inline]
    fn long(buf: &mut BytesMut) -> i64 {
        buf.get_i64_le()
    }

    #[inline]
    fn float(buf: &mut BytesMut) -> f32 {
        buf.get_f32_le()
    }

    #[inline]
    fn double(buf: &mut BytesMut) -> f64 {
        buf.get_f64_le()
    }
    #[inline]
    fn string(buf: &mut BytesMut) -> String {
        buf.get_short_string()
    }

    #[inline]
    fn eat_byte(buf: &mut BytesMut) {
        buf.advance(1)
    }

    #[inline]
    fn eat_short(buf: &mut BytesMut) {
        buf.advance(2)
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
    fn eat_float(buf: &mut BytesMut) {
        buf.advance(4)
    }

    #[inline]
    fn eat_double(buf: &mut BytesMut) {
        buf.advance(8)
    }

    #[inline]
    fn eat_string(buf: &mut BytesMut) {
        let len = buf.get_i16_le();
        buf.advance(len as usize)
    }
}

impl BinaryFormat for VarInt {
    #[inline]
    fn byte(buf: &mut BytesMut) -> i8 {
        buf.get_i8()
    }

    #[inline]
    fn short(buf: &mut BytesMut) -> i16 {
        buf.get_i16_le()
    }

    #[inline]
    fn int(buf: &mut BytesMut) -> i32 {
        buf.get_zigzag32()
    }

    #[inline]
    fn long(buf: &mut BytesMut) -> i64 {
        buf.get_zigzag64()
    }

    #[inline]
    fn float(buf: &mut BytesMut) -> f32 {
        buf.get_f32_le()
    }

    #[inline]
    fn double(buf: &mut BytesMut) -> f64 {
        buf.get_f64_le()
    }
    #[inline]
    fn string(buf: &mut BytesMut) -> String {
        buf.get_cstring()
    }

    #[inline]
    fn eat_byte(buf: &mut BytesMut) {
        buf.advance(1)
    }

    #[inline]
    fn eat_short(buf: &mut BytesMut) {
        buf.advance(2)
    }

    #[inline]
    fn eat_int(buf: &mut BytesMut) {
        buf.get_zigzag32();
    }

    #[inline]
    fn eat_long(buf: &mut BytesMut) {
        buf.get_zigzag64();
    }

    #[inline]
    fn eat_float(buf: &mut BytesMut) {
        buf.advance(4)
    }

    #[inline]
    fn eat_double(buf: &mut BytesMut) {
        buf.advance(8)
    }

    #[inline]
    fn eat_string(buf: &mut BytesMut) {
        let len = buf.get_varint();
        buf.advance(len as usize)
    }
}
