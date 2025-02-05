use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::token::Token;
use std::io::Write;
use std::path::PathBuf;
use std::{fs, io, process};

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
pub struct Lox {

}

//pub static mut had_error: bool = false;
impl Lox {
    pub fn new() -> Self {
        Self {  }
    }

    // Init method
    pub fn init(&mut self, args: Vec<String>) {
        match args.len() {
            0 => {
                eprintln!("[ERROR]: No arguments provided");
                process::exit(64);
            }
            1 => {
                println!("[LOX]: Running prompt");
                self.run_prompt();
            }

            2 => {
                println!("[LOX]: :/");
            }

            3 => match args.get(1).map(String::as_str) {
                Some("-v") => {
                    println!("[LOX]: Running file");
                    self.run_file(&args[2]);
                }
                _ => {
                    eprintln!("[ERROR]: Invalid argument. Expected '-v' for file execution.");
                    process::exit(64);
                }
            },
            _ => {
                eprintln!("[ERROR]: Too many arguments.");
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
            println!("[ERROR]: Can't read your file.")
        }
    }

    fn run_prompt(&mut self) {
        // Clear stdout
        println!("Clear terminal");

        // let _ = io::stdout().flush();
        // Flush out
        std::io::stdout().flush().unwrap();

        // Prompt loop
        loop {
            print!(" -> ");

            let _input: String = String::new();

            read_line!(_input);

            if _input.trim().is_empty() {
                break;
            }

            println!("Ejecutando: {}", _input.trim());

            // Exec line
            self.run(_input);

            // Reset error
            // had_error = false;
            
        }
    }
    fn run(&mut self, source: String) {
        let mut scanner: Scanner = Scanner::new(source.clone());
        let tokens: Vec<Token> = scanner.scan_tokens();

        println!("\n---------- TOKENS ESCANEADOS ----------");
        for token in &tokens {
            println!("{}", token);
        }
        let mut parser = Parser::new(tokens.clone());

        match parser.parse() {
            Ok(statements) => {
                println!("========== RESULTADO ==========");

                match Interpreter.interpret(statements) {
                    Ok(_) => {
                        println!("End");
                    }
                    Err(e) => {
                        println!("\n❌ [ERROR]: on interpretation: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("\n❌ [ERROR]: on parsing {:?}", e);
            }
        }
    }
}
