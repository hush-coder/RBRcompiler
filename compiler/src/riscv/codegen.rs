use crate::ir::function::IRFunction;
use crate::ir::instruction::IRInstruction;
use crate::ir::basic_block::BasicBlock;
use super::register::RegisterAllocator;

pub struct RiscvCodegen {
    register_allocator: RegisterAllocator,
    label_counter: u32,
    temp_counter: u32,
}

impl RiscvCodegen {
    pub fn new() -> Self {
        Self {
            register_allocator: RegisterAllocator::new(),
            label_counter: 0,
            temp_counter: 0,
        }
    }

    pub fn generate(&mut self, funcs: &[IRFunction]) -> String {
        let mut output = String::new();
        
        // 生成汇编头部 - 模仿bisheng格式
        output.push_str("\t.text\n");
        output.push_str("\t.file\t\"test.c\"\n");
        
        // 生成每个函数
        for (i, func) in funcs.iter().enumerate() {
            output.push_str(&self.generate_function(func, i));
            output.push_str("\n");
        }
        
        // 生成尾部信息 - 模仿bisheng格式
        output.push_str("\t.ident\t\"RISC-V Compiler v1.0.0\"\n");
        output.push_str("\t.section\t\".note.GNU-stack\",\"\",@progbits\n");
        output.push_str("\t.addrsig\n");
        
        output
    }

    fn generate_function(&mut self, func: &IRFunction, func_index: usize) -> String {
        let mut output = String::new();
        
        // 函数头部 - 模仿bisheng格式
        output.push_str(&format!("\t.globl\t{}\t\t\t\t\t// -- Begin function {}\n", func.name, func.name));
        output.push_str("\t.p2align\t2\n");
        output.push_str(&format!("\t.type\t{},@function\n", func.name));
        output.push_str(&format!("{}:\t\t\t\t\t\t\t\t// @{}\n", func.name, func.name));
        output.push_str("\t.cfi_startproc\n");
        output.push_str("// %bb.0:\n");
        
        // 计算栈空间大小
        let stack_size = 16 + (func.parameters.len() * 4);
        output.push_str(&format!("\taddi\tsp, sp, -{}\n", stack_size));
        output.push_str(&format!("\t.cfi_def_cfa_offset {}\n", stack_size));
        
        // 保存返回地址
        output.push_str(&format!("\tsw\tra, {}(sp)\n", stack_size - 4));
        
        // 处理函数参数
        for (i, param) in func.parameters.iter().enumerate() {
            if i < 8 { // RISC-V有8个参数寄存器 a0-a7
                output.push_str(&format!("\tsw\ta{}, {}(sp)\n", i, stack_size - 8 - (i * 4)));
            }
        }
        
        // 生成基本块
        for block in &func.blocks {
            output.push_str(&self.generate_basic_block(block));
        }
        
        // 函数尾声
        output.push_str(&format!("\taddi\tsp, sp, {}\n", stack_size));
        output.push_str("\t.cfi_def_cfa_offset 0\n");
        output.push_str("\tret\n");
        output.push_str(&format!(".Lfunc_end{}:\n", func_index));
        output.push_str(&format!("\t.size\t{}, .Lfunc_end{}-{}\n", func.name, func_index, func.name));
        output.push_str("\t.cfi_endproc\n");
        output.push_str("\t\t\t\t\t\t\t\t\t// -- End function\n");
        
        output
    }

    fn generate_basic_block(&mut self, block: &BasicBlock) -> String {
        let mut output = String::new();
        
        // 基本块标签
        if !block.label.is_empty() && block.label != "entry" {
            output.push_str(&format!(".{}:\n", block.label));
        }
        
        // 生成指令
        for instruction in &block.instructions {
            output.push_str(&self.generate_instruction(instruction));
        }
        
        output
    }

    fn generate_instruction(&mut self, instruction: &IRInstruction) -> String {
        match instruction {
            IRInstruction::Add { dst, src1, src2 } => {
                let reg1 = self.get_register(src1);
                let reg2 = self.get_register(src2);
                let dst_reg = self.allocate_register(dst);
                format!("\tadd\t{}, {}, {}\n", dst_reg, reg1, reg2)
            }
            IRInstruction::Sub { dst, src1, src2 } => {
                let reg1 = self.get_register(src1);
                let reg2 = self.get_register(src2);
                let dst_reg = self.allocate_register(dst);
                format!("\tsub\t{}, {}, {}\n", dst_reg, reg1, reg2)
            }
            IRInstruction::Mul { dst, src1, src2 } => {
                let reg1 = self.get_register(src1);
                let reg2 = self.get_register(src2);
                let dst_reg = self.allocate_register(dst);
                format!("\tmul\t{}, {}, {}\n", dst_reg, reg1, reg2)
            }
            IRInstruction::Div { dst, src1, src2 } => {
                let reg1 = self.get_register(src1);
                let reg2 = self.get_register(src2);
                let dst_reg = self.allocate_register(dst);
                format!("\tdiv\t{}, {}, {}\n", dst_reg, reg1, reg2)
            }
            IRInstruction::CmpGt { dst, src1, src2 } => {
                let reg1 = self.get_register(src1);
                let reg2 = self.get_register(src2);
                let dst_reg = self.allocate_register(dst);
                format!("\tslt\t{}, {}, {}\n", dst_reg, reg2, reg1)
            }
            IRInstruction::CmpLt { dst, src1, src2 } => {
                let reg1 = self.get_register(src1);
                let reg2 = self.get_register(src2);
                let dst_reg = self.allocate_register(dst);
                format!("\tslt\t{}, {}, {}\n", dst_reg, reg1, reg2)
            }
            IRInstruction::CmpEq { dst, src1, src2 } => {
                let reg1 = self.get_register(src1);
                let reg2 = self.get_register(src2);
                let dst_reg = self.allocate_register(dst);
                format!("\txor\t{}, {}, {}\n", dst_reg, reg1, reg2)
            }
            IRInstruction::LoadConst { dst, value } => {
                let dst_reg = self.allocate_register(dst);
                if *value >= -2048 && *value <= 2047 {
                    format!("\tli\t{}, {}\t\t\t\t\t// =0x{:x}\n", dst_reg, value, value)
                } else {
                    format!("\tlui\t{}, {}\t\t\t\t// 加载高16位\n\taddi\t{}, {}, {}\t\t\t// {} = {}\n", 
                           dst_reg, (*value >> 12) & 0xFFFF, dst_reg, dst_reg, *value & 0xFFF, dst, value)
                }
            }
            IRInstruction::Load { dst, addr } => {
                let dst_reg = self.allocate_register(dst);
                let addr_reg = self.get_register(addr);
                format!("\tlw\t{}, 0({})\n", dst_reg, addr_reg)
            }
            IRInstruction::Store { addr, src } => {
                let addr_reg = self.get_register(addr);
                let src_reg = self.get_register(src);
                format!("\tsw\t{}, 0({})\n", src_reg, addr_reg)
            }
            IRInstruction::Alloca { dst, size } => {
                let dst_reg = self.allocate_register(dst);
                format!("\taddi\t{}, sp, {}\n", dst_reg, -(*size as i32))
            }
            IRInstruction::Jump { target } => {
                format!("\tj\t{}\n", target)
            }
            IRInstruction::JumpIf { condition, target } => {
                let cond_reg = self.get_register(condition);
                format!("\tbne\t{}, zero, {}\n", cond_reg, target)
            }
            IRInstruction::JumpIfNot { condition, target } => {
                let cond_reg = self.get_register(condition);
                format!("\tbeq\t{}, zero, {}\n", cond_reg, target)
            }
            IRInstruction::Call { dst, func, args } => {
                let mut output = String::new();
                
                // 设置参数
                for (i, arg) in args.iter().enumerate() {
                    if i < 8 {
                        let arg_reg = self.get_register(arg);
                        output.push_str(&format!("\tmv\ta{}, {}\n", i, arg_reg));
                    }
                }
                
                // 调用函数
                output.push_str(&format!("\tcall\t{}\n", func));
                
                // 保存返回值
                if let Some(dst) = dst {
                    let dst_reg = self.allocate_register(dst);
                    output.push_str(&format!("\tmv\t{}, a0\n", dst_reg));
                }
                
                output
            }
            IRInstruction::Ret { value } => {
                if let Some(value) = value {
                    let value_reg = self.get_register(value);
                    format!("\tmv\ta0, {}\n", value_reg)
                } else {
                    String::new()
                }
            }
            IRInstruction::Move { dst, src } => {
                let dst_reg = self.allocate_register(dst);
                let src_reg = self.get_register(src);
                format!("\tmv\t{}, {}\n", dst_reg, src_reg)
            }
            IRInstruction::Label { name } => {
                format!(".{}:\n", name)
            }
            IRInstruction::Nop => {
                "\tnop\n".to_string()
            }
            _ => {
                format!("\t# 未实现的指令: {:?}\n", instruction)
            }
        }
    }

    fn get_register(&self, var: &str) -> String {
        self.register_allocator.get_register(var)
    }

    fn allocate_register(&mut self, var: &str) -> String {
        self.register_allocator.allocate_register(var)
    }
}

impl Default for RiscvCodegen {
    fn default() -> Self {
        Self::new()
    }
} 