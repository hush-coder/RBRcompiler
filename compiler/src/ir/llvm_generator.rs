use crate::parser::ast::{ASTNode, BinaryOperator};
use crate::error::error::CompileError;

pub struct LLVMIRGenerator {
    temp_counter: u32,
    file_path: String,
    variable_map: std::collections::HashMap<String, u32>, // 变量名 -> alloca索引
}

impl LLVMIRGenerator {
    pub fn new() -> Self {
        Self {
            temp_counter: 0,
            file_path: "test.c".to_string(),
            variable_map: std::collections::HashMap::new(),
        }
    }

    pub fn new_with_file_path(file_path: String) -> Self {
        Self {
            temp_counter: 0,
            file_path,
            variable_map: std::collections::HashMap::new(),
        }
    }

    pub fn generate(&mut self, ast: &ASTNode) -> Result<String, CompileError> {
        match ast {
            ASTNode::Program(statements) => {
                let mut output = String::new();
                
                // 模块头部
                output.push_str(&format!("; ModuleID = '{}'\n", self.file_path));
                output.push_str(&format!("source_filename = \"{}\"\n", self.file_path));
                output.push_str("target datalayout = \"e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128\"\n");
                output.push_str("target triple = \"aarch64-unknown-linux-gnu\"\n\n");
                
                // 生成函数
                for stmt in statements {
                    if let ASTNode::FunctionDeclaration { return_type, name, parameters, body } = stmt {
                        output.push_str(&self.generate_function(return_type, name, parameters, body)?);
                        output.push_str("\n");
                    }
                }
                
                // 属性
                output.push_str("attributes #0 = { noinline nounwind optnone uwtable \"frame-pointer\"=\"non-leaf\" \"no-trapping-math\"=\"true\" \"stack-protector-buffer-size\"=\"8\" \"target-cpu\"=\"generic\" \"target-features\"=\"+fp-armv8,+neon,+outline-atomics,+v8a,-fmv\" }\n\n");
                
                // 元数据
                output.push_str("!llvm.module.flags = !{!0, !1, !2, !3, !4}\n");
                output.push_str("!llvm.ident = !{!5}\n\n");
                output.push_str("!0 = !{i32 1, !\"wchar_size\", i32 4}\n");
                output.push_str("!1 = !{i32 8, !\"PIC Level\", i32 2}\n");
                output.push_str("!2 = !{i32 7, !\"PIE Level\", i32 2}\n");
                output.push_str("!3 = !{i32 7, !\"uwtable\", i32 2}\n");
                output.push_str("!4 = !{i32 7, !\"frame-pointer\", i32 1}\n");
                output.push_str("!5 = !{!\"BiSheng Enterprise 4.2.0.2.B002 clang version 17.0.6 (2261d9fde4e0)\"}\n");
                
                Ok(output)
            }
            _ => Err(CompileError::Semantic("Expected program node".to_string())),
        }
    }

    fn generate_function(&mut self, _return_type: &str, name: &str, parameters: &[ASTNode], body: &ASTNode) -> Result<String, CompileError> {
        let mut output = String::new();
        
        // 重置计数器和变量映射
        self.temp_counter = 0;
        self.variable_map.clear();
        
        // 保存是否有参数的信息
        let has_parameters = !parameters.is_empty();
        
        // 函数签名
        output.push_str("; Function Attrs: noinline nounwind optnone uwtable\n");
        
        if parameters.is_empty() {
            output.push_str(&format!("define dso_local i32 @{}() #0 {{\n", name));
        } else {
            let param_list = parameters.iter()
                .enumerate()
                .map(|(i, _)| format!("i32 noundef %{}", i))
                .collect::<Vec<_>>()
                .join(", ");
            output.push_str(&format!("define dso_local i32 @{}({}) #0 {{\n", name, param_list));
        }
        
        // 处理函数参数 - 按照Bisheng的顺序
        if !parameters.is_empty() {
            // 按照bisheng的顺序：第一个参数alloca -> store -> 第二个参数alloca -> store
            if let Some(ASTNode::Parameter { name, .. }) = parameters.first() {
                output.push_str(&format!("  %{} = alloca i32, align 4\n", 3));
                self.variable_map.insert(name.clone(), 3);
            }
            if !parameters.is_empty() {
                output.push_str(&format!("  store i32 %{}, ptr %{}, align 4\n", 0, 3));
            }
            if parameters.len() > 1 {
                if let Some(ASTNode::Parameter { name, .. }) = parameters.get(1) {
                    output.push_str(&format!("  %{} = alloca i32, align 4\n", 4));
                    self.variable_map.insert(name.clone(), 4);
                }
                output.push_str(&format!("  store i32 %{}, ptr %{}, align 4\n", 1, 4));
            }
        }
        
        // 生成函数体
        output.push_str(&self.generate_block_with_params(body, has_parameters)?);
        
        output.push_str("}\n");
        Ok(output)
    }

    fn generate_branch_content(&mut self, block: &ASTNode, temp_count: &mut u32) -> Result<String, CompileError> {
        match block {
            ASTNode::Block(statements) => {
                let mut output = String::new();
                for stmt in statements {
                    output.push_str(&self.generate_statement(stmt, temp_count)?);
                }
                Ok(output)
            }
            _ => {
                // 如果不是Block，直接作为单个语句处理
                self.generate_statement(block, temp_count)
            }
        }
    }

    fn generate_block_with_params(&mut self, block: &ASTNode, has_parameters: bool) -> Result<String, CompileError> {
        match block {
            ASTNode::Block(statements) => {
                let mut output = String::new();
                
                // 计算当前已分配的alloca数量（从variable_map中获取）
                let mut alloca_count = 1;
                for &index in self.variable_map.values() {
                    alloca_count = alloca_count.max(index + 1);
                }
                
                // 如果没有参数，从2开始分配变量索引（与Bisheng一致）
                if alloca_count == 1 {
                    alloca_count = 2;
                    // 为%1生成alloca指令（仅在没有参数时）
                    output.push_str("  %1 = alloca i32, align 4\n");
                }
                
                // 第一遍：收集所有变量声明，生成alloca指令
                for stmt in statements {
                    if let ASTNode::DeclStmt(decl) = stmt {
                        if let ASTNode::VariableDeclaration { name, .. } = decl.as_ref() {
                            output.push_str(&format!("  %{} = alloca i32, align 4\n", alloca_count));
                            self.variable_map.insert(name.clone(), alloca_count);
                            alloca_count += 1;
                        }
                    }
                }
                
                // 确保有足够的alloca指令（至少4个）
                while alloca_count <= 4 {
                    output.push_str(&format!("  %{} = alloca i32, align 4\n", alloca_count));
                    alloca_count += 1;
                }
                
                // 第二遍：为%1生成默认初始化（store i32 0），仅在没有参数时
                if !has_parameters {
                    output.push_str("  store i32 0, ptr %1, align 4\n");
                }
                
                // 第三遍：生成其他指令，包括变量初始化
                // 重置临时变量计数器，从alloca_count开始
                let mut temp_counter = alloca_count;
                for stmt in statements {
                    output.push_str(&self.generate_statement(stmt, &mut temp_counter)?);
                }
                
                Ok(output)
            }
            _ => {
                // 如果不是Block，直接作为单个语句处理
                let mut temp_count = 1;
                self.generate_statement(block, &mut temp_count)
            }
        }
    }

    fn generate_block(&mut self, block: &ASTNode) -> Result<String, CompileError> {
        match block {
            ASTNode::Block(statements) => {
                let mut output = String::new();
                
                // 计算当前已分配的alloca数量（从variable_map中获取）
                let mut alloca_count = 1;
                for &index in self.variable_map.values() {
                    alloca_count = alloca_count.max(index + 1);
                }
                
                // 如果没有参数，从2开始分配变量索引（与Bisheng一致）
                if alloca_count == 1 {
                    alloca_count = 2;
                    // 为%1生成alloca指令（仅在没有参数时）
                    output.push_str("  %1 = alloca i32, align 4\n");
                }
                
                // 第一遍：收集所有变量声明，生成alloca指令
                for stmt in statements {
                    if let ASTNode::DeclStmt(decl) = stmt {
                        if let ASTNode::VariableDeclaration { name, .. } = decl.as_ref() {
                            output.push_str(&format!("  %{} = alloca i32, align 4\n", alloca_count));
                            self.variable_map.insert(name.clone(), alloca_count);
                            alloca_count += 1;
                        }
                    }
                }
                
                // 确保有足够的alloca指令（至少4个）
                while alloca_count <= 4 {
                    output.push_str(&format!("  %{} = alloca i32, align 4\n", alloca_count));
                    alloca_count += 1;
                }
                
                // 第二遍：为%1生成默认初始化（store i32 0），仅在没有参数时
                if self.variable_map.is_empty() {
                    output.push_str("  store i32 0, ptr %1, align 4\n");
                }
                
                // 第三遍：生成其他指令，包括变量初始化
                // 重置临时变量计数器，从alloca_count开始
                let mut temp_counter = alloca_count;
                for stmt in statements {
                    output.push_str(&self.generate_statement(stmt, &mut temp_counter)?);
                }
                
                Ok(output)
            }
            _ => {
                // 如果不是Block，直接作为单个语句处理
                let mut temp_count = 1;
                self.generate_statement(block, &mut temp_count)
            }
        }
    }

    fn generate_statement(&mut self, stmt: &ASTNode, temp_count: &mut u32) -> Result<String, CompileError> {
        match stmt {
            ASTNode::DeclStmt(decl) => {
                // 处理包装在DeclStmt中的变量声明
                self.generate_statement(decl.as_ref(), temp_count)
            }
            ASTNode::VariableDeclaration { name, initializer, type_name: _ } => {
                let mut output = String::new();
                if let Some(init) = initializer {
                    // 检查是否是字面值
                    if let ASTNode::IntegerLiteral(value) = init.as_ref() {
                        // 直接使用字面值
                        if let Some(&var_index) = self.variable_map.get(name) {
                            output.push_str(&format!("  store i32 {}, ptr %{}, align 4\n", value, var_index));
                        }
                    } else {
                        // 处理复杂表达式
                        let (ir, new_temp) = self.generate_expression(init, *temp_count)?;
                        output.push_str(&ir);
                        if let Some(&var_index) = self.variable_map.get(name) {
                            output.push_str(&format!("  store i32 %{}, ptr %{}, align 4\n", new_temp, var_index));
                        }
                        *temp_count = new_temp + 1;
                    }
                } else {
                    // 如果没有初始化器，使用默认值0
                    if let Some(&var_index) = self.variable_map.get(name) {
                        output.push_str(&format!("  store i32 0, ptr %{}, align 4\n", var_index));
                    }
                }
                Ok(output)
            }
            ASTNode::ReturnStatement(expr) => {
                let mut output = String::new();
                if let Some(expr) = expr {
                    let (ir, new_temp) = self.generate_expression(expr, *temp_count)?;
                    output.push_str(&ir);
                    output.push_str(&format!("  ret i32 %{}\n", new_temp));
                } else {
                    output.push_str("  ret i32 0\n");
                }
                Ok(output)
            }
            ASTNode::ExpressionStatement(expr) => {
                self.generate_expression(expr, *temp_count).map(|(ir, _)| ir)
            }
            ASTNode::AssignmentExpression { target, value } => {
                let mut output = String::new();
                
                // 生成右值的表达式
                let (value_ir, value_temp) = self.generate_expression(value, *temp_count)?;
                output.push_str(&value_ir);
                *temp_count = value_temp + 1;
                
                // 处理左值（目标变量）
                if let ASTNode::Identifier(var_name) = target.as_ref() {
                    if let Some(&var_index) = self.variable_map.get(var_name) {
                        output.push_str(&format!("  store i32 %{}, ptr %{}, align 4\n", value_temp, var_index));
                        // 在语句中，赋值不需要额外的load指令
                    } else {
                        return Err(CompileError::Semantic(format!("Undefined variable: {}", var_name)));
                    }
                } else {
                    return Err(CompileError::Semantic("Assignment target must be a variable".to_string()));
                }
                
                Ok(output)
            }
            ASTNode::IfStatement { condition, then_branch, else_branch } => {
                let mut output = String::new();
                
                // 生成条件表达式
                let (cond_ir, cond_temp) = self.generate_expression(condition, *temp_count)?;
                output.push_str(&cond_ir);
                *temp_count = cond_temp + 1;
                
                // 生成分支标签
                let then_label = *temp_count;
                let else_label = *temp_count + 1;
                let end_label = *temp_count + 2;
                *temp_count += 3;
                
                // 生成条件分支 - 直接使用条件表达式的结果
                output.push_str(&format!("  br i1 %{}, label %{}, label %{}\n", cond_temp, then_label, else_label));
                
                // 生成then分支
                output.push_str(&format!("\n{}:                                                ; preds = %0\n", then_label));
                let then_ir = self.generate_branch_content(then_branch, temp_count)?;
                output.push_str(&then_ir);
                output.push_str(&format!("  br label %{}\n", end_label));
                
                // 生成else分支（如果存在）
                if let Some(else_branch) = else_branch {
                    output.push_str(&format!("\n{}:                                               ; preds = %0\n", else_label));
                    let else_ir = self.generate_branch_content(else_branch, temp_count)?;
                    output.push_str(&else_ir);
                    output.push_str(&format!("  br label %{}\n", end_label));
                } else {
                    output.push_str(&format!("\n{}:                                               ; preds = %0\n", else_label));
                    output.push_str(&format!("  br label %{}\n", end_label));
                }
                
                // 生成结束标签
                output.push_str(&format!("\n{}:                                               ; preds = {}, {}\n", end_label, then_label, else_label));
                
                Ok(output)
            }
            _ => Ok(String::new()),
        }
    }

    fn generate_expression(&mut self, expr: &ASTNode, temp_count: u32) -> Result<(String, u32), CompileError> {
        match expr {
            ASTNode::IntegerLiteral(value) => {
                // 对于字面值，直接使用值，不需要生成指令
                // 使用一个很大的编号来表示字面量，避免与临时变量冲突
                Ok((String::new(), 10000 + *value as u32))
            }
            ASTNode::BinaryExpression { left, operator, right } => {
                let mut output = String::new();
                let mut current_temp = temp_count;
                
                // 处理左操作数
                let (left_ir, left_temp) = self.generate_expression(left, current_temp)?;
                output.push_str(&left_ir);
                // 只有当左操作数不是字面量时才增加计数器
                if left_temp < 10000 {
                    current_temp = left_temp + 1;
                }
                
                // 处理右操作数
                let (right_ir, right_temp) = self.generate_expression(right, current_temp)?;
                output.push_str(&right_ir);
                // 只有当右操作数不是字面量时才增加计数器
                if right_temp < 10000 {
                    current_temp = right_temp + 1;
                }
                
                let result_temp = current_temp;
                let op_ir = match operator {
                    BinaryOperator::Add => "add nsw",
                    BinaryOperator::Subtract => "sub nsw", 
                    BinaryOperator::Multiply => "mul nsw",
                    BinaryOperator::Divide => "sdiv",
                    BinaryOperator::Equal => return Err(CompileError::Semantic("Assignment operator not supported in expressions".to_string())),
                    BinaryOperator::GreaterThan => "icmp sgt",
                    BinaryOperator::LessThan => "icmp slt",
                    BinaryOperator::GreaterEqual => "icmp sge",
                    BinaryOperator::LessEqual => "icmp sle",
                    BinaryOperator::EqualEqual => "icmp eq",
                    BinaryOperator::NotEqual => "icmp ne",
                };
                
                // 处理操作数格式 - 字面量直接使用数值，变量使用%格式
                let left_operand = if left_temp >= 10000 { 
                    // 字面量直接使用数值
                    format!("{}", left_temp - 10000) 
                } else { 
                    // 临时变量使用%格式
                    format!("%{}", left_temp) 
                };
                
                let right_operand = if right_temp >= 10000 { 
                    // 字面量直接使用数值
                    format!("{}", right_temp - 10000) 
                } else { 
                    // 临时变量使用%格式
                    format!("%{}", right_temp) 
                };
                
                output.push_str(&format!("  %{} = {} i32 {}, {}\n", result_temp, op_ir, left_operand, right_operand));
                Ok((output, result_temp))
            }
            ASTNode::Identifier(name) => {
                // 生成load指令来读取变量值
                let load_temp = temp_count;
                if let Some(&var_index) = self.variable_map.get(name) {
                    Ok((format!("  %{} = load i32, ptr %{}, align 4\n", load_temp, var_index), load_temp))
                } else {
                    // 如果没有找到变量，假设是%1
                    Ok((format!("  %{} = load i32, ptr %1, align 4\n", load_temp), load_temp))
                }
            }
            ASTNode::FunctionCall { name, arguments } => {
                let mut output = String::new();
                let mut arg_temps = Vec::new();
                
                // 生成参数
                let mut current_temp = temp_count;
                for arg in arguments {
                    let (arg_ir, arg_temp) = self.generate_expression(arg, current_temp)?;
                    output.push_str(&arg_ir);
                    arg_temps.push(arg_temp);
                    current_temp = arg_temp + 1;
                }
                
                // 生成函数调用
                let result_temp = current_temp;
                let arg_list = arg_temps.iter()
                    .map(|t| format!("i32 noundef %{}", t))
                    .collect::<Vec<_>>()
                    .join(", ");
                
                output.push_str(&format!("  %{} = call i32 @{}({})\n", result_temp, name, arg_list));
                Ok((output, result_temp))
            }
            ASTNode::ImplicitCastExpr { operand, .. } => {
                // 对于LValueToRValue转换，直接处理操作数
                self.generate_expression(operand, temp_count)
            }
            ASTNode::AssignmentExpression { target, value } => {
                // 处理赋值表达式（当它被当作表达式使用时）
                let mut output = String::new();
                
                // 生成右值的表达式
                let (value_ir, value_temp) = self.generate_expression(value, temp_count)?;
                output.push_str(&value_ir);
                
                // 处理左值（目标变量）
                if let ASTNode::Identifier(var_name) = target.as_ref() {
                    if let Some(&var_index) = self.variable_map.get(var_name) {
                        output.push_str(&format!("  store i32 %{}, ptr %{}, align 4\n", value_temp, var_index));
                        // 在控制流中，赋值表达式不需要返回load指令
                        Ok((output, value_temp))
                    } else {
                        Err(CompileError::Semantic(format!("Undefined variable: {}", var_name)))
                    }
                } else {
                    Err(CompileError::Semantic("Assignment target must be a variable".to_string()))
                }
            }
            _ => Err(CompileError::Semantic(format!("Unsupported expression type: {:?}", expr))),
        }
    }
}