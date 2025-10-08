use bytecode::{
    chunk::Chunk,
    opcode::OpCode,
    value::{Value, ValueType},
};

use anyhow::Result;
use thiserror::Error;

use crate::memory::stack::Stack;

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct Core {
    chunk: Option<Chunk>,
    counter: usize,

    stack: Stack<Value>,
}

impl Core {
    pub fn insterpret(&mut self, chunk: Chunk) -> Result<()> {
        self.chunk = Some(chunk);
        self.counter = 0;
        self.stack = Stack::new();

        Ok(())
    }

    fn run(&mut self) -> Result<()> {
        while let Some(instruction) = self
            .chunk
            .as_ref()
            .and_then(|chunk| chunk.get_code(self.counter))
        {
            let instruction = instruction?;

            match instruction {
                OpCode::Return => break,
                OpCode::Constant => unimplemented!(),
                OpCode::Negate => {
                    let value = self.stack.pop()?;
                    match value {
                        Value::Number(n) => {
                            self.stack.push(Value::Number(-n));
                        }
                        _ => return Err(CoreError::UnexpectedType(value.into()).into()),
                    }
                }
                OpCode::Add => {
                    let b: f64 = self.stack.pop()?.try_into()?;
                    let a: f64 = self.stack.pop()?.try_into()?;
                    self.stack.push((a + b).into())
                }
                OpCode::Subtract => {
                    let b: f64 = self.stack.pop()?.try_into()?;
                    let a: f64 = self.stack.pop()?.try_into()?;
                    self.stack.push((a - b).into())
                }
                OpCode::Multiply => {
                    let b: f64 = self.stack.pop()?.try_into()?;
                    let a: f64 = self.stack.pop()?.try_into()?;
                    self.stack.push((a * b).into())
                }
                OpCode::Divide => {
                    let b: f64 = self.stack.pop()?.try_into()?;
                    let a: f64 = self.stack.pop()?.try_into()?;
                    self.stack.push((a / b).into())
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
