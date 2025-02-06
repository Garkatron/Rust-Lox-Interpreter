use std::fmt;

use super::token::Token;

#[derive(Debug, Clone)]
pub enum ParseError {
    UnexpectedToken(Token, usize),
    MissingToken(Token, usize),
    InvalidExpression(String, usize),
    UnterminatedString(usize),
    DivisionByZero(usize),
    UndefinedVariable(String, usize),
    UnexpectedEOF(usize),
    ExpectedRightParen(usize),
    ExpectedTernaryBranch(usize, usize),
    MissingLeftOperand(usize),
    EspectSemicolonAfterValue(usize),
    EspectSemicolonAfterExpression(usize),
    ExpectedVariableName(usize),
    ExpectedVariableDeclaration(usize),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken(token, line) => {
                write!(f, "Unexpected token '{}' at line {}", token.lexeme, line)
            }
            ParseError::MissingToken(token, line) => {
                write!(f, "Expected token '{}' but it was missing at line {}", token.lexeme, line)
            }
            ParseError::InvalidExpression(expr, line) => {
                write!(f, "Invalid expression: '{}' at line {}", expr, line)
            }
            ParseError::UnterminatedString(line) => {
                write!(f, "Unterminated string literal at line {}", line)
            }
            ParseError::DivisionByZero(line) => {
                write!(f, "Division by zero detected at line {}", line)
            }
            ParseError::UndefinedVariable(var_name, line) => {
                write!(f, "Undefined variable '{}' at line {}", var_name, line)
            }
            ParseError::UnexpectedEOF(line) => {
                write!(f, "Unexpected end of file at line {}", line)
            }
            ParseError::ExpectedRightParen(line) => {
                write!(f, "Expect ')' after expression, at line {}", line)
            }
            ParseError::ExpectedTernaryBranch(line, expr_line) => {
                write!(f, "Expect ':' after then branch of ternary expression at line {} (expression at line {})", line, expr_line)
            }
            ParseError::MissingLeftOperand(line) => {
                write!(f, "Expect a left operand after an unary expression, at line: {}", line)
            }
            ParseError::EspectSemicolonAfterValue(line) => {
                write!(f, "Expect ';' after value. At line: {}", line)
            }
            ParseError::EspectSemicolonAfterExpression(line) => {
                write!(f, "Expect ';' after expression. At line: {}", line)
            }
            ParseError::ExpectedVariableName(line) => {
                write!(f, "Expect variable name. At line: {}", line)
            }
            ParseError::ExpectedVariableDeclaration(line) => {
                write!(f, "Expect variable declaration. At line: {}", line)
            }
        }
    }
}

impl std::error::Error for ParseError {}

// Permite convertir `String` a `ParseError::InvalidExpression` automáticamente.
impl From<(String, usize)> for ParseError {
    fn from(message: (String, usize)) -> Self {
        ParseError::InvalidExpression(message.0, message.1)
    }
}
