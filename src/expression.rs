// Suponiendo que Token es una enumeración ya definida en tu código
use crate::token::Token;

#[derive(Clone)]
pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

#[derive(Clone)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
    pub lexeme: String,
}

#[derive(Clone)]
pub struct Grouping {
    pub expression: Box<Expr>,
}

#[derive(Clone)]
pub struct Literal {
    pub value: LiteralValue,
}

#[derive(Clone)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
    pub lexeme: String,
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

pub trait Visitor<R> {
    fn visit_binary(&self, expr: &Binary) -> R;
    fn visit_grouping(&self, expr: &Grouping) -> R;
    fn visit_literal(&self, expr: &Literal) -> R;
    fn visit_unary(&self, expr: &Unary) -> R;
}

impl Expr {
    pub fn accept<R>(&self, visitor: &dyn Visitor<R>) -> R {
        match self {
            Expr::Binary(expr) => visitor.visit_binary(expr),
            Expr::Grouping(expr) => visitor.visit_grouping(expr),
            Expr::Literal(expr) => visitor.visit_literal(expr),
            Expr::Unary(expr) => visitor.visit_unary(expr),
        }
    }
}
