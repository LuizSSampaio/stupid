use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum ValueType {
    Number,
    Bool,
    Nil,
}

impl From<Value> for ValueType {
    fn from(value: Value) -> Self {
        match value {
            Value::Number(_) => ValueType::Number,
            Value::Bool(_) => ValueType::Bool,
            Value::Nil => ValueType::Nil,
        }
    }
}

impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::Number => write!(f, "Number"),
            ValueType::Bool => write!(f, "Bool"),
            ValueType::Nil => write!(f, "Nil"),
        }
    }
}
