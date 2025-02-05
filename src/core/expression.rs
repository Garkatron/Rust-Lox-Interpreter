use std::fmt;

use super::{runtime_error::RuntimeError, token::Token};

#[derive(Clone, Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: LiteralValue,
    },
    Unary {
        operator: Token,
        right: Box<Expr>
    },
    Comma {
        left: Box<Expr>,
        right: Box<Expr>
    },
    Ternary {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Box<Expr>
    }
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

pub trait Visitor<R> {
    fn visit_binary(&self, left: &Expr, operator: &Token, right: &Expr) -> Result<R, RuntimeError>;
    fn visit_grouping(&self, expression: &Expr) -> Result<R, RuntimeError>;
    fn visit_literal(&self, value: &LiteralValue) -> Result<R, RuntimeError>;
    fn visit_comma(&self, left: &Expr, right: &Expr) -> Result<R, RuntimeError>;
    fn visit_unary(&self, operator: &Token, right: &Expr) -> Result<R, RuntimeError>;
    fn visit_ternary(&self, condition: &Expr, then_branch: &Expr, else_branch: &Expr) -> Result<R, RuntimeError>;
}

impl Expr {
    pub fn accept<R>(&self, visitor: &dyn Visitor<R>) -> Result<R, RuntimeError> {
        match self {
            Expr::Binary { left, operator, right } => visitor.visit_binary(left, operator, right),
            Expr::Grouping { expression } => visitor.visit_grouping(expression),
            Expr::Literal { value } => visitor.visit_literal(value),
            Expr::Unary { operator, right } => visitor.visit_unary(operator, right),
            Expr::Comma { left, right } => visitor.visit_comma(left, right),
            Expr::Ternary { condition, then_branch, else_branch } => {
                visitor.visit_ternary(condition, then_branch, else_branch)
            }
        }
    }
}


impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LiteralValue::Number(n) => write!(f, "{}", n),
            LiteralValue::String(s) => write!(f, "\"{}\"", s),
            LiteralValue::Boolean(b) => write!(f, "{}", b),
            LiteralValue::Nil => write!(f, "nil"),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Binary { left, operator, right } => {
                write!(f, "({} {} {})", operator.lexeme, left, right)
            }
            Expr::Grouping { expression } => {
                write!(f, "(group {})", expression)
            }
            Expr::Literal { value } => {
                write!(f, "{}", value)
            }
            Expr::Unary { operator, right } => {
                write!(f, "({} {})", operator.lexeme, right)
            }
            Expr::Comma { left, right } => {
                write!(f, "({}, {})", left, right)
            },
            Expr::Ternary { condition, then_branch, else_branch } => {
                write!(f, "({}) ? {} : {}", condition, then_branch, else_branch)
            }
        }
    }
}


