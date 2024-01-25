#[macro_export]
macro_rules! ensure_nbt {
    ($cond:expr,$fmt:expr,$($arg:tt)*) => {
        if !$cond {
            return Err(DeserializeError::Message(format!($fmt, $($arg)*)))
        }
    };
}

#[macro_export(local_inner_macros)]
macro_rules! unimplemented_serealize {
    ($($prem:ident)*) => {
        $(unimplemented_serealize_helper!{$prem})*
    }
}

#[macro_export]
macro_rules! unimplemented_serealize_helper {
    (str) => {
        fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
            Err(SerializeError::Unsupported("Unsupported Type".into()))
        }
    };
    (bytes) => {
        fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
            Err(SerializeError::Unsupported("Unsupported Type".into()))
        }
    };
    (none) => {
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
            Err(SerializeError::Unsupported("Unsupported Type".into()))
        }
    };
    (some) => {
        fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize,
        {
            Err(SerializeError::Unsupported("Unsupported Type".into()))
        }
    };
    (unit) => {
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
            Err(SerializeError::Unsupported("Unsupported Type".into()))
        }
    };
    (unit_struct) => {
        fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
            Err(SerializeError::Unsupported("Unsupported Type".into()))
        }
    };
    (unit_variant) => {
        fn serialize_unit_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            _variant: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            Err(SerializeError::Unsupported("Unsupported Type".into()))
        }
    };
    (newtype_struct) => {
        fn serialize_newtype_struct<T: ?Sized>(
            self,
            _name: &'static str,
            _value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize,
        {
            Err(SerializeError::Unsupported("Unsupported Type".into()))
        }
    };
    (newtype_variant) => {
        fn serialize_newtype_variant<T: ?Sized>(
            self,
            _name: &'static str,
            _variant_index: u32,
            _variant: &'static str,
            _value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize,
        {
            Err(SerializeError::Unsupported("Unsupported Type".into()))
        }
    };
    (seq) => {
        fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
            Err(SerializeError::Unsupported("Unsupported Type".into()))
        }
    };
    (tuple) => {
        fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
            Err(SerializeError::Unsupported("Unsupported Type".into()))
        }
    };
    (tuple_struct) => {
        fn serialize_tuple_struct(
            self,
            _name: &'static str,
            _len: usize,
        ) -> Result<Self::SerializeTupleStruct, Self::Error> {
            Err(SerializeError::Unsupported("Unsupported Type".into()))
        }
    };
    (map) => {
        fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
            Err(SerializeError::Unsupported("Unsupported Type".into()))
        }
    };
    (struct) => {
        fn serialize_struct(
            self,
            _name: &'static str,
            _len: usize,
        ) -> Result<Self::SerializeStruct, Self::Error> {
            Err(SerializeError::Unsupported("Unsupported Type".into()))
        }
    };
    (tuple_variant) => {
        fn serialize_tuple_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            _variant: &'static str,
            _len: usize,
        ) -> Result<Self::SerializeTupleVariant, Self::Error> {
            Err(SerializeError::Unsupported("Unsupported Type".into()))
        }
    };
    (struct_variant) => {
        fn serialize_struct_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            _variant: &'static str,
            _len: usize,
        ) -> Result<Self::SerializeStructVariant, Self::Error> {
            Err(SerializeError::Unsupported("Unsupported Type".into()))
        }
    };
    ($prem:ident) => {
        paste::paste! {
            fn [<serialize_ $prem>](self, _v: $prem) -> Result<Self::Ok, Self::Error> {
                Err(SerializeError::Unsupported("Unsupported Type".into()))
            }
        }
    };
}
