use errors::*;
use token::Token;
use token_type::TokenType;
use literal::{Literal, Number};

pub struct Scanner {
    characters: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        let source: String = source.into();
        let chars = source.chars().collect::<Vec<_>>();
        Scanner {
            characters: chars,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.characters.len()
    }

    pub fn scan(mut self) -> Result<Vec<Token>> {
        let mut tokens = vec![];

        while !self.is_at_end() {
            self.start = self.current;
            let token = self.scan_token()?;
            tokens.push(token);
        }

        tokens.push(Token::new(TokenType::Eof, "", None, self.line));
        Ok(tokens)
    }

    fn scan_token(&mut self) -> Result<Token> {
        let ch = self.next();
        match ch {
            // Single-char tokens
            '(' => return Ok(self.make_token(TokenType::LeftParen, None)),
            ')' => return Ok(self.make_token(TokenType::RightParen, None)),
            '{' => return Ok(self.make_token(TokenType::LeftBrace, None)),
            '}' => return Ok(self.make_token(TokenType::RightBrace, None)),
            ',' => return Ok(self.make_token(TokenType::Comma, None)),
            '.' => return Ok(self.make_token(TokenType::Dot, None)),
            '-' => return Ok(self.make_token(TokenType::Minus, None)),
            '+' => return Ok(self.make_token(TokenType::Plus, None)),
            ';' => return Ok(self.make_token(TokenType::Semicolon, None)),
            '*' => return Ok(self.make_token(TokenType::Star, None)),

            // Single-or-double-char tokens
            '!' => {
                if self.match_char('=') {
                    return Ok(self.make_token(TokenType::BangEqual, None));
                } else {
                    return Ok(self.make_token(TokenType::Bang, None));
                }
            },
            '=' => {
                if self.match_char('=') {
                    return Ok(self.make_token(TokenType::EqualEqual, None));
                } else {
                    return Ok(self.make_token(TokenType::Equal, None));
                }
            },
            '<' => {
                if self.match_char('=') {
                    return Ok(self.make_token(TokenType::LesserEqual, None));
                } else {
                    return Ok(self.make_token(TokenType::Lesser, None));
                }
            },
            '>' => {
                if self.match_char('=') {
                    return Ok(self.make_token(TokenType::GreaterEqual, None));
                } else {
                    return Ok(self.make_token(TokenType::Greater, None));
                }
            },

            // Could be single-char token, or a comment
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.next();
                    }
                } else {
                    return Ok(self.make_token(TokenType::Slash, None));
                }
            },

            // Whitespace
            // The original impl just discarded it, but I've been
            // having problems just skipping it, so for now I'm
            // emitting a token
            ' ' | '\r' | 't' => {
                return Ok(self.make_token(TokenType::Ws, None));
            },

            '\n' => {
                self.line += 1;
                return self.scan_token();
            },

            '"' => return self.scan_string(),

            digit if self.is_digit(digit) => {
                return self.scan_number();
            },

            alpha if self.is_alpha(alpha) => {
                return self.scan_ident();
            },

            _ => (),
        };
        Err(ErrorKind::LoxError(self.line, "no matched token".into()).into())
    }

    fn is_whitespace(&self, ch: char) -> bool {
        ch == '\t' || ch == ' ' || ch == '\r'
    }

    fn is_digit(&self, ch: char) -> bool {
        ch >= '0' && ch <= '9'
    }

    fn is_alpha(&self, ch: char) -> bool {
        (ch >= 'a' && ch <= 'z') ||
        (ch >= 'A' && ch <= 'Z') ||
        ch == '_'
    }

    fn is_alphanumeric(&self, ch: char) -> bool {
        self.is_alpha(ch) || self.is_digit(ch)
    }

    fn scan_ident(&mut self) -> Result<Token> {
        while self.is_alphanumeric(self.peek()) {
            self.next();
        }

        let text = self.characters[self.start..self.current].iter().cloned().collect::<String>();
        if let Some(ttype) = TokenType::keyword(&text) {
            Ok(self.make_token(ttype, None))
        } else {
            Ok(self.make_token(TokenType::Identifier, None))
        }
    }

    fn scan_number(&mut self) -> Result<Token> {
        let mut is_float = false;
        while self.is_digit(self.peek()) {
            self.next();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            is_float = true;
            // Consume the '.'
            self.next();

            while self.is_digit(self.peek()) {
                self.next();
            }
        }

        let lexeme = self.characters[self.start..self.current].iter().cloned().collect::<String>();
        Ok(self.make_token(TokenType::Number, if is_float {
            let parsed: f64 = lexeme.parse()?;
            let literal = Number::Float(parsed);
            Some(Literal::Number(literal))
        } else {
            let parsed: i64 = lexeme.parse()?;
            let literal = Number::Int(parsed);
            Some(Literal::Number(literal))
        }))
    }

    fn scan_string(&mut self) -> Result<Token> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.next();
        }

        if self.is_at_end() {
            return Err(ErrorKind::LoxError(self.line, "unterminated string!".into()).into());
        }

        // the closing '"'
        self.next();

        let start = self.start + 1;
        let end = self.current - 1;
        let val = self.characters[start..end].iter().cloned().collect::<String>();
        let lit = Literal::String(val);
        Ok(self.make_token(TokenType::String, Some(lit)))
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.characters[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.characters.len() {
            '\0'
        } else {
            self.characters[self.current + 1]
        }
    }

    fn next(&mut self) -> char {
        self.current += 1;
        self.characters[self.current - 1]
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.characters[self.current] != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn make_token(&mut self, ttype: TokenType, literal: Option<Literal>) -> Token {
        let lexeme = self.characters[self.start..self.current].iter().cloned().collect::<String>();
        Token::new(ttype, &lexeme, literal, self.line)
    }
}

