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
}
