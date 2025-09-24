use crate::value::Value;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
}
