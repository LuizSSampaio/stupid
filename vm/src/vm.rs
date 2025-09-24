use bytecode::chunk::Chunk;

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
}
