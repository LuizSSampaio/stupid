use thiserror::Error;

use crate::scanner::{
    Scanner,
    token::{Token, TokenType},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parser {
    scanner: Scanner,

    previous: Token,
    current: Token,

    had_error: bool,
}

impl Parser {
    pub fn new(mut scanner: Scanner) -> Self {
        let first_token = scanner.scan_token().unwrap();

        Self {
            scanner,
            previous: first_token.clone(),
            current: first_token,
            had_error: false,
        }
    }

    pub fn advance(&mut self) {
        self.previous = self.current.clone();

        loop {
            self.current = match self.scanner.scan_token() {
                Ok(token) => token,
                Err(err) => {
                    self.had_error = true;
                    println!("Error: {}", err);
                    continue;
                }
            };

            break;
        }
    }

    pub fn consume(&mut self, expected: TokenType) -> Result<(), ParserError> {
        if self.current.token_type == expected {
            self.advance();
            return Ok(());
        }
        Err(ParserError::UnexpectedToken(
            expected,
            self.current.token_type.clone(),
        ))
    }

    pub fn had_error(&self) -> bool {
        self.had_error
    }
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Expected token '{0}', but found '{1}'")]
    UnexpectedToken(TokenType, TokenType),
}
