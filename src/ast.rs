use crate::{helpers::format_float, token::Token};

#[derive(Debug)]
pub(crate) enum LiteralValue {
    Number(f64),
    Str(String),
    Bool(bool),
    Nil,
}

#[derive(Debug)]
pub(crate) enum Expr {
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Literal(LiteralValue),
    Variable(Token),
    Assign {
        left: Token,
        right: Box<Expr>,
    },
}

#[derive(Debug)]
pub(crate) enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var {
        identifier: String,
        expression: Option<Expr>,
    },
    Block(Vec<Stmt>),
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
}

impl LiteralValue {
    pub(crate) fn to_string(&self) -> String {
        match self {
            LiteralValue::Number(num) => format_float(*num),
            LiteralValue::Str(s) => s.to_string(),
            LiteralValue::Bool(b) => b.to_string(),
            LiteralValue::Nil => String::from("nil"),
        }
    }
}

impl Expr {
    pub(crate) fn to_string(&self) -> String {
        match self {
            Expr::Unary { operator, right } => {
                format!("({} {})", operator.lexeme, right.to_string())
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => format!(
                "({} {} {})",
                operator.lexeme,
                left.to_string(),
                right.to_string()
            ),
            Expr::Grouping(expr) => format!("(group {})", expr.to_string()),
            Expr::Literal(literal_value) => literal_value.to_string(),
            Expr::Variable(token) => token.lexeme.to_string(),
            Expr::Assign {
                left: name,
                right: value,
            } => format!("(assign {} {})", name.lexeme, value.to_string()),
        }
    }
}
