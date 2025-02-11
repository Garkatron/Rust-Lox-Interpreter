use super::{error_types::runtime_error::RuntimeError, interpreter::Interpreter, syntax::components::expression::LiteralValue};

pub trait LoxCallable {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<LiteralValue>) -> Result<LiteralValue, RuntimeError>;
    fn arity(&self) -> usize;
}