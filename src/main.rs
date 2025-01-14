use std::env;

use crate::expression::LiteralValue;
use ast_utils::{ast_printer::AstPrinter, ast_rpn::AstRpn};
use expression::Expr;
use lox::Lox;
use token::Token;
use token_type::TokenType;

pub mod ast_utils;
pub mod error_reporter;
pub mod expression;
#[warn(unused_imports)]
pub mod lox;
pub mod parser;
pub mod scanner;
pub mod token;
pub mod token_type;

/*fn main() {
    // Arguments
    let args: Vec<String> = env::args().collect();
    let mut lox = Lox::new();

    // Initializing lox interpreter
    lox.init(args);

    // https://www.geeksforgeeks.org/error-handling-compiler-design/
}
    */

fn main() {
    // Creamos un nuevo `AstRpn` (asumiendo que existe una implementación)
    let printer = AstRpn::new();

    // Creamos la expresión: (1 + 2) * (4 - 3)
    let expression = Expr::Binary {
        lexeme: "*".to_string(),
        left: Box::new(Expr::Grouping {
            expression: Box::new(Expr::Binary {
                lexeme: "+".to_string(),
                left: Box::new(Expr::Literal {
                    value: LiteralValue::Number(1.0),
                }),
                operator: Token {
                    t_type: TokenType::PLUS,
                    lexeme: "+".to_string(),
                    literal: LiteralValue::Nil,
                    line: 1,
                },
                right: Box::new(Expr::Literal {
                    value: LiteralValue::Number(2.0),
                }),
            }),
        }),
        operator: Token {
            t_type: TokenType::STAR,
            lexeme: "*".to_string(),
            literal: LiteralValue::Nil,
            line: 1,
        },
        right: Box::new(Expr::Grouping {
            expression: Box::new(Expr::Binary {
                lexeme: "-".to_string(),
                left: Box::new(Expr::Literal {
                    value: LiteralValue::Number(4.0),
                }),
                operator: Token {
                    t_type: TokenType::MINUS,
                    lexeme: "-".to_string(),
                    literal: LiteralValue::Nil,
                    line: 1,
                },
                right: Box::new(Expr::Literal {
                    value: LiteralValue::Number(3.0),
                }),
            }),
        }),
    };

    // Asumiendo que `AstRpn` tiene un método para procesar y mostrar la expresión
    let result = printer.print(&expression);
    println!("{}", result);
}
