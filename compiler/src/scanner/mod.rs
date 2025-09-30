use thiserror::Error;

use crate::scanner::{
    reader::Reader,
    token::{Token, TokenType},
};

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
            if token.token_type == TokenType::Eof {
                break;
            }
        }

        Ok(tokens)
    }

    fn scan_token(&mut self) -> Result<Token, ScanError> {
        self.reader.start_to_current();

        let c = match self.reader.advance() {
            Ok(c) => c,
            Err(_) => return Ok(self.make_token(TokenType::Eof)),
        };

        match c {
            '(' => Ok(self.make_token(TokenType::LeftParen)),
            ')' => Ok(self.make_token(TokenType::RightParen)),
            '{' => Ok(self.make_token(TokenType::LeftBrace)),
            '}' => Ok(self.make_token(TokenType::RightBrace)),
            ';' => Ok(self.make_token(TokenType::Semicolon)),
            ',' => Ok(self.make_token(TokenType::Comma)),
            '.' => Ok(self.make_token(TokenType::Dot)),
            '-' => Ok(self.make_token(TokenType::Minus)),
            '+' => Ok(self.make_token(TokenType::Plus)),
            '/' => Ok(self.make_token(TokenType::Slash)),
            '*' => Ok(self.make_token(TokenType::Star)),
            '!' => {
                if self.reader.next_is('=') {
                    Ok(self.make_token(TokenType::BangEqual))
                } else {
                    Ok(self.make_token(TokenType::Bang))
                }
            }
            '=' => {
                if self.reader.next_is('=') {
                    Ok(self.make_token(TokenType::EqualEqual))
                } else {
                    Ok(self.make_token(TokenType::Equal))
                }
            }
            '<' => {
                if self.reader.next_is('=') {
                    Ok(self.make_token(TokenType::LessEqual))
                } else {
                    Ok(self.make_token(TokenType::Less))
                }
            }
            '>' => {
                if self.reader.next_is('=') {
                    Ok(self.make_token(TokenType::GreaterEqual))
                } else {
                    Ok(self.make_token(TokenType::Greater))
                }
            }
            _ => Err(ScanError::UnexpectedCharacter(
                self.reader.peek(),
                self.reader.row(),
                self.reader.column(),
            )),
        }
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            lexeme: self.reader.lexeme(),
            row: self.reader.row(),
            column: self.reader.column(),
        }
    }
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ScanError {
    #[error("Unexpected character '{0}' at {1}:{2}")]
    UnexpectedCharacter(char, usize, usize),
}
