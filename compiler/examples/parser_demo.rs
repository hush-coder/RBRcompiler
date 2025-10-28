use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::parser::ast::ASTNode;

fn main() {
    println!("=== Parser 功能测试 ===\n");

    // 测试用例1: 简单函数
    test_parser("int main() { return 42; }", "简单函数声明");

    // 测试用例2: 算术表达式
    test_parser("1 + 2 * 3;", "算术表达式");

    // 测试用例3: 带参数的函数
    test_parser("int add(int x, int y) { return x + y; }", "带参数的函数");

    // 测试用例4: 变量赋值
    test_parser("x = 5;", "变量赋值");

    // 测试用例5: 函数调用
    test_parser("foo(1, 2);", "函数调用");
}

fn test_parser(input: &str, description: &str) {
    println!("--- {} ---", description);
    println!("输入: {}", input);
    
    // 步骤1: 词法分析
    let mut lexer = Lexer::new(input);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(e) => {
            println!("词法分析错误: {}", e);
            return;
        }
    };
    
    println!("Token流: {:?}", tokens);
    
    // 步骤2: 语法分析
    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            println!("语法分析错误: {}", e);
            return;
        }
    };
    
    // 步骤3: 输出AST
    println!("AST结构:");
    print_ast(&ast, 0);
    println!();
}

fn print_ast(ast: &ASTNode, indent: usize) {
    let spaces = "  ".repeat(indent);
    
    match ast {
        ASTNode::Program(statements) => {
            println!("{}Program({} statements)", spaces, statements.len());
            for stmt in statements {
                print_ast(stmt, indent + 1);
            }
        }
        ASTNode::FunctionDeclaration { return_type, name, parameters, body } => {
            println!("{}FunctionDeclaration: {} {} ({} params)", 
                    spaces, return_type, name, parameters.len());
            print_ast(body, indent + 1);
        }
        ASTNode::Block(statements) => {
            println!("{}Block({} statements)", spaces, statements.len());
            for stmt in statements {
                print_ast(stmt, indent + 1);
            }
        }
        ASTNode::ReturnStatement(value) => {
            println!("{}ReturnStatement", spaces);
            if let Some(expr) = value {
                print_ast(expr, indent + 1);
            }
        }
        ASTNode::ExpressionStatement(expr) => {
            println!("{}ExpressionStatement", spaces);
            print_ast(expr, indent + 1);
        }
        ASTNode::BinaryExpression { operator, left, right } => {
            println!("{}BinaryExpression: {:?}", spaces, operator);
            print_ast(left, indent + 1);
            print_ast(right, indent + 1);
        }
        ASTNode::UnaryExpression { operator, operand } => {
            println!("{}UnaryExpression: {:?}", spaces, operator);
            print_ast(operand, indent + 1);
        }
        ASTNode::IntegerLiteral(value) => {
            println!("{}IntegerLiteral: {}", spaces, value);
        }
        ASTNode::Identifier(name) => {
            println!("{}Identifier: {}", spaces, name);
        }
        ASTNode::FunctionCall { name, arguments } => {
            println!("{}FunctionCall: {} ({} args)", spaces, name, arguments.len());
            for arg in arguments {
                print_ast(arg, indent + 1);
            }
        }
        ASTNode::Parameter { type_name, name } => {
            println!("{}Parameter: {} {}", spaces, type_name, name);
        }
        ASTNode::IfStatement { condition, then_branch, else_branch } => {
            println!("{}IfStatement", spaces);
            print_ast(condition, indent + 1);
            print_ast(then_branch, indent + 1);
            if let Some(else_branch) = else_branch {
                print_ast(else_branch, indent + 1);
            }
        }
        ASTNode::WhileStatement { condition, body } => {
            println!("{}WhileStatement", spaces);
            print_ast(condition, indent + 1);
            print_ast(body, indent + 1);
        }
        ASTNode::ForStatement { init, condition, update, body } => {
            println!("{}ForStatement", spaces);
            if let Some(init) = init {
                print_ast(init, indent + 1);
            }
            if let Some(condition) = condition {
                print_ast(condition, indent + 1);
            }
            if let Some(update) = update {
                print_ast(update, indent + 1);
            }
            print_ast(body, indent + 1);
        }
        ASTNode::AssignmentExpression { target, value } => {
            println!("{}AssignmentExpression", spaces);
            print_ast(target, indent + 1);
            print_ast(value, indent + 1);
        }
        ASTNode::Empty => {
            println!("{}Empty", spaces);
        }
    }
}


