use super::token::{Token, TokenWithInfo};
use super::keywords::c_keywords;

pub struct Lexer {
    pub input: Vec<char>,
    pub position: usize,
    pub line: usize,
    pub column: usize,
    pub file_path: String,
    pub at_start_of_line: bool,
    pub has_leading_space: bool,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self { 
            input: input.chars().collect(), 
            position: 0,
            line: 1,
            column: 1,
            file_path: "".to_string(),
            at_start_of_line: true,
            has_leading_space: false,
        }
    }

    pub fn new_with_file(input: &str, file_path: String) -> Self {
        Self { 
            input: input.chars().collect(), 
            position: 0,
            line: 1,
            column: 1,
            file_path,
            at_start_of_line: true,
            has_leading_space: false,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        
        while self.position < self.input.len() {
            self.skip_whitespace();
            
            if self.position >= self.input.len() {
                break;
            }
            
            let token = self.next_token()?;
            tokens.push(token);
        }
        
        tokens.push(Token::EOF);
        Ok(tokens)
    }

    pub fn tokenize_with_info(&mut self) -> Result<Vec<TokenWithInfo>, String> {
        let mut tokens = Vec::new();
        
        while self.position < self.input.len() {
            self.skip_whitespace();
            
            if self.position >= self.input.len() {
                break;
            }
            
            let token_info = self.next_token_with_info()?;
            tokens.push(token_info);
        }
        
        // Add EOF token
        let eof_token = TokenWithInfo::new(
            Token::EOF, 
            self.line, 
            self.column, 
            self.file_path.clone()
        );
        tokens.push(eof_token);
        Ok(tokens)
    }

    fn skip_whitespace(&mut self) {
        self.has_leading_space = false;
        // Don't reset at_start_of_line here, it should be managed by newline handling
        
        while self.position < self.input.len() {
            let ch = self.input[self.position];
            match ch {
                ' ' | '\t' => {
                    self.position += 1;
                    self.column += 1;
                    self.has_leading_space = true;
                }
                '\n' => {
                    self.position += 1;
                    self.line += 1;
                    self.column = 1;
                    self.at_start_of_line = true;
                    self.has_leading_space = false;
                }
                '\r' => {
                    self.position += 1;
                    if self.position < self.input.len() && self.input[self.position] == '\n' {
                        self.position += 1;
                    }
                    self.line += 1;
                    self.column = 1;
                    self.at_start_of_line = true;
                    self.has_leading_space = false;
                }
                _ => break,
            }
        }
    }

    fn next_token(&mut self) -> Result<Token, String> {
        let ch = self.input[self.position];
        
        match ch {
            'a'..='z' | 'A'..='Z' | '_' => {
                self.read_identifier_or_keyword()
            }
            '0'..='9' => {
                self.read_number()
            }
            '"' => {
                self.read_string()
            }
            '/' => {
                if self.position + 1 < self.input.len() && self.input[self.position + 1] == '/' {
                    self.read_line_comment()
                } else {
                    self.position += 1;
                    self.column += 1;
                    Ok(Token::Divide)
                }
            }
            '+' => {
                self.position += 1;
                self.column += 1;
                Ok(Token::Plus)
            }
            '-' => {
                self.position += 1;
                self.column += 1;
                Ok(Token::Minus)
            }
            '*' => {
                self.position += 1;
                self.column += 1;
                Ok(Token::Multiply)
            }
            '=' => {
                self.position += 1;
                self.column += 1;
                Ok(Token::Assign)
            }
            ';' => {
                self.position += 1;
                self.column += 1;
                Ok(Token::Semicolon)
            }
            ',' => {
                self.position += 1;
                self.column += 1;
                Ok(Token::Comma)
            }
            '(' => {
                self.position += 1;
                self.column += 1;
                Ok(Token::LeftParen)
            }
            ')' => {
                self.position += 1;
                self.column += 1;
                Ok(Token::RightParen)
            }
            '{' => {
                self.position += 1;
                self.column += 1;
                Ok(Token::LeftBrace)
            }
            '}' => {
                self.position += 1;
                self.column += 1;
                Ok(Token::RightBrace)
            }
            _ => {
                Err(format!("Unexpected character '{}' at line {}, column {}", 
                           ch, self.line, self.column))
            }
        }
    }

    fn next_token_with_info(&mut self) -> Result<TokenWithInfo, String> {
        let ch = self.input[self.position];
        let line = self.line;
        let column = self.column;
        
        let mut flags = Vec::new();
        if self.at_start_of_line {
            flags.push("StartOfLine".to_string());
        }
        if self.has_leading_space {
            flags.push("LeadingSpace".to_string());
        }
        
        let token = match ch {
            'a'..='z' | 'A'..='Z' | '_' => {
                self.read_identifier_or_keyword()?
            }
            '0'..='9' => {
                self.read_number()?
            }
            '"' => {
                self.read_string()?
            }
            '+' => {
                self.position += 1;
                self.column += 1;
                Token::Plus
            }
            '-' => {
                self.position += 1;
                self.column += 1;
                Token::Minus
            }
            '*' => {
                self.position += 1;
                self.column += 1;
                Token::Multiply
            }
            '/' => {
                self.position += 1;
                self.column += 1;
                Token::Divide
            }
            '=' => {
                self.position += 1;
                self.column += 1;
                Token::Assign
            }
            ';' => {
                self.position += 1;
                self.column += 1;
                Token::Semicolon
            }
            ',' => {
                self.position += 1;
                self.column += 1;
                Token::Comma
            }
            '(' => {
                self.position += 1;
                self.column += 1;
                Token::LeftParen
            }
            ')' => {
                self.position += 1;
                self.column += 1;
                Token::RightParen
            }
            '{' => {
                self.position += 1;
                self.column += 1;
                Token::LeftBrace
            }
            '}' => {
                self.position += 1;
                self.column += 1;
                Token::RightBrace
            }
            _ => {
                return Err(format!("Unexpected character '{}' at line {}, column {}", 
                           ch, self.line, self.column));
            }
        };
        
        let mut token_info = TokenWithInfo::new(token, line, column, self.file_path.clone());
        token_info.flags = flags;
        
        // Reset at_start_of_line after processing the token
        self.at_start_of_line = false;
        
        Ok(token_info)
    }

    fn read_identifier_or_keyword(&mut self) -> Result<Token, String> {
        let start = self.position;
        
        while self.position < self.input.len() {
            let ch = self.input[self.position];
            if ch.is_alphanumeric() || ch == '_' {
                self.position += 1;
                self.column += 1;
            } else {
                break;
            }
        }
        
        let identifier: String = self.input[start..self.position].iter().collect();
        
        if c_keywords().contains(identifier.as_str()) {
            Ok(Token::Keyword(identifier))
        } else {
            Ok(Token::Identifier(identifier))
        }
    }

    fn read_number(&mut self) -> Result<Token, String> {
        let start = self.position;
        let start_column = self.column;
        
        while self.position < self.input.len() {
            let ch = self.input[self.position];
            if ch.is_digit(10) {
                self.position += 1;
                self.column += 1;
            } else {
                break;
            }
        }
        
        let number_str: String = self.input[start..self.position].iter().collect();
        match number_str.parse::<i64>() {
            Ok(num) => Ok(Token::IntegerLiteral(num)),
            Err(_) => Err(format!("Invalid number '{}' at line {}, column {}", 
                                 number_str, self.line, start_column))
        }
    }

    fn read_line_comment(&mut self) -> Result<Token, String> {
        // Skip the '//' characters
        self.position += 2;
        self.column += 2;
        
        // Skip until end of line
        while self.position < self.input.len() && self.input[self.position] != '\n' {
            self.position += 1;
            self.column += 1;
        }
        
        // Return a whitespace token to continue parsing
        Ok(Token::Whitespace)
    }

    fn read_string(&mut self) -> Result<Token, String> {
        let start_column = self.column;
        self.position += 1; // 跳过开始的引号
        self.column += 1;
        
        let mut string_content = String::new();
        
        while self.position < self.input.len() {
            let ch = self.input[self.position];
            if ch == '"' {
                self.position += 1; // 跳过结束的引号
                self.column += 1;
                return Ok(Token::StringLiteral(string_content));
            } else {
                string_content.push(ch);
                self.position += 1;
                self.column += 1;
            }
        }
        
        Err(format!("Unterminated string literal at line {}, column {}", 
                   self.line, start_column))
    }

    pub fn get_position_info(&self) -> (usize, usize) {
        (self.line, self.column)
    }
} 