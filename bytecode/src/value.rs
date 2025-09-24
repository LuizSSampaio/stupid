#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Nil,
}
