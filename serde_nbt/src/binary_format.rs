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

pub struct LittleEndian;
impl BinaryFormat for LittleEndian {
    fn byte(buf: &mut BytesMut) -> i8 {
        buf.get_i8()
    }

    fn short(buf: &mut BytesMut) -> i16 {
        buf.get_i16_le()
    }

    fn int(buf: &mut BytesMut) -> i32 {
        buf.get_i32_le()
    }

    fn long(buf: &mut BytesMut) -> i64 {
        buf.get_i64_le()
    }

    fn float(buf: &mut BytesMut) -> f32 {
        buf.get_f32_le()
    }

    fn double(buf: &mut BytesMut) -> f64 {
        buf.get_f64_le()
    }
    fn string(buf: &mut BytesMut) -> String {
        buf.get_short_string()
    }

    fn eat_byte(buf: &mut BytesMut) {
        buf.advance(1)
    }

    fn eat_short(buf: &mut BytesMut) {
        buf.advance(2)
    }

    fn eat_int(buf: &mut BytesMut) {
        buf.advance(4)
    }

    fn eat_long(buf: &mut BytesMut) {
        buf.advance(8)
    }

    fn eat_float(buf: &mut BytesMut) {
        buf.advance(4)
    }

    fn eat_double(buf: &mut BytesMut) {
        buf.advance(8)
    }

    fn eat_string(buf: &mut BytesMut) {
        let len = buf.get_i16_le();
        buf.advance(len as usize)
    }
}

pub struct VarInt;
impl BinaryFormat for VarInt {
    fn byte(buf: &mut BytesMut) -> i8 {
        buf.get_i8()
    }

    fn short(buf: &mut BytesMut) -> i16 {
        buf.get_i16_le()
    }

    fn int(buf: &mut BytesMut) -> i32 {
        buf.get_zigzag32()
    }

    fn long(buf: &mut BytesMut) -> i64 {
        buf.get_zigzag64()
    }

    fn float(buf: &mut BytesMut) -> f32 {
        buf.get_f32_le()
    }

    fn double(buf: &mut BytesMut) -> f64 {
        buf.get_f64_le()
    }
    fn string(buf: &mut BytesMut) -> String {
        buf.get_cstring()
    }

    fn eat_byte(buf: &mut BytesMut) {
        buf.advance(1)
    }

    fn eat_short(buf: &mut BytesMut) {
        buf.advance(2)
    }

    fn eat_int(buf: &mut BytesMut) {
        buf.get_zigzag32();
    }

    fn eat_long(buf: &mut BytesMut) {
        buf.get_zigzag64();
    }

    fn eat_float(buf: &mut BytesMut) {
        buf.advance(4)
    }

    fn eat_double(buf: &mut BytesMut) {
        buf.advance(8)
    }

    fn eat_string(buf: &mut BytesMut) {
        let len = buf.get_varint();
        buf.advance(len as usize)
    }
}
