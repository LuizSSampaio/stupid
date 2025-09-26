use std::fmt::Display;

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Nil,
}

macro_rules! impl_from_numeric {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Value {
                fn from(v: $t) -> Self {
                    Self::Number(v as f64)
                }
            }
        )*
    };
}

impl_from_numeric!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64);

impl TryFrom<Value> for f64 {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Number(value) => Ok(value),
            _ => Err(ValueError::Convertion(value, ValueType::Number)),
        }
    }
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

#[derive(Error, Debug)]
pub enum ValueError {
    #[error("can't convert {0} to {1}")]
    Convertion(Value, ValueType),
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
