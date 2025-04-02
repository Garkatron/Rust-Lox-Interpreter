use std::cell::RefCell;
use std::io::Write;
use std::path::PathBuf;
use std::rc::Rc;
use std::{fs, io, process};

use crate::core::interpreter::Interpreter;
use crate::utils::colors::Color;

use super::environment::Environment;
use super::syntax::analysing::parser::Parser;
use super::syntax::analysing::scanner::Scanner;
use super::syntax::resolver::Resolver;
use super::syntax::token::Token;

// Macros
macro_rules! read_line {
    ($input:ident) => {
        let mut $input = String::new();
        io::stdin()
            .read_line(&mut $input)
            .expect("Error on read line");
    };
}

// Lox
pub struct Lox {}

impl Lox {
    pub fn new() -> Self {
        Self {}
    }

    pub fn print_error(msg: &str) {
        Color::ecprintln(&format!("[ERROR]: {}", msg), Color::Red);
    }

    pub fn print_message(msg: &str) {
        Color::cprintln(&format!("[LOX]: {}", msg), Color::Green);
    }

    pub fn init(&mut self, args: Vec<String>) {
        match args.len() {
            0 => {
                Self::print_error("No arguments provided.");
                process::exit(64);
            }
            1 => {
                Self::print_message("Running prompt");
                self.run_prompt();
            }
            2 => {
                Self::print_message("Running file");
            }
            3 => match args.get(1).map(String::as_str) {
                Some("-v") => {
                    Self::print_message("Running file");
                    self.run_file(&args[2]);
                }
                _ => {
                    Self::print_error("Invalid argument. Expected '-v' for file execution.");
                    process::exit(64);
                }
            },
            _ => {
                Self::print_error("Too many arguments.");
                process::exit(64);
            }
        }
    }

    fn run_file(&mut self, path: &String) {
        let file = PathBuf::from(path);
        let content = fs::read_to_string(file);
        if let Ok(ok_content) = content {
            self.run(ok_content)
        } else {
            Self::print_error("Can't read your file.");
        }
    }

    fn run_prompt(&mut self) {
        // Clear stdout
        println!("Clear terminal");

        // let _ = io::stdout().flush();
        std::io::stdout().flush().unwrap();

        // Prompt loop
        loop {
            print!(" -> ");

            let _input: String = String::new();

            read_line!(_input);

            if _input.trim().is_empty() {
                break;
            }

            Self::print_message(&format!("Ejecutando: {}", _input.trim()));

            // Ejecutar línea
            self.run(_input);
        }
    }

    fn run(&mut self, source: String) {
        let mut scanner: Scanner = Scanner::new(source.clone());
        let tokens: Vec<Token> = scanner.scan_tokens();

        
        let mut parser = Parser::new(tokens.clone());
        let interpreter = Rc::new(RefCell::new(Interpreter::new(Box::new(Environment::new(None)))));
        
        let mut resolver = Resolver::new(Rc::clone(&interpreter));


        match parser.parse() {
            Ok(statements) => {
                Color::cprintln("========== RESULTADO ==========", Color::Yellow);      

                if let Err(e) = resolver.resolve_statements(&statements) {
                    Self::print_error(&format!("On resolving: {}", e));
                    return;
                }

                match interpreter.borrow_mut().interpret(statements) {
                    Ok(_) => {
                        Self::print_message("End");
                    }
                    Err(e) => {
                        Self::print_error(&format!("on interpretation: {}", e));
                    }
                }
            }
            Err(e) => {
                Self::print_error(&format!("on parsing {:?}", e));
            }
        }
    

    }
}
