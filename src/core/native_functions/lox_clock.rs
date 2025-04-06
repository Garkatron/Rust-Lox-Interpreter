use crate::core::{
    error_types::runtime_error::RuntimeError, fuctions::lox_callable::LoxCallable, interpreter::Interpreter, syntax::components::expression::LoxValue
};
use std::time::SystemTime;

pub struct LoxClock {}

impl LoxClock {
    pub fn new() -> Self {
        Self {}
    }
}

impl LoxCallable for LoxClock {
    fn arity(&self) -> usize {
        0
    }
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<LoxValue>,
    ) -> Result<LoxValue, RuntimeError> {
        let now = SystemTime::now();
        let duration = now
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards");
        Ok(LoxValue::Number(
            duration.as_secs() as f64 + duration.subsec_nanos() as f64 / 1_000_000_000.0,
        ))
    }
}
