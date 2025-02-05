use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lox = Lox::new();
    lox.init(args);
}
