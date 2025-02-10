use crate::core::{expression::LiteralValue, interpreter::Interpreter, lox_callable::LoxCallable, runtime_error::RuntimeError};
use std::time::SystemTime;

pub struct LoxClock {

}

impl LoxClock {
    pub fn new() -> Self {
        Self {}
    }
}

impl LoxCallable for LoxClock {
    fn arity(&self) -> usize {
        0
    }
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<LiteralValue>) -> Result<LiteralValue, RuntimeError> {
        let now = SystemTime::now();
        let duration = now.duration_since(SystemTime::UNIX_EPOCH).expect("Time went backwards");
        Ok(LiteralValue::Number(duration.as_secs() as f64 + duration.subsec_nanos() as f64 / 1_000_000_000.0))
    }
}