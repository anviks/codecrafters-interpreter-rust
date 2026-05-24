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
                '{' => {
                    tokens.push(Token {
                        token_type: TokenType::LeftBrace,
                        lexeme: self.consume().to_string(),
                        line: self.line,
                    });
                }
                '}' => {
                    tokens.push(Token {
                        token_type: TokenType::RightBrace,
                        lexeme: self.consume().to_string(),
                        line: self.line,
                    });
                }
                '*' => {
                    tokens.push(Token {
                        token_type: TokenType::Star,
                        lexeme: self.consume().to_string(),
                        line: self.line,
                    });
                }
                '.' => {
                    tokens.push(Token {
                        token_type: TokenType::Dot,
                        lexeme: self.consume().to_string(),
                        line: self.line,
                    });
                }
                ',' => {
                    tokens.push(Token {
                        token_type: TokenType::Comma,
                        lexeme: self.consume().to_string(),
                        line: self.line,
                    });
                }
                '+' => {
                    tokens.push(Token {
                        token_type: TokenType::Plus,
                        lexeme: self.consume().to_string(),
                        line: self.line,
                    });
                }
                '-' => {
                    tokens.push(Token {
                        token_type: TokenType::Minus,
                        lexeme: self.consume().to_string(),
                        line: self.line,
                    });
                }
                ';' => {
                    tokens.push(Token {
                        token_type: TokenType::Semicolon,
                        lexeme: self.consume().to_string(),
                        line: self.line,
                    });
                }
                c => {
                    eprintln!("[line {}] Error: Unexpected character: {}", self.line, c);
                    self.encountered_error = true;
                    self.consume();
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
