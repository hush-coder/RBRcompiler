use crate::parser::ast::ASTNode;
use crate::semantic::symbol_table::SymbolTable;
use crate::semantic::types::CType;
use crate::CompileError;
use std::collections::HashSet;

pub struct SemanticAnalyzer {
    symbol_table: SymbolTable,
    current_function_return_type: Option<CType>,
    errors: Vec<CompileError>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            current_function_return_type: None,
            errors: Vec::new(),
        }
    }

    pub fn analyze(&mut self, ast: &ASTNode) -> Vec<CompileError> {
        self.errors.clear();
        self.symbol_table = SymbolTable::new();
        self.current_function_return_type = None;
        
        self.analyze_node(ast);
        
        // 去重错误
        let mut unique_errors = HashSet::new();
        let mut result = Vec::new();
        
        for error in &self.errors {
            let error_str = format!("{:?}", error);
            if unique_errors.insert(error_str) {
                result.push(error.clone());
            }
        }
        
        result
    }

    fn analyze_node(&mut self, node: &ASTNode) {
        match node {
            ASTNode::Program(functions) => {
                // 先声明所有函数
                for func in functions {
                    if let ASTNode::FunctionDeclaration { name, parameters, return_type, .. } = func {
                        let func_type = CType::from_str(return_type).unwrap_or(CType::Int);
                        let param_types: Vec<CType> = parameters.iter()
                            .map(|p| {
                                if let ASTNode::Parameter { type_name, .. } = p {
                                    CType::from_str(type_name).unwrap_or(CType::Int)
                                } else {
                                    CType::Int
                                }
                            })
                            .collect();
                        
                        if let Err(e) = self.symbol_table.declare_function(name, func_type, &param_types) {
                            self.errors.push(CompileError::Semantic(e));
                        }
                    }
                }
                
                // 再分析函数体
                for func in functions {
                    self.analyze_node(func);
                }
            }
            
            ASTNode::FunctionDeclaration { name, parameters, return_type, body } => {
                let func_type = CType::from_str(return_type).unwrap_or(CType::Int);
                self.current_function_return_type = Some(func_type);
                
                // 进入函数作用域
                self.symbol_table.enter_scope();
                
                // 声明参数
                for param in parameters {
                    if let ASTNode::Parameter { type_name, name } = param {
                        let param_type = CType::from_str(type_name).unwrap_or(CType::Int);
                        if let Err(e) = self.symbol_table.declare_variable(name, param_type) {
                            self.errors.push(CompileError::Semantic(e));
                        }
                    }
                }
                
                // 分析函数体
                self.analyze_node(body);
                
                // 退出函数作用域
                self.symbol_table.exit_scope();
                self.current_function_return_type = None;
            }
            
            ASTNode::Block(statements) => {
                self.symbol_table.enter_scope();
                for stmt in statements {
                    self.analyze_node(stmt);
                }
                self.symbol_table.exit_scope();
            }
            
            ASTNode::DeclStmt(decl) => {
                self.analyze_declaration(decl);
            }
            
            ASTNode::ExpressionStatement(expr) => {
                self.analyze_expression(expr);
            }
            
            ASTNode::IfStatement { condition, then_branch, else_branch } => {
                self.analyze_expression(condition);
                self.analyze_node(then_branch);
                if let Some(else_stmt) = else_branch {
                    self.analyze_node(else_stmt);
                }
            }
            
            ASTNode::ReturnStatement(expr) => {
                if let Some(expr) = expr {
                    let expr_type = self.analyze_expression(expr);
                    if let Some(expected_type) = &self.current_function_return_type {
                        if expr_type != *expected_type {
                            self.errors.push(CompileError::Semantic(
                                format!("return type mismatch: expected {:?}, got {:?}", expected_type, expr_type)
                            ));
                        }
                    }
                }
            }
            
            _ => {}
        }
    }

    fn analyze_declaration(&mut self, decl: &ASTNode) {
        match decl {
            ASTNode::MultiVarDecl { type_name, declarations } => {
                let var_type = CType::from_str(type_name).unwrap_or(CType::Int);
                
                for decl in declarations {
                    if let ASTNode::VariableDeclaration { name, initializer, .. } = decl {
                        // 声明变量
                        if let Err(e) = self.symbol_table.declare_variable(name, var_type.clone()) {
                            self.errors.push(CompileError::Semantic(e));
                        }
                        
                        // 分析初始化表达式
                        if let Some(init) = initializer {
                            self.analyze_expression(init);
                        }
                    }
                }
            }
            
            ASTNode::VariableDeclaration { name, type_name, initializer } => {
                let var_type = CType::from_str(type_name).unwrap_or(CType::Int);
                
                // 声明变量
                if let Err(e) = self.symbol_table.declare_variable(name, var_type) {
                    self.errors.push(CompileError::Semantic(e));
                }
                
                // 分析初始化表达式
                if let Some(init) = initializer {
                    self.analyze_expression(init);
                }
            }
            
            _ => {}
        }
    }

    fn analyze_expression(&mut self, expr: &ASTNode) -> CType {
        match expr {
            ASTNode::Identifier(name) => {
                match self.symbol_table.lookup_variable(name) {
                    Ok(symbol) => symbol.type_.clone(),
                    Err(e) => {
                        self.errors.push(CompileError::Semantic(e));
                        CType::Int // 默认类型
                    }
                }
            }
            
            ASTNode::IntegerLiteral(_) => CType::Int,
            
            ASTNode::BinaryExpression { left, operator: _, right } => {
                let left_type = self.analyze_expression(left);
                let right_type = self.analyze_expression(right);
                
                // 简单的类型检查
                if left_type != right_type {
                    self.errors.push(CompileError::Semantic(
                        format!("type mismatch in binary expression: {:?} vs {:?}", left_type, right_type)
                    ));
                }
                
                left_type
            }
            
            ASTNode::UnaryExpression { operand, operator: _ } => {
                self.analyze_expression(operand)
            }
            
            ASTNode::AssignmentExpression { target, value } => {
                let target_type = self.analyze_expression(target);
                let value_type = self.analyze_expression(value);
                
                if target_type != value_type {
                    self.errors.push(CompileError::Semantic(
                        format!("assignment type mismatch: {:?} vs {:?}", target_type, value_type)
                    ));
                }
                
                target_type
            }
            
            ASTNode::FunctionCall { name, arguments } => {
                match self.symbol_table.lookup_function(name) {
                    Ok(symbol) => {
                        let symbol_type = symbol.type_.clone();
                        let params = symbol.params.clone();
                        
                        if let Some(params) = params {
                            if arguments.len() != params.len() {
                                self.errors.push(CompileError::Semantic(
                                    format!("function '{}' expects {} arguments, got {}", name, params.len(), arguments.len())
                                ));
                            }
                            
                            // 检查参数类型
                            for (i, arg) in arguments.iter().enumerate() {
                                if i < params.len() {
                                    let arg_type = self.analyze_expression(arg);
                                    if arg_type != params[i] {
                                        self.errors.push(CompileError::Semantic(
                                            format!("argument {} type mismatch: expected {:?}, got {:?}", i + 1, params[i], arg_type)
                                        ));
                                    }
                                }
                            }
                        }
                        
                        symbol_type
                    }
                    Err(e) => {
                        self.errors.push(CompileError::Semantic(e));
                        CType::Int // 默认类型
                    }
                }
            }
            
            ASTNode::ImplicitCastExpr { operand, .. } => {
                self.analyze_expression(operand)
            }
            
            _ => CType::Int
        }
    }
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
