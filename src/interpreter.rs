use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{Expr, Stmt},
    environment::Environment,
    natives::clock,
    token::TokenType,
    value::{LoxValue, RuntimeError},
};

pub(crate) struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub(crate) fn new() -> Self {
        let mut env = Environment::new();
        let clock_fn = LoxValue::NativeFunction {
            name: "clock".to_string(),
            arity: 0,
            func: clock,
        };
        env.define("clock".to_string(), clock_fn);

        Self {
            environment: Rc::new(RefCell::new(env)),
        }
    }

    pub(crate) fn evaluate(&mut self, expr: &Expr) -> Result<LoxValue, RuntimeError> {
        match expr {
            Expr::Unary { operator, right } => {
                let r = self.evaluate(right)?;

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
                let l = self.evaluate(left)?;
                let r = self.evaluate(right)?;

                match &operator.token_type {
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
            Expr::Grouping(ex) => Ok(self.evaluate(ex)?),
            Expr::Literal(literal_value) => Ok(literal_value.clone().into()),
            Expr::Variable(token) => Ok(self.environment.borrow().get(token.clone())?.clone()),
            Expr::Assign { left, right } => {
                let value = self.evaluate(right)?;
                self.environment
                    .borrow_mut()
                    .assign(left.clone(), value.clone())?;
                Ok(value)
            }
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let l = self.evaluate(left)?;

                match operator.token_type {
                    TokenType::And if !l.is_truthy() => Ok(l),
                    TokenType::Or if l.is_truthy() => Ok(l),
                    _ => self.evaluate(right),
                }
            }
            Expr::Call {
                callee,
                paren,
                arguments,
            } => {
                let callable = self.evaluate(callee)?;

                let mut args = vec![];
                for arg in arguments {
                    args.push(self.evaluate(arg)?);
                }

                callable.call(self, args)
            }
        }
    }

    pub(crate) fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
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
                    Ok(self
                        .environment
                        .borrow_mut()
                        .define(identifier.to_string(), value))
                }
                None => Ok(self
                    .environment
                    .borrow_mut()
                    .define(identifier.to_string(), LoxValue::Nil)),
            },
            Stmt::Block(stmts) => {
                let previous = Rc::clone(&self.environment);
                let child = Environment::new_with_parent(Rc::clone(&previous));
                self.environment = Rc::new(RefCell::new(child));

                let result = stmts.into_iter().try_for_each(|stmt| self.execute(stmt));
                self.environment = previous;

                result
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if self.evaluate(condition)?.is_truthy() {
                    self.execute(then_branch)?;
                } else if let Some(stmt) = else_branch {
                    self.execute(stmt)?;
                }

                Ok(())
            }
            Stmt::While { condition, body } => {
                while self.evaluate(condition)?.is_truthy() {
                    self.execute(body)?;
                }
                Ok(())
            }
        }
    }

    pub(crate) fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), RuntimeError> {
        for stmt in statements {
            self.execute(&stmt)?;
        }
        Ok(())
    }
}
