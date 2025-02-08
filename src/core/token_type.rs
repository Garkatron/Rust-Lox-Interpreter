use std::fmt;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR, QUESTION_MARK, COLON,

    // One or two character tokens.
    BANG, BANG_EQUAL,
    EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL,
    LESS, LESS_EQUAL,

    // Literals.
    IDENTIFIER, STRING, NUMBER,

    // Keywords.
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE, LOOP,

    EOF
}
impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}


impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let token_str = match self {
            TokenType::LEFT_PAREN => "LEFT_PAREN",
            TokenType::RIGHT_PAREN => "RIGHT_PAREN",
            TokenType::LEFT_BRACE => "LEFT_BRACE",
            TokenType::RIGHT_BRACE => "RIGHT_BRACE",
            TokenType::COMMA => "COMMA",
            TokenType::DOT => "DOT",
            TokenType::MINUS => "MINUS",
            TokenType::PLUS => "PLUS",
            TokenType::SEMICOLON => "SEMICOLON",
            TokenType::SLASH => "SLASH",
            TokenType::STAR => "STAR",
            TokenType::QUESTION_MARK => "QUESTION_MARK",
            TokenType::COLON => "COLON",
            TokenType::BANG => "BANG",
            TokenType::BANG_EQUAL => "BANG_EQUAL",
            TokenType::EQUAL => "EQUAL",
            TokenType::EQUAL_EQUAL => "EQUAL_EQUAL",
            TokenType::GREATER => "GREATER",
            TokenType::GREATER_EQUAL => "GREATER_EQUAL",
            TokenType::LESS => "LESS",
            TokenType::LESS_EQUAL => "LESS_EQUAL",
            TokenType::IDENTIFIER => "IDENTIFIER",
            TokenType::STRING => "STRING",
            TokenType::NUMBER => "NUMBER",
            TokenType::AND => "AND",
            TokenType::CLASS => "CLASS",
            TokenType::ELSE => "ELSE",
            TokenType::FALSE => "FALSE",
            TokenType::FUN => "FUN",
            TokenType::FOR => "FOR",
            TokenType::IF => "IF",
            TokenType::NIL => "NIL",
            TokenType::OR => "OR",
            TokenType::PRINT => "PRINT",
            TokenType::RETURN => "RETURN",
            TokenType::SUPER => "SUPER",
            TokenType::THIS => "THIS",
            TokenType::TRUE => "TRUE",
            TokenType::VAR => "VAR",
            TokenType::WHILE => "WHILE",
            TokenType::EOF => "EOF",
            TokenType::LOOP => "LOOP"
        };
        write!(f, "{}", token_str)
    }
}
