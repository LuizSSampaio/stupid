use crate::scanner::{Scanner, token::Token};

pub struct Parser {
    previous: Token,
    current: Token,
}

impl Parser {
    pub fn new(scanner: &mut Scanner) -> Self {
        let first_token = scanner.scan_token().unwrap();

        Self {
            previous: first_token.clone(),
            current: first_token,
        }
    }
}
