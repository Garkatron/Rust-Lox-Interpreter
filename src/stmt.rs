use crate::expression::Expr;
use crate::runtime_error::RuntimeError;

#[derive(Clone, Debug)]
pub enum Stmt {
    Expression { expression: Box<Expr> },
    Print { expression: Box<Expr> },
}
pub trait Visitor<R> {
    fn visit_expression(&self, stmt: &Stmt) -> Result<R, RuntimeError>;
    fn visit_print(&self, stmt: &Stmt) -> Result<R, RuntimeError>;
}

impl Stmt {
    pub fn accept<R>(&self, visitor: &dyn Visitor<R>) -> Result<R, RuntimeError> {
        match self {
            Stmt::Expression { .. } => visitor.visit_expression(self),
            Stmt::Print { .. } => visitor.visit_print(self),
        }
    }
}
