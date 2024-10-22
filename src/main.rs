use std::env;

use ast_utils::{ast_printer::AstPrinter, ast_rpn::AstRpn};
use expression::{Binary, Expr, Grouping, Literal, LiteralValue, Unary};
use lox::Lox;
use token::Token;
use token_type::TokenType;
#[warn(unused_imports)]
pub mod lox;
pub mod scanner;
pub mod token_type;
pub mod token;
pub mod error_reporter;
pub mod expression;
pub mod ast_utils;

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
    // Creamos un nuevo `AstPrinter`
    let printer = AstRpn::new();

    // Creamos la expresión: (1 + 2) * (4 - 3)
    let expression = Expr::Binary(Binary {
        lexeme: "test".to_string(),
        left: Box::new(Expr::Grouping(Grouping {
            expression: Box::new(Expr::Binary(Binary {
                lexeme: "test".to_string(),
                left: Box::new(Expr::Literal(Literal {
                    value: LiteralValue::Number(1.0),
                })),
                operator: Token {
                    t_type: TokenType::PLUS,
                    lexeme: "+".to_string(),
                    literal: LiteralValue::Nil,
                    line: 1,
                },
                right: Box::new(Expr::Literal(Literal {
                    value: LiteralValue::Number(2.0),
                })),
            })),
        })),
        operator: Token {
            t_type: TokenType::STAR,
            lexeme: "*".to_string(),
            literal: LiteralValue::Nil,
            line: 1,
        },
        right: Box::new(Expr::Grouping(Grouping {
            expression: Box::new(Expr::Binary(Binary {
                lexeme: "test".to_string(),
                left: Box::new(Expr::Literal(Literal {
                    value: LiteralValue::Number(4.0),
                })),
                operator: Token {
                    t_type: TokenType::MINUS,
                    lexeme: "-".to_string(),
                    literal: LiteralValue::Nil,
                    line: 1,
                },
                right: Box::new(Expr::Literal(Literal {
                    value: LiteralValue::Number(3.0),
                })),
            })),
        })),
    });

    // Imprimimos el resultado del AST
    let result = printer.print(expression); // Asegúrate de pasar una referencia a la expresión
    println!("{}", result);  // Debería imprimir: "(* (- 123) (group 45.67))"
}