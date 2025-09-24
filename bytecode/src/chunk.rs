use crate::{opcode::OpCode, value::Value};

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct Chunk {
    code: Vec<OpCode>,

    constants: Vec<Value>,
}

impl Chunk {
    pub fn write(&mut self, opcode: OpCode, line: usize) {
        self.code.push(opcode);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn get_code(&self, index: usize) -> Option<OpCode> {
        self.code.get(index).copied()
    }

    pub fn get_constant(&self, index: usize) -> Option<Value> {
        self.constants.get(index).copied()
    }
}
