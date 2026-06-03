use std::any::Any;

use crate::{
    ast::{Expr, LiteralValue},
    token::{Token, TokenType},
};

pub(crate) struct Parser {
    pub(crate) tokens: Vec<Token>,
    pub(crate) current: usize,
}

impl Parser {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
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

    fn primary(&mut self) -> Expr {
        match &self.consume().token_type {
            TokenType::False => Expr::Literal(LiteralValue::Bool(false)),
            TokenType::True => Expr::Literal(LiteralValue::Bool(true)),
            TokenType::Nil => Expr::Literal(LiteralValue::Nil),
            TokenType::Number => Expr::Literal(LiteralValue::Number(
                self.previous()
                    .literal
                    .clone()
                    .unwrap()
                    .parse::<f64>()
                    .unwrap(),
            )),
            TokenType::String => {
                Expr::Literal(LiteralValue::Str(self.previous().literal.clone().unwrap()))
            }
            TokenType::LeftParen => {
                let expr = self.expression();
                assert_eq!(
                    self.consume().token_type,
                    TokenType::RightParen,
                    "Expected ')' after expression."
                );
                Expr::Grouping(Box::new(expr))
            }
            t => panic!("Unknown token type: {}", t),
        }
    }

    fn unary(&mut self) -> Expr {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.consume().clone();
            let right = self.unary();
            return Expr::Unary {
                operator,
                right: Box::new(right),
            };
        }

        self.primary()
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.consume().clone();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.consume().clone();
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.matches(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.consume().clone();
            let right = self.term();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.consume().clone();
            let right = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    pub(crate) fn parse(&mut self) -> Expr {
        self.expression()
    }
}
