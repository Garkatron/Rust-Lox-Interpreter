use super::{
    environment::Environment, error_types::runtime_error::RuntimeError, interpreter::Interpreter, lox_callable::LoxCallable, syntax::components::{expression::LiteralValue, stmt::Stmt}
};

pub struct LoxFunction {
    declaration: Stmt,
}

impl LoxFunction {
    pub fn new(declaration: Stmt) -> Self {
        Self { declaration }
    }
}

impl LoxCallable for LoxFunction {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<LiteralValue>,
    ) -> Result<LiteralValue, RuntimeError> {
        let env = Environment::new(Some(interpreter.globals.clone()));
        if let Stmt::Function {
            token: _,
            params,
            body,
        } = &self.declaration
        {
            for (i, param) in params.iter().enumerate() {
                env.borrow_mut().define(&param.lexeme, arguments[i].clone())?;
            }
            match interpreter.execute_block(&body, env) {
                Ok(_) => {
                    return Ok(LiteralValue::Nil)
                }
                Err(e) => {
                    match e {
                        RuntimeError::Return(v) => {
                            return Ok(v);
                        }
                        _=> {
                            return Err(e);
                        }
                    }
                }
            }
        }
        Ok(LiteralValue::Nil)
    }

    fn arity(&self) -> usize {
        if let Stmt::Function { params, .. } = &self.declaration {
            params.len()
        } else {
            0
        }
    }
}
