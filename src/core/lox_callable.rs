use super::{expression::LiteralValue, interpreter::Interpreter, runtime_error::RuntimeError};

pub trait LoxCallable {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<LiteralValue>) -> Result<LiteralValue, RuntimeError>;
    fn arity(&self) -> usize;
}