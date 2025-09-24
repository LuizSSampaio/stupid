use crate::{opcode::OpCode, value::Value};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
}
