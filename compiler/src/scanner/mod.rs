use thiserror::Error;

use crate::scanner::{reader::Reader, token::Token};

mod reader;
mod token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scanner {
    reader: Reader,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            reader: Reader::new(source),
        }
    }

    pub fn scan(&mut self) -> Result<Vec<Token>, ScanError> {
        let mut tokens = Vec::new();

        loop {
            let token = self.scan_token()?;
            tokens.push(token.clone());
            if token.token_type == token::TokenType::Eof {
                break;
            }
        }

        Ok(tokens)
    }

    fn scan_token(&mut self) -> Result<Token, ScanError> {
        self.reader.start_to_current();

        if self.reader.is_at_end() {
            return Ok(Token {
                token_type: token::TokenType::Eof,
                lexeme: "".to_string(),
                row: self.reader.row(),
                column: self.reader.column(),
            });
        }

        Err(ScanError::UnexpectedCharacter(
            self.reader.peek(),
            self.reader.row(),
            self.reader.column(),
        ))
    }
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ScanError {
    #[error("Unexpected character '{0}' at {1}:{2}")]
    UnexpectedCharacter(char, usize, usize),
}
