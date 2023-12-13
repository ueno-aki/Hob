use from_num::from_num;

#[derive(Debug,Clone)]
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
