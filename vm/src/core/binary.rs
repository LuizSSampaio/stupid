use bytecode::opcode::Binary;

use anyhow::Result;

use crate::core::Core;

impl Core {
    pub(super) fn binary(&mut self, binary: Binary) -> Result<()> {
        let b: f64 = self.stack.pop()?.try_into()?;
        let a: f64 = self.stack.pop()?.try_into()?;

        match binary {
            Binary::Add => self.stack.push((a + b).into()),
            Binary::Subtract => self.stack.push((a - b).into()),
            Binary::Multiply => self.stack.push((a * b).into()),
            Binary::Divide => self.stack.push((a / b).into()),
        }

        Ok(())
    }
}
