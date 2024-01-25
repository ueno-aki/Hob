pub mod de;

use std::collections::HashMap;

#[derive(Debug,PartialEq)]
pub enum Value {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(Vec<Value>),
    Compound(HashMap<String, Value>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl Value {
    pub fn as_byte(&self) -> Option<&i8> {
        match self {
            Value::Byte(v) => Some(v),
            _ => None
        }
    }
    pub fn is_byte(&self) -> bool {
        self.as_byte().is_some()
    }

    pub fn as_short(&self) -> Option<&i16> {
        match self {
            Value::Short(v) => Some(v),
            _ => None
        }
    }
    pub fn is_short(&self) -> bool {
        self.as_short().is_some()
    }

    pub fn as_int(&self) -> Option<&i32> {
        match self {
            Value::Int(v) => Some(v),
            _ => None
        }
    }
    pub fn is_int(&self) -> bool {
        self.as_int().is_some()
    }

    pub fn as_long(&self) -> Option<&i64> {
        match self {
            Value::Long(v) => Some(v),
            _ => None
        }
    }
    pub fn is_long(&self) -> bool {
        self.as_long().is_some()
    }


    pub fn as_float(&self) -> Option<&f32> {
        match self {
            Value::Float(v) => Some(v),
            _ => None
        }
    }
    pub fn is_float(&self) -> bool {
        self.as_float().is_some()
    }

    pub fn as_double(&self) -> Option<&f64> {
        match self {
            Value::Double(v) => Some(v),
            _ => None
        }
    }
    pub fn is_double(&self) -> bool {
        self.as_double().is_some()
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(v) => Some(v),
            _ => None
        }
    }
    pub fn is_str(&self) -> bool {
        self.as_str().is_some()
    }

    pub fn as_compound(&self) -> Option<&HashMap<String,Value>> {
        match self {
            Value::Compound(v) => Some(v),
            _ => None
        }
    }
    pub fn is_compound(&self) -> bool {
        self.as_compound().is_some()
    }

    pub fn as_byte_array(&self) -> Option<&[i8]> {
        match self {
            Value::ByteArray(v) => Some(v),
            _ => None
        }
    }
    pub fn is_byte_array(&self) -> bool {
        self.as_byte_array().is_some()
    }

    pub fn as_int_array(&self) -> Option<&[i32]> {
        match self {
            Value::IntArray(v) => Some(v),
            _ => None
        }
    }
    pub fn is_int_array(&self) -> bool {
        self.as_int_array().is_some()
    }

    pub fn as_long_array(&self) -> Option<&[i64]> {
        match self {
            Value::LongArray(v) => Some(v),
            _ => None
        }
    }
    pub fn is_long_array(&self) -> bool {
        self.as_long_array().is_some()
    }

    pub fn as_list(&self) -> Option<&[Value]> {
        match self {
            Value::List(v) => Some(v),
            _ => None
        }
    }
    pub fn is_list(&self) -> bool {
        self.as_list().is_some()
    }

}