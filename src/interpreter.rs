use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{Expr, LiteralValue, Stmt},
    environment::Environment,
    token::TokenType,
};

pub(crate) struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

pub(crate) struct RuntimeError {
    pub(crate) message: String,
}

#[derive(Debug, Clone)]
pub(crate) enum LoxValue {
    Number(f64),
    Str(String),
    Bool(bool),
    Nil,
}

impl From<LiteralValue> for LoxValue {
    fn from(literal: LiteralValue) -> Self {
        match literal {
            LiteralValue::Number(n) => LoxValue::Number(n),
            LiteralValue::Str(s) => LoxValue::Str(s),
            LiteralValue::Bool(b) => LoxValue::Bool(b),
            LiteralValue::Nil => LoxValue::Nil,
        }
    }
}

impl LoxValue {
    fn as_number(&self) -> Result<f64, RuntimeError> {
        if let LoxValue::Number(num) = self {
            Ok(*num)
        } else {
            return Err(RuntimeError {
                message: "Operand must be a number.".to_string(),
            });
        }
    }

    pub(crate) fn to_string(&self) -> String {
        match self {
            LoxValue::Number(n) => n.to_string(),
            LoxValue::Str(s) => s.to_string(),
            LoxValue::Bool(b) => b.to_string(),
            LoxValue::Nil => String::from("nil"),
        }
    }

    fn is_truthy(&self) -> bool {
        if let LoxValue::Bool(b) = self {
            return *b;
        }

        if let LoxValue::Nil = self {
            return false;
        }

        true
    }

    fn is_equal_to(&self, other: &Self) -> bool {
        match (self, other) {
            (LoxValue::Number(n1), LoxValue::Number(n2)) => n1 == n2,
            (LoxValue::Str(s1), LoxValue::Str(s2)) => s1 == s2,
            (LoxValue::Bool(b1), LoxValue::Bool(b2)) => b1 == b2,
            (LoxValue::Nil, LoxValue::Nil) => true,
            _ => false,
        }
    }
}

impl Interpreter {
    pub(crate) fn new() -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub(crate) fn evaluate(&mut self, expr: Expr) -> Result<LoxValue, RuntimeError> {
        match expr {
            Expr::Unary { operator, right } => {
                let r = self.evaluate(*right)?;

                match operator.token_type {
                    TokenType::Minus => Ok(LoxValue::Number(-r.as_number()?)),
                    TokenType::Bang => Ok(LoxValue::Bool(!r.is_truthy())),
                    _ => Ok(LoxValue::Nil),
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let l = self.evaluate(*left)?;
                let r = self.evaluate(*right)?;

                match operator.token_type {
                    TokenType::Slash => Ok(LoxValue::Number(l.as_number()? / r.as_number()?)),
                    TokenType::Star => Ok(LoxValue::Number(l.as_number()? * r.as_number()?)),
                    TokenType::Minus => Ok(LoxValue::Number(l.as_number()? - r.as_number()?)),
                    TokenType::Plus => match (l, r) {
                        (LoxValue::Number(l_num), LoxValue::Number(r_num)) => {
                            Ok(LoxValue::Number(l_num + r_num))
                        }
                        (LoxValue::Str(l_str), LoxValue::Str(r_str)) => {
                            Ok(LoxValue::Str(l_str + r_str.as_str()))
                        }
                        _ => Err(RuntimeError {
                            message: "Operands must be two numbers or two strings.".to_string(),
                        }),
                    },
                    TokenType::Less => Ok(LoxValue::Bool(l.as_number()? < r.as_number()?)),
                    TokenType::LessEqual => Ok(LoxValue::Bool(l.as_number()? <= r.as_number()?)),
                    TokenType::Greater => Ok(LoxValue::Bool(l.as_number()? > r.as_number()?)),
                    TokenType::GreaterEqual => Ok(LoxValue::Bool(l.as_number()? >= r.as_number()?)),
                    TokenType::EqualEqual => Ok(LoxValue::Bool(l.is_equal_to(&r))),
                    TokenType::BangEqual => Ok(LoxValue::Bool(!l.is_equal_to(&r))),
                    typ => Err(RuntimeError {
                        message: format!("Bad operator for binary expression: {}", typ.to_string()),
                    }),
                }
            }
            Expr::Grouping(ex) => Ok(self.evaluate(*ex)?),
            Expr::Literal(literal_value) => Ok(literal_value.into()),
            Expr::Variable(token) => Ok(self.environment.borrow().get(token)?.clone()),
            Expr::Assign { left, right } => {
                let value = self.evaluate(*right)?;
                self.environment.borrow_mut().assign(left, value.clone())?;
                Ok(value)
            }
        }
    }

    pub(crate) fn execute(&mut self, stmt: Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Expression(expr) => {
                self.evaluate(expr)?;
                Ok(())
            }
            Stmt::Print(expr) => {
                let val = self.evaluate(expr)?;
                println!("{}", val.to_string());
                Ok(())
            }
            Stmt::Var {
                identifier,
                expression,
            } => match expression {
                Some(expr) => {
                    let value = self.evaluate(expr)?;
                    Ok(self.environment.borrow_mut().define(identifier, value))
                }
                None => Ok(self
                    .environment
                    .borrow_mut()
                    .define(identifier, LoxValue::Nil)),
            },
            Stmt::Block(stmts) => {
                let previous = Rc::clone(&self.environment);
                let child = Environment::new_with_parent(Rc::clone(&previous));
                self.environment = Rc::new(RefCell::new(child));

                let result = stmts.into_iter().try_for_each(|stmt| self.execute(stmt));
                self.environment = previous;

                result
            }
        }
    }

    pub(crate) fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), RuntimeError> {
        for stmt in statements {
            self.execute(stmt)?;
        }
        Ok(())
    }
}
