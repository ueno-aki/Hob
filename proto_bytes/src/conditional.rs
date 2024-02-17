use std::str::from_utf8;

use bytes::{Buf, BufMut};

pub trait ConditionalBufMut {
    fn put_varint(&mut self, n: u64) -> usize;
    fn put_zigzag32(&mut self, n: i32) -> usize;
    fn put_zigzag64(&mut self, n: i64) -> usize;
    fn put_string_varint(&mut self, str: impl AsRef<str>) -> usize;
    fn put_string_lu16(&mut self, str: impl AsRef<str>) -> usize;
    fn put_string_lu32(&mut self, str: impl AsRef<str>) -> usize;
    fn put_bool(&mut self, v: bool);
}

impl<T: BufMut + ?Sized> ConditionalBufMut for T {
    fn put_varint(&mut self, n: u64) -> usize {
        let mut cursor: usize = 0;
        let mut v = n;
        while (v & !0x7f) != 0 {
            self.put_u8((v & 0x7f | 0x80) as u8);
            cursor += 1;
            v >>= 7;
        }
        self.put_u8(v as u8);
        cursor + 1
    }
    fn put_zigzag32(&mut self, n: i32) -> usize {
        let v = (n >> 31) ^ (n << 1);
        self.put_varint(v as u32 as u64)
    }
    fn put_zigzag64(&mut self, n: i64) -> usize {
        let v = (n >> 63) ^ (n << 1);
        self.put_varint(v as u64)
    }
    fn put_string_varint(&mut self, str: impl AsRef<str>) -> usize {
        let mut size = str.as_ref().as_bytes().len();
        size += self.put_varint(size as u64);
        self.put_slice(str.as_ref().as_bytes());
        size
    }
    fn put_string_lu16(&mut self, str: impl AsRef<str>) -> usize {
        let len = str.as_ref().as_bytes().len();
        self.put_u16_le(len as u16);
        self.put_slice(str.as_ref().as_bytes());
        len + 2
    }
    fn put_string_lu32(&mut self, str: impl AsRef<str>) -> usize {
        let len = str.as_ref().as_bytes().len();
        self.put_u32_le(len as u32);
        self.put_slice(str.as_ref().as_bytes());
        len + 4
    }
    fn put_bool(&mut self, v: bool) {
        self.put_i8(v as i8);
    }
}

pub trait ConditionalBuf {
    fn get_varint(&mut self) -> u64;
    fn get_zigzag32(&mut self) -> i32;
    fn get_zigzag64(&mut self) -> i64;
    fn get_string_varint(&mut self) -> String;
    fn get_string_lu16(&mut self) -> String;
    fn get_string_lu32(&mut self) -> String;
    fn get_bool(&mut self) -> bool;
}

impl<T: Buf + ?Sized> ConditionalBuf for T {
    fn get_varint(&mut self) -> u64 {
        let mut value = 0;
        let mut shift = 0;
        loop {
            assert!(shift < 63, "varint is too big");
            let b = self.get_u8() as u64;
            value |= (b & 0x7f) << shift;
            if b & 0x80 == 0 {
                break value;
            }
            shift += 7;
        }
    }
    fn get_zigzag32(&mut self) -> i32 {
        let value = self.get_varint();
        ((value >> 1) as i32) ^ (-((value & 1) as i32))
    }
    fn get_zigzag64(&mut self) -> i64 {
        let value = self.get_varint();
        ((value >> 1) as i64) ^ (-((value & 1) as i64))
    }
    fn get_string_varint(&mut self) -> String {
        let len = self.get_varint();
        let bytes = self.copy_to_bytes(len as usize);
        from_utf8(&bytes).unwrap().to_owned()
    }
    fn get_string_lu16(&mut self) -> String {
        let len = self.get_u16_le();
        let bytes = self.copy_to_bytes(len as usize);
        from_utf8(&bytes).unwrap().to_owned()
    }
    fn get_string_lu32(&mut self) -> String {
        let len = self.get_u32_le();
        let bytes = self.copy_to_bytes(len as usize);
        from_utf8(&bytes).unwrap().to_owned()
    }
    fn get_bool(&mut self) -> bool {
        !matches!(self.get_i8(), 0)
    }
}
