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

    fn advance(&mut self) -> &Token {
        let index = self.current;
        self.current += 1;
        &self.tokens[index]
    }

    fn expect(&mut self, token_type: TokenType, err_msg: &str) -> Result<Token, ParseError> {
        let token = self.peek().clone();
        if token.token_type == token_type {
            self.current += 1;
            Ok(token)
        } else {
            Err(ParseError {
                message: err_msg.to_string(),
                token,
            })
        }
    }

    fn matches(&mut self, types: &[TokenType]) -> bool {
        for typ in types {
            if self.peek().token_type == *typ {
                self.advance();
                return true;
            }
        }
        false
    }

    fn synchronize(&mut self) {
        self.advance();

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
                _ => self.advance(),
            };
        }
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        match &self.advance().token_type {
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
                self.expect(TokenType::RightParen, "Expected ')' after expression.")?;
                Ok(Expr::Grouping(Box::new(expr)))
            }
            TokenType::Identifier => Ok(Expr::Variable(self.previous().clone())),
            _ => Err(ParseError {
                token: self.previous().clone(),
                message: String::from("Unexpected token."),
            }),
        }
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
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
            let operator = self.previous().clone();
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
            let operator = self.previous().clone();
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
            let operator = self.previous().clone();
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
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;

        while self.matches(&[TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.and()?;

        while self.matches(&[TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.or()?;

        if self.matches(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            if let Expr::Variable(token) = expr {
                return Ok(Expr::Assign {
                    left: token,
                    right: Box::new(value),
                });
            }

            return Err(ParseError {
                token: equals,
                message: "Invalid assignment target.".to_string(),
            });
        }

        Ok(expr)
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.expect(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(expr))
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.expect(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Stmt::Expression(expr))
    }

    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.expect(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.expect(TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = self.statement()?;
        let mut else_branch: Option<Box<Stmt>> = None;

        if self.matches(&[TokenType::Else]) {
            else_branch = Some(Box::new(self.statement()?));
        }

        Ok(Stmt::If {
            condition,
            then_branch: Box::new(then_branch),
            else_branch,
        })
    }

    fn while_statement(&mut self) -> Result<Stmt, ParseError> {
        self.expect(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.expect(TokenType::RightParen, "Expect ')' after condition.")?;
        let body = self.statement()?;

        Ok(Stmt::While {
            condition,
            body: Box::new(body),
        })
    }

    fn block(&mut self) -> Result<Stmt, ParseError> {
        let mut statements = vec![];

        while self.peek().token_type != TokenType::RightBrace {
            statements.push(self.declaration()?);
        }

        self.expect(TokenType::RightBrace, "Expect '}' after block.")?;

        Ok(Stmt::Block(statements))
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.matches(&[TokenType::Print]) {
            self.print_statement()
        } else if self.matches(&[TokenType::LeftBrace]) {
            self.block()
        } else if self.matches(&[TokenType::If]) {
            self.if_statement()
        } else if self.matches(&[TokenType::While]) {
            self.while_statement()
        } else {
            self.expression_statement()
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name_tok = self.expect(TokenType::Identifier, "Expect variable name.")?;

        let mut init: Option<Expr> = None;
        if self.matches(&[TokenType::Equal]) {
            init = Some(self.expression()?);
        }

        self.expect(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Stmt::Var {
            identifier: name_tok.lexeme,
            expression: init,
        })
    }

    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.matches(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        }
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
