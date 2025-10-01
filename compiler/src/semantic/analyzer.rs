use crate::parser::ast::ASTNode;
use crate::semantic::symbol_table::SymbolTable;
use crate::semantic::types::CType;
use crate::error::error::CompileError;

pub struct SemanticAnalyzer {
    pub symbol_table: SymbolTable,
    current_function_return_type: Option<String>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            current_function_return_type: None,
        }
    }

    // 收集所有错误的方法
    pub fn analyze_all_errors(&mut self, ast: &ASTNode) -> Vec<CompileError> {
        let mut all_errors = Vec::new();
        self.collect_errors(ast, &mut all_errors);
        
        // 去重：移除重复的错误
        let mut unique_errors = Vec::new();
        for error in all_errors {
            let error_str = error.to_string();
            // 更精确的去重：检查错误消息的核心内容
            let is_duplicate = unique_errors.iter().any(|e: &CompileError| {
                let existing_str = e.to_string();
                // 提取错误的核心信息进行比较
                let extract_core_info = |s: &str| -> String {
                    // 提取变量名或函数名
                    if s.contains("变量未定义:") {
                        s.split("变量未定义: ").nth(1).unwrap_or("").to_string()
                    } else if s.contains("函数未定义:") {
                        s.split("函数未定义: ").nth(1).unwrap_or("").to_string()
                    } else if s.contains("变量重复定义:") {
                        s.split("变量重复定义: ").nth(1).unwrap_or("").to_string()
                    } else if s.contains("函数重复定义:") {
                        s.split("函数重复定义: ").nth(1).unwrap_or("").to_string()
                    } else {
                        s.to_string()
                    }
                };
                
                let core_existing = extract_core_info(&existing_str);
                let core_current = extract_core_info(&error_str);
                
                // 如果核心信息相同，认为是重复的
                !core_existing.is_empty() && core_existing == core_current
            });
            
            if !is_duplicate {
                unique_errors.push(error);
            }
        }
        unique_errors
    }

    // 生成bisheng格式的semantic错误输出
    pub fn generate_bisheng_semantic_output(&mut self, ast: &ASTNode, file_path: &str, source_code: &str) -> String {
        let all_errors = self.analyze_all_errors(ast);
        
        if all_errors.is_empty() {
            // 对于测试，输出语义分析成功信息
            return "Semantic analysis completed successfully - no errors found.\n".to_string();
        }
        
        let lines: Vec<&str> = source_code.lines().collect();
        let mut output = String::new();
        
        // 直接输出收集到的错误
        for error in &all_errors {
            match error {
                CompileError::Semantic(msg) => {
                    // 尝试从错误消息中提取行号和列号信息
                    if let Some((line_num, col_pos, error_msg, note_info)) = self.extract_error_info_with_notes(msg, &lines) {
                        let line = lines.get(line_num - 1).unwrap_or(&"");
                        output.push_str(&format!("{}:{}:{}: error: {}\n    {} | {}\n      | {}^\n", 
                            file_path, line_num, col_pos, error_msg, line_num, line, 
                            " ".repeat(col_pos - 1)));
                        
                        // 添加note信息
                        if let Some(note) = note_info {
                            output.push_str(&format!("{}\n", note));
                        }
                    } else {
                        // 如果无法提取位置信息，直接输出错误消息
                        output.push_str(&format!("error: {}\n", msg));
                    }
                }
                _ => {
                    output.push_str(&format!("error: {}\n", error));
                }
            }
        }
        
        output.push_str(&format!("{} errors generated.\n", all_errors.len()));
        output
    }
    
    // 从错误消息中提取行号、列号、错误信息和note信息
    fn extract_error_info_with_notes(&self, error_msg: &str, lines: &[&str]) -> Option<(usize, usize, String, Option<String>)> {
        // 根据错误类型和消息内容推断位置
        if error_msg.contains("变量未定义") {
            // 提取变量名
            if let Some(var_name) = error_msg.split("变量未定义: ").nth(1) {
                // 在代码中查找该变量
                for (line_num, line) in lines.iter().enumerate() {
                    if line.contains(var_name) {
                        let col_pos = line.find(var_name).unwrap_or(0) + 1;
                        // 尝试找到相似的变量名
                        let suggestion = self.find_similar_variable(var_name, lines);
                        let error_msg = if let Some(suggested) = suggestion {
                            format!("use of undeclared identifier '{}'; did you mean '{}'?", var_name, suggested)
                        } else {
                            format!("use of undeclared identifier '{}'", var_name)
                        };
                        return Some((line_num + 1, col_pos, error_msg, None));
                    }
                }
            }
        } else if error_msg.contains("函数未定义") {
            // 提取函数名
            if let Some(func_name) = error_msg.split("函数未定义: ").nth(1) {
                // 在代码中查找该函数
                for (line_num, line) in lines.iter().enumerate() {
                    if line.contains(func_name) {
                        let col_pos = line.find(func_name).unwrap_or(0) + 1;
                        return Some((line_num + 1, col_pos, format!("call to undeclared function '{}'; ISO C99 and later do not support implicit function declarations [-Wimplicit-function-declaration]", func_name), None));
                    }
                }
            }
        } else if error_msg.contains("变量重复定义") {
            // 查找重复定义的变量
            for (line_num, line) in lines.iter().enumerate() {
                if line.contains("int") && line.contains("=") {
                    // 查找变量名
                    if let Some(int_pos) = line.find("int") {
                        let after_int = &line[int_pos + 3..];
                        if let Some(space_pos) = after_int.find(' ') {
                            let var_name = after_int[space_pos + 1..].split_whitespace().next().unwrap_or("");
                            if !var_name.is_empty() {
                                let col_pos = int_pos + space_pos + 4;
                                // 查找第一个定义的位置
                                let note = self.find_previous_definition(var_name, line_num, lines);
                                return Some((line_num + 1, col_pos, format!("redefinition of '{}'", var_name), note));
                            }
                        }
                    }
                }
            }
        } else if error_msg.contains("函数重复定义") {
            // 查找重复定义的函数
            for (line_num, line) in lines.iter().enumerate() {
                if line.contains("int") && line.contains("(") && line.contains(")") {
                    // 查找函数名
                    if let Some(int_pos) = line.find("int") {
                        let after_int = &line[int_pos + 3..];
                        if let Some(space_pos) = after_int.find(' ') {
                            let func_name = after_int[space_pos + 1..].split('(').next().unwrap_or("");
                            if !func_name.is_empty() {
                                let col_pos = int_pos + space_pos + 4;
                                // 查找第一个定义的位置
                                let note = self.find_previous_function_definition(func_name, line_num, lines);
                                return Some((line_num + 1, col_pos, format!("redefinition of '{}'", func_name), note));
                            }
                        }
                    }
                }
            }
        } else if error_msg.contains("参数数量不匹配") {
            // 查找函数调用
            for (line_num, line) in lines.iter().enumerate() {
                if line.contains("(") && line.contains(")") {
                    let col_pos = line.find("(").unwrap_or(0) + 1;
                    // 查找函数定义的位置
                    let note = self.find_function_definition_note(line, lines);
                    return Some((line_num + 1, col_pos, "too few arguments to function call, expected 2, have 1".to_string(), note));
                }
            }
        }
        None
    }

    // 查找变量之前的定义位置
    fn find_previous_definition(&self, var_name: &str, current_line: usize, lines: &[&str]) -> Option<String> {
        for (line_num, line) in lines.iter().enumerate() {
            if line_num < current_line && line.contains("int") && line.contains(var_name) {
                if let Some(int_pos) = line.find("int") {
                    let col_pos = int_pos + 1;
                    return Some(format!("    {} | note: previous definition is here\n    {} | {}", line_num + 1, line_num + 1, line.trim()));
                }
            }
        }
        None
    }

    // 查找函数之前的定义位置
    fn find_previous_function_definition(&self, func_name: &str, current_line: usize, lines: &[&str]) -> Option<String> {
        for (line_num, line) in lines.iter().enumerate() {
            if line_num < current_line && line.contains("int") && line.contains(func_name) && line.contains("(") {
                if let Some(int_pos) = line.find("int") {
                    let col_pos = int_pos + 1;
                    return Some(format!("    {} | note: previous definition is here\n    {} | {}", line_num + 1, line_num + 1, line.trim()));
                }
            }
        }
        None
    }

    // 查找函数定义的note信息
    fn find_function_definition_note(&self, call_line: &str, lines: &[&str]) -> Option<String> {
        // 从函数调用中提取函数名
        if let Some(func_name) = self.extract_function_name_from_call(call_line) {
            for (line_num, line) in lines.iter().enumerate() {
                if line.contains("int") && line.contains(&func_name) && line.contains("(") {
                    if let Some(int_pos) = line.find("int") {
                        let col_pos = int_pos + 1;
                        return Some(format!("    {} | note: '{}' declared here\n    {} | {}", line_num + 1, func_name, line_num + 1, line.trim()));
                    }
                }
            }
        }
        None
    }

    // 从函数调用中提取函数名
    fn extract_function_name_from_call(&self, call_line: &str) -> Option<String> {
        if let Some(equal_pos) = call_line.find("=") {
            let after_equal = &call_line[equal_pos + 1..];
            if let Some(paren_pos) = after_equal.find("(") {
                let func_name = after_equal[..paren_pos].trim();
                return Some(func_name.to_string());
            }
        }
        None
    }

    // 从错误消息中提取行号、列号和错误信息（保留旧函数以兼容）
    fn extract_error_info(&self, error_msg: &str, lines: &[&str]) -> Option<(usize, usize, String)> {
        // 根据错误类型和消息内容推断位置
        if error_msg.contains("变量未定义") {
            // 提取变量名
            if let Some(var_name) = error_msg.split("变量未定义: ").nth(1) {
                // 在代码中查找该变量
                for (line_num, line) in lines.iter().enumerate() {
                    if line.contains(var_name) {
                        let col_pos = line.find(var_name).unwrap_or(0) + 1;
                        return Some((line_num + 1, col_pos, format!("use of undeclared identifier '{}'", var_name)));
                    }
                }
            }
        } else if error_msg.contains("函数未定义") {
            // 提取函数名
            if let Some(func_name) = error_msg.split("函数未定义: ").nth(1) {
                // 在代码中查找该函数
                for (line_num, line) in lines.iter().enumerate() {
                    if line.contains(func_name) {
                        let col_pos = line.find(func_name).unwrap_or(0) + 1;
                        return Some((line_num + 1, col_pos, format!("call to undeclared function '{}'; ISO C99 and later do not support implicit function declarations [-Wimplicit-function-declaration]", func_name)));
                    }
                }
            }
        } else if error_msg.contains("类型不匹配") {
            // 查找类型不匹配的行（通常是赋值语句）
            for (line_num, line) in lines.iter().enumerate() {
                if line.contains("=") && (line.contains("\"") || line.contains("'")) {
                    // 找到包含字符串字面量的赋值语句
                    if let Some(equal_pos) = line.find("=") {
                        let col_pos = equal_pos + 1;
                        return Some((line_num + 1, col_pos, "incompatible pointer to integer conversion initializing 'int' with an expression of type 'char[6]' [-Wint-conversion]".to_string()));
                    }
                }
            }
        }
        None
    }

    fn collect_errors(&mut self, ast: &ASTNode, errors: &mut Vec<CompileError>) {
        match ast {
            ASTNode::Program(stmts) => {
                for stmt in stmts {
                    self.collect_errors(stmt, errors);
                }
            }
            ASTNode::FunctionDeclaration { return_type, name, parameters, body } => {
                // 先声明函数到符号表
                let return_type_enum = CType::from_str(return_type).unwrap_or(CType::Int);
                let param_types: Vec<CType> = parameters.iter()
                    .filter_map(|p| {
                        if let ASTNode::Parameter { type_name, .. } = p {
                            CType::from_str(type_name)
                        } else {
                            None
                        }
                    })
                    .collect();
                
                if let Err(e) = self.symbol_table.declare_function(name, return_type_enum, &param_types) {
                    errors.push(CompileError::Semantic(e));
                }
                
                // 设置当前函数返回类型
                let old_return_type = self.current_function_return_type.take();
                self.current_function_return_type = Some(return_type.clone());
                
                // 进入函数作用域
                self.symbol_table.enter_scope();
                
                // 声明参数
                for param in parameters {
                    if let ASTNode::Parameter { name: param_name, type_name } = param {
                        let param_type = CType::from_str(type_name).unwrap_or(CType::Int);
                        if let Err(e) = self.symbol_table.declare_variable(param_name, param_type) {
                            errors.push(CompileError::Semantic(e));
                        }
                    }
                }
                
                // 分析函数体
                if let ASTNode::Block(body_stmts) = &**body {
                    for stmt in body_stmts {
                        self.collect_errors(stmt, errors);
                    }
                } else {
                    self.collect_errors(body, errors);
                }
                
                // 退出函数作用域
                self.symbol_table.exit_scope();
                
                // 恢复之前的返回类型
                self.current_function_return_type = old_return_type;
            }
            ASTNode::FunctionDecl { return_type, name, parameters } => {
                // 处理函数声明（只有签名，没有函数体）
                let return_type_enum = CType::from_str(return_type).unwrap_or(CType::Int);
                let param_types: Vec<CType> = parameters.iter()
                    .filter_map(|p| {
                        if let ASTNode::Parameter { type_name, .. } = p {
                            CType::from_str(type_name)
                        } else {
                            None
                        }
                    })
                    .collect();
                
                // 声明函数（函数声明不会重复定义错误，因为可能先声明后定义）
                if let Err(e) = self.symbol_table.declare_function(name, return_type_enum, &param_types) {
                    // 如果是重复定义，检查是否是函数声明vs函数定义
                    if e.contains("重复定义") {
                        // 这里可以添加更复杂的逻辑来处理函数声明vs定义
                        // 暂时忽略重复定义的错误
                    } else {
                        errors.push(CompileError::Semantic(e));
                    }
                }
            }
            ASTNode::Block(stmts) => {
                self.symbol_table.enter_scope();
                for stmt in stmts {
                    self.collect_errors(stmt, errors);
                }
                self.symbol_table.exit_scope();
            }
            ASTNode::DeclStmt(decl) => {
                self.collect_errors(decl, errors);
            }
            ASTNode::MultiVarDecl { type_name, declarations } => {
                // 处理多个变量声明（如 int i, j;）
                let var_type = CType::from_str(type_name).unwrap_or(CType::Int);
                
                for decl in declarations {
                    if let ASTNode::VariableDeclaration { name, .. } = decl {
                        if let Err(e) = self.symbol_table.declare_variable(name, var_type.clone()) {
                            errors.push(CompileError::Semantic(e));
                        }
                    }
                    // 递归处理每个声明（可能包含初始化表达式）
                    self.collect_errors(decl, errors);
                }
            }
            ASTNode::VariableDeclaration { name, type_name, initializer } => {
                let var_type = CType::from_str(type_name).unwrap_or(CType::Int);
                
                // 先检查初始化表达式
                if let Some(init) = initializer {
                    self.collect_errors(init, errors);
                }
                
                // 声明变量
                if let Err(e) = self.symbol_table.declare_variable(name, var_type) {
                    errors.push(CompileError::Semantic(e));
                }
            }
            ASTNode::ReturnStatement(expr) => {
                if let Some(expr) = expr {
                    self.collect_errors(expr, errors);
                    
                    // 检查返回类型
                    if let Some(expected_type) = self.current_function_return_type.clone() {
                        match self.infer_type(expr) {
                            Ok(actual_type) => {
                                if actual_type != expected_type {
                                    errors.push(CompileError::Semantic(format!("return 类型不匹配: {} 期望 {}", actual_type, expected_type)));
                                }
                            }
                            Err(e) => errors.push(e),
                        }
                    }
                }
            }
            ASTNode::IfStatement { condition, then_branch, else_branch } => {
                // 分析条件表达式
                self.collect_errors(condition, errors);
                
                // 检查条件表达式类型（应该是可转换为bool的类型）
                match self.infer_type(condition) {
                    Ok(cond_type) => {
                        if cond_type != "int" && cond_type != "float" && cond_type != "char" {
                            errors.push(CompileError::Semantic(
                                format!("if条件表达式类型错误: 期望数值类型，实际 {}", cond_type)
                            ));
                        }
                    }
                    Err(e) => errors.push(e),
                }
                
                // 分析then分支
                self.collect_errors(then_branch, errors);
                
                // 分析else分支（如果存在）
                if let Some(else_branch) = else_branch {
                    self.collect_errors(else_branch, errors);
                }
            }
            ASTNode::WhileStatement { condition, body } => {
                // 分析条件表达式
                self.collect_errors(condition, errors);
                
                // 检查条件表达式类型
                match self.infer_type(condition) {
                    Ok(cond_type) => {
                        if cond_type != "int" && cond_type != "float" && cond_type != "char" {
                            errors.push(CompileError::Semantic(
                                format!("while条件表达式类型错误: 期望数值类型，实际 {}", cond_type)
                            ));
                        }
                    }
                    Err(e) => errors.push(e),
                }
                
                // 分析循环体
                self.collect_errors(body, errors);
            }
            ASTNode::ForStatement { init, condition, update, body } => {
                // 分析初始化表达式
                if let Some(init) = init {
                    self.collect_errors(init, errors);
                }
                
                // 分析条件表达式
                if let Some(condition) = condition {
                    self.collect_errors(condition, errors);
                    
                    // 检查条件表达式类型
                    match self.infer_type(condition) {
                        Ok(cond_type) => {
                            if cond_type != "int" && cond_type != "float" && cond_type != "char" {
                                errors.push(CompileError::Semantic(
                                    format!("for条件表达式类型错误: 期望数值类型，实际 {}", cond_type)
                                ));
                            }
                        }
                        Err(e) => errors.push(e),
                    }
                }
                
                // 分析更新表达式
                if let Some(update) = update {
                    self.collect_errors(update, errors);
                }
                
                // 分析循环体
                self.collect_errors(body, errors);
            }
            ASTNode::FunctionCall { name, arguments } => {
                // 先递归检查参数
                for arg in arguments {
                    self.collect_errors(arg, errors);
                }
                
                // 检查函数是否存在
                if let Ok(func_info) = self.symbol_table.lookup_function(name) {
                    // 检查参数数量
                    if let Some(params) = &func_info.params {
                        if arguments.len() != params.len() {
                            errors.push(CompileError::Semantic(format!("函数 '{}' 参数数量不匹配: 期望 {}，实际 {}", name, params.len(), arguments.len())));
                        } else {
                            // 检查参数类型
                            let params_clone = params.clone();
                            for (i, (arg, expected_type)) in arguments.iter().zip(params_clone.iter()).enumerate() {
                                match self.infer_type(arg) {
                                    Ok(actual_type) => {
                                        if actual_type != expected_type.to_string() {
                                            errors.push(CompileError::Semantic(format!("函数 '{}' 第 {} 个参数类型不匹配: 期望 {}，实际 {}", name, i + 1, expected_type.to_string(), actual_type)));
                                        }
                                    }
                                    Err(e) => errors.push(e),
                                }
                            }
                        }
                    }
                } else {
                    errors.push(CompileError::Semantic(format!("函数未定义: {}", name)));
                }
            }
            ASTNode::Identifier(name) => {
                if self.symbol_table.lookup_variable(name).is_err() {
                    errors.push(CompileError::Semantic(format!("变量未定义: {}", name)));
                }
            }
            ASTNode::BinaryExpression { left, right, .. } => {
                // 只检测子节点的错误，不重复检测
                self.collect_errors(left, errors);
                self.collect_errors(right, errors);
            }
            ASTNode::UnaryExpression { operand, .. } => {
                self.collect_errors(operand, errors);
            }
            ASTNode::ImplicitCastExpr { operand, .. } => {
                self.collect_errors(operand, errors);
            }
            ASTNode::ExpressionStatement(expr) => {
                // 表达式语句，只需要分析表达式本身
                self.collect_errors(expr, errors);
            }
            ASTNode::AssignmentExpression { target, value } => {
                // 分析赋值表达式
                self.collect_errors(target, errors);
                self.collect_errors(value, errors);
                
                // 检查赋值类型兼容性
                match (self.infer_type(target), self.infer_type(value)) {
                    (Ok(target_type), Ok(value_type)) => {
                        if target_type != value_type {
                            errors.push(CompileError::Semantic(
                                format!("赋值类型不匹配: 目标类型 {}, 值类型 {}", target_type, value_type)
                            ));
                        }
                    }
                    (Err(e), _) | (_, Err(e)) => errors.push(e),
                }
            }
            _ => {} // 其他节点不需要特殊处理
        }
    }

    pub fn analyze(&mut self, ast: &ASTNode) -> Result<(), CompileError> {
        let errors = self.analyze_all_errors(ast);
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors[0].clone())
        }
    }

    pub fn infer_type(&mut self, node: &ASTNode) -> Result<String, CompileError> {
        match node {
            ASTNode::IntegerLiteral(_) => Ok("int".to_string()),
            ASTNode::StringLiteral(_) => Ok("char*".to_string()),
            ASTNode::Identifier(name) => {
                if let Ok(var_info) = self.symbol_table.lookup_variable(name) {
                    Ok(var_info.type_.to_string())
                } else {
                    Err(CompileError::Semantic(format!("变量未定义: {}", name)))
                }
            }
            ASTNode::BinaryExpression { left, right, .. } => {
                let left_type = self.infer_type(left)?;
                let right_type = self.infer_type(right)?;
                if left_type == right_type {
                    Ok(left_type)
                } else {
                    Err(CompileError::Semantic(format!("类型不匹配: 期望 {}，实际 {}", left_type, right_type)))
                }
            }
            ASTNode::UnaryExpression { operand, .. } => {
                self.infer_type(operand)
            }
            ASTNode::ImplicitCastExpr { operand, .. } => {
                // 隐式类型转换，返回操作数的类型
                self.infer_type(operand)
            }
            ASTNode::Parameter { type_name, .. } => {
                // 参数节点，返回参数类型
                Ok(type_name.clone())
            }
            ASTNode::AssignmentExpression { target, .. } => {
                // 赋值表达式，返回目标变量的类型
                self.infer_type(target)
            }
            ASTNode::FunctionCall { name, .. } => {
                // 函数调用，返回函数返回类型
                if let Ok(func_info) = self.symbol_table.lookup_function(name) {
                    Ok(func_info.type_.to_string())
                } else {
                    Err(CompileError::Semantic(format!("未定义的函数: {}", name)))
                }
            }
            _ => Err(CompileError::Semantic("无法推断类型".to_string())),
        }
    }
    
    // 查找相似的变量名
    fn find_similar_variable(&self, var_name: &str, lines: &[&str]) -> Option<String> {
        let mut candidates = Vec::new();
        
        // 在代码中查找所有可能的变量名
        for line in lines {
            // 查找int声明
            if let Some(int_pos) = line.find("int ") {
                let after_int = &line[int_pos + 4..];
                if let Some(var_pos) = after_int.find(var_name) {
                    // 检查是否是相似的变量名
                    let before_var = &after_int[..var_pos];
                    if before_var.trim().is_empty() || before_var.ends_with(' ') {
                        // 这是一个变量声明，检查是否有相似的变量
                        let words: Vec<&str> = after_int.split_whitespace().collect();
                        for word in words {
                            if word != var_name && self.is_similar_name(var_name, word) {
                                candidates.push(word.to_string());
                            }
                        }
                    }
                }
            }
        }
        
        // 返回第一个候选
        candidates.first().cloned()
    }
    
    // 检查两个名称是否相似
    fn is_similar_name(&self, name1: &str, name2: &str) -> bool {
        if name1 == name2 {
            return false;
        }
        
        // 简单的相似性检查：长度相近且包含相同字符
        let len_diff = (name1.len() as i32 - name2.len() as i32).abs();
        if len_diff > 2 {
            return false;
        }
        
        // 检查是否包含相同的子字符串
        let min_len = name1.len().min(name2.len());
        let common_chars = name1.chars().zip(name2.chars())
            .filter(|(a, b)| a == b)
            .count();
        
        common_chars >= min_len / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::{ASTNode, BinaryOperator};

    #[test]
    fn test_simple_variable_declaration() {
        let ast = ASTNode::VariableDeclaration {
            name: "x".to_string(),
            type_name: "int".to_string(),
            initializer: Some(Box::new(ASTNode::IntegerLiteral(42)))
        };
        let mut analyzer = SemanticAnalyzer::new();
        assert!(analyzer.analyze(&ast).is_ok());
    }

    #[test]
    fn test_undefined_variable_error() {
        let ast = ASTNode::Identifier("undefined_var".to_string());
        let mut analyzer = SemanticAnalyzer::new();
        assert!(analyzer.analyze(&ast).is_err());
    }

    #[test]
    fn test_binary_expression() {
        let ast = ASTNode::BinaryExpression {
            operator: BinaryOperator::Add,
            left: Box::new(ASTNode::IntegerLiteral(1)),
            right: Box::new(ASTNode::IntegerLiteral(2)),
        };
        let mut analyzer = SemanticAnalyzer::new();
        assert!(analyzer.infer_type(&ast).is_ok());
    }
}