use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum CompileError {
    #[error("lex error: {0}")]
    Lex(String),
    #[error("parse error: {0}")]
    Parse(String),
    #[error("semantic error: {0}")]
    Semantic(String),
}

#[derive(Debug, Clone, Error)]
pub enum LexerError {
    #[error("unexpected character '{0}' at line {1}, column {2}")]
    UnexpectedCharacter(char, usize, usize),
} 