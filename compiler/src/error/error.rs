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