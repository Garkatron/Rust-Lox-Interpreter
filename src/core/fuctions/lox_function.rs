use core::fmt;
use std::{cell::RefCell, fmt::{Display, Formatter}, rc::Rc};

use crate::{core::{environment::Environment, error_types::runtime_error::RuntimeError, interpreter::Interpreter, oop::{lox_class::LoxClass, lox_instance::LoxInstance}, syntax::components::{expression::LoxValue, stmt::Stmt}}, debug_dbg};

use super::lox_callable::LoxCallable;
#[derive(Clone, Debug)]

pub struct LoxFunction {
    declaration: Stmt,
    closure: Rc<RefCell<Environment>>,
    is_initializer: bool,
    is_public: bool,
    is_static: bool
}

impl LoxFunction {
    pub fn new(declaration: Stmt, closure: Rc<RefCell<Environment>>, is_initializer: bool, is_public: bool, is_static: bool) -> Self {
        Self { declaration, closure, is_initializer, is_public, is_static}
    }
    pub fn bind(&self, instance: Rc<RefCell<LoxInstance>>) -> Result<LoxFunction, RuntimeError> {
        let env = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(&self.closure))))); 
        env.borrow_mut().define("this", LoxValue::LoxInstance(instance))?;
        Ok(LoxFunction::new(self.declaration.clone(), env, self.is_initializer, self.is_public, self.is_static))
    }

    pub fn inject(&self, lox_class: &LoxClass) -> Result<LoxFunction, RuntimeError> {
        let env = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(&self.closure))))); 
        env.borrow_mut().define(&lox_class.name, LoxValue::LoxClass(lox_class.clone()))?;
        Ok(LoxFunction::new(self.declaration.clone(), env, self.is_initializer, self.is_public, self.is_static))
    }

    pub fn is_public(&self) -> bool {
        self.is_public
    }
    pub fn is_static(&self) -> bool {
        self.is_static
    }
    
}

impl LoxCallable for LoxFunction {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<LoxValue>,
    ) -> Result<LoxValue, RuntimeError> {
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

            if self.is_initializer {
                return Ok(self.closure.borrow().get_at(0, "this")?);
            }  
            
            
            match interpreter.execute_block(body, Rc::clone(&env)) {
                Ok(_) => {
                    return Ok(LoxValue::Nil);
                },
                Err(RuntimeError::Return(LoxValue::Nil)) => {
                    return Ok(LoxValue::Nil);
                },
                Err(RuntimeError::Return(v)) => {
                    if self.is_initializer {
                        return self.closure.borrow().get_at(0, "this");
                    }
                    return Ok(v);
                },
                Err(e) => {
                    return Err(e);
                }
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

impl Display for LoxFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Stmt::Function { token, .. } = &self.declaration {
            if self.is_public {
                if self.is_static {
                    write!(f, "Static Public Function({})", token.lexeme)
                } else {
                    write!(f, "Public Function({})", token.lexeme)
                }
            } else {
                if self.is_static {
                    write!(f, "Private Static Function({})", token.lexeme)
                } else {
                    write!(f, "Private Function({})", token.lexeme)
                }            }
        } else {
            write!(f, "Function(anonymous)")
        }
    }
}


