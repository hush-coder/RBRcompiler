use crate::parser::ast::ASTNode;
use crate::semantic::types::CType;
use crate::ir::builder::IRBuilder;
use inkwell::values::{BasicValueEnum};
use inkwell::module::Module;
use inkwell::context::Context;

pub struct IRGenerator<'ctx> {
    builder: IRBuilder<'ctx>,
}

impl<'ctx> IRGenerator<'ctx> {
    pub fn new(context: &'ctx Context, name: &str) -> Self {
        Self {
            builder: IRBuilder::new(context, name),
        }
    }

    pub fn generate(&mut self, ast: &ASTNode) -> Result<Module<'ctx>, String> {
        match ast {
            ASTNode::Program(functions) => {
                for func in functions {
                    self.generate_function(func)?;
                }
                // 确保所有基本块都有终止指令，避免生成不合法的 LLVM 模块
                for function in self.builder.module.get_functions() {
                    for bb in function.get_basic_blocks() {
                        if bb.get_terminator().is_none() {
                            // 将插入点移动到该基本块末尾并添加隐式返回
                            self.builder.builder.position_at_end(bb);
                            let func_ret = function.get_type().get_return_type();
                            if func_ret.is_some() && func_ret.unwrap().is_int_type() {
                                let zero = self.builder.create_int_constant(0);
                                self.builder.build_return(Some(zero));
                            } else {
                                self.builder.build_return(None);
                            }
                        }
                    }
                }

                Ok(self.builder.module.clone())
            }
            _ => Err("Expected Program node".to_string()),
        }
    }

    fn generate_function(&mut self, func: &ASTNode) -> Result<(), String> {
        match func {
            ASTNode::FunctionDeclaration { name, parameters, return_type, body } => {
                // 创建函数
                let param_types: Vec<CType> = parameters.iter()
                    .map(|p| {
                        if let ASTNode::Parameter { type_name, .. } = p {
                            CType::from_str(type_name).unwrap_or(CType::Int)
                        } else {
                            CType::Int
                        }
                    })
                    .collect();
                
                let function = self.builder.create_function(name, &CType::from_str(return_type).unwrap_or(CType::Int), &param_types);
                
                // 创建入口块
                let _entry_block = self.builder.create_entry_block(function);
                
                // 处理参数 - 为每个参数创建alloca
                for (i, param) in parameters.iter().enumerate() {
                    if let ASTNode::Parameter { name, .. } = param {
                        let param_value = function.get_nth_param(i as u32).unwrap().into_int_value();
                        let alloca = self.builder.create_alloca(name);
                        self.builder.build_store(alloca, param_value);
                        self.builder.variables.insert(name.clone(), alloca.into());
                    }
                }
                
                // 生成函数体
                self.generate_block(body)?;
                
                // 检查当前基本块是否被终止
                // 这种情况通常发生在函数以一个非返回语句（如 if 语句）结束时
                if let Some(current_block) = self.builder.builder.get_insert_block() {
                    if current_block.get_terminator().is_none() {
                        // 如果当前块没有终止指令，添加一个隐式返回
                        let func_type = function.get_type().get_return_type();
                        
                        if func_type.is_some() && func_type.unwrap().is_int_type() {
                            // 如果返回类型是整数（i32），添加 return 0
                            let zero = self.builder.create_int_constant(0);
                            self.builder.build_return(Some(zero));
                        } else {
                            // 否则，添加无参数 return（适用于 void 函数）
                            self.builder.build_return(None);
                        }
                    }
                }
                
                Ok(())
            }
            _ => Err("Expected FunctionDeclaration node".to_string()),
        }
    }

    fn generate_block(&mut self, block: &ASTNode) -> Result<(), String> {
        match block {
            ASTNode::Block(statements) => {
                for stmt in statements {
                    self.generate_statement(stmt)?;
                }
                Ok(())
            }
            _ => Err("Expected Block node".to_string()),
        }
    }

    fn generate_statement(&mut self, stmt: &ASTNode) -> Result<(), String> {
        match stmt {
            ASTNode::DeclStmt(decl) => {
                self.generate_declaration(decl)?;
                Ok(())
            }
            ASTNode::ReturnStatement(expr) => {
                if let Some(expr) = expr {
                    let value = self.generate_expression(expr)?;
                    if let BasicValueEnum::IntValue(int_val) = value {
                        self.builder.build_return(Some(int_val));
                    } else {
                        return Err("Return value must be integer".to_string());
                    }
                } else {
                    self.builder.build_return(None);
                }
                Ok(())
            }
            ASTNode::ExpressionStatement(expr) => {
                self.generate_expression(expr)?;
                Ok(())
            }
            ASTNode::AssignmentExpression { target, value } => {
                // 对于赋值，我们需要获取变量的地址，而不是值
                let target_ptr = match &**target {
                    ASTNode::Identifier(name) => {
                        if let Some(value) = self.builder.variables.get(name) {
                            match value {
                                BasicValueEnum::PointerValue(ptr) => *ptr,
                                _ => return Err(format!("Variable {} is not a pointer", name)),
                            }
                        } else {
                            return Err(format!("Undefined variable: {}", name));
                        }
                    }
                    _ => return Err("Assignment target must be an identifier".to_string()),
                };
                
                let value_val = self.generate_expression(value)?;
                
                if let BasicValueEnum::IntValue(val) = value_val {
                    self.builder.build_store(target_ptr, val);
                } else {
                    return Err("Assignment value must be an integer".to_string());
                }
                Ok(())
            }
            ASTNode::IfStatement { condition, then_branch, else_branch } => {
                self.generate_if_statement(condition, then_branch, else_branch)?;
                Ok(())
            }
            _ => Err(format!("Unsupported statement: {:?}", stmt)),
        }
    }

    fn generate_declaration(&mut self, decl: &ASTNode) -> Result<(), String> {
        match decl {
            ASTNode::MultiVarDecl { declarations, .. } => {
                for decl in declarations {
                    if let ASTNode::VariableDeclaration { name, initializer, .. } = decl {
                        // 创建alloca
                        let alloca = self.builder.create_alloca(name);
                        self.builder.variables.insert(name.clone(), alloca.into());
                        
                        // 如果有初始化表达式
                        if let Some(init) = initializer {
                            let value = self.generate_expression(init)?;
                            if let BasicValueEnum::IntValue(int_val) = value {
                                self.builder.build_store(alloca, int_val);
                            }
                        }
                    }
                }
                Ok(())
            }
            ASTNode::VariableDeclaration { name, initializer, .. } => {
                // 创建alloca
                let alloca = self.builder.create_alloca(name);
                self.builder.variables.insert(name.clone(), alloca.into());
                
                // 如果有初始化表达式
                if let Some(init) = initializer {
                    let value = self.generate_expression(init)?;
                    if let BasicValueEnum::IntValue(int_val) = value {
                        self.builder.build_store(alloca, int_val);
                    }
                }
                Ok(())
            }
            _ => Err(format!("Unsupported declaration: {:?}", decl)),
        }
    }

    fn generate_expression(&mut self, expr: &ASTNode) -> Result<BasicValueEnum<'ctx>, String> {
        match expr {
            ASTNode::Identifier(name) => {
                if let Some(value) = self.builder.variables.get(name) {
                    match value {
                        BasicValueEnum::PointerValue(ptr) => {
                            // 变量是 alloca 的指针，需要加载其值
                            let int_val = self.builder.build_load(*ptr, name);
                            Ok(int_val.into())
                        }
                        BasicValueEnum::IntValue(int_val) => {
                            // 变量已经是整数值，直接返回
                            Ok(int_val.clone().into())
                        }
                        _ => Err(format!("Unsupported variable type for identifier: {}", name)),
                    }
                } else {
                    Err(format!("Undefined variable: {}", name))
                }
            }
            ASTNode::IntegerLiteral(value) => {
                Ok(self.builder.create_int_constant(*value).into())
            }
            ASTNode::BinaryExpression { left, operator, right } => {
                let left_val = self.generate_expression(left)?;
                let right_val = self.generate_expression(right)?;
                
                if let (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) = (left_val, right_val) {
                    match operator {
                        // 算术运算
                        crate::parser::ast::BinaryOperator::Add => {
                            Ok(self.builder.build_add(l, r).into())
                        }
                        crate::parser::ast::BinaryOperator::Subtract => {
                            Ok(self.builder.build_sub(l, r).into())
                        }
                        crate::parser::ast::BinaryOperator::Multiply => {
                            Ok(self.builder.build_mul(l, r).into())
                        }
                        crate::parser::ast::BinaryOperator::Divide => {
                            Ok(self.builder.build_div(l, r).into())
                        }
                        // 比较运算
                        crate::parser::ast::BinaryOperator::EqualEqual => {
                            Ok(self.builder.build_eq(l, r).into())
                        }
                        crate::parser::ast::BinaryOperator::NotEqual => {
                            Ok(self.builder.build_ne(l, r).into())
                        }
                        crate::parser::ast::BinaryOperator::LessThan => {
                            Ok(self.builder.build_lt(l, r).into())
                        }
                        crate::parser::ast::BinaryOperator::LessEqual => {
                            Ok(self.builder.build_le(l, r).into())
                        }
                        crate::parser::ast::BinaryOperator::GreaterThan => {
                            Ok(self.builder.build_gt(l, r).into())
                        }
                        crate::parser::ast::BinaryOperator::GreaterEqual => {
                            Ok(self.builder.build_ge(l, r).into())
                        }
                        // 逻辑运算 - 简化为位运算（暂时）
                        crate::parser::ast::BinaryOperator::LogicalAnd => {
                            // 对于 && 操作符，使用位运算 AND
                            Ok(self.builder.build_and(l, r).into())
                        }
                        crate::parser::ast::BinaryOperator::LogicalOr => {
                            // 对于 || 操作符，使用位运算 OR
                            Ok(self.builder.build_or(l, r).into())
                        }
                        _ => Err(format!("Unsupported binary operator: {:?}", operator)),
                    }
                } else {
                    Err("Binary expression operands must be integers".to_string())
                }
            }
            ASTNode::UnaryExpression { operand, operator } => {
                let operand_val = self.generate_expression(operand)?;
                
                if let BasicValueEnum::IntValue(op) = operand_val {
                    match operator {
                        crate::parser::ast::UnaryOperator::Not => {
                            Ok(self.builder.build_not(op).into())
                        }
                        crate::parser::ast::UnaryOperator::Minus => {
                            Ok(self.builder.build_neg(op).into())
                        }
                        crate::parser::ast::UnaryOperator::Plus => {
                            // 正号不需要操作，直接返回原值
                            Ok(op.into())
                        }
                    }
                } else {
                    Err("Unary expression operand must be integer".to_string())
                }
            }
            ASTNode::FunctionCall { name, arguments } => {
                // 简化处理：假设所有函数都返回i32
                let function = *self.builder.functions.get(name)
                    .ok_or_else(|| format!("Function '{}' not found", name))?;
                
                let mut arg_values = Vec::new();
                for arg in arguments {
                    let arg_val = self.generate_expression(arg)?;
                    if let BasicValueEnum::IntValue(int_val) = arg_val {
                        arg_values.push(int_val.into());
                    } else {
                        return Err("Function arguments must be integers".to_string());
                    }
                }
                
                let call_result = self.builder.builder.build_call(
                    function,
                    &arg_values,
                    "call"
                ).unwrap();
                
                Ok(call_result.try_as_basic_value().left().unwrap().into())
            }
            _ => Err(format!("Unsupported expression: {:?}", expr)),
        }
    }

    fn generate_if_statement(&mut self, condition: &ASTNode, then_branch: &ASTNode, else_branch: &Option<Box<ASTNode>>) -> Result<(), String> {
        // 生成条件表达式
        let cond_val = self.generate_expression(condition)?;
        
        // 将条件转换为 i1 布尔值（用于条件分支）
        let bool_cond_val = if let BasicValueEnum::IntValue(int_val) = cond_val {
            // 检查 LLVM 内部类型宽度
            if int_val.get_type().get_bit_width() == 1 {
                // 如果已经是 i1 类型（比较运算的结果），直接使用
                int_val
            } else {
                // 如果是 i32 或其他整数类型，需要与 0 比较，得到 i1
                // 逻辑是：如果 int_val != 0，则为真
                self.builder.build_icmp_ne(int_val, self.builder.create_int_constant(0))
            }
        } else {
            return Err("If condition must be an integer expression".to_string());
        };
        
        // 获取当前函数和基本块
        let function = self.builder.get_current_function();
        let then_block = self.builder.create_basic_block("then");
        
        if let Some(_) = else_branch {
            // 有 else 分支的情况
            let else_block = self.builder.create_basic_block("else");
            let merge_block = self.builder.create_basic_block("merge");
            
            // 创建条件分支
            self.builder.build_cond_br(bool_cond_val, then_block, else_block);
            
            // 生成 then 分支
            self.builder.set_insert_point(then_block);
            self.generate_block(then_branch)?;
            // 检查当前插入点的基本块是否有终止指令
            if let Some(current_block) = self.builder.builder.get_insert_block() {
                if current_block.get_terminator().is_none() {
                    self.builder.build_br(merge_block);
                }
            }
            
            // 生成 else 分支
            self.builder.set_insert_point(else_block);
            self.generate_block(else_branch.as_ref().unwrap())?;
            // 检查当前插入点的基本块是否有终止指令
            if let Some(current_block) = self.builder.builder.get_insert_block() {
                if current_block.get_terminator().is_none() {
                    self.builder.build_br(merge_block);
                }
            }
            
            // 设置插入点到合并块
            self.builder.set_insert_point(merge_block);
        } else {
            // 没有 else 分支的情况
            let after_block = self.builder.create_basic_block("after");
            
            // 创建条件分支
            self.builder.build_cond_br(bool_cond_val, then_block, after_block);
            
            // 生成 then 分支
            self.builder.set_insert_point(then_block);
            self.generate_block(then_branch)?;
            // 检查当前插入点的基本块是否有终止指令
            if let Some(current_block) = self.builder.builder.get_insert_block() {
                if current_block.get_terminator().is_none() {
                    self.builder.build_br(after_block);
                }
            }
            
            // 设置插入点到 after 块
            self.builder.set_insert_point(after_block);
        }
        
        Ok(())
    }
}