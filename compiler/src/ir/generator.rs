use super::{instruction::IRInstruction, basic_block::BasicBlock, function::IRFunction};
use crate::parser::ast::ASTNode;
use crate::error::error::CompileError;
use std::collections::HashMap;

pub struct IRGenerator {
    functions: Vec<IRFunction>,
    current_function: Option<usize>,
    current_block: Option<usize>,
    temp_counter: u32,
    label_counter: u32,
    variables: HashMap<String, String>, // 变量名 -> 临时变量名
}

impl IRGenerator {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
            current_function: None,
            current_block: None,
            temp_counter: 0,
            label_counter: 0,
            variables: HashMap::new(),
        }
    }

    pub fn generate(&mut self, ast: &ASTNode) -> Result<Vec<IRFunction>, CompileError> {
        match ast {
            ASTNode::Program(statements) => {
                for stmt in statements {
                    self.generate_statement(stmt)?;
                }
                Ok(self.functions.clone())
            }
            _ => Err(CompileError::Semantic("Expected program node".to_string())),
        }
    }

    fn generate_statement(&mut self, stmt: &ASTNode) -> Result<(), CompileError> {
        match stmt {
            ASTNode::FunctionDeclaration { return_type, name, parameters, body } => {
                self.generate_function(return_type, name, parameters, body)?;
            }
            ASTNode::Block(statements) => {
                for stmt in statements {
                    self.generate_statement(stmt)?;
                }
            }
            ASTNode::DeclStmt(decl) => {
                self.generate_statement(decl)?;
            }
            ASTNode::VariableDeclaration { name, type_name, initializer } => {
                self.generate_variable_declaration(name, type_name, initializer)?;
            }
            ASTNode::ReturnStatement(expr) => {
                self.generate_return(expr)?;
            }
            ASTNode::ExpressionStatement(expr) => {
                self.generate_expression(expr)?;
            }
            ASTNode::IfStatement { condition, then_branch, else_branch } => {
                self.generate_if_statement(condition, then_branch, else_branch)?;
            }
            ASTNode::WhileStatement { condition, body } => {
                self.generate_while_statement(condition, body)?;
            }
            _ => {} // 其他语句暂时忽略
        }
        Ok(())
    }

    fn generate_function(&mut self, return_type: &str, name: &str, parameters: &[ASTNode], body: &ASTNode) -> Result<(), CompileError> {
        // 创建函数
        let param_names: Vec<String> = parameters.iter()
            .filter_map(|p| {
                if let ASTNode::Parameter { name, .. } = p {
                    Some(name.clone())
                } else {
                    None
                }
            })
            .collect();

        let mut function = IRFunction::new(
            name.to_string(),
            param_names,
            return_type.to_string(),
        );

        // 创建入口基本块
        let entry_label = format!("{}_entry", name);
        let mut entry_block = BasicBlock::new(entry_label.clone());
        
        // 为参数分配临时变量
        for (i, param) in parameters.iter().enumerate() {
            if let ASTNode::Parameter { name, .. } = param {
                let temp = self.new_temp();
                self.variables.insert(name.clone(), temp.clone());
                entry_block.add_instruction(IRInstruction::Move {
                    dst: temp,
                    src: format!("arg{}", i),
                });
            }
        }

        function.add_block(entry_block);
        
        // 设置当前函数和基本块
        self.functions.push(function);
        self.current_function = Some(self.functions.len() - 1);
        self.current_block = Some(0);

        // 生成函数体
        self.generate_statement(body)?;

        // 如果没有显式返回，添加默认返回
        if let Some(func_idx) = self.current_function {
            if let Some(block_idx) = self.current_block {
                let block = &mut self.functions[func_idx].blocks[block_idx];
                if !block.instructions.iter().any(|inst| matches!(inst, IRInstruction::Ret { .. })) {
                    block.add_instruction(IRInstruction::Ret { value: None });
                }
            }
        }

        self.current_function = None;
        self.current_block = None;
        Ok(())
    }

    fn generate_variable_declaration(&mut self, name: &str, _type_name: &str, initializer: &Option<Box<ASTNode>>) -> Result<(), CompileError> {
        if let Some(init) = initializer {
            let value_temp = self.generate_expression(init)?;
            self.variables.insert(name.to_string(), value_temp);
        } else {
            let temp = self.new_temp();
            self.variables.insert(name.to_string(), temp);
        }
        Ok(())
    }

    fn generate_return(&mut self, expr: &Option<Box<ASTNode>>) -> Result<(), CompileError> {
        if let Some(expr) = expr {
            let value_temp = self.generate_expression(expr)?;
            self.add_instruction(IRInstruction::Ret { value: Some(value_temp) });
        } else {
            self.add_instruction(IRInstruction::Ret { value: None });
        }
        Ok(())
    }

    fn generate_expression(&mut self, expr: &ASTNode) -> Result<String, CompileError> {
        match expr {
            ASTNode::IntegerLiteral(value) => {
                let temp = self.new_temp();
                self.add_instruction(IRInstruction::LoadConst {
                    dst: temp.clone(),
                    value: *value,
                });
                Ok(temp)
            }
            ASTNode::StringLiteral(value) => {
                let temp = self.new_temp();
                self.add_instruction(IRInstruction::LoadString {
                    dst: temp.clone(),
                    value: value.clone(),
                });
                Ok(temp)
            }
            ASTNode::Identifier(name) => {
                if let Some(temp) = self.variables.get(name) {
                    Ok(temp.clone())
                } else {
                    Err(CompileError::Semantic(format!("Undefined variable: {}", name)))
                }
            }
            ASTNode::BinaryExpression { operator, left, right } => {
                let left_temp = self.generate_expression(left)?;
                let right_temp = self.generate_expression(right)?;
                let result_temp = self.new_temp();

                let instruction = match operator {
                    crate::parser::ast::BinaryOperator::Add => {
                        IRInstruction::Add {
                            dst: result_temp.clone(),
                            src1: left_temp,
                            src2: right_temp,
                        }
                    }
                    crate::parser::ast::BinaryOperator::Subtract => {
                        IRInstruction::Sub {
                            dst: result_temp.clone(),
                            src1: left_temp,
                            src2: right_temp,
                        }
                    }
                    crate::parser::ast::BinaryOperator::Multiply => {
                        IRInstruction::Mul {
                            dst: result_temp.clone(),
                            src1: left_temp,
                            src2: right_temp,
                        }
                    }
                    crate::parser::ast::BinaryOperator::Divide => {
                        IRInstruction::Div {
                            dst: result_temp.clone(),
                            src1: left_temp,
                            src2: right_temp,
                        }
                    }
                    _ => return Err(CompileError::Semantic("Unsupported binary operator".to_string())),
                };

                self.add_instruction(instruction);
                Ok(result_temp)
            }
            ASTNode::FunctionCall { name, arguments } => {
                let mut arg_temps = Vec::new();
                for arg in arguments {
                    let arg_temp = self.generate_expression(arg)?;
                    arg_temps.push(arg_temp);
                }

                let result_temp = self.new_temp();
                self.add_instruction(IRInstruction::Call {
                    dst: Some(result_temp.clone()),
                    func: name.clone(),
                    args: arg_temps,
                });
                Ok(result_temp)
            }
            ASTNode::ImplicitCastExpr { operand, .. } => {
                self.generate_expression(operand)
            }
            _ => Err(CompileError::Semantic("Unsupported expression type".to_string())),
        }
    }

    fn add_instruction(&mut self, instruction: IRInstruction) {
        if let Some(func_idx) = self.current_function {
            if let Some(block_idx) = self.current_block {
                self.functions[func_idx].blocks[block_idx].add_instruction(instruction);
            }
        }
    }

    fn new_temp(&mut self) -> String {
        self.temp_counter += 1;
        format!("t{}", self.temp_counter)
    }

    fn new_label(&mut self) -> String {
        self.label_counter += 1;
        format!("L{}", self.label_counter)
    }

    fn generate_if_statement(&mut self, condition: &ASTNode, then_branch: &ASTNode, else_branch: &Option<Box<ASTNode>>) -> Result<(), CompileError> {
        let condition_temp = self.generate_expression(condition)?;
        let then_label = self.new_label();
        let else_label = self.new_label();
        let end_label = self.new_label();

        // 跳转到then分支
        self.add_instruction(IRInstruction::JumpIf {
            condition: condition_temp,
            target: then_label.clone(),
        });

        // else分支
        if let Some(else_branch) = else_branch {
            self.add_instruction(IRInstruction::Label { name: else_label.clone() });
            self.generate_statement(else_branch)?;
        }

        // 跳转到结束
        self.add_instruction(IRInstruction::Jump { target: end_label.clone() });

        // then分支
        self.add_instruction(IRInstruction::Label { name: then_label });
        self.generate_statement(then_branch)?;

        // 结束标签
        self.add_instruction(IRInstruction::Label { name: end_label });

        Ok(())
    }

    fn generate_while_statement(&mut self, condition: &ASTNode, body: &ASTNode) -> Result<(), CompileError> {
        let loop_label = self.new_label();
        let body_label = self.new_label();
        let end_label = self.new_label();

        // 循环开始
        self.add_instruction(IRInstruction::Label { name: loop_label.clone() });
        
        // 检查条件
        let condition_temp = self.generate_expression(condition)?;
        self.add_instruction(IRInstruction::JumpIfNot {
            condition: condition_temp,
            target: end_label.clone(),
        });

        // 循环体
        self.add_instruction(IRInstruction::Label { name: body_label });
        self.generate_statement(body)?;

        // 跳回循环开始
        self.add_instruction(IRInstruction::Jump { target: loop_label });

        // 结束标签
        self.add_instruction(IRInstruction::Label { name: end_label });

        Ok(())
    }
}
