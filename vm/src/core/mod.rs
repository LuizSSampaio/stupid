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
    pub fn interpret(&mut self, chunk: Chunk) -> Result<()> {
        self.chunk = Some(chunk);
        self.counter = 0;
        self.stack = Stack::new();

        Ok(())
    }

    fn run(&mut self) -> Result<()> {
        loop {
            let instruction = self
                .chunk
                .as_ref()
                .and_then(|chunk| chunk.get_code(self.counter))
                .transpose()?;

            match instruction {
                Some(OpCode::Return) => break,
                Some(opcode) => self.execute_instruction(opcode)?,
                None => break,
            }

            self.counter += 1;
        }

        Ok(())
    }

    fn execute_instruction(&mut self, opcode: OpCode) -> Result<()> {
        match opcode {
            OpCode::Constant => self.execute_constant(),
            OpCode::Negate => self.execute_negate(),
            OpCode::Add => self.execute_binary_op(|a, b| a + b),
            OpCode::Subtract => self.execute_binary_op(|a, b| a - b),
            OpCode::Multiply => self.execute_binary_op(|a, b| a * b),
            OpCode::Divide => self.execute_binary_op(|a, b| a / b),
            OpCode::Return => unreachable!("Return should be handled in run loop"),
        }
    }

    fn execute_constant(&mut self) -> Result<()> {
        unimplemented!("Constant instruction not yet implemented")
    }

    fn execute_negate(&mut self) -> Result<()> {
        let value = self.stack.pop()?;
        match value {
            Value::Number(n) => {
                self.stack.push(Value::Number(-n));
                Ok(())
            }
            _ => Err(CoreError::UnexpectedType(value.into()).into()),
        }
    }

    fn execute_binary_op<F>(&mut self, op: F) -> Result<()>
    where
        F: Fn(f64, f64) -> f64,
    {
        let b: f64 = self.stack.pop()?.try_into()?;
        let a: f64 = self.stack.pop()?.try_into()?;
        let result = op(a, b);
        self.stack.push(result.into());
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
