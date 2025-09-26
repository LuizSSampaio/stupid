#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OpCode {
    Constant(usize),
    Unary(Unary),
    Binary(Binary),
    Return,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Unary {
    Negate,
}

impl From<Unary> for OpCode {
    fn from(value: Unary) -> Self {
        Self::Unary(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Binary {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl From<Binary> for OpCode {
    fn from(value: Binary) -> Self {
        Self::Binary(value)
    }
}
