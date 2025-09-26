use bytecode::{
    chunk::Chunk,
    opcode::OpCode,
    value::{Value, ValueType},
};

use anyhow::Result;
use thiserror::Error;

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct Core {
    chunk: Option<Chunk>,
    counter: usize,

    stack: Vec<Value>,
}

impl Core {
    pub fn insterpret(&mut self, chunk: Chunk) -> Result<()> {
        self.chunk = Some(chunk);
        self.counter = 0;
        self.stack = Vec::new();

        Ok(())
    }

    fn run(&mut self) -> Result<()> {
        while let Some(instruction) = self
            .chunk
            .as_ref()
            .and_then(|chunk| chunk.get_code(self.counter))
        {
            match instruction {
                OpCode::Return => break,
                OpCode::Constant(index) => {
                    if let Some(constant) = self
                        .chunk
                        .as_ref()
                        .and_then(|chunk| chunk.get_constant(index))
                    {
                        self.stack.push(constant);
                    } else {
                        return Err(CoreError::StackOverflow(instruction).into());
                    }
                }
                OpCode::Negate => {
                    if let Some(value) = self.stack.pop() {
                        match value {
                            Value::Number(n) => self.stack.push(Value::Number(-n)),
                            _ => {
                                return Err(VMError::UnexpectedType(value.into()).into());
                            }
                        }
                    } else {
                        return Err(VMError::StackOverflow(instruction).into());
                    }
                }
                _ => {
                    return Err(CoreError::InvalidInstruction(instruction).into());
                }
            }

            self.counter += 1;
        }

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Invalid instruction: {0:?}")]
    InvalidInstruction(OpCode),
    #[error("Unexpected type: {0}")]
    UnexpectedType(ValueType),
    #[error("Stack overflow: {0:?}")]
    StackOverflow(OpCode),
}
