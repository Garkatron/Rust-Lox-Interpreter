use core::fmt;
use std::{cell::RefCell, fmt::{Display, Formatter}, rc::Rc};
use rustc_hash::FxHashMap;
use crate::core::{error_types::runtime_error::RuntimeError, interpreter::Interpreter, lox_callable::LoxCallable, syntax::components::expression::LiteralValue};
// use std::collections::HashMap;
use super::{lox_function::LoxFunction, lox_instance::LoxInstance};
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
    pub fn find_method(&self, name: &str) -> LiteralValue {
        if let Some(method) = self.methods.get(name) {
            return LiteralValue::LoxFunction(Rc::new(method.clone()));
        }
        LiteralValue::Nil

    }
}


impl LoxCallable for LoxClass {
    fn arity(&self) -> usize {
        0
    }
    
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<LiteralValue>) -> Result<LiteralValue, RuntimeError> {
        let loxinstance = Rc::new(RefCell::new(LoxInstance::new(self.clone())));
        Ok(LiteralValue::LoxInstance(Rc::clone(&loxinstance)))
    }
    
}

impl Display for LoxClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "LoxInstance with class: {}", self.name)
    }
}