use crate::{
    ast::{Expr, LiteralValue},
    token::TokenType,
};

pub(crate) struct RuntimeError {
    pub(crate) message: String,
}

#[derive(Debug)]
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
                message: "Expected a number".to_string(),
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

pub(crate) fn evaluate(expr: Expr) -> Result<LoxValue, RuntimeError> {
    match expr {
        Expr::Unary { operator, right } => {
            let r = evaluate(*right)?;

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
            let l = evaluate(*left)?;
            let r = evaluate(*right)?;

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
                        message: "Operands must be two numbers or two strings".to_string(),
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
        Expr::Grouping(ex) => Ok(evaluate(*ex)?),
        Expr::Literal(literal_value) => Ok(literal_value.into()),
    }
}
