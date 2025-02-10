use crate::core::{
    expression::LiteralValue, interpreter::Interpreter, lox_callable::LoxCallable,
    runtime_error::RuntimeError,
};

pub struct LoxPrint;

impl LoxPrint {
    pub fn new() -> Self {
        Self
    }
}

impl LoxCallable for LoxPrint {
    fn arity(&self) -> usize {
        1 // O más si se permite más de un argumento
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: Vec<LiteralValue>,
    ) -> Result<LiteralValue, RuntimeError> {
        if arguments.is_empty() {
            return Err(RuntimeError::NativeFunctionError(
                "LoxPrint requires 1 argument".to_string(),
            ));
        }

        arguments.iter().for_each(|f| print!("{}", f));

        Ok(LiteralValue::Nil)
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
        arguments: Vec<LiteralValue>,
    ) -> Result<LiteralValue, RuntimeError> {
        arguments.iter().for_each(|f| {
            if let LiteralValue::String(s) = f {
                println!("{}", s);
            } else {
                println!("{}", f);
            }
        });
        

        Ok(LiteralValue::Nil)
    }
}
