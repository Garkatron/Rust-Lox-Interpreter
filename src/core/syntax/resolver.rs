use std::cell::RefCell;

use std::rc::Rc;

use rustc_hash::FxHashMap;

use super::components::expression::{Expr, LiteralValue, Visitor as ExpressionVisitor};
use super::components::stmt::{Stmt, Visitor as StatementVisitor};
use super::token::Token;
use crate::core::error_types::resolver_error::ResolverError;
use crate::core::error_types::runtime_error::RuntimeError;
use crate::core::interpreter::Interpreter;
use crate::core::lox::Lox;
use crate::utils::colors::Color;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FunctionType {
    NONE,
    FUNCTION
}

pub struct Resolver {
    interpreter: Rc<RefCell<Interpreter>>,
    scopes: Vec<FxHashMap<String, bool>>,
    current_function: FunctionType
}

impl ExpressionVisitor<()> for Resolver {
    fn visit_variable(&mut self, name: &Token, expr: &Expr) -> Result<(), RuntimeError> {
        if self.scopes.is_empty() {
            if let Some(scope) = self.scopes.last() {
                if let Some(b) = scope.get(&name.lexeme) {
                    if *b == false {
                        Color::ecprintln("[RESOLVER]: Can't read local variable in its own initializer.", Color::Cyan);
                    } else {
                        self.resolve_local(expr, name);
                    }   
                }
            }
        }
        Ok(())
    }
    fn visit_assing(&mut self, name: &Token, value: &Expr) -> Result<(), RuntimeError> {
        self.resolve_expr(value);
        self.resolve_local(value, name);
        Ok(())
    }
    fn visit_binary(&mut self, left: &Expr, _operator: &Token, right: &Expr) -> Result<(), RuntimeError> {
        self.resolve_expr(left);
        self.resolve_expr(right);
        Ok(())
    }
    fn visit_call(&mut self, callee: &Expr, _paren: &Token, arguments: &[Expr]) -> Result<(), RuntimeError> {
        self.resolve_expr(callee);
        for expr in arguments {
            self.resolve_expr(expr);
        }
        Ok(())
    }
    fn visit_grouping(&mut self, expression: &Expr) -> Result<(), RuntimeError> {
        self.resolve_expr(expression);
        Ok(())
    }
    fn visit_literal(&mut self, _value: &LiteralValue) -> Result<(), RuntimeError> {
        Ok(())
    }
    fn visit_logical(&mut self, left: &Expr, _operator: &Token, right: &Expr) -> Result<(), RuntimeError> {
        self.resolve_expr(left);
        self.resolve_expr(right);
        Ok(())
    }
    fn visit_unary(&mut self, _operator: &Token, right: &Expr) -> Result<(), RuntimeError> {
        self.resolve_expr(right);
        Ok(())
    }
    fn visit_ternary(
            &mut self,
            condition: &Expr,
            then_branch: &Expr,
            else_branch: &Expr,
        ) -> Result<(), RuntimeError> {
            self.resolve_expr(condition);
            self.resolve_expr(then_branch);
            self.resolve_expr(else_branch);
            Ok(())
    }
    fn visit_comma(&mut self, _left: &Expr, _right: &Expr) -> Result<(), RuntimeError> {
        Ok(())
    }
}

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
    fn visit_function(&mut self, token: &Token, params: &[Token], body: &[Stmt]) -> Result<(), RuntimeError> {
        self.declare(token);
        self.define(token);
        self.resolve_function(Stmt::Function { token: token.clone(), params: params.to_vec(), body: body.to_vec() }, FunctionType::FUNCTION);
        Ok(())
    }
    fn visit_expression(&mut self, expression: &Expr) -> Result<(), RuntimeError> {
        self.resolve_expr(expression);
        Ok(())
    }
    fn visit_if(&mut self, condition: &Expr, then_branch: &Stmt, else_branch: Option<&Stmt>) -> Result<(), RuntimeError> {
        self.resolve_expr(condition);
        self.resolve_statement(then_branch);
        if let Some(else_b) = else_branch {
            self.resolve_statement(else_b);
        }
        Ok(())
    }
    fn visit_print(&mut self, expression: &Expr) -> Result<(), RuntimeError> {
        self.resolve_expr(expression);
        Ok(())
    }
    fn visit_return(&mut self, _keyword: &Token, value: &Expr) -> Result<(), RuntimeError> {
        if self.current_function == FunctionType::NONE {
            // Lox::print_error("Can't return from top-level code.");

            return Err(RuntimeError::Return(LiteralValue::Nil));
        }
        
        // ! NEED CHECK IF RETURN HAS A VALUE, MAKE THE VALUE OPTIONAL 

        self.resolve_expr(value);
        Ok(())
    }
    fn visit_while(&mut self, condition: &Expr, body: &Stmt, else_branch: Option<&Stmt>) -> Result<(), RuntimeError> {
        self.resolve_expr(condition);
        self.resolve_statement(body);
        if let Some(else_b) = else_branch {
            self.resolve_statement(else_b);
        }
        Ok(())
    }
    fn visit_break(&mut self) -> Result<(), RuntimeError> {
        Ok(())
    }
    fn visit_loop(&mut self, body: &Stmt) -> Result<(), RuntimeError> {
        self.resolve_statement(body);
        Ok(())
    }

}

impl Resolver {
    pub fn new(interpreter: Rc<RefCell<Interpreter>>) -> Self {
        Self { interpreter, scopes: vec![], current_function: FunctionType::NONE }
    }

    pub fn resolve_statements(&mut self, statements: &[Stmt]) -> Result<(), RuntimeError> {
        for stmt in statements {
            self.resolve_statement(stmt)?;
        }
        Ok(())
    }
    
    fn resolve_statement(&mut self, statement: &Stmt) -> Result<(), RuntimeError>{
        statement.accept(self)?;
        Ok(())
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), RuntimeError> {
        expr.accept(self)?;
        Ok(())
    }
    fn resolve_function(&mut self, function: Stmt, ftype: FunctionType) -> Result<(), RuntimeError> {
        if let Stmt::Function { params, body , ..} = function {
            let enclosing_function = self.current_function;
            self.current_function = ftype;
            self.being_scope();
            for param in params {
                self.declare(&param);
                self.define(&param);
            }
            self.resolve_statements(&body)?;
            self.end_scope();
            self.current_function = enclosing_function;
        }
        Ok(())
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

        if scope.contains_key(&name.lexeme) {
            Lox::print_error(&format!("Already a variable called [{}] with this name in this scope.", &name.lexeme));
        }

        scope.insert(name.lexeme.clone(), false);
    }
    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }
        let scope = self.scopes.last_mut().unwrap();
        scope.insert(name.lexeme.clone(), true);
    }

    fn resolve_local(&self, expr: &Expr, name: &Token) {
        for i in 0..self.scopes.len()-1 {
            if let Some(scope) = self.scopes.get(i) {
                if scope.contains_key(&name.lexeme) {
                    self.interpreter.borrow_mut().resolve(expr.clone(), self.scopes.len()-1);
                    return;
                }
            }
        }
    }
}
