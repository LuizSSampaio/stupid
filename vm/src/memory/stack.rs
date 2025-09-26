use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Stack<T>(Vec<T>);

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn push(&mut self, value: T) {
        self.0.push(value);
    }

    pub fn pop(&mut self) -> Result<T, StackError> {
        match self.0.pop() {
            Some(value) => Ok(value),
            None => Err(StackError::StackOverflow),
        }
    }
}

#[derive(Error, Debug)]
pub enum StackError {
    #[error("Stack overflow")]
    StackOverflow,
}
