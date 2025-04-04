use super::{error_types::runtime_error::RuntimeError, interpreter::Interpreter, syntax::components::expression::LoxValue};

pub trait LoxCallable {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<LoxValue>) -> Result<LoxValue, RuntimeError>;
    fn arity(&self) -> usize;
}