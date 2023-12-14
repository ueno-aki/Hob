use from_num::from_num;

#[derive(Debug, PartialEq, Clone, Copy)]
#[from_num(i8)]
pub enum NBTTypes {
    Void,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    ByteArray,
    String,
    List,
    Compound,
    IntArray,
    LongArray,
}

impl std::fmt::Display for NBTTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use NBTTypes::*;
        write!(
            f,
            "{}",
            match self {
                Void => "Void",
                Byte => "Byte",
                Short => "Short",
                Int => "Int",
                Long => "Long",
                Float => "Float",
                Double => "Double",
                ByteArray => "ByteArray",
                String => "String",
                List => "List",
                Compound => "Compound",
                IntArray => "IntArray",
                LongArray => "LongArray",
            }
        )
    }
}
