use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Token {
    Identifier(String),
    Keyword(String),
    IntegerLiteral(i64),
    StringLiteral(String),
    Plus,
    Minus,
    Multiply,
    Divide,
    Assign,
    Semicolon,
    Comma,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Whitespace,
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenWithInfo {
    pub token: Token,
    pub line: usize,
    pub column: usize,
    pub flags: Vec<String>,
    pub file_path: String,
}

impl Token {
    pub fn to_string(&self) -> String {
        match self {
            Token::Identifier(s) => format!("Identifier({})", s),
            Token::Keyword(s) => format!("Keyword({})", s),
            Token::IntegerLiteral(n) => format!("Integer({})", n),
            Token::StringLiteral(s) => format!("String({})", s),
            Token::Plus => "Plus".to_string(),
            Token::Minus => "Minus".to_string(),
            Token::Multiply => "Multiply".to_string(),
            Token::Divide => "Divide".to_string(),
            Token::Assign => "Assign".to_string(),
            Token::Semicolon => "Semicolon".to_string(),
            Token::Comma => "Comma".to_string(),
            Token::LeftParen => "LeftParen".to_string(),
            Token::RightParen => "RightParen".to_string(),
            Token::LeftBrace => "LeftBrace".to_string(),
            Token::RightBrace => "RightBrace".to_string(),
            Token::Whitespace => "Whitespace".to_string(),
            Token::EOF => "EOF".to_string(),
        }
    }

    pub fn get_bisheng_kind(&self) -> String {
        match self {
            Token::Identifier(_) => "identifier".to_string(),
            Token::Keyword(s) => s.clone(),
            Token::IntegerLiteral(_) => "numeric_constant".to_string(),
            Token::StringLiteral(_) => "string_literal".to_string(),
            Token::Plus => "plus".to_string(),
            Token::Minus => "minus".to_string(),
            Token::Multiply => "star".to_string(),
            Token::Divide => "slash".to_string(),
            Token::Assign => "equal".to_string(),
            Token::Semicolon => "semi".to_string(),
            Token::Comma => "comma".to_string(),
            Token::LeftParen => "l_paren".to_string(),
            Token::RightParen => "r_paren".to_string(),
            Token::LeftBrace => "l_brace".to_string(),
            Token::RightBrace => "r_brace".to_string(),
            Token::Whitespace => "whitespace".to_string(),
            Token::EOF => "eof".to_string(),
        }
    }

    pub fn get_bisheng_value(&self) -> String {
        match self {
            Token::Identifier(s) => s.clone(),
            Token::Keyword(s) => s.clone(),
            Token::IntegerLiteral(n) => n.to_string(),
            Token::StringLiteral(s) => s.clone(),
            Token::Plus => "+".to_string(),
            Token::Minus => "-".to_string(),
            Token::Multiply => "*".to_string(),
            Token::Divide => "/".to_string(),
            Token::Assign => "=".to_string(),
            Token::Semicolon => ";".to_string(),
            Token::Comma => ",".to_string(),
            Token::LeftParen => "(".to_string(),
            Token::RightParen => ")".to_string(),
            Token::LeftBrace => "{".to_string(),
            Token::RightBrace => "}".to_string(),
            Token::Whitespace => " ".to_string(),
            Token::EOF => "".to_string(),
        }
    }
}

impl TokenWithInfo {
    pub fn new(token: Token, line: usize, column: usize, file_path: String) -> Self {
        Self {
            token,
            line,
            column,
            flags: Vec::new(),
            file_path,
        }
    }

    pub fn to_bisheng_format(&self) -> String {
        let kind = self.token.get_bisheng_kind();
        let value = self.token.get_bisheng_value();
        let flags_str = if self.flags.is_empty() {
            "".to_string()
        } else {
            format!("\t {}", self.flags.iter().map(|f| format!("[{}]", f)).collect::<Vec<_>>().join(" "))
        };
        
        format!("{} '{}'{}\tLoc=<{}:{}:{}>", 
                kind, value, flags_str, self.file_path, self.line, self.column)
    }
} 