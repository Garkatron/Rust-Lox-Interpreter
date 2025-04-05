use core::fmt;
use std::{cell::RefCell, fmt::{Display, Formatter}, rc::Rc};
use rustc_hash::FxHashMap;
use crate::core::{error_types::runtime_error::RuntimeError, fuctions::{lox_callable::LoxCallable, lox_function::LoxFunction}, interpreter::Interpreter, syntax::components::expression::LoxValue};

use super::lox_instance::LoxInstance;
// use std::collections::HashMap;
#[derive(Clone)]
pub struct LoxClass {
    pub name: String,
    methods: FxHashMap<String, LoxFunction>}

impl LoxClass {
    pub fn new(name: String, methods:FxHashMap<String, LoxFunction> ) -> Self {
        Self {
            name,
            methods
        }
    }
    pub fn find_method(&self, name: &str) -> LoxValue {
        if let Some(method) = self.methods.get(name) {
            return LoxValue::LoxFunction(Rc::new(method.clone()));
        }
        LoxValue::Nil

    }
}


impl LoxCallable for LoxClass {
    fn arity(&self) -> usize {
        0
    }
    
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<LoxValue>) -> Result<LoxValue, RuntimeError> {
        let loxinstance = Rc::new(RefCell::new(LoxInstance::new(self.clone())));
        Ok(LoxValue::LoxInstance(Rc::clone(&loxinstance)))
    }
    
}

impl Display for LoxClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "LoxClass({})", self.name)
    }
}