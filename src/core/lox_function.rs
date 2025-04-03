use std::{cell::RefCell, rc::Rc};

use super::{
    environment::Environment, 
    error_types::runtime_error::RuntimeError, 
    interpreter::Interpreter, 
    lox_callable::LoxCallable, 
    syntax::components::{expression::LiteralValue, stmt::Stmt}
};

pub struct LoxFunction {
    declaration: Stmt,
    closure: Rc<RefCell<Environment>>,
}

impl LoxFunction {
    pub fn new(declaration: Stmt, closure: Rc<RefCell<Environment>>) -> Self {
        Self { declaration, closure }
    }
}

impl LoxCallable for LoxFunction {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<LiteralValue>,
    ) -> Result<LiteralValue, RuntimeError> {
        let env = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(&self.closure)))));
        
        if let Stmt::Function { params, body, .. } = &self.declaration {
           
            if arguments.len() < params.len() {
                return Err(RuntimeError::BadArguments(format!(
                    "Expected {} arguments but got {}.",
                    params.len(),
                    arguments.len()
                )));
            }

            for (i, param) in params.iter().enumerate() {
                env.borrow_mut().define(&param.lexeme, arguments[i].clone())?;
            }

            match interpreter.execute_block(body, Rc::clone(&env)) {
                Ok(_) | Err(RuntimeError::Return(LiteralValue::Nil)) => Ok(LiteralValue::Nil),
                Err(RuntimeError::Return(v)) => Ok(v),
                Err(e) => Err(e),
            }
        } else {
            Err(RuntimeError::InvalidFunction("Invalid function declaration.".to_string()))
        }
    }

    fn arity(&self) -> usize {
        if let Stmt::Function { params, .. } = &self.declaration {
            params.len()
        } else {
            0
        }
    }
}
