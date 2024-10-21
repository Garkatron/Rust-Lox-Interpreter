use std::env;
mod lox;
mod scanner;
mod token_type;
mod token;
mod object;
mod error_reporter;
mod expr;
mod ast_printer;
use crate::expr::expression::{self, Expr, LiteralValue, Unary, Grouping, Binary, Literal};
use crate::ast_printer::AstPrinter;
use crate::token::{Token};
use crate::token_type::TokenType;  
/*
fn main() {
    // Arguments
    let args: Vec<String> = env::args().collect();
    let mut lox = lox::Lox::new();
    
    // Initializing lox interpreter
    lox.init(args)
    
    // https://www.geeksforgeeks.org/error-handling-compiler-design/
}*/
fn main() {
    // Creamos un nuevo `AstPrinter`
    let printer = AstPrinter::new();

    // Creamos la expresión: (-123) * (45.67)
    let expression = Expr::Binary(Binary {
        lexeme: "test".to_string(),        
        left: Box::new(Expr::Unary(Unary {
            operator: Token {
                t_type: TokenType::MINUS,
                lexeme: "-".to_string(),
                literal: LiteralValue::Nil,
                line: 1,
            },
            right: Box::new(Expr::Literal(Literal {
                value: LiteralValue::Number(123.0),
            })),
            lexeme: "test".to_string()
        })),
        operator: Token {
            t_type: TokenType::STAR,
            lexeme: "*".to_string(),
            literal: LiteralValue::Nil,
            line: 1,
        },
        right: Box::new(Expr::Grouping(Grouping {
            expression: Box::new(Expr::Literal(Literal {
                value: LiteralValue::Number(45.67),
            })),
        })),
    });

    // Imprimimos el resultado del AST
    let result = printer.print(expression); // Asegúrate de pasar una referencia a la expresión
    println!("{}", result);  // Debería imprimir: "(* (- 123) (group 45.67))"
}