use std::{cell::RefCell, rc::Rc};

use super::{
    environment::Environment, error_types::runtime_error::RuntimeError, interpreter::Interpreter, lox_callable::LoxCallable, syntax::components::{expression::LiteralValue, stmt::Stmt}
};

pub struct LoxFunction {
    declaration: Stmt,
    closure: Rc<Environment>,
}

impl LoxFunction {
    pub fn new(declaration: Stmt, closure: Rc<Environment>) -> Self {
        Self { declaration, closure }
    }
}

impl LoxCallable for LoxFunction {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<LiteralValue>,
    ) -> Result<LiteralValue, RuntimeError> {
        let mut env = Environment::new(Some(self.closure));
        
        if let Stmt::Function {
            params,
            body,
            ..
        } = &self.declaration
        {
            for (i, param) in params.iter().enumerate() {
                env.define(&param.lexeme, arguments[i].clone())?;
            }

            match interpreter.execute_block(&body, Box::new(env)) {
                Ok(_) => Ok(LiteralValue::Nil),
                Err(RuntimeError::Return(v)) => Ok(v),
                Err(e) => Err(e),
            }
        } else {
            Ok(LiteralValue::Nil)
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

