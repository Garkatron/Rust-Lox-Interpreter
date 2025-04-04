use core::fmt;
use std::{cell::RefCell, fmt::{Display, Formatter}, rc::Rc};

use crate::core::{error_types::runtime_error::RuntimeError, interpreter::Interpreter, lox_callable::LoxCallable, syntax::components::expression::LiteralValue};

use super::lox_instance::LoxInstance;

#[derive(Clone)]
pub struct LoxClass {
    pub name: String
}

impl LoxClass {
    pub fn new(name: String) -> Self {

        Self {
            name
        }
    }
}


impl LoxCallable for Rc<RefCell<LoxClass>> {
    fn arity(&self) -> usize {
        0
    }
    
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<LiteralValue>) -> Result<LiteralValue, RuntimeError> {
        let loxinstance = Rc::new(RefCell::new(LoxInstance::new(Rc::clone(self))));
        Ok(LiteralValue::LoxInstance(Rc::clone(&loxinstance)))
    }
    
}

impl Display for LoxClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "LoxInstance with class: {}", self.name)
    }
}