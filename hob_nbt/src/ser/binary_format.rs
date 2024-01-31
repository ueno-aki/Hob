use proto_bytes::{BufMut, BytesMut, ConditionalBufMut};

use crate::{BigEndian, LittleEndian, VarInt};

pub trait BinaryFormat {
    #[inline]
    fn put_byte(buf: &mut BytesMut, v: i8) {
        buf.put_i8(v);
    }

    fn put_short(buf: &mut BytesMut, v: i16);

    fn put_int(buf: &mut BytesMut, v: i32);

    fn put_long(buf: &mut BytesMut, v: i64);

    fn put_float(buf: &mut BytesMut, v: f32);

    fn put_double(buf: &mut BytesMut, v: f64);

    fn put_string(buf: &mut BytesMut, v: &str);

    #[inline]
    fn put_byte_array_elem(buf: &mut BytesMut, v: i8) {
        Self::put_byte(buf, v);
    }

    fn put_int_array_elem(buf: &mut BytesMut, v: i32);

    fn put_long_array_elem(buf: &mut BytesMut, v: i64);
}

impl BinaryFormat for BigEndian {
    #[inline]
    fn put_short(buf: &mut BytesMut, v: i16) {
        buf.put_i16(v);
    }

    #[inline]
    fn put_int(buf: &mut BytesMut, v: i32) {
        buf.put_i32(v);
    }

    #[inline]
    fn put_long(buf: &mut BytesMut, v: i64) {
        buf.put_i64(v);
    }

    #[inline]
    fn put_float(buf: &mut BytesMut, v: f32) {
        buf.put_f32(v);
    }

    #[inline]
    fn put_double(buf: &mut BytesMut, v: f64) {
        buf.put_f64(v);
    }

    #[inline]
    fn put_string(buf: &mut BytesMut, v: &str) {
        buf.put_u16(v.len() as u16);
        buf.put(cesu8::to_java_cesu8(v).as_ref());
    }

    #[inline]
    fn put_int_array_elem(buf: &mut BytesMut, v: i32) {
        Self::put_int(buf, v);
    }

    #[inline]
    fn put_long_array_elem(buf: &mut BytesMut, v: i64) {
        Self::put_long(buf, v);
    }
}

impl BinaryFormat for LittleEndian {
    #[inline]
    fn put_short(buf: &mut BytesMut, v: i16) {
        buf.put_i16_le(v);
    }

    #[inline]
    fn put_int(buf: &mut BytesMut, v: i32) {
        buf.put_i32_le(v);
    }

    #[inline]
    fn put_long(buf: &mut BytesMut, v: i64) {
        buf.put_i64_le(v);
    }

    #[inline]
    fn put_float(buf: &mut BytesMut, v: f32) {
        buf.put_f32_le(v);
    }

    #[inline]
    fn put_double(buf: &mut BytesMut, v: f64) {
        buf.put_f64_le(v);
    }

    #[inline]
    fn put_string(buf: &mut BytesMut, v: &str) {
        buf.put_string_lu16(v);
    }

    #[inline]
    fn put_int_array_elem(buf: &mut BytesMut, v: i32) {
        Self::put_int(buf, v);
    }

    #[inline]
    fn put_long_array_elem(buf: &mut BytesMut, v: i64) {
        Self::put_long(buf, v);
    }
}

impl BinaryFormat for VarInt {
    #[inline]
    fn put_short(buf: &mut BytesMut, v: i16) {
        buf.put_i16_le(v);
    }

    #[inline]
    fn put_int(buf: &mut BytesMut, v: i32) {
        buf.put_zigzag32(v);
    }

    #[inline]
    fn put_long(buf: &mut BytesMut, v: i64) {
        buf.put_zigzag64(v);
    }

    #[inline]
    fn put_float(buf: &mut BytesMut, v: f32) {
        buf.put_f32_le(v);
    }

    #[inline]
    fn put_double(buf: &mut BytesMut, v: f64) {
        buf.put_f64_le(v);
    }

    #[inline]
    fn put_string(buf: &mut BytesMut, v: &str) {
        buf.put_string_varint(v);
    }

    #[inline]
    fn put_int_array_elem(buf: &mut BytesMut, v: i32) {
        buf.put_i32_le(v);
    }

    #[inline]
    fn put_long_array_elem(buf: &mut BytesMut, v: i64) {
        buf.put_i64_le(v);
    }
}
