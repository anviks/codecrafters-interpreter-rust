use crate::token::{Token, TokenType};

fn format_float(f: f64) -> String {
    let s = format!("{}", f);
    if s.contains('.') { s } else { s + ".0" }
}

fn get_token_type_for_identifier(identifier: &str) -> TokenType {
    match identifier {
        "and" => TokenType::And,
        "class" => TokenType::Class,
        "else" => TokenType::Else,
        "false" => TokenType::False,
        "for" => TokenType::For,
        "fun" => TokenType::Fun,
        "if" => TokenType::If,
        "nil" => TokenType::Nil,
        "or" => TokenType::Or,
        "print" => TokenType::Print,
        "return" => TokenType::Return,
        "super" => TokenType::Super,
        "this" => TokenType::This,
        "true" => TokenType::True,
        "var" => TokenType::Var,
        "while" => TokenType::While,
        _ => TokenType::Identifier,
    }
}

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

    fn string(&mut self) -> Option<Token> {
        let mut s = String::new();
        self.consume();
        while !self.eof() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1
            }
            s.push(self.consume());
        }
        if self.eof() {
            eprintln!("[line {}] Error: Unterminated string.", self.line);
            self.encountered_error = true;
            None
        } else {
            self.consume();
            Some(Token {
                token_type: TokenType::String,
                lexeme: format!("\"{}\"", s),
                literal: s,
                line: self.line,
            })
        }
    }

    fn number(&mut self) -> Option<Token> {
        let mut num_str = String::new();
        while !self.eof() && (self.peek().is_ascii_digit() || self.peek() == '.') {
            num_str.push(self.consume());
        }

        let parsed_num = num_str.parse::<f64>();

        match parsed_num {
            Ok(num) => Some(Token {
                token_type: TokenType::Number,
                lexeme: format!("{}", num_str),
                literal: format_float(num),
                line: self.line,
            }),
            Err(_) => {
                eprintln!(
                    "[line {}] Error: Invalid number literal: {}",
                    self.line, num_str
                );
                self.encountered_error = true;
                None
            }
        }
    }

    pub fn analyze(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        while !self.eof() {
            let double_char = match (self.peek(), self.peek_at(1)) {
                ('=', '=') => Some(TokenType::EqualEqual),
                ('!', '=') => Some(TokenType::BangEqual),
                ('<', '=') => Some(TokenType::LessEqual),
                ('>', '=') => Some(TokenType::GreaterEqual),
                ('/', '/') => {
                    while !self.eof() && self.peek() != '\n' {
                        self.consume();
                    }
                    continue;
                }
                ('/', '*') => {
                    let mut depth = 0;
                    while !self.eof() {
                        if self.peek() == '/' && self.peek_at(1) == '*' {
                            self.consume();
                            depth += 1;
                        } else if self.peek() == '*' && self.peek_at(1) == '/' {
                            self.consume();
                            depth -= 1;
                        } else if self.peek() == '\n' {
                            self.line += 1;
                        }

                        self.consume();

                        if depth == 0 {
                            break;
                        }
                    }
                    continue;
                }
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
                '<' => Some(TokenType::Less),
                '>' => Some(TokenType::Greater),
                '/' => Some(TokenType::Slash),
                ' ' | '\t' => {
                    self.consume();
                    continue;
                }
                '\n' => {
                    self.line += 1;
                    self.consume();
                    continue;
                }
                _ => None,
            };

            if let Some(token_type) = double_char {
                tokens.push(Token {
                    token_type,
                    lexeme: self.consume().to_string() + &self.consume().to_string(),
                    line: self.line,
                    literal: "null".to_string(),
                });
            } else if let Some(token_type) = single_char {
                tokens.push(Token {
                    token_type,
                    lexeme: self.consume().to_string(),
                    line: self.line,
                    literal: "null".to_string(),
                });
            } else {
                match self.peek() {
                    '"' => match self.string() {
                        Some(token) => tokens.push(token),
                        None => {}
                    },
                    '0'..='9' => match self.number() {
                        Some(token) => tokens.push(token),
                        None => {}
                    },
                    'a'..='z' | 'A'..='Z' | '_' => {
                        let mut identifier = self.consume().to_string();
                        while !self.eof() {
                            let c = self.peek();
                            if !c.is_ascii_alphanumeric() && c != '_' {
                                break;
                            }
                            identifier.push(self.consume());
                        }

                        tokens.push(Token {
                            token_type: get_token_type_for_identifier(identifier.as_str()),
                            lexeme: identifier,
                            literal: "null".to_string(),
                            line: self.line,
                        });
                    }
                    _ => {
                        let c = self.consume();
                        eprintln!("[line {}] Error: Unexpected character: {}", self.line, c);
                        self.encountered_error = true;
                    }
                }
            }
        }

        tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: String::new(),
            line: self.line,
            literal: "null".to_string(),
        });

        tokens
    }
}
