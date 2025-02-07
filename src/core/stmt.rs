use super::{expression::Expr, runtime_error::RuntimeError, token::Token};

#[derive(Clone, Debug)]
pub enum Stmt {
    Expression { expression: Box<Expr> },
    Print { expression: Box<Expr> },
    Var { name: Token, initializer: Box<Expr> },
}
pub trait Visitor<R> {
    fn visit_expression(&mut self, stmt: &Stmt) -> Result<R, RuntimeError>;
    fn visit_print(&mut self, stmt: &Stmt) -> Result<R, RuntimeError>;
    fn visit_var(&mut self, stmt: &Stmt) -> Result<R, RuntimeError>;
}

impl Stmt {
    pub fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> Result<R, RuntimeError> {
        match self {
            Stmt::Expression { .. } => visitor.visit_expression(self),
            Stmt::Print { .. } => visitor.visit_print(self),
            Stmt::Var { .. } => visitor.visit_var(self),
        }
    }
}
