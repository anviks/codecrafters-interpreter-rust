use crate::token::{Token, TokenType};

pub struct Lexer {
    pub source: Vec<char>,
    pub current: usize,
    pub line: u32,
    pub encountered_error: bool,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            current: 0,
            line: 1,
            encountered_error: false,
        }
    }

    fn eof(&self) -> bool {
        self.current >= self.source.len()
    }

    fn peek_at(&self, offset: usize) -> char {
        if self.current + offset >= self.source.len() {
            return '\0';
        }
        self.source[self.current + offset]
    }

    fn peek(&self) -> char {
        self.peek_at(0)
    }

    fn consume(&mut self) -> char {
        let c = self.peek();
        self.current += 1;
        c
    }

    pub fn analyze(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        while !self.eof() {
            let double_char = match (self.peek(), self.peek_at(1)) {
                ('=', '=') => Some(TokenType::EqualEqual),
                ('!', '=') => Some(TokenType::BangEqual),
                _ => None,
            };

            let single_char = match self.peek() {
                '(' => Some(TokenType::LeftParen),
                ')' => Some(TokenType::RightParen),
                '{' => Some(TokenType::LeftBrace),
                '}' => Some(TokenType::RightBrace),
                '*' => Some(TokenType::Star),
                '.' => Some(TokenType::Dot),
                ',' => Some(TokenType::Comma),
                '+' => Some(TokenType::Plus),
                '-' => Some(TokenType::Minus),
                ';' => Some(TokenType::Semicolon),
                '=' => Some(TokenType::Equal),
                '!' => Some(TokenType::Bang),
                _ => None,
            };

            if let Some(token_type) = double_char {
                tokens.push(Token {
                    token_type,
                    lexeme: self.consume().to_string() + &self.consume().to_string(),
                    line: self.line,
                });
            } else if let Some(token_type) = single_char {
                tokens.push(Token {
                    token_type,
                    lexeme: self.consume().to_string(),
                    line: self.line,
                });
            } else {
                match self.peek() {
                    c => {
                        eprintln!("[line {}] Error: Unexpected character: {}", self.line, c);
                        self.encountered_error = true;
                        self.consume();
                    }
                }
            }
        }

        tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: String::new(),
            line: self.line,
        });

        tokens
    }
}
