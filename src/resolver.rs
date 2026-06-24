use std::collections::HashMap;

use crate::{
    ast::{Expr, Stmt},
    interpreter::Interpreter,
};

#[derive(Clone, Copy)]
enum FunctionType {
    None,
    Function,
}

pub(crate) struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
}

pub(crate) struct ResolveError {
    pub(crate) message: String,
}

impl<'a> Resolver<'a> {
    pub(crate) fn new(interpreter: &'a mut Interpreter) -> Self {
        Resolver {
            interpreter,
            scopes: vec![],
            current_function: FunctionType::None,
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: String) -> Result<(), ResolveError> {
        if let Some(map) = self.scopes.last_mut()
            && let Some(_) = map.insert(name, false)
        {
            return Err(ResolveError {
                message: "Already a variable with this name in this scope.".to_string(),
            });
        }
        Ok(())
    }

    fn define(&mut self, name: String) {
        if let Some(map) = self.scopes.last_mut() {
            map.insert(name, true);
        }
    }

    pub(crate) fn resolve_statements(&mut self, stmts: &Vec<Stmt>) -> Result<(), ResolveError> {
        for stmt in stmts {
            self.resolve_statement(stmt)?;
        }
        Ok(())
    }

    fn resolve_local(&mut self, expr: &Expr, name: &str) {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(name) {
                self.interpreter.resolve(expr, i);
                return;
            }
        }
    }

    fn resolve_statement(&mut self, stmt: &Stmt) -> Result<(), ResolveError> {
        match stmt {
            Stmt::Expression(expr) => self.resolve_expression(expr),
            Stmt::Print(expr) => self.resolve_expression(expr),
            Stmt::Var {
                identifier,
                expression,
            } => {
                self.declare(identifier.clone())?;
                if let Some(init) = expression {
                    self.resolve_expression(init)?;
                }
                self.define(identifier.clone());
                Ok(())
            }
            Stmt::Block(stmts) => {
                self.begin_scope();
                self.resolve_statements(stmts)?;
                self.end_scope();
                Ok(())
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.resolve_expression(condition)?;
                self.resolve_statement(then_branch)?;
                match else_branch {
                    Some(branch) => self.resolve_statement(branch),
                    None => Ok(()),
                }
            }
            Stmt::While { condition, body } => {
                self.resolve_expression(condition)?;
                self.resolve_statement(body)
            }
            Stmt::Function {
                name,
                parameters,
                body,
            } => {
                self.declare(name.lexeme.clone())?;
                self.define(name.lexeme.clone());

                let enclosing_func = self.current_function;
                self.current_function = FunctionType::Function;

                self.begin_scope();
                for param in parameters {
                    self.declare(param.lexeme.clone())?;
                    self.define(param.lexeme.clone());
                }
                self.resolve_statements(body)?;
                self.end_scope();

                self.current_function = enclosing_func;

                Ok(())
            }
            Stmt::Return { keyword: _, value } => match self.current_function {
                FunctionType::None => Err(ResolveError {
                    message: "Can't return from top-level code.".to_string(),
                }),
                FunctionType::Function => self.resolve_expression(value),
            },
        }
    }

    fn resolve_expression(&mut self, expr: &Expr) -> Result<(), ResolveError> {
        match expr {
            Expr::Unary { operator: _, right } => self.resolve_expression(right),
            Expr::Grouping(expr) => self.resolve_expression(expr),
            Expr::Literal(_literal_value) => Ok(()),
            Expr::Variable(token) => {
                if let Some(scope) = self.scopes.last()
                    && let Some(b) = scope.get(&token.lexeme)
                    && !*b
                {
                    Err(ResolveError {
                        message: "Can't read local variable in its own initializer.".to_string(),
                    })
                } else {
                    self.resolve_local(expr, &token.lexeme);
                    Ok(())
                }
            }
            Expr::Assign { left, right } => {
                self.resolve_expression(right)?;
                self.resolve_local(expr, &left.lexeme);
                Ok(())
            }
            Expr::Logical {
                left,
                operator: _,
                right,
            }
            | Expr::Binary {
                left,
                operator: _,
                right,
            } => {
                self.resolve_expression(left)?;
                self.resolve_expression(right)
            }
            Expr::Call {
                callee,
                paren: _,
                arguments,
            } => {
                self.resolve_expression(callee)?;
                for arg in arguments {
                    self.resolve_expression(arg)?;
                }
                Ok(())
            }
        }
    }
}
