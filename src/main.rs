use std::env;
mod lox;
mod scanner;
mod token_type;
mod token;
mod object;
mod error_reporter;
mod expression;

fn main() {
    // Arguments
    let args: Vec<String> = env::args().collect();
    let mut lox = lox::Lox::new();

    // Initializing lox interpreter
    lox.init(args)
    
    // https://www.geeksforgeeks.org/error-handling-compiler-design/
}
