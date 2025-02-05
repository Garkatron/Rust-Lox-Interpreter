use std::env;

use rust_lox_interpreter::core::lox::Lox;


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lox = Lox::new();
    lox.init(args);
}
