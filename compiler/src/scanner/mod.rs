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
        self.skip_whitespace();
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
            '/' => {
                if self.reader.next_is('/') {
                    while self.reader.peek() != '\n' && !self.reader.is_at_end() {
                        let _ = self.reader.advance();
                    }
                    self.scan_token()
                } else if self.reader.next_is('*') {
                    while !self.reader.is_at_end() {
                        if self.reader.peek() == '*' && self.reader.peek_next() == '/' {
                            let _ = self.reader.advance(); // consume '*'
                            let _ = self.reader.advance(); // consume '/'
                            break;
                        } else {
                            let _ = self.reader.advance();
                        }
                    }
                    self.scan_token()
                } else {
                    Ok(self.make_token(TokenType::Slash))
                }
            }
            '"' => self.string(),
            '0'..='9' => Ok(self.number()),
            'a'..='z' | 'A'..='Z' | '_' => Ok(self.identifier()),
            _ => Err(ScanError::UnexpectedCharacter(
                self.reader.peek(),
                self.reader.row(),
                self.reader.column(),
            )),
        }
    }

    fn string(&mut self) -> Result<Token, ScanError> {
        while self.reader.peek() != '"' && !self.reader.is_at_end() {
            let _ = self.reader.advance();
        }

        if self.reader.is_at_end() {
            return Err(ScanError::UnterminatedString(
                self.reader.row(),
                self.reader.column(),
            ));
        }

        // The closing ".
        let _ = self.reader.advance();
        Ok(self.make_token(TokenType::String))
    }

    fn number(&mut self) -> Token {
        while self.reader.peek().is_ascii_digit() {
            let _ = self.reader.advance();
        }

        if self.reader.peek() == '.' && self.reader.peek_next().is_ascii_digit() {
            let _ = self.reader.advance();

            while self.reader.peek().is_ascii_digit() {
                let _ = self.reader.advance();
            }
        }

        self.make_token(TokenType::Number)
    }

    fn identifier(&mut self) -> Token {
        while self.reader.peek().is_ascii_alphanumeric()
            || self.reader.peek() == '_'
            || self.reader.peek().is_ascii_digit()
        {
            let _ = self.reader.advance();
        }

        let ident_type = self.identifier_type();
        self.make_token(ident_type)
    }

    fn identifier_type(&mut self) -> TokenType {
        match self.reader.lexeme().as_str() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.reader.peek();
            match c {
                ' ' | '\t' | '\r' | '\n' => {
                    let _ = self.reader.advance();
                }
                _ => break,
            }
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
    #[error("Unterminated string at {0}:{1}")]
    UnterminatedString(usize, usize),
}
