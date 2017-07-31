use std::fmt;

use token_type::TokenType;
use literal::Literal;

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: &str, literal: Option<Literal>, line: usize) -> Token {
        Token {
            token_type: ttype,
            lexeme: lexeme.into(),
            literal: literal,
            line: line,
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.literal.is_some() {
            write!(f, "{:?} {:?} {:?}", self.token_type, self.lexeme, self.literal)
        } else {
            write!(f, "{:?} {:?}", self.token_type, self.lexeme)
        }
    }
}

