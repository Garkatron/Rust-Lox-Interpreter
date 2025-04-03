
use crate::core::{error_types::runtime_error::RuntimeError, syntax::token::Token};

use super::expression::Expr;


#[derive(Clone, Debug)]
pub enum Stmt {
    Expression { expression: Expr },
    Print { expression: Expr },
    Var { name: Token, initializer: Expr },
    Class { name: Token, methods: Vec<Stmt>},
    Block { statements: Vec<Stmt> },
    If { condition: Expr, then_branch: Box<Stmt>, else_branch: Option<Box<Stmt>> },
    While { condition: Expr, body: Box<Stmt>, else_branch: Option<Box<Stmt>> },
    Loop { body: Box<Stmt> },
    Break {},
    Function { token: Token, params: Vec<Token>, body: Vec<Stmt>},
    Return { keyword: Token, value: Expr }
}

pub trait Visitor<R> {
    fn visit_expression(&mut self, expression: &Expr) -> Result<R, RuntimeError>;
    fn visit_print(&mut self, expression: &Expr) -> Result<R, RuntimeError>;
    fn visit_var(&mut self, name: &Token, initializer: &Expr) -> Result<R, RuntimeError>;
    fn visit_block(&mut self, statements: &[Stmt]) -> Result<R, RuntimeError>;
    fn visit_if(&mut self, condition: &Expr, then_branch: &Stmt, else_branch: Option<&Stmt>) -> Result<R, RuntimeError>;
    fn visit_while(&mut self, condition: &Expr, body: &Stmt, else_branch: Option<&Stmt>) -> Result<R, RuntimeError>;
    fn visit_loop(&mut self, body: &Stmt) -> Result<R, RuntimeError>;
    fn visit_break(&mut self) -> Result<R, RuntimeError>;
    fn visit_function(&mut self, token: &Token, params: &[Token], body: &[Stmt]) -> Result<R, RuntimeError>;
    fn visit_class(&mut self, name: &Token, methods: &[Stmt]) -> Result<R, RuntimeError>;
    fn visit_return(&mut self, keyword: &Token, value: &Expr) -> Result<R, RuntimeError>;
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
            Stmt::Break { .. } => {
                visitor.visit_break()
            }
            Stmt::Function { token, params, body } => {
                visitor.visit_function(token, params, body)
            }
            Stmt::Return { keyword, value } => {
                visitor.visit_return(keyword, value)
            }
            Stmt::Class { name, methods } => {
                visitor.visit_class(name, methods)
            }
        }
    }
}