use super::ast::{ASTNode, BinaryOperator, UnaryOperator};
use super::precedence::Precedence;
use crate::lexer::token::Token;
use crate::error::error::CompileError;

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<ASTNode, CompileError> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => {
                    self.synchronize();
                    return Err(e);
                }
            }
        }
        
        Ok(ASTNode::Program(statements))
    }

    // Declaration parsing
    fn declaration(&mut self) -> Result<ASTNode, CompileError> {
        // 判断是否为类型关键字
        if self.check_keyword("int") || self.check_keyword("float") || self.check_keyword("char") || self.check_keyword("void") {
            // 只用 peek 判断类型+标识符+左括号
            let is_func = match (self.tokens.get(self.current), self.tokens.get(self.current + 1), self.tokens.get(self.current + 2)) {
                (Some(Token::Keyword(_)), Some(Token::Identifier(_)), Some(Token::LeftParen)) => true,
                _ => false,
            };
            if is_func {
                self.function_declaration()
            } else {
                let type_name = self.consume_type()?;
                let name = self.consume_identifier()?;
                let init = if self.match_token(&Token::Assign) {
                    Some(self.expression()?)
                } else {
                    None
                };
                self.consume(Token::Semicolon, "Expected ';' after variable declaration")?;
                let var_decl = ASTNode::VariableDeclaration {
                    type_name,
                    name,
                    initializer: init.map(Box::new),
                };
                Ok(ASTNode::DeclStmt(Box::new(var_decl)))
            }
        } else {
            self.statement()
        }
    }

    fn function_declaration(&mut self) -> Result<ASTNode, CompileError> {
        let return_type = self.consume_type()?;
        let name = self.consume_identifier()?;
        
        self.consume(Token::LeftParen, "Expected '(' after function name")?;
        
        let mut parameters = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                let param_type = self.consume_type()?;
                let param_name = self.consume_identifier()?;
                parameters.push(ASTNode::Parameter {
                    type_name: param_type,
                    name: param_name,
                });
                
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }
        
        self.consume(Token::RightParen, "Expected ')' after parameters")?;
        self.consume(Token::LeftBrace, "Expected '{' before function body")?;
        // block() 不再 advance，由调用者 consume '{'
        let body = self.block()?;
        Ok(ASTNode::FunctionDeclaration {
            return_type,
            name,
            parameters,
            body: Box::new(body),
        })
    }

    // Statement parsing
    fn statement(&mut self) -> Result<ASTNode, CompileError> {
        // 调试：打印当前 token
        #[cfg(debug_assertions)]
        if self.check_keyword("if") {
            self.if_statement()
        } else if self.check_keyword("while") {
            self.while_statement()
        } else if self.check_keyword("for") {
            self.for_statement()
        } else if self.check_keyword("return") {
            #[cfg(debug_assertions)]
            self.return_statement()
        } else if self.check(&Token::LeftBrace) {
            self.block()
        } else {
            match self.peek() {
                Token::Identifier(_) | Token::IntegerLiteral(_) | Token::LeftParen => self.expression_statement(),
                _ => Err(CompileError::Parse(format!("Unexpected token in statement: {:?}", self.peek())))
            }
        }
    }

    fn if_statement(&mut self) -> Result<ASTNode, CompileError> {
        self.advance(); // consume 'if'
        self.consume(Token::LeftParen, "Expected '(' after 'if'")?;
        let condition = self.expression()?;
        self.consume(Token::RightParen, "Expected ')' after if condition")?;
        
        let then_branch = self.statement()?;
        let else_branch = if self.match_keyword("else") {
            Some(self.statement()?)
        } else {
            None
        };
        
        Ok(ASTNode::IfStatement {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: else_branch.map(Box::new),
        })
    }

    fn while_statement(&mut self) -> Result<ASTNode, CompileError> {
        self.advance(); // consume 'while'
        self.consume(Token::LeftParen, "Expected '(' after 'while'")?;
        let condition = self.expression()?;
        self.consume(Token::RightParen, "Expected ')' after while condition")?;
        let body = self.statement()?;
        
        Ok(ASTNode::WhileStatement {
            condition: Box::new(condition),
            body: Box::new(body),
        })
    }

    fn for_statement(&mut self) -> Result<ASTNode, CompileError> {
        self.advance(); // consume 'for'
        self.consume(Token::LeftParen, "Expected '(' after 'for'")?;
        
        let init = if self.check(&Token::Semicolon) {
            self.advance();
            None
        } else {
            Some(self.expression()?)
        };
        
        let condition = if self.check(&Token::Semicolon) {
            self.advance();
            None
        } else {
            Some(self.expression()?)
        };
        
        let update = if self.check(&Token::RightParen) {
            None
        } else {
            Some(self.expression()?)
        };
        
        self.consume(Token::RightParen, "Expected ')' after for clauses")?;
        let body = self.statement()?;
        
        Ok(ASTNode::ForStatement {
            init: init.map(Box::new),
            condition: condition.map(Box::new),
            update: update.map(Box::new),
            body: Box::new(body),
        })
    }

    fn return_statement(&mut self) -> Result<ASTNode, CompileError> {
    #[cfg(debug_assertions)]
    self.advance(); // consume 'return'
        let value = if !self.check(&Token::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(Token::Semicolon, "Expected ';' after return value")?;
        
        Ok(ASTNode::ReturnStatement(value.map(Box::new)))
    }

    fn block(&mut self) -> Result<ASTNode, CompileError> {
        // 不再 advance，由调用者 consume '{'
        let mut statements = Vec::new();
        loop {
            if self.is_at_end() {
                // 只允许空 block 情况下 EOF
                if statements.is_empty() {
                    return Ok(ASTNode::Block(statements));
                } else {
                    return Err(CompileError::Parse("Expected '}' after block: EOF".to_string()));
                }
            }
            if self.check(&Token::RightBrace) {
                self.advance();
                break;
            }
            // declaration 必须推进 token，否则死循环
            let before = self.current;
            let stmt = self.declaration()?;
            if self.current == before {
                return Err(CompileError::Parse("Parser did not advance in block; possible infinite loop".to_string()));
            }
            statements.push(stmt);
        }
        Ok(ASTNode::Block(statements))
    }

    fn expression_statement(&mut self) -> Result<ASTNode, CompileError> {
        let expr = self.expression()?;
        self.consume(Token::Semicolon, "Expected ';' after expression")?;
        Ok(ASTNode::ExpressionStatement(Box::new(expr)))
    }

    // Expression parsing using Pratt parser
    fn expression(&mut self) -> Result<ASTNode, CompileError> {
        self.parse_precedence(Precedence::Assignment)
    }

    fn parse_precedence(&mut self, precedence: Precedence) -> Result<ASTNode, CompileError> {
        let mut expr = self.unary()?;
        
        while precedence < self.get_precedence() {
            let operator = self.get_binary_operator()?;
            self.advance();
            let right = self.parse_precedence(self.get_precedence())?;
            expr = ASTNode::BinaryExpression {
                operator,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn unary(&mut self) -> Result<ASTNode, CompileError> {
        if self.match_token(&Token::Minus) {
            Ok(ASTNode::UnaryExpression {
                operator: UnaryOperator::Minus,
                operand: Box::new(self.unary()?),
            })
        } else if self.match_token(&Token::Plus) {
            Ok(ASTNode::UnaryExpression {
                operator: UnaryOperator::Plus,
                operand: Box::new(self.unary()?),
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<ASTNode, CompileError> {
        if let Some(value) = self.match_integer() {
            Ok(ASTNode::IntegerLiteral(value))
        } else if let Some(value) = self.match_string() {
            Ok(ASTNode::StringLiteral(value))
        } else if let Some(name) = self.match_identifier() {
            if self.check(&Token::LeftParen) {
                self.function_call(name)
            } else {
                let identifier = ASTNode::Identifier(name);
                Ok(ASTNode::ImplicitCastExpr {
                    cast_kind: "LValueToRValue".to_string(),
                    operand: Box::new(identifier),
                })
            }
        } else if self.match_token(&Token::LeftParen) {
            let expr = self.expression()?;
            self.consume(Token::RightParen, "Expected ')' after expression")?;
            Ok(expr)
        } else {
            Err(CompileError::Parse("Expected expression".to_string()))
        }
    }

    fn function_call(&mut self, name: String) -> Result<ASTNode, CompileError> {
        self.advance(); // consume '('
        let mut arguments = Vec::new();
        
        if !self.check(&Token::RightParen) {
            loop {
                arguments.push(self.expression()?);
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }
        
        self.consume(Token::RightParen, "Expected ')' after arguments")?;
        Ok(ASTNode::FunctionCall { name, arguments })
    }

    // Helper methods
    fn is_at_end(&self) -> bool {
        self.peek() == &Token::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        
        // Skip whitespace tokens
        while !self.is_at_end() && matches!(self.peek(), Token::Whitespace) {
            self.current += 1;
        }
        
        self.previous()
    }

    fn check(&self, token: &Token) -> bool {
        if self.is_at_end() {
            false
        } else {
            match (self.peek(), token) {
                (Token::Identifier(_), Token::Identifier(_)) => true,
                (Token::Keyword(_), Token::Keyword(_)) => true,
                (Token::IntegerLiteral(_), Token::IntegerLiteral(_)) => true,
                (a, b) => std::mem::discriminant(a) == std::mem::discriminant(b),
            }
        }
    }

    fn check_keyword(&self, keyword: &str) -> bool {
        if self.is_at_end() {
            false
        } else {
            matches!(self.peek(), Token::Keyword(k) if k == keyword)
        }
    }

    fn match_token(&mut self, token: &Token) -> bool {
        if self.check(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_identifier(&mut self) -> Option<String> {
        if let Token::Identifier(name) = self.peek() {
            let name = name.clone();
            self.advance();
            Some(name)
        } else {
            None
        }
    }

    fn match_integer(&mut self) -> Option<i64> {
        if let Token::IntegerLiteral(value) = self.peek() {
            let value = *value;
            self.advance();
            Some(value)
        } else {
            None
        }
    }

    fn match_string(&mut self) -> Option<String> {
        if let Token::StringLiteral(value) = self.peek() {
            let value = value.clone();
            self.advance();
            Some(value)
        } else {
            None
        }
    }

    fn match_keyword(&mut self, keyword: &str) -> bool {
        if self.check_keyword(keyword) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn consume(&mut self, token: Token, message: &str) -> Result<&Token, CompileError> {
        if self.check(&token) {
            Ok(self.advance())
        } else {
            Err(CompileError::Parse(format!("{}: {}", message, self.peek().to_string())))
        }
    }

    fn consume_type(&mut self) -> Result<String, CompileError> {
        if let Some(type_name) = self.match_type_keyword() {
            Ok(type_name)
        } else {
            Err(CompileError::Parse("Expected type name".to_string()))
        }
    }

    fn match_type_keyword(&mut self) -> Option<String> {
        if let Token::Keyword(ref k) = self.peek() {
            if ["int", "float", "char", "void"].contains(&k.as_str()) {
                let name = k.clone();
                self.advance();
                return Some(name);
            }
        }
        None
    }

    fn consume_identifier(&mut self) -> Result<String, CompileError> {
        self.match_identifier().ok_or_else(|| CompileError::Parse("Expected identifier".to_string()))
    }

    fn get_precedence(&self) -> Precedence {
        match self.peek() {
            Token::Assign => Precedence::Assignment,
            Token::Plus | Token::Minus => Precedence::Additive,
            Token::Multiply | Token::Divide => Precedence::Multiplicative,
            _ => Precedence::Lowest,
        }
    }

    fn get_binary_operator(&self) -> Result<BinaryOperator, CompileError> {
        match self.peek() {
            Token::Plus => Ok(BinaryOperator::Add),
            Token::Minus => Ok(BinaryOperator::Subtract),
            Token::Multiply => Ok(BinaryOperator::Multiply),
            Token::Divide => Ok(BinaryOperator::Divide),
            Token::Assign => Ok(BinaryOperator::Equal), // Simplified assignment
            _ => Err(CompileError::Parse("Expected binary operator".to_string())),
        }
    }

    fn synchronize(&mut self) {
        self.advance();
        
        while !self.is_at_end() {
            if self.previous() == &Token::Semicolon {
                return;
            }
            
            match self.peek() {
                Token::Keyword(k) if k == "int" || k == "float" || k == "char" || k == "void" => return,
                Token::Keyword(k) if k == "if" || k == "while" || k == "for" || k == "return" => return,
                _ => { self.advance(); }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::Token;
    use crate::parser::ast::{ASTNode, BinaryOperator, UnaryOperator};

    fn create_test_tokens(tokens: Vec<Token>) -> Vec<Token> {
        let mut result = tokens;
        result.push(Token::EOF);
        result
    }

    #[test]
    fn test_parse_integer_literal() {
        let tokens = create_test_tokens(vec![Token::IntegerLiteral(42), Token::Semicolon]);
        let mut parser = Parser::new(tokens);
        let result = parser.parse().unwrap();
        
        match result {
            ASTNode::Program(statements) => {
                assert_eq!(statements.len(), 1);
                match &statements[0] {
                    ASTNode::ExpressionStatement(expr) => {
                        match expr.as_ref() {
                            ASTNode::IntegerLiteral(value) => assert_eq!(*value, 42),
                            _ => panic!("Expected integer literal"),
                        }
                    }
                    _ => panic!("Expected expression statement"),
                }
            }
            _ => panic!("Expected program"),
        }
    }

    #[test]
    fn test_parse_binary_expression() {
        let tokens = create_test_tokens(vec![
            Token::IntegerLiteral(10),
            Token::Plus,
            Token::IntegerLiteral(5),
            Token::Semicolon,
        ]);
        let mut parser = Parser::new(tokens);
        let result = parser.parse().unwrap();
        
        match result {
            ASTNode::Program(statements) => {
                assert_eq!(statements.len(), 1);
                match &statements[0] {
                    ASTNode::ExpressionStatement(expr) => {
                        match expr.as_ref() {
                            ASTNode::BinaryExpression { operator, left, right } => {
                                assert_eq!(*operator, BinaryOperator::Add);
                                match left.as_ref() {
                                    ASTNode::IntegerLiteral(value) => assert_eq!(*value, 10),
                                    _ => panic!("Expected integer literal on left"),
                                }
                                match right.as_ref() {
                                    ASTNode::IntegerLiteral(value) => assert_eq!(*value, 5),
                                    _ => panic!("Expected integer literal on right"),
                                }
                            }
                            _ => panic!("Expected binary expression"),
                        }
                    }
                    _ => panic!("Expected expression statement"),
                }
            }
            _ => panic!("Expected program"),
        }
    }

    #[test]
    fn test_parse_function_declaration() {
        let tokens = create_test_tokens(vec![
            Token::Keyword("int".to_string()),
            Token::Identifier("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::RightBrace,
        ]);
        let mut parser = Parser::new(tokens);
        let result = parser.parse().unwrap();
        
        match result {
            ASTNode::Program(statements) => {
                assert_eq!(statements.len(), 1);
                match &statements[0] {
                    ASTNode::FunctionDeclaration { return_type, name, parameters, body } => {
                        assert_eq!(return_type, "int");
                        assert_eq!(name, "main");
                        assert_eq!(parameters.len(), 0);
                        match body.as_ref() {
                            ASTNode::Block(block_statements) => {
                                assert_eq!(block_statements.len(), 0);
                            }
                            _ => panic!("Expected block body"),
                        }
                    }
                    _ => panic!("Expected function declaration"),
                }
            }
            _ => panic!("Expected program"),
        }
    }

    #[test]
    fn test_lexer_parser_integration() {
        use crate::lexer::lexer::Lexer;
        
        let input = "int main() { return 42; }";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().expect("Lexing failed");
        
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Parsing failed");
        
        // Verify we got a program with one function declaration
        match ast {
            ASTNode::Program(statements) => {
                assert_eq!(statements.len(), 1);
                match &statements[0] {
                    ASTNode::FunctionDeclaration { return_type, name, parameters, body } => {
                        assert_eq!(return_type, "int");
                        assert_eq!(name, "main");
                        assert_eq!(parameters.len(), 0);
                        match body.as_ref() {
                            ASTNode::Block(block_statements) => {
                                assert_eq!(block_statements.len(), 1);
                                match &block_statements[0] {
                                    ASTNode::ReturnStatement(value) => {
                                        assert!(value.is_some());
                                        match value.as_ref().unwrap().as_ref() {
                                            ASTNode::IntegerLiteral(val) => assert_eq!(*val, 42),
                                            _ => panic!("Expected integer return value"),
                                        }
                                    }
                                    _ => panic!("Expected return statement"),
                                }
                            }
                            _ => panic!("Expected block body"),
                        }
                    }
                    _ => panic!("Expected function declaration"),
                }
            }
            _ => panic!("Expected program"),
        }
    }

    #[test]
    fn test_simple_arithmetic() {
        use crate::lexer::lexer::Lexer;
        
        let input = "1 + 2 * 3;";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().expect("Lexing failed");
        
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Parsing failed");
        
        // Should parse as 1 + (2 * 3) due to precedence
        match ast {
            ASTNode::Program(statements) => {
                assert_eq!(statements.len(), 1);
                match &statements[0] {
                    ASTNode::ExpressionStatement(expr) => {
                        match expr.as_ref() {
                            ASTNode::BinaryExpression { operator, left, right } => {
                                assert_eq!(*operator, BinaryOperator::Add);
                                match left.as_ref() {
                                    ASTNode::IntegerLiteral(value) => assert_eq!(*value, 1),
                                    _ => panic!("Expected integer literal on left"),
                                }
                                match right.as_ref() {
                                    ASTNode::BinaryExpression { operator, left, right } => {
                                        assert_eq!(*operator, BinaryOperator::Multiply);
                                        match left.as_ref() {
                                            ASTNode::IntegerLiteral(value) => assert_eq!(*value, 2),
                                            _ => panic!("Expected integer literal on left"),
                                        }
                                        match right.as_ref() {
                                            ASTNode::IntegerLiteral(value) => assert_eq!(*value, 3),
                                            _ => panic!("Expected integer literal on right"),
                                        }
                                    }
                                    _ => panic!("Expected binary expression on right"),
                                }
                            }
                            _ => panic!("Expected binary expression"),
                        }
                    }
                    _ => panic!("Expected expression statement"),
                }
            }
            _ => panic!("Expected program"),
        }
    }

    #[test]
    fn test_parse_main_with_vars_and_return() {
        use crate::lexer::lexer::Lexer;

        let input = r#"
            int main() {
                int x = 42;
                int y = 10;
                int result = x + y;
                return result;
            }
        "#;
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().expect("Lexing failed");

        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Parsing failed");

        match ast {
            ASTNode::Program(statements) => {
                assert_eq!(statements.len(), 1);
                match &statements[0] {
                    ASTNode::FunctionDeclaration { return_type, name, parameters, body } => {
                        assert_eq!(return_type, "int");
                        assert_eq!(name, "main");
                        assert_eq!(parameters.len(), 0);
                        match body.as_ref() {
                            ASTNode::Block(block_statements) => {
                                assert_eq!(block_statements.len(), 4);
                                // int x = 42;
                                match &block_statements[0] {
                                    ASTNode::VariableDeclaration { type_name, name, initializer } => {
                                        assert_eq!(type_name, "int");
                                        assert_eq!(name, "x");
                                        match initializer {
                                            Some(expr) => match expr.as_ref() {
                                                ASTNode::IntegerLiteral(val) => assert_eq!(*val, 42),
                                                _ => panic!("Expected integer literal for x"),
                                            },
                                            None => panic!("Expected initializer for x"),
                                        }
                                    }
                                    _ => panic!("Expected variable declaration for x"),
                                }
                                // int y = 10;
                                match &block_statements[1] {
                                    ASTNode::VariableDeclaration { type_name, name, initializer } => {
                                        assert_eq!(type_name, "int");
                                        assert_eq!(name, "y");
                                        match initializer {
                                            Some(expr) => match expr.as_ref() {
                                                ASTNode::IntegerLiteral(val) => assert_eq!(*val, 10),
                                                _ => panic!("Expected integer literal for y"),
                                            },
                                            None => panic!("Expected initializer for y"),
                                        }
                                    }
                                    _ => panic!("Expected variable declaration for y"),
                                }
                                // int result = x + y;
                                match &block_statements[2] {
                                    ASTNode::VariableDeclaration { type_name, name, initializer } => {
                                        assert_eq!(type_name, "int");
                                        assert_eq!(name, "result");
                                        match initializer {
                                            Some(expr) => match expr.as_ref() {
                                                ASTNode::BinaryExpression { operator, left, right } => {
                                                    assert_eq!(*operator, BinaryOperator::Add);
                                                    match left.as_ref() {
                                                        ASTNode::Identifier(id) => assert_eq!(id, "x"),
                                                        _ => panic!("Expected identifier x in result"),
                                                    }
                                                    match right.as_ref() {
                                                        ASTNode::Identifier(id) => assert_eq!(id, "y"),
                                                        _ => panic!("Expected identifier y in result"),
                                                    }
                                                }
                                                _ => panic!("Expected binary expression for result"),
                                            },
                                            None => panic!("Expected initializer for result"),
                                        }
                                    }
                                    _ => panic!("Expected variable declaration for result"),
                                }
                                // return result;
                                match &block_statements[3] {
                                    ASTNode::ReturnStatement(Some(expr)) => match expr.as_ref() {
                                        ASTNode::Identifier(id) => assert_eq!(id, "result"),
                                        _ => panic!("Expected identifier in return"),
                                    },
                                    _ => panic!("Expected return statement"),
                                }
                            }
                            _ => panic!("Expected block body"),
                        }
                    }
                    _ => panic!("Expected function declaration"),
                }
            }
            _ => panic!("Expected program"),
        }
    }
} 