use crate::Token;

#[derive(Debug)]
pub struct RuntimeError {
    operator: Token,
    message: String,
}

impl RuntimeError {
    pub fn new(operator: Token, message: String) -> Self {
        RuntimeError { operator, message }
    }
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[Error at {}] {}", self.operator, self.message)
    }
}

impl std::error::Error for RuntimeError {}
