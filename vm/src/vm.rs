use bytecode::{chunk::Chunk, opcode::OpCode};

use anyhow::Result;

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
                _ => {
                    println!("Executing instruction: {:?}", instruction);
                }
            }

            self.counter += 1;
        }

        Ok(())
    }
}
