#[derive(Debug, Clone)]
pub enum ScannerError {
    UnexpectedCharacter(char, usize), // Carácter inesperado en una posición
    UnterminatedString(usize),        // Cadena no terminada
    UnfinishedMultilineComment(usize), // Comentario multilinea sin terminar
    InvalidEscapeSequence(usize),     // Secuencia de escape no válida
}

impl std::fmt::Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ScannerError::UnexpectedCharacter(c, line) => {
                write!(f, "[SCANNER]: Unexpected character '{}' at line {}", c, line)
            }
            ScannerError::UnterminatedString(line) => {
                write!(f, "[SCANNER]: Unterminated string at line {}", line)
            }
            ScannerError::UnfinishedMultilineComment(line) => {
                write!(f, "[SCANNER]: Unfinished multiline comment at line {}", line)
            }
            ScannerError::InvalidEscapeSequence(line) => {
                write!(f, "[SCANNER]: Invalid escape sequence at line {}", line)
            }
        }
    }
}

impl std::error::Error for ScannerError {}
