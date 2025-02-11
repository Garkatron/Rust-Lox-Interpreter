use std::cell::RefCell;


use std::rc::Rc;

use super::environment::Environment;
use super::error_types::runtime_error::RuntimeError;
use super::lox_function::LoxFunction;
use super::native_functions::lox_clock::LoxClock;
use super::native_functions::lox_print::{LoxPrint, LoxPrintLn};
use super::syntax::components::expression::{Expr, LiteralValue, Visitor as ExpressionVisitor};
use super::syntax::components::stmt::{Stmt, Visitor as StatementVisitor};
use super::syntax::token::Token;
use super::syntax::token_type::TokenType;
pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
}

impl ExpressionVisitor<LiteralValue> for Interpreter {
    fn visit_unary(
        &mut self,
        operator: &Token,
        right: &Expr,
    ) -> Result<LiteralValue, RuntimeError> {
        let lit = self.evaluate(right)?;
        match operator.t_type {
            TokenType::MINUS => match lit {
                LiteralValue::Number(n) => Ok(LiteralValue::Number(-n)),
                _ => Err(RuntimeError::BadOperator(
                    operator.clone(),
                    "Operand must be a number.".to_string(),
                )),
            },
            TokenType::BANG => Ok(LiteralValue::Boolean(!self.is_truthy(&lit))),
            _ => Err(RuntimeError::BadOperator(
                operator.clone(),
                "Invalid unary operator.".to_string(),
            )),
        }
    }

    fn visit_binary(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<LiteralValue, RuntimeError> {
        let left_lit = self.evaluate(left)?;
        let right_lit = self.evaluate(right)?;

        match (operator.t_type.clone(), &left_lit, &right_lit) {
            (TokenType::PLUS, LiteralValue::Number(n1), LiteralValue::Number(n2)) => {
                Ok(LiteralValue::Number(n1 + n2))
            }
            (TokenType::PLUS, LiteralValue::String(s1), LiteralValue::String(s2)) => {
                Ok(LiteralValue::String(format!("{}{}", s1, s2)))
            }
            (TokenType::PLUS, LiteralValue::String(s1), LiteralValue::Number(n)) => {
                Ok(LiteralValue::String(format!("{}{}", s1, n)))
            }
            (TokenType::MINUS, LiteralValue::Number(n1), LiteralValue::Number(n2)) => {
                Ok(LiteralValue::Number(n1 - n2))
            }
            (TokenType::MINUS, LiteralValue::String(s1), LiteralValue::String(s2)) => {
                Ok(LiteralValue::String(s1.replacen(s2, "", 1)))
            }
            (TokenType::SLASH, LiteralValue::Number(n1), LiteralValue::Number(n2)) => {
                if *n2 == 0.0 {
                    return Err(RuntimeError::BadOperator(
                        operator.clone(),
                        "Division by zero.".to_string(),
                    ));
                }
                Ok(LiteralValue::Number(n1 / n2))
            }
            (TokenType::STAR, LiteralValue::Number(n1), LiteralValue::Number(n2)) => {
                Ok(LiteralValue::Number(n1 * n2))
            }
            (TokenType::GREATER, LiteralValue::Number(n1), LiteralValue::Number(n2)) => {
                Ok(LiteralValue::Boolean(n1 > n2))
            }
            (TokenType::GREATER_EQUAL, LiteralValue::Number(n1), LiteralValue::Number(n2)) => {
                Ok(LiteralValue::Boolean(n1 >= n2))
            }
            (TokenType::LESS, LiteralValue::Number(n1), LiteralValue::Number(n2)) => {
                Ok(LiteralValue::Boolean(n1 < n2))
            }
            (TokenType::LESS_EQUAL, LiteralValue::Number(n1), LiteralValue::Number(n2)) => {
                Ok(LiteralValue::Boolean(n1 <= n2))
            }
            (TokenType::BANG_EQUAL, _, _) => {
                Ok(LiteralValue::Boolean(!self.is_equal(&left_lit, &right_lit)))
            }
            (TokenType::EQUAL_EQUAL, _, _) => {
                Ok(LiteralValue::Boolean(self.is_equal(&left_lit, &right_lit)))
            }
            _ => Err(RuntimeError::BadOperator(
                operator.clone(),
                "Invalid binary operation.".to_string(),
            )),
        }
    }

    fn visit_literal(&mut self, value: &LiteralValue) -> Result<LiteralValue, RuntimeError> {
        Ok(value.clone())
    }

    fn visit_grouping(&mut self, expr: &Expr) -> Result<LiteralValue, RuntimeError> {
        self.evaluate(expr)
    }

    fn visit_comma(&mut self, _: &Expr, right: &Expr) -> Result<LiteralValue, RuntimeError> {
        self.evaluate(right)
    }

    fn visit_ternary(
        &mut self,
        condition: &Expr,
        then_branch: &Expr,
        else_branch: &Expr,
    ) -> Result<LiteralValue, RuntimeError> {
        let condition_value = self.evaluate(condition)?;

        if self.is_truthy(&condition_value) {
            self.evaluate(then_branch)
        } else {
            self.evaluate(else_branch)
        }
    }

    fn visit_variable(&mut self, name: &Token) -> Result<LiteralValue, RuntimeError> {
        Ok(self.environment.borrow().get(name)?)
    }

    fn visit_assing(&mut self, name: &Token, expr: &Expr) -> Result<LiteralValue, RuntimeError> {
        let value = self.evaluate(expr)?;
        self.environment.borrow_mut().assign(name, value.clone())?;
        Ok(value)
    }

    fn visit_logical(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<LiteralValue, RuntimeError> {
        let left_value = self.evaluate(left)?;
        if operator.t_type == TokenType::OR {
            if self.is_truthy(&left_value) {
                return Ok(left_value);
            }
        } else {
            if !self.is_truthy(&left_value) {
                return Ok(left_value);
            }
        }
        Ok(self.evaluate(right)?)
    }

    fn visit_call(&mut self, callee: &Expr, paren: &Token, arguments: &[Expr]) -> Result<LiteralValue, RuntimeError> {
        let callee_val = self.evaluate(callee)?;
        let mut args = vec![];
        for arg in arguments {
            args.push(self.evaluate(arg)?);
        }

        if let Some(fun) = callee_val.return_fn_if_callable() {
            if arguments.len() != fun.arity() {
                return Err(RuntimeError::ToMantyArguments(paren.clone(), fun.arity(), arguments.len()))
            } else {
                return Ok(fun.call(self, args)?)
            }
        } else {
            return Err(RuntimeError::BadCallable())
        }        
    }
}
impl StatementVisitor<()> for Interpreter {
    fn visit_expression(&mut self, expression: &Expr) -> Result<(), RuntimeError> {
        let _ = self.evaluate(expression)?;
        Ok(())
    }

    fn visit_print(&mut self, expression: &Expr) -> Result<(), RuntimeError> {
        let value = self.evaluate(expression)?;
        println!("{}", self.stringify(&value));
        Ok(())
    }

    fn visit_var(&mut self, name: &Token, initializer: &Expr) -> Result<(), RuntimeError> {
        let value = self.evaluate(initializer)?;
        self.environment.borrow_mut().define(&name.lexeme, value)?;
        Ok(())
    }

    fn visit_block(&mut self, statements: &[Stmt]) -> Result<(), RuntimeError> {
        self.execute_block(
            statements,
            Environment::new(Some(Rc::clone(&self.environment))),
        )
    }

    fn visit_if(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: Option<&Stmt>,
    ) -> Result<(), RuntimeError> {
        let value = self.evaluate(condition)?;
        if self.is_truthy(&value) {
            self.execute(then_branch)
        } else if let Some(t_else_branch) = else_branch {
            self.execute(t_else_branch)
        } else {
            Ok(())
        }
    }

    fn visit_while(
        &mut self,
        condition: &Expr,
        body: &Stmt,
        else_branch: Option<&Stmt>,
    ) -> Result<(), RuntimeError> {
        loop {
            let value = self.evaluate(condition)?;
            let truthy = self.is_truthy(&value);
            if truthy {
                match self.execute(&body) {
                    Ok(_) => {}
                    Err(err) => {
                        match err {
                            RuntimeError::Break() => {
                                break;
                            }
                            _ => {
                                return Err(err);
                            }
                        }
                    }
                }
            } else if let Some(t_else_branch) = else_branch {
                match self.execute(&t_else_branch) {
                    Ok(_) => {}
                    Err(err) => {
                        match err {
                            RuntimeError::Break() => {
                                break;
                            }
                            _ => {
                                return Err(err);
                            }
                        }
                    }
                }
            } else {
                break;
            }
        } 

        
    
        Ok(())
    }
    
    fn visit_loop(&mut self, body: &Stmt) -> Result<(), RuntimeError> {
        loop {
            match self.execute(body) {
                Ok(_) => {}
                Err(err) => {
                    match err {
                        RuntimeError::Break() => {
                            break;
                        }
                        _ => {
                            return Err(err);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn visit_break(&mut self) -> Result<(), RuntimeError> {
        Err(RuntimeError::Break())
    }

    fn visit_function(&mut self, token: &Token, params: &[Token], body: &[Stmt]) -> Result<(), RuntimeError> {
        let function = LoxFunction::new(Stmt::Function {
            token: token.clone(),  
            params: params.to_vec(), 
            body: body.to_vec()
        }, Rc::clone(&self.environment));
    
        self.environment.borrow_mut().define(
            &token.lexeme,
            LiteralValue::Callable(Rc::new(function)),
        )?;        Ok(())
    }
    
    fn visit_return(&mut self, _: &Token, v: &Expr) -> Result<(), RuntimeError> {
        let val = self.evaluate(v)?;
        Err(RuntimeError::Return(val))
    }

}

impl Interpreter {
    pub fn new() -> Self {
        let g = Environment::new(None);

        let _ = g.borrow_mut().define("clock", LiteralValue::Callable(Rc::new(LoxClock::new())));
        let _ = g.borrow_mut().define("print", LiteralValue::Callable(Rc::new(LoxPrint::new())));
        let _ = g.borrow_mut().define("println", LiteralValue::Callable(Rc::new(LoxPrintLn::new())));

        let x = Self {
            globals: g.clone(),
            environment: g.clone(),
        }; x
    }

    pub fn execute_block(
        &mut self,
        statements: &[Stmt],
        env: Rc<RefCell<Environment>>,
    ) -> Result<(), RuntimeError> {
        let prev_env = std::mem::replace(&mut self.environment, env);

        let result = statements.iter().try_for_each(|stmt| self.execute(stmt));

        self.environment = prev_env;
        result
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<LiteralValue, RuntimeError> {
        expr.accept(self)
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), RuntimeError> {
        for statement in statements {
            self.execute(&statement)?;
        }
        Ok(())
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        stmt.accept(self)?;
        Ok(())
    }

    fn is_equal(&self, left: &LiteralValue, right: &LiteralValue) -> bool {
        match (left, right) {
            (LiteralValue::Nil, LiteralValue::Nil) => true,
            (LiteralValue::Nil, _) | (_, LiteralValue::Nil) => false,
            (LiteralValue::Number(n1), LiteralValue::Number(n2)) => n1 == n2,
            (LiteralValue::String(s1), LiteralValue::String(s2)) => s1 == s2,
            (LiteralValue::Boolean(b1), LiteralValue::Boolean(b2)) => b1 == b2,
            _ => false,
        }
    }

    fn stringify(&self, lit: &LiteralValue) -> String {
        match lit {
            LiteralValue::Nil => "nil".to_string(),
            LiteralValue::Number(n) => n.to_string(),
            LiteralValue::String(s) => s.clone(),
            LiteralValue::Boolean(b) => b.to_string(),
            LiteralValue::Callable(_) => "Function".to_string()
        }
    }

    pub fn is_truthy(&self, value: &LiteralValue) -> bool {
        match value {
            LiteralValue::Boolean(b) => *b,
            LiteralValue::Nil => false,
            _ => true,
        }
    }
}
