use std::{fs, io, process};
use std::io::{stdin, Write};
use std::path::{Path, PathBuf};
use crate::error_reporter::ErrorReporter;
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
pub struct Lox {
    error_reporter: ErrorReporter
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
        println!("{}", &args.len());
        println!("Args {}", &args.join(" "));
        println!("Args {}", args.len() == 2 && &args[1] == "-v");

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
    fn run_file(&mut self, path: &String){
        let file = PathBuf::from(path);
        let content = fs::read_to_string(file);
        if let Ok(ok_content) = content {
            self.run(ok_content)
        } else {
            // ! Error on read file
        }
    }

    fn run_prompt(&mut self){
        // Clear stdout
        println!("Clear terminal");
        // let _ = io::stdout().flush();

        // Prompt loop
        loop {

            print!(" -> ");

            let mut input: String = String::new();

            read_line!(input);

            if input.is_empty() {
                //break;
            }
            // Exec line
            self.run(input);

            // Reset error
            //had_error = false;
            self.error_reporter.reset()

        }
    }

    fn run(&mut self, source: String){

        // Scanning tokens

        //todo Fix it

        let mut scanner: Scanner = Scanner::new(source, &mut self.error_reporter);
        let tokens: Vec<Token> = scanner.scan_tokens();

        //  Here doesn't can continue, before it has an error, the program can't continue.

        // Printing tokens
        for token in tokens {
            println!("{}", token);
        }

    }




}