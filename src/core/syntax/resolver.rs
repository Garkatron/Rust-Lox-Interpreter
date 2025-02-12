use std::cell::RefCell;

use std::fs::soft_link;
use std::rc::Rc;

use rustc_hash::FxHashMap;

use super::components::expression::{Expr, LiteralValue, Visitor as ExpressionVisitor};
use super::components::stmt::{Stmt, Visitor as StatementVisitor};
use super::token::Token;
use crate::core::error_types::runtime_error::RuntimeError;
use crate::core::interpreter::Interpreter;

pub struct Resolver {
    interpreter: Rc<RefCell<Interpreter>>,
    scopes: Vec<FxHashMap<String, bool>>,
}

impl ExpressionVisitor<()> for Resolver {}

impl StatementVisitor<()> for Resolver {
    fn visit_block(&mut self, statements: &[Stmt]) -> Result<(), RuntimeError> {
        self.being_scope();
        self.resolve_statements(statements);
        self.end_scope();
        Ok(())
    }
    fn visit_var(&mut self, name: &Token, initializer: &Expr) -> Result<(), RuntimeError> {
        self.declare(name);
        self.resolve_expr(initializer);
        self.define(name);
        Ok(())
    }
}

impl Resolver {
    fn new(interpreter: Rc<RefCell<Interpreter>>) -> Self {
        Self { interpreter, scopes: vec![] }
    }

    fn resolve_statements(&mut self, statements: &[Stmt]) {
        statements.iter().for_each(|f| self.resolve_statement(f));
    }

    fn resolve_statement(&mut self, statement: &Stmt) {
        statement.accept(self);
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        expr.accept(self);
    }

    fn being_scope(&mut self) {
        self.scopes.push(FxHashMap::default());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }
        let scope = self.scopes.last_mut().unwrap();
        scope.insert(name.lexeme.clone(), false);
    }
    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }
        let scope = self.scopes.last_mut().unwrap();
        scope.insert(name.lexeme.clone(), true);
    }
    
    
}
