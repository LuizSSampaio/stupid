use bytecode::{chunk::Chunk, opcode::OpCode};

use anyhow::Result;
use thiserror::Error;

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct VM {
    chunk: Option<Chunk>,
    counter: usize,
}

impl VM {
    pub fn insterpret(&mut self, chunk: Chunk) -> Result<()> {
        self.chunk = Some(chunk);
        self.counter = 0;

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
                        println!("Pushing constant: {:?}", constant);
                    } else {
                        println!("Invalid constant index: {}", index);
                    }
                }
                _ => {
                    return Err(VMError::InvalidInstruction(instruction).into());
                }
            }

            self.counter += 1;
        }

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum VMError {
    #[error("Invalid instruction: {0:?}")]
    InvalidInstruction(OpCode),
}
