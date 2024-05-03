use std::{fs, io, process};
use std::io::{stdin, Write};
use std::path::{Path, PathBuf};
use crate::scanner::Scanner;
use crate::token::Token;

// Macros
macro_rules! read_line {
    ($input:ident) => {
        let mut $input = String::new();
        io::stdin().read_line(&mut $input).expect("Error on read line");
    };
}

// Lox
pub struct Lox {}

pub static mut had_error: bool = false;
impl Lox {

    fn new() -> Self {
        Self {

        }
    }

    // Init method
    pub unsafe fn init(&mut self, args: Vec<String>) {
        if args.len() > 1 {
            process::exit(64);
        } else if args.len() == 1 {
            Self.runFile(&args[0])
        } else {
            self.run_prompt()
        }
    }
    fn run_file(path: &String){
        let file = PathBuf::from(path);
        let content = fs::read_to_string(file);
        if let Ok(ok_content) = content {
            Self::run(ok_content)
        } else {
            // ! Error on read file
        }
    }

    unsafe fn run_prompt(&mut self){
        // Clear stdout
        let _ = io::stdout().flush();

        // Prompt loop
        loop {
            print!(" -> ");

            let mut input: String = String::new();
            read_line!(input);

            if input == None {
                break;
            }
            // Exec line
            self.run(input);

            // Reset error
            had_error = false;
        }
    }

    fn run(source: String){
        // Scanning tokens
        let mut scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens();

        // Printing tokens
        for token in tokens {
            println!("{}", token);
        }

    }

    pub unsafe fn error(line: usize, message: String) {
        Lox::report(line," ".to_string(), message)
    }

    unsafe fn report(line: i32, where_is: String, message: String){
        println!("Error {} at line {}\nMessage: {} ", where_is, line, message);
        had_error = true;
    }



}