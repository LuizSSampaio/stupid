#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OpCode {
    Constant(usize),
    Unary(Unary),
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
