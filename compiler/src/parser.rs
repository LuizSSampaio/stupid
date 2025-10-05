use crate::scanner::{Scanner, token::Token};

pub struct Parser {
    scanner: Scanner,

    previous: Token,
    current: Token,
}

impl Parser {
    pub fn new(mut scanner: Scanner) -> Self {
        let first_token = scanner.scan_token().unwrap();

        Self {
            scanner,
            previous: first_token.clone(),
            current: first_token,
        }
    }

    pub fn advance(&mut self) {
        self.previous = self.current.clone();

        loop {
            self.current = match self.scanner.scan_token() {
                Ok(token) => token,
                Err(err) => {
                    println!("Error: {}", err);
                    continue;
                }
            };

            break;
        }
    }

    pub fn consume(&mut self, expected: &Token) -> anyhow::Result<()> {
        if self.current.token_type == expected.token_type {
            self.advance();
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Expected token {:?}, but found {:?} at row {}, column {}",
                expected.token_type,
                self.current.token_type,
                self.current.row,
                self.current.column
            ))
        }
    }
}
