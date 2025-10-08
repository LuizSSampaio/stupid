use crate::{
    opcode::{OpCode, OpCodeError},
    value::Value,
};

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct Chunk {
    code: Vec<u8>,
    lines: Vec<usize>,

    constants: Vec<Value>,
}

impl Chunk {
    pub fn write(&mut self, opcode: OpCode, line: usize) {
        self.code.push(opcode as u8);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn get_code(&self, index: usize) -> Option<Result<OpCode, OpCodeError>> {
        self.code.get(index).copied().map(|val| val.try_into())
    }

    pub fn get_line(&self, index: usize) -> Option<usize> {
        self.lines.get(index).copied()
    }

    pub fn get_constant(&self, index: usize) -> Option<Value> {
        self.constants.get(index).copied()
    }
}
