use super::{instruction::IRInstruction, basic_block::BasicBlock, function::IRFunction};
use std::fmt;

pub struct IRFormatter;

impl IRFormatter {
    pub fn format_functions(functions: &[IRFunction]) -> String {
        let mut output = String::new();
        
        for (i, function) in functions.iter().enumerate() {
            if i > 0 {
                output.push('\n');
            }
            output.push_str(&Self::format_function(function));
        }
        
        output
    }

    fn format_function(function: &IRFunction) -> String {
        let mut output = String::new();
        
        // 函数签名
        let params_str = function.parameters.join(", ");
        output.push_str(&format!("function {} ({}) -> {} {{\n", 
            function.name, params_str, function.return_type));
        
        // 基本块
        for block in &function.blocks {
            output.push_str(&Self::format_block(block));
        }
        
        output.push_str("}\n");
        output
    }

    fn format_block(block: &BasicBlock) -> String {
        let mut output = String::new();
        
        // 标签
        output.push_str(&format!("{}:\n", block.label));
        
        // 指令
        for instruction in &block.instructions {
            output.push_str(&format!("  {}\n", instruction));
        }
        
        output
    }

    pub fn format_instruction(instruction: &IRInstruction) -> String {
        format!("{}", instruction)
    }
}

// 移除重复的Display实现，使用原有的
