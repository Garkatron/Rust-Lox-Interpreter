use crate::expression::Expr;

pub enum Stmt {
    Expression { expression: Box<Expr> },
    Print { expression: Box<Expr> },
}