use bytes::{BufMut, BytesMut};
use proto_bytes::ConditionalWriter;

use crate::{BigEndian, LittleEndian, VarInt};

pub trait BinaryFormat {
    fn put_byte(buf: &mut BytesMut, v: i8) {
        buf.put_i8(v);
    }
    fn put_short(buf: &mut BytesMut, v: i16);
    fn put_int(buf: &mut BytesMut, v: i32);
    fn put_long(buf: &mut BytesMut, v: i64);
    fn put_float(buf: &mut BytesMut, v: f32);
    fn put_double(buf: &mut BytesMut, v: f64);
    fn put_string(buf: &mut BytesMut, v: &str);
    fn put_byte_array_elem(buf: &mut BytesMut, v: i8) {
        Self::put_byte(buf, v);
    }
    fn put_int_array_elem(buf: &mut BytesMut, v: i32);
    fn put_long_array_elem(buf: &mut BytesMut, v: i64);
}

impl BinaryFormat for BigEndian {
    fn put_short(buf: &mut BytesMut, v: i16) {
        buf.put_i16(v);
    }

    fn put_int(buf: &mut BytesMut, v: i32) {
        buf.put_i32(v);
    }

    fn put_long(buf: &mut BytesMut, v: i64) {
        buf.put_i64(v);
    }

    fn put_float(buf: &mut BytesMut, v: f32) {
        buf.put_f32(v);
    }

    fn put_double(buf: &mut BytesMut, v: f64) {
        buf.put_f64(v);
    }

    fn put_string(buf: &mut BytesMut, v: &str) {
        buf.put_u16(v.len() as u16);
        buf.put(cesu8::to_java_cesu8(v).as_ref());
    }

    fn put_int_array_elem(buf: &mut BytesMut, v: i32) {
        Self::put_int(buf, v);
    }

    fn put_long_array_elem(buf: &mut BytesMut, v: i64) {
        Self::put_long(buf, v);
    }
}

impl BinaryFormat for LittleEndian {
    fn put_short(buf: &mut BytesMut, v: i16) {
        buf.put_i16_le(v);
    }

    fn put_int(buf: &mut BytesMut, v: i32) {
        buf.put_i32_le(v);
    }

    fn put_long(buf: &mut BytesMut, v: i64) {
        buf.put_i64_le(v);
    }

    fn put_float(buf: &mut BytesMut, v: f32) {
        buf.put_f32_le(v);
    }

    fn put_double(buf: &mut BytesMut, v: f64) {
        buf.put_f64_le(v);
    }

    fn put_string(buf: &mut BytesMut, v: &str) {
        buf.put_u16_le(v.len() as u16);
        buf.put(v.as_bytes());
    }

    fn put_int_array_elem(buf: &mut BytesMut, v: i32) {
        Self::put_int(buf, v);
    }

    fn put_long_array_elem(buf: &mut BytesMut, v: i64) {
        Self::put_long(buf, v);
    }
}

impl BinaryFormat for VarInt {
    fn put_short(buf: &mut BytesMut, v: i16) {
        buf.put_i16_le(v);
    }

    fn put_int(buf: &mut BytesMut, v: i32) {
        buf.put_zigzag32(v);
    }

    fn put_long(buf: &mut BytesMut, v: i64) {
        buf.put_zigzag64(v);
    }

    fn put_float(buf: &mut BytesMut, v: f32) {
        buf.put_f32_le(v);
    }

    fn put_double(buf: &mut BytesMut, v: f64) {
        buf.put_f64_le(v);
    }

    fn put_string(buf: &mut BytesMut, v: &str) {
        buf.put_varint(v.len() as u64);
        buf.put(v.as_bytes());
    }

    fn put_int_array_elem(buf: &mut BytesMut, v: i32) {
        buf.put_i32_le(v);
    }

    fn put_long_array_elem(buf: &mut BytesMut, v: i64) {
        buf.put_i64_le(v);
    }
}
