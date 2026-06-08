use crate::{ast::LiteralValue, interpreter::Interpreter};

pub(crate) struct RuntimeError {
    pub(crate) message: String,
}

#[derive(Debug, Clone)]
pub(crate) struct LoxFunction {
    arity: usize,
}

impl LoxFunction {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<LoxValue>,
    ) -> Result<LoxValue, RuntimeError> {
        if args.len() != self.arity {
            return Err(RuntimeError {
                message: format!("Expected {} arguments but got {}.", self.arity, args.len()),
            });
        }

        todo!()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct LoxClass {
    arity: usize,
}

impl LoxClass {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<LoxValue>,
    ) -> Result<LoxValue, RuntimeError> {
        if args.len() != self.arity {
            return Err(RuntimeError {
                message: format!("Expected {} arguments but got {}.", self.arity, args.len()),
            });
        }

        todo!()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum LoxValue {
    Number(f64),
    Str(String),
    Bool(bool),
    Nil,
    Function(LoxFunction),
    NativeFunction {
        name: String,
        arity: usize,
        func: fn(&mut Interpreter, Vec<LoxValue>) -> Result<LoxValue, RuntimeError>,
    },
    Class(LoxClass),
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
    pub(crate) fn as_number(&self) -> Result<f64, RuntimeError> {
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
            LoxValue::Function(lox_function) => format!("<function {}>", lox_function.arity),
            LoxValue::Class(lox_class) => format!("<class '{}'>", lox_class.arity),
            LoxValue::NativeFunction { name, arity, func } => {
                format!("<built-in function {}>", name)
            }
        }
    }

    pub(crate) fn is_truthy(&self) -> bool {
        if let LoxValue::Bool(b) = self {
            return *b;
        }

        if let LoxValue::Nil = self {
            return false;
        }

        true
    }

    pub(crate) fn is_equal_to(&self, other: &Self) -> bool {
        match (self, other) {
            (LoxValue::Number(n1), LoxValue::Number(n2)) => n1 == n2,
            (LoxValue::Str(s1), LoxValue::Str(s2)) => s1 == s2,
            (LoxValue::Bool(b1), LoxValue::Bool(b2)) => b1 == b2,
            (LoxValue::Nil, LoxValue::Nil) => true,
            _ => false,
        }
    }

    pub(crate) fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<LoxValue>,
    ) -> Result<LoxValue, RuntimeError> {
        match self {
            LoxValue::Function(lox_function) => lox_function.call(interpreter, args),
            LoxValue::Class(lox_class) => lox_class.call(interpreter, args),
            LoxValue::NativeFunction { name, arity, func } => func(interpreter, args),
            _ => Err(RuntimeError {
                message: "Not callable".to_string(),
            }),
        }
    }
}
