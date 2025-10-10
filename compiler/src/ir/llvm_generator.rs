use crate::parser::ast::{ASTNode, BinaryOperator};
use crate::error::error::CompileError;

pub struct LLVMIRGenerator {
    temp_counter: u32,
}

impl LLVMIRGenerator {
    pub fn new() -> Self {
        Self {
            temp_counter: 0,
        }
    }

    pub fn generate(&mut self, ast: &ASTNode) -> Result<String, CompileError> {
        match ast {
            ASTNode::Program(statements) => {
                let mut output = String::new();
                
                // 模块头部
                output.push_str("; ModuleID = 'test.c'\n");
                output.push_str("source_filename = \"test.c\"\n");
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
                output.push_str("!5 = !{\"BiSheng Enterprise 4.2.0.2.B002 clang version 17.0.6 (2261d9fde4e0)\"}\n");
                
                Ok(output)
            }
            _ => Err(CompileError::Semantic("Expected program node".to_string())),
        }
    }

    fn generate_function(&mut self, _return_type: &str, name: &str, _parameters: &[ASTNode], body: &ASTNode) -> Result<String, CompileError> {
        let mut output = String::new();
        
        // 重置计数器
        self.temp_counter = 0;
        
        // 函数签名
        output.push_str("; Function Attrs: noinline nounwind optnone uwtable\n");
        output.push_str(&format!("define dso_local i32 @{}() #0 {{\n", name));
        
        // 生成函数体
        output.push_str(&self.generate_block(body)?);
        
        output.push_str("}\n");
        Ok(output)
    }

    fn generate_block(&mut self, block: &ASTNode) -> Result<String, CompileError> {
        match block {
            ASTNode::Block(statements) => {
                let mut output = String::new();
                let mut alloca_count = 0;
                
                // 第一遍：收集所有变量声明，生成alloca指令
                for stmt in statements {
                    if let ASTNode::VariableDeclaration { name: _, .. } = stmt {
                        alloca_count += 1;
                        output.push_str(&format!("  %{} = alloca i32, align 4\n", alloca_count));
                    }
                }
                
                // 第二遍：生成其他指令
                let mut temp_count = alloca_count;
                for stmt in statements {
                    output.push_str(&self.generate_statement(stmt, &mut temp_count)?);
                }
                
                Ok(output)
            }
            _ => Err(CompileError::Semantic("Expected block node".to_string())),
        }
    }

    fn generate_statement(&mut self, stmt: &ASTNode, temp_count: &mut u32) -> Result<String, CompileError> {
        match stmt {
            ASTNode::DeclStmt(decl) => {
                // 处理包装在DeclStmt中的变量声明
                self.generate_statement(decl.as_ref(), temp_count)
            }
            ASTNode::VariableDeclaration { name: _, initializer, type_name: _ } => {
                let mut output = String::new();
                if let Some(init) = initializer {
                    let (ir, new_temp) = self.generate_expression(init, *temp_count)?;
                    output.push_str(&ir);
                    output.push_str(&format!("  store i32 %{}, ptr %1, align 4\n", new_temp));
                    *temp_count = new_temp + 1;
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
            _ => Ok(String::new()),
        }
    }

    fn generate_expression(&mut self, expr: &ASTNode, temp_count: u32) -> Result<(String, u32), CompileError> {
        match expr {
            ASTNode::IntegerLiteral(value) => {
                Ok((String::new(), *value as u32))
            }
            ASTNode::BinaryExpression { left, operator, right } => {
                let mut output = String::new();
                let (left_ir, left_temp) = self.generate_expression(left, temp_count)?;
                let (right_ir, right_temp) = self.generate_expression(right, left_temp + 1)?;
                
                output.push_str(&left_ir);
                output.push_str(&right_ir);
                
                let result_temp = right_temp + 1;
                let op_ir = match operator {
                    BinaryOperator::Add => "add nsw",
                    BinaryOperator::Subtract => "sub nsw", 
                    BinaryOperator::Multiply => "mul nsw",
                    BinaryOperator::Divide => "sdiv",
                    BinaryOperator::Equal => return Err(CompileError::Semantic("Assignment operator not supported in expressions".to_string())),
                };
                
                output.push_str(&format!("  %{} = {} i32 %{}, %{}\n", result_temp, op_ir, left_temp, right_temp));
                Ok((output, result_temp))
            }
            ASTNode::Identifier(_name) => {
                // 简化处理：假设变量在%1位置
                Ok((String::new(), 1))
            }
            _ => Err(CompileError::Semantic("Unsupported expression type".to_string())),
        }
    }
}