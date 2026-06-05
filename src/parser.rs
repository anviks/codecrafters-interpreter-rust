use crate::{
    ast::{Expr, LiteralValue, Stmt},
    token::{Token, TokenType},
};

#[derive(Debug)]
pub(crate) struct ParseError {
    token: Token,
    pub(crate) message: String,
}

pub(crate) struct Parser {
    pub(crate) tokens: Vec<Token>,
    pub(crate) current: usize,
    pub(crate) encountered_error: bool,
}

impl Parser {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            encountered_error: false,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self) -> &Token {
        let index = self.current;
        self.current += 1;
        &self.tokens[index]
    }

    fn synchronize(&mut self) {
        self.consume();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => self.consume(),
            };
        }
    }

    fn matches(&self, types: &[TokenType]) -> bool {
        for typ in types {
            if self.peek().token_type == *typ {
                return true;
            }
        }
        false
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        match &self.consume().token_type {
            TokenType::False => Ok(Expr::Literal(LiteralValue::Bool(false))),
            TokenType::True => Ok(Expr::Literal(LiteralValue::Bool(true))),
            TokenType::Nil => Ok(Expr::Literal(LiteralValue::Nil)),
            TokenType::Number => {
                let value = self.previous().literal.clone().unwrap();
                Ok(Expr::Literal(LiteralValue::Number(
                    value.parse::<f64>().map_err(|_| ParseError {
                        token: self.previous().clone(),
                        message: "Failed to parse number".to_string(),
                    })?,
                )))
            }
            TokenType::String => Ok(Expr::Literal(LiteralValue::Str(
                self.previous().literal.clone().unwrap(),
            ))),
            TokenType::LeftParen => {
                let expr = self.expression()?;
                if self.consume().token_type != TokenType::RightParen {
                    return Err(ParseError {
                        token: self.previous().clone(),
                        message: String::from("Expected ')' after expression."),
                    });
                }
                Ok(Expr::Grouping(Box::new(expr)))
            }
            TokenType::Identifier => {}
            _ => Err(ParseError {
                token: self.previous().clone(),
                message: String::from("Unexpected token."),
            }),
        }
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.consume().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        self.primary()
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.consume().clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.consume().clone();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while self.matches(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.consume().clone();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.consume().clone();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        let tok = self.consume();
        if let TokenType::Semicolon = tok.token_type {
            return Ok(Stmt::Print(expr));
        }
        Err(ParseError {
            token: tok.clone(),
            message: "Expect ';' after value.".to_string(),
        })
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        let tok = self.consume();
        if let TokenType::Semicolon = tok.token_type {
            return Ok(Stmt::Expression(expr));
        }
        Err(ParseError {
            token: tok.clone(),
            message: "Expect ';' after expression.".to_string(),
        })
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if let TokenType::Print = self.peek().token_type {
            self.consume();
            return self.print_statement();
        }
        self.expression_statement()
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name_tok = self.consume().clone();
        if let TokenType::Identifier = name_tok.token_type {
            let mut init: Option<Expr> = None;
            let eq_tok = self.peek();
            if let TokenType::Equal = eq_tok.token_type {
                init = Some(self.expression()?);
            }

            let semicolon = self.peek();
            if let TokenType::Semicolon = semicolon.token_type {
                self.consume();
                return Ok(Stmt::Var {
                    identifier: name_tok.lexeme,
                    expression: init,
                });
            }

            Err(ParseError {
                token: semicolon.clone(),
                message: "Expect ';' after variable declaration.".to_string(),
            })
        } else {
            return Err(ParseError {
                token: name_tok.clone(),
                message: "Expect variable name.".to_string(),
            });
        }
    }

    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if let TokenType::Var = self.peek().token_type {}
        self.statement()
    }

    pub(crate) fn parse(&mut self) -> Option<Expr> {
        self.encountered_error = false;
        match self.expression() {
            Ok(expr) => Some(expr),
            Err(e) => {
                eprintln!(
                    "[line {}] Error at '{}': Expect expression.",
                    e.token.line, e.token.lexeme
                );
                self.encountered_error = true;
                None
            }
        }
    }

    pub(crate) fn parse_stmts(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = vec![];

        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }
}
