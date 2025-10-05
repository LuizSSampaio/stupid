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
}
