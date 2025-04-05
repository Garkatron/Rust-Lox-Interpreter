use std::cell::RefCell;
use std::rc::Rc;
use std::usize;

use rustc_hash::FxHashMap;

use super::environment::Environment;
use super::error_types::runtime_error::RuntimeError;

use super::fuctions::lox_function::LoxFunction;
use super::native_functions::lox_clock::LoxClock;
use super::native_functions::lox_print::LoxPrint;
use super::oop::lox_class::LoxClass;
use super::syntax::components::expression::{Expr, LoxValue, Visitor as ExpressionVisitor};
use super::syntax::components::stmt::{Stmt, Visitor as StatementVisitor};
use super::syntax::token::Token;
use super::syntax::token_type::TokenType;

pub struct Interpreter {
    pub globals: Box<Environment>,
    pub environment: Rc<RefCell<Environment>>,
    pub locals: FxHashMap<Expr, usize>,
}

impl ExpressionVisitor<LoxValue> for Interpreter {
    fn visit_unary(&mut self, operator: &Token, right: &Expr,) -> Result<LoxValue, RuntimeError> {
        let lit = self.evaluate(right)?;
        match operator.t_type {
            TokenType::MINUS => match lit {
                LoxValue::Number(n) => Ok(LoxValue::Number(-n)),
                _ => Err(RuntimeError::BadOperator(
                    operator.clone(),
                    "Operand must be a number.".to_string(),
                )),
            },
            TokenType::BANG => Ok(LoxValue::Boolean(!self.is_truthy(&lit))),
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
    ) -> Result<LoxValue, RuntimeError> {
        let left_lit = self.evaluate(left)?;
        let right_lit = self.evaluate(right)?;

        match (operator.t_type.clone(), &left_lit, &right_lit) {
            (TokenType::PLUS, LoxValue::Number(n1), LoxValue::Number(n2)) => {
                Ok(LoxValue::Number(n1 + n2))
            }
            (TokenType::PLUS, LoxValue::String(s1), LoxValue::String(s2)) => {
                Ok(LoxValue::String(format!("{}{}", s1, s2)))
            }
            (TokenType::PLUS, LoxValue::String(s1), LoxValue::Number(n)) => {
                Ok(LoxValue::String(format!("{}{}", s1, n)))
            }
            (TokenType::MINUS, LoxValue::Number(n1), LoxValue::Number(n2)) => {
                Ok(LoxValue::Number(n1 - n2))
            }
            (TokenType::MINUS, LoxValue::String(s1), LoxValue::String(s2)) => {
                Ok(LoxValue::String(s1.replacen(s2, "", 1)))
            }
            (TokenType::SLASH, LoxValue::Number(n1), LoxValue::Number(n2)) => {
                if *n2 == 0.0 {
                    return Err(RuntimeError::BadOperator(
                        operator.clone(),
                        "Division by zero.".to_string(),
                    ));
                }
                Ok(LoxValue::Number(n1 / n2))
            }
            (TokenType::STAR, LoxValue::Number(n1), LoxValue::Number(n2)) => {
                Ok(LoxValue::Number(n1 * n2))
            }
            (TokenType::GREATER, LoxValue::Number(n1), LoxValue::Number(n2)) => {
                Ok(LoxValue::Boolean(n1 > n2))
            }
            (TokenType::GREATER_EQUAL, LoxValue::Number(n1), LoxValue::Number(n2)) => {
                Ok(LoxValue::Boolean(n1 >= n2))
            }
            (TokenType::LESS, LoxValue::Number(n1), LoxValue::Number(n2)) => {
                Ok(LoxValue::Boolean(n1 < n2))
            }
            (TokenType::LESS_EQUAL, LoxValue::Number(n1), LoxValue::Number(n2)) => {
                Ok(LoxValue::Boolean(n1 <= n2))
            }
            (TokenType::BANG_EQUAL, _, _) => {
                Ok(LoxValue::Boolean(!self.is_equal(&left_lit, &right_lit)))
            }
            (TokenType::EQUAL_EQUAL, _, _) => {
                Ok(LoxValue::Boolean(self.is_equal(&left_lit, &right_lit)))
            }
            _ => Err(RuntimeError::BadOperator(
                operator.clone(),
                "Invalid binary operation.".to_string(),
            )),
        }
    }

    fn visit_literal(&mut self, value: &LoxValue) -> Result<LoxValue, RuntimeError> {
        Ok(value.clone())
    }

    fn visit_grouping(&mut self, expr: &Expr) -> Result<LoxValue, RuntimeError> {
        self.evaluate(expr)
    }

    fn visit_comma(&mut self, _: &Expr, right: &Expr) -> Result<LoxValue, RuntimeError> {
        self.evaluate(right)
    }

    fn visit_ternary(
        &mut self,
        condition: &Expr,
        then_branch: &Expr,
        else_branch: &Expr,
    ) -> Result<LoxValue, RuntimeError> {
        let condition_value = self.evaluate(condition)?;

        if self.is_truthy(&condition_value) {
            self.evaluate(then_branch)
        } else {
            self.evaluate(else_branch)
        }
    }

    fn visit_variable(&mut self, name: &Token, e: &Expr) -> Result<LoxValue, RuntimeError> {
        Ok(self.look_up_variable(name, e)?)
        // Ok(self.environment.borrow().get(name)?)
    }

    fn visit_assing(&mut self, name: &Token, expr: &Expr) -> Result<LoxValue, RuntimeError> {
        let value = self.evaluate(expr)?;
        let distance = self.locals.get(expr);
        match distance {
            Some(d) => {
                self.environment.borrow_mut().assing_at(*d, &name.lexeme, value.clone());
            }
            None => {
                self.globals.assign(name, value.clone())?;
            }
        }

        // self.environment.borrow_mut().assign(name, value.clone())?;
        Ok(value)
    }

    fn visit_logical(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<LoxValue, RuntimeError> {
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

    fn visit_call(&mut self, callee: &Expr, paren: &Token, arguments: &[Expr]) -> Result<LoxValue, RuntimeError> {        
        let callee_val = self.evaluate(callee)?;
        let mut args = vec![];
        for arg in arguments {
            args.push(self.evaluate(arg)?);
        }
        
        if let Some(fun) = callee_val.return_fn_if_callable() {
            if arguments.len() != fun.arity() {
                return Err(RuntimeError::ToManyArguments(
                    paren.clone(),
                    fun.arity(),
                    arguments.len(),
                ));
            } else {
                return Ok(fun.call(self, args)?);
            }
        } else {
            return Err(RuntimeError::BadCallable());
        }
    }

    fn visit_get(&mut self, name: &Token, object: &Expr) -> Result<LoxValue, RuntimeError> {
        let obj = self.evaluate(object)?;
        
        if let LoxValue::LoxInstance(i) = obj {
            return Ok(i.borrow_mut().get(name)?)
        }
        
        Err(RuntimeError::OnlyInstancesHaveProperties())
    }

    fn visit_set(&mut self, object: &Expr, name: &Token, value: &Expr) -> Result<LoxValue, RuntimeError> {
        let obj = self.evaluate(object)?;

        match obj {
            LoxValue::LoxInstance(i) => {
                let value = self.evaluate(value)?;
                i.borrow_mut().set(name.clone(), value.clone());
                Ok(value)
            }
            _ => {
                Err(RuntimeError::OnlyInstancesHaveProperties())
            }
        }
    }
    fn visit_this(&mut self, keyword: &Token) -> Result<LoxValue, RuntimeError> {
        self.look_up_variable(keyword, &Expr::Literal { id: 0, value: LoxValue::Nil })
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
        self.execute_block(statements, Rc::clone(&self.environment))
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
                    Err(err) => match err {
                        RuntimeError::Break() => {
                            break;
                        }
                        _ => {
                            return Err(err);
                        }
                    },
                }
            } else if let Some(t_else_branch) = else_branch {
                match self.execute(&t_else_branch) {
                    Ok(_) => {}
                    Err(err) => match err {
                        RuntimeError::Break() => {
                            break;
                        }
                        _ => {
                            return Err(err);
                        }
                    },
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
                Err(err) => match err {
                    RuntimeError::Break() => {
                        break;
                    }
                    _ => {
                        return Err(err);
                    }
                },
            }
        }
        Ok(())
    }

    fn visit_break(&mut self) -> Result<(), RuntimeError> {
        Err(RuntimeError::Break())
    }

    fn visit_function(&mut self, token: &Token, params: &[Token], body: &[Stmt]) -> Result<(), RuntimeError> {
        let function = LoxFunction::new(
            Stmt::Function {
                token: token.clone(),
                params: params.to_vec(),
                body: body.to_vec(),
            },
            Rc::clone(&self.environment),
            false
        );

        self.environment.borrow_mut().define(&token.lexeme, LoxValue::Callable(Rc::new(function)))?;
        Ok(())
    }

    fn visit_return(&mut self, _: &Token, v: &Expr) -> Result<(), RuntimeError> {
        let val = self.evaluate(v)?;
        Err(RuntimeError::Return(val))
    }

    fn visit_class(&mut self, name: &Token, methods: &[Stmt]) -> Result<(), RuntimeError> {
        self.environment.borrow_mut().define(&name.lexeme, LoxValue::Nil)?;

        let mut met = FxHashMap::default();
        for method in methods  {
            if let Stmt::Function { token, .. } = method {
                let function = LoxFunction::new(method.clone(), Rc::clone(&self.environment), token.lexeme == "init");
                met.insert(token.lexeme.clone(), function);
            }
        }
        
        let loxclass = LoxClass::new(name.lexeme.clone(), met);
        self.environment.borrow_mut().assign(name, LoxValue::LoxClass(loxclass))?;
        Ok(())
    }
}

impl Interpreter {
    pub fn new(mut global_env: Box<Environment>) -> Self {

        let _ = global_env.define("clock", LoxValue::Callable(Rc::new(LoxClock::new())));
        let _ = global_env.define("print", LoxValue::Callable(Rc::new(LoxPrint::new())));

        Self {
            globals: global_env.clone(),
            environment: Rc::new(RefCell::new(*global_env)),
            locals: FxHashMap::default(),
        }
    }

    pub fn execute_block(&mut self, statements: &[Stmt], environment: Rc<RefCell<Environment>>) -> Result<(), RuntimeError> {    
        let previous = std::mem::replace(&mut self.environment, environment);
    
        for statement in statements {
            self.execute(statement)?;
        }
    
        self.environment = previous;
        Ok(())
    }
    
    fn evaluate(&mut self, expr: &Expr) -> Result<LoxValue, RuntimeError> {
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

    fn is_equal(&self, left: &LoxValue, right: &LoxValue) -> bool {
        match (left, right) {
            (LoxValue::Nil, LoxValue::Nil) => true,
            (LoxValue::Nil, _) | (_, LoxValue::Nil) => false,
            (LoxValue::Number(n1), LoxValue::Number(n2)) => n1 == n2,
            (LoxValue::String(s1), LoxValue::String(s2)) => s1 == s2,
            (LoxValue::Boolean(b1), LoxValue::Boolean(b2)) => b1 == b2,
            _ => false,
        }
    }

    fn stringify(&self, lit: &LoxValue) -> String {
        match lit {
            LoxValue::Nil => "nil".to_string(),
            LoxValue::Number(n) => n.to_string(),
            LoxValue::String(s) => s.clone(),
            LoxValue::Boolean(b) => b.to_string(),
            LoxValue::Callable(_) => "Callable".to_string(),
            LoxValue::LoxInstance(l) => l.borrow().to_string(),
            LoxValue::LoxClass(c) => c.to_string(),
            LoxValue::LoxFunction(f) => f.to_string()
        }
    }

    pub fn is_truthy(&self, value: &LoxValue) -> bool {
        match value {
            LoxValue::Boolean(b) => *b,
            LoxValue::Nil => false,
            _ => true,
        }
    }

    pub fn resolve(&mut self, expr: Expr, depth: usize) {
        self.locals.insert(expr, depth);
    }

    pub fn look_up_variable(&mut self, name: &Token, expr: &Expr) -> Result<LoxValue, RuntimeError> {
        if let Some(opt) = self.locals.get(expr) {
            return Ok(self.environment.borrow().get_at(*opt, &name.lexeme)?);
        } else {
            return Ok(self.globals.get(name)?);
        }
    }
}
