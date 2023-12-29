#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NBTTag {
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
#[derive(Debug)]
pub struct NBTTagError(i8);
impl std::fmt::Display for NBTTagError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Failed conversion from {}",self.0)
    }
}
impl NBTTag {
    pub fn from_i8(value: i8) -> Result<Self,NBTTagError> {
        match value {
            0 => Ok(Self::Void),
            1 => Ok(Self::Byte),
            2 => Ok(Self::Short),
            3 => Ok(Self::Int),
            4 => Ok(Self::Long),
            5 => Ok(Self::Float),
            6 => Ok(Self::Double),
            7 => Ok(Self::ByteArray),
            8 => Ok(Self::String),
            9 => Ok(Self::List),
            10 => Ok(Self::Compound),
            11 => Ok(Self::IntArray),
            12 => Ok(Self::LongArray),
            n => Err(NBTTagError(n))
        }
    }
}