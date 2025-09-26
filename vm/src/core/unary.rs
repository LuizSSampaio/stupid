use bytecode::{opcode::Unary, value::Value};

use anyhow::Result;

use crate::core::{Core, CoreError};

impl Core {
    pub(super) fn unary(&mut self, unary: Unary) -> Result<()> {
        match unary {
            Unary::Negate => self.negate(),
        }
    }

    fn negate(&mut self) -> Result<()> {
        let value = self.stack.pop()?;
        match value {
            Value::Number(n) => {
                self.stack.push(Value::Number(-n));
                Ok(())
            }
            _ => Err(CoreError::UnexpectedType(value.into()).into()),
        }
    }
}
