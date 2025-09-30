#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reader {
    source: Vec<char>,

    start: usize,
    current: usize,

    row: usize,
    column: usize,
}

impl Reader {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            row: 1,
            column: 1,
        }
    }

    pub fn peek(&self) -> char {
        self.source.get(self.current).copied().unwrap_or('\0')
    }

    pub fn lexeme(&self) -> String {
        self.source
            .iter()
            .skip(self.start)
            .take(self.current - self.start)
            .collect()
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn start_to_current(&mut self) {
        self.start = self.current;
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn column(&self) -> usize {
        self.column
    }
}
