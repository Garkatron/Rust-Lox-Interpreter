use super::{expression::LiteralValue, interpreter::Interpreter};

pub trait LoxCallable {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<LiteralValue>) -> LiteralValue;
    fn arity(&self) -> usize;
}