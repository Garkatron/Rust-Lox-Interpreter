use std::cell::RefCell;

use std::rc::Rc;

use rustc_hash::FxHashMap;

use super::components::expression::{Expr, LiteralValue, Visitor as ExpressionVisitor};
use super::components::stmt::{Stmt, Visitor as StatementVisitor};
use super::token::Token;
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
    unused_variables: Vec<String>,
    current_function: FunctionType
}

impl ExpressionVisitor<()> for Resolver {
    fn visit_variable(&mut self, name: &Token, expr: &Expr) -> Result<(), RuntimeError> {
        if let Some(scope) = self.scopes.last() {
            if scope.get(&name.lexeme) == Some(&false) {
                Color::ecprintln(
                    "[RESOLVER]: Can't read local variable in its own initializer.",
                    Color::Cyan,
                );
            }
        }
        
        self.mark_as_used(&name.lexeme);

        self.resolve_local(expr, name);
        Ok(())
    }
    fn visit_assing(&mut self, name: &Token, value: &Expr) -> Result<(), RuntimeError> {
        self.resolve_expr(value)?;
        self.resolve_local(value, name);
        self.mark_as_used(&name.lexeme);

        Ok(())
    }
    fn visit_binary(&mut self, left: &Expr, _operator: &Token, right: &Expr) -> Result<(), RuntimeError> {
        self.resolve_expr(left)?;
        self.resolve_expr(right)?;
        Ok(())
    }
    fn visit_call(&mut self, callee: &Expr, _paren: &Token, arguments: &[Expr]) -> Result<(), RuntimeError> {
        self.resolve_expr(callee)?;
        for expr in arguments {
            self.resolve_expr(expr)?;
        }
        Ok(())
    }
    fn visit_grouping(&mut self, expression: &Expr) -> Result<(), RuntimeError> {
        self.resolve_expr(expression)?;
        Ok(())
    }
    fn visit_literal(&mut self, _value: &LiteralValue) -> Result<(), RuntimeError> {
        Ok(())
    }
    fn visit_logical(&mut self, left: &Expr, _operator: &Token, right: &Expr) -> Result<(), RuntimeError> {
        self.resolve_expr(left)?;
        self.resolve_expr(right)?;
        Ok(())
    }
    fn visit_unary(&mut self, _operator: &Token, right: &Expr) -> Result<(), RuntimeError> {
        self.resolve_expr(right)?;
        Ok(())
    }
    fn visit_ternary(
            &mut self,
            condition: &Expr,
            then_branch: &Expr,
            else_branch: &Expr,
        ) -> Result<(), RuntimeError> {
            self.resolve_expr(condition)?;
            self.resolve_expr(then_branch)?;
            self.resolve_expr(else_branch)?;
            Ok(())
    }
    fn visit_comma(&mut self, _left: &Expr, _right: &Expr) -> Result<(), RuntimeError> {
        Ok(())
    }

    fn visit_get(&mut self, _name: &Token, object: &Expr) -> Result<(), RuntimeError> {
        self.resolve_expr(object)?;
        Ok(())
    }
}

impl StatementVisitor<()> for Resolver {
    fn visit_block(&mut self, statements: &[Stmt]) -> Result<(), RuntimeError> {
        self.begin_scope();
        self.resolve_statements(statements)?;
        self.end_scope();
        Ok(())
    }
    fn visit_var(&mut self, name: &Token, initializer: &Expr) -> Result<(), RuntimeError> {
        self.declare(name);
        
        match initializer {
            Expr::Literal { value, .. } => {
                if *value != LiteralValue::Nil {
                    self.resolve_expr(initializer)?;
                }
            }
            _ => {
               
            }
        }
    
        self.define(name);
        self.unused_variables.push(name.lexeme.clone());
    
        Ok(())
    }
    
    fn visit_function(&mut self, token: &Token, params: &[Token], body: &[Stmt]) -> Result<(), RuntimeError> {
        self.declare(token);
        self.define(token);
        self.resolve_function(Stmt::Function { token: token.clone(), params: params.to_vec(), body: body.to_vec() }, FunctionType::FUNCTION)?;
        Ok(())
    }
    fn visit_expression(&mut self, expression: &Expr) -> Result<(), RuntimeError> {
        self.resolve_expr(expression)?;
        Ok(())
    }
    fn visit_if(&mut self, condition: &Expr, then_branch: &Stmt, else_branch: Option<&Stmt>) -> Result<(), RuntimeError> {
        self.resolve_expr(condition)?;
        self.resolve_statement(then_branch)?;
        if let Some(else_b) = else_branch {
            self.resolve_statement(else_b)?;
        }
        Ok(())
    }
    fn visit_print(&mut self, expression: &Expr) -> Result<(), RuntimeError> {
        self.resolve_expr(expression)?;
        Ok(())
    }
    fn visit_return(&mut self, _keyword: &Token, value: &Expr) -> Result<(), RuntimeError> {
        if self.current_function == FunctionType::NONE {
            // Lox::print_error("Can't return from top-level code.");

            return Err(RuntimeError::Return(LiteralValue::Nil));
        }
        
        // ! NEED CHECK IF RETURN HAS A VALUE, MAKE THE VALUE OPTIONAL 

        self.resolve_expr(value)?;
        Ok(())
    }
    fn visit_while(&mut self, condition: &Expr, body: &Stmt, else_branch: Option<&Stmt>) -> Result<(), RuntimeError> {
        self.resolve_expr(condition)?;
        self.resolve_statement(body)?;
        if let Some(else_b) = else_branch {
            self.resolve_statement(else_b)?;
        }
        Ok(())
    }
    fn visit_break(&mut self) -> Result<(), RuntimeError> {
        Ok(())
    }
    fn visit_loop(&mut self, body: &Stmt) -> Result<(), RuntimeError> {
        self.resolve_statement(body)?;
        Ok(())
    }
    fn visit_class(&mut self, name: &Token, _methods: &[Stmt]) -> Result<(), RuntimeError> {
        self.declare(name);
        self.define(name);
        Ok(())
    }

}

impl Resolver {
    pub fn new(interpreter: Rc<RefCell<Interpreter>>) -> Self {
        let mut resolver = Self {
            interpreter,
            scopes: vec![],
            unused_variables: vec![],
            current_function: FunctionType::NONE,
        };
    
        resolver.begin_scope();
        resolver
    }

    fn mark_as_used(&mut self, name: &String) {
        if let Some(pos) = self.unused_variables.iter().position(|x| *x==*name) {
            self.unused_variables.remove(pos);
        }
    }

    pub fn get_unused_variables(&self) -> &Vec<String> {
        &self.unused_variables
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
            self.begin_scope();
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

    fn begin_scope(&mut self) {
        self.scopes.push(FxHashMap::default());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;  
        }
    
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(&name.lexeme) {
                Lox::print_error(&format!(
                    "Already exists a variable called [{}] at the current scope.",
                    &name.lexeme
                ));
                return;  
            }
    
            scope.insert(name.lexeme.clone(), false);
        } else {
            Lox::print_error("Doens't exists valid scope to declare this variable.");
        }
    }
    

    fn define(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), true);
        } else {
            Lox::print_error("Doens't exists a valid scope to define this variable.");
        }
    }
    

    fn resolve_local(&self, expr: &Expr, name: &Token) {
        for i in 0..self.scopes.len() {
            if let Some(scope) = self.scopes.get(i) {
                if scope.contains_key(&name.lexeme) {
                    let distance = self.scopes.len() - 1 - i;
                    self.interpreter.borrow_mut().resolve(expr.clone(), distance);
                    return;
                }
            }
        }
    }
    
}
