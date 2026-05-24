use crate::token::{Token, TokenType};

pub struct Lexer {
    pub source: Vec<char>,
    pub current: usize,
    pub line: u32,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            current: 0,
            line: 1,
        }
    }

    fn eof(&self) -> bool {
        self.current >= self.source.len()
    }

    fn peek(&self) -> char {
        self.source[self.current]
    }

    fn consume(&mut self) -> char {
        let c = self.peek();
        self.current += 1;
        c
    }

    pub fn analyze(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        while !self.eof() {
            match self.peek() {
                '(' => {
                    tokens.push(Token {
                        token_type: TokenType::LeftParen,
                        lexeme: self.consume().to_string(),
                        line: self.line,
                    });
                }
                ')' => {
                    tokens.push(Token {
                        token_type: TokenType::RightParen,
                        lexeme: self.consume().to_string(),
                        line: self.line,
                    });
                }
                c => {
                    panic!("Wtf is that char? => '{}'", c)
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
