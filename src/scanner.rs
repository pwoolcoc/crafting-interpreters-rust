use token::Token;

pub struct Scanner {
    source: String,
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        Scanner { source: source.into() }
    }

    pub fn scan(self) -> Vec<Token> {
        vec![]
    }
}

