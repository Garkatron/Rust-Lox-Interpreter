use crate::ast_utils::ast_printer::AstPrinter;
use crate::error_reporter::ErrorReporter;
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
    error_reporter: ErrorReporter,
}

//pub static mut had_error: bool = false;
impl Lox {
    pub fn new() -> Self {
        Self {
            error_reporter: ErrorReporter::new()
        }
    }

    // Init method
    pub fn init(&mut self, args: Vec<String>) {
        println!("Args size: {}", &args.len());
        println!("Args: {}", &args.join(" "));
        println!("Args: {}", args.len() == 2 && &args[1] == "-v");

        if args.len() < 1 {
            println!("Error:");
            process::exit(64);
        } else if args.len() == 2 && &args[1] == "-v" {
            println!("Running prompt");
            self.run_prompt();
        } else if args.len() == 2 {
            println!("Running file");
            self.run_file(&args[1]);
        }
    }
    fn run_file(&mut self, path: &String) {
        let file = PathBuf::from(path);
        let content = fs::read_to_string(file);
        if let Ok(ok_content) = content {
            self.run(ok_content)
        } else {
            // ! Error on read file
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
            self.error_reporter.reset()
        }
    }

    fn run(&mut self, source: String) {
        let mut scanner: Scanner = Scanner::new(source.clone(), &mut self.error_reporter);
        let tokens: Vec<Token> = scanner.scan_tokens();
    
        let mut parser = Parser::new(tokens.clone(), &mut self.error_reporter);
        
        println!("{}", source);

        println!("---------------TOKEN--------------");

        for token in tokens.clone() {
            println!("{}",token);
        }

        println!("----------------------------------");

        match parser.parse() {
            Ok(expr) => {
                if self.error_reporter.had_error {
                    return;
                }
                println!("{}", expr.clone());
                
                match Interpreter.interpret(&expr) {
                    Ok(r) => {
                        println!("RESULT: {}", r)
                    }
                    Err(e)=>{println!("ERROR: {}", e)}
                }
            }
            Err(e) => {
                eprintln!("Error parsing expr: {:?}", e);
            }
        }
    }
    
}
