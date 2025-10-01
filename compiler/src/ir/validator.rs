use super::{instruction::IRInstruction, function::IRFunction};
use std::collections::HashSet;

pub struct IRValidator {
    defined_vars: HashSet<String>,
    defined_labels: HashSet<String>,
    used_vars: HashSet<String>,
    used_labels: HashSet<String>,
}

impl IRValidator {
    pub fn new() -> Self {
        Self {
            defined_vars: HashSet::new(),
            defined_labels: HashSet::new(),
            used_vars: HashSet::new(),
            used_labels: HashSet::new(),
        }
    }

    pub fn validate_function(&mut self, function: &IRFunction) -> Result<(), String> {
        self.defined_vars.clear();
        self.defined_labels.clear();
        self.used_vars.clear();
        self.used_labels.clear();

        // 收集所有定义和使用的变量/标签
        for block in &function.blocks {
            self.defined_labels.insert(block.label.clone());
            for instruction in &block.instructions {
                self.collect_instruction_info(instruction);
            }
        }

        // 检查未定义的变量
        let undefined_vars: Vec<_> = self.used_vars.difference(&self.defined_vars).collect();
        if !undefined_vars.is_empty() {
            return Err(format!("Undefined variables: {:?}", undefined_vars));
        }

        // 检查未定义的标签
        let undefined_labels: Vec<_> = self.used_labels.difference(&self.defined_labels).collect();
        if !undefined_labels.is_empty() {
            return Err(format!("Undefined labels: {:?}", undefined_labels));
        }

        Ok(())
    }

    fn collect_instruction_info(&mut self, instruction: &IRInstruction) {
        match instruction {
            IRInstruction::Add { dst, src1, src2 } => {
                self.defined_vars.insert(dst.clone());
                self.used_vars.insert(src1.clone());
                self.used_vars.insert(src2.clone());
            }
            IRInstruction::Sub { dst, src1, src2 } => {
                self.defined_vars.insert(dst.clone());
                self.used_vars.insert(src1.clone());
                self.used_vars.insert(src2.clone());
            }
            IRInstruction::Mul { dst, src1, src2 } => {
                self.defined_vars.insert(dst.clone());
                self.used_vars.insert(src1.clone());
                self.used_vars.insert(src2.clone());
            }
            IRInstruction::Div { dst, src1, src2 } => {
                self.defined_vars.insert(dst.clone());
                self.used_vars.insert(src1.clone());
                self.used_vars.insert(src2.clone());
            }
            IRInstruction::LoadConst { dst, .. } => {
                self.defined_vars.insert(dst.clone());
            }
            IRInstruction::LoadString { dst, .. } => {
                self.defined_vars.insert(dst.clone());
            }
            IRInstruction::Move { dst, src } => {
                self.defined_vars.insert(dst.clone());
                self.used_vars.insert(src.clone());
            }
            IRInstruction::Call { dst, args, .. } => {
                if let Some(dst) = dst {
                    self.defined_vars.insert(dst.clone());
                }
                for arg in args {
                    self.used_vars.insert(arg.clone());
                }
            }
            IRInstruction::Ret { value } => {
                if let Some(value) = value {
                    self.used_vars.insert(value.clone());
                }
            }
            IRInstruction::Jump { target } => {
                self.used_labels.insert(target.clone());
            }
            IRInstruction::JumpIf { target, .. } => {
                self.used_labels.insert(target.clone());
            }
            IRInstruction::JumpIfNot { target, .. } => {
                self.used_labels.insert(target.clone());
            }
            IRInstruction::Label { name } => {
                self.defined_labels.insert(name.clone());
            }
            _ => {} // 其他指令暂时忽略
        }
    }
}
