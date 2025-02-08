use super::{expression::Expr, runtime_error::RuntimeError, token::Token};

#[derive(Clone, Debug)]
pub enum Stmt {
    Expression { expression: Expr },
    Print { expression: Expr },
    Var { name: Token, initializer: Expr },
    Block { statements: Vec<Stmt> },
    If { condition: Expr, then_branch: Box<Stmt>, else_branch: Option<Box<Stmt>> },
    While { condition: Expr, body: Box<Stmt>, else_branch: Option<Box<Stmt>> },
    Loop { body: Box<Stmt> }
}

pub trait Visitor<R> {
    fn visit_expression(&mut self, expression: &Expr) -> Result<R, RuntimeError>;
    fn visit_print(&mut self, expression: &Expr) -> Result<R, RuntimeError>;
    fn visit_var(&mut self, name: &Token, initializer: &Expr) -> Result<R, RuntimeError>;
    fn visit_block(&mut self, statements: &[Stmt]) -> Result<R, RuntimeError>;
    fn visit_if(&mut self, condition: &Expr, then_branch: &Stmt, else_branch: Option<&Stmt>) -> Result<R, RuntimeError>;
    fn visit_while(&mut self, condition: &Expr, body: &Stmt, else_branch: Option<&Stmt>) -> Result<R, RuntimeError>;
    fn visit_loop(&mut self, body: &Stmt) -> Result<R, RuntimeError>;
}

impl Stmt {
    pub fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> Result<R, RuntimeError> {
        match self {
            Stmt::Expression { expression } => visitor.visit_expression(expression),
            Stmt::Print { expression } => visitor.visit_print(expression),
            Stmt::Var { name, initializer } => visitor.visit_var(name, initializer),
            Stmt::Block { statements } => visitor.visit_block(statements),
            Stmt::If { condition, then_branch, else_branch } => visitor.visit_if(condition, then_branch, else_branch.as_deref()),
            Stmt::While { condition, body, else_branch } => {
                visitor.visit_while(condition, body, else_branch.as_deref())
            }
            Stmt::Loop { body } => {
                visitor.visit_loop(body)
            }
        }
    }
}