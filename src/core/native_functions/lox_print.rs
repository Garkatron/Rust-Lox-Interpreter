use crate::{core::{
    error_types::runtime_error::RuntimeError, fuctions::lox_callable::LoxCallable, interpreter::Interpreter, syntax::components::expression::LoxValue
}, debug_dbg};

pub struct LoxPrint;

impl LoxPrint {
    pub fn new() -> Self {
        Self
    }
}

impl LoxCallable for LoxPrint {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: Vec<LoxValue>,
    ) -> Result<LoxValue, RuntimeError> {
        if arguments.is_empty() {
            return Err(RuntimeError::NativeFunctionError(
                "LoxPrint requires 1 argument".to_string(),
            ));
        }

        arguments.iter().for_each(|f| print!("{}", f));

        Ok(LoxValue::Nil)
    }
}

pub struct LoxPrintLn;

impl LoxPrintLn {
    pub fn new() -> Self {
        Self
    }
}

impl LoxCallable for LoxPrintLn {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: Vec<LoxValue>,
    ) -> Result<LoxValue, RuntimeError> {
        arguments.iter().for_each(|f| {
            println!("{}", f);
        });
        

        Ok(LoxValue::Nil)
    }
}

pub struct LoxDbg;

impl LoxDbg {
    pub fn new() -> Self {
        Self
    }
}

impl LoxCallable for LoxDbg {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: Vec<LoxValue>,
    ) -> Result<LoxValue, RuntimeError> {
        arguments.iter().for_each(|f| {
            debug_dbg!(f);
        });
        

        Ok(LoxValue::Nil)
    }
}
