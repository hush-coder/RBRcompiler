use inkwell::module::Module;
use inkwell::values::{FunctionValue, BasicValueEnum, InstructionValue, AnyValue};
use inkwell::basic_block::BasicBlock;
use super::arm::ArmInstruction;
use super::allocator::RegisterAllocator;
use std::collections::HashMap;

/// 新的ARM64汇编代码生成器
/// 使用更清晰的架构和更好的寄存器分配
pub struct AssemblyCodegenV2 {
    /// 寄存器分配器
    allocator: RegisterAllocator,
    /// 生成的汇编指令列表
    instructions: Vec<ArmInstruction>,
    /// 调试信息
    debug_mode: bool,
    /// LLVM 指令到栈偏移的映射（用于 alloca）
    alloca_map: HashMap<String, i32>,
    /// LLVM 指令到寄存器的映射（用于临时值）
    value_map: HashMap<String, String>,
    /// 当前栈偏移
    current_stack_offset: i32,
    /// 临时寄存器计数器
    temp_register_counter: u32,
    /// 函数计数器（用于生成唯一的函数结束标签）
    function_counter: u32,
}

impl AssemblyCodegenV2 {
    /// 创建新的汇编代码生成器
    pub fn new() -> Self {
        Self {
            allocator: RegisterAllocator::new(),
            instructions: Vec::new(),
            debug_mode: true,
            alloca_map: HashMap::new(),
            value_map: HashMap::new(),
            current_stack_offset: 0,
            temp_register_counter: 0,
            function_counter: 0,
        }
    }
    
    /// 分配一个临时寄存器（w13, w14, w15...循环使用）
    fn allocate_temp_register(&mut self) -> String {
        let regs = ["w13", "w14", "w15", "w16", "w17"];
        let reg = regs[(self.temp_register_counter as usize) % regs.len()];
        self.temp_register_counter += 1;
        reg.to_string()
    }
    
    /// 获取指令的唯一标识符
    fn get_instruction_id(&self, instruction: &InstructionValue) -> String {
        // 使用 LLVM 值的字符串表示作为唯一 ID（更稳定）
        instruction.print_to_string().to_string()
    }
    
    /// 获取值的唯一标识符（用于 BasicValueEnum）
    fn get_value_id(&self, value: &BasicValueEnum) -> String {
        // 使用 LLVM 值的字符串表示作为唯一 ID
        // 这样可以确保 alloca 指令和它的 PointerValue 结果有相同的 ID
        match value {
            BasicValueEnum::IntValue(v) => v.print_to_string().to_string(),
            BasicValueEnum::PointerValue(v) => {
                // 对于指针，尝试转换为指令
                if let Some(instr) = v.as_instruction() {
                    instr.print_to_string().to_string()
                } else {
                    v.print_to_string().to_string()
                }
            },
            _ => format!("{:?}", value),
        }
    }

    /// 从LLVM模块生成ARM64汇编代码
    pub fn generate_from_module(&mut self, module: &Module) -> Result<String, String> {
        let mut assembly = String::new();
        
        // 添加文件头
        assembly.push_str("\t.text\n");
        assembly.push_str("\t.file\t\"input.c\"\n");

        // 遍历模块中的每个函数（生成所有函数，不只是 main）
        for function in module.get_functions() {
            if let Some(_func_name) = function.get_name().to_str().ok() {
                    assembly.push_str(&self.generate_function(&function)?);
            }
        }

        Ok(assembly)
    }

    /// 生成单个函数的汇编代码
    fn generate_function(&mut self, function: &FunctionValue) -> Result<String, String> {
        let mut assembly = String::new();
        
        // 重置函数级别的状态
        self.alloca_map.clear();
        self.value_map.clear();
        self.current_stack_offset = 0;
        self.temp_register_counter = 0;
        
        // 函数头部
        assembly.push_str(&format!("\t.globl\t{}\n", function.get_name().to_str().unwrap()));
        assembly.push_str("\t.p2align\t2\n");
        assembly.push_str(&format!("\t.type\t{},@function\n", function.get_name().to_str().unwrap()));
        assembly.push_str(&format!("{}:\n", function.get_name().to_str().unwrap()));
        assembly.push_str("\t.cfi_startproc\n");
        
        // 函数序言 (Prologue)
        assembly.push_str("\tstp\tx29, x30, [sp, #-16]!\n");
        assembly.push_str("\tmov\tx29, sp\n");
        
        // 第一遍扫描：收集所有 alloca 指令
        let stack_size = self.analyze_allocas(function);
        
        // 为局部变量分配栈空间（需要16字节对齐）
        let aligned_stack_size = ((stack_size + 15) / 16) * 16;
        if aligned_stack_size > 0 {
            assembly.push_str(&format!("\tsub\tsp, sp, #{}\n", aligned_stack_size));
        }
        
        // 处理函数参数：将参数寄存器中的值映射到 value_map
        // ARM64 调用约定：前8个参数在 w0-w7
        for (i, param) in function.get_params().into_iter().enumerate() {
            if i < 8 {
                let param_id = self.get_value_id(&param);
                let param_reg = format!("w{}", i);
                
                if self.debug_mode {
                    println!("DEBUG: Function param[{}] is in {}", i, param_reg);
                }
                
                self.value_map.insert(param_id, param_reg);
            }
        }
        
        // 生成函数体的指令
        for basic_block in function.get_basic_blocks() {
            assembly.push_str(&self.generate_basic_block(&basic_block)?);
        }
        
        // 函数尾部 (Epilogue) - 在 return 指令中已经处理
        
        assembly.push_str("\t.cfi_endproc\n");
        
        // 使用唯一的函数结束标签
        let func_end_label = format!(".Lfunc_end{}", self.function_counter);
        assembly.push_str(&format!("{}:\n", func_end_label));
        assembly.push_str(&format!("\t.size\t{}, {}-{}\n\n", 
            function.get_name().to_str().unwrap(), 
            func_end_label,
            function.get_name().to_str().unwrap()));
        
        // 递增函数计数器
        self.function_counter += 1;
        
        Ok(assembly)
    }
    
    /// 分析函数中的所有 alloca 指令，分配栈偏移
    fn analyze_allocas(&mut self, function: &FunctionValue) -> i32 {
        let mut offset = 0;
        for basic_block in function.get_basic_blocks() {
            for instruction in basic_block.get_instructions() {
                if instruction.get_opcode() == inkwell::values::InstructionOpcode::Alloca {
                    offset += 4; // 每个 int 变量 4 字节
                    let instr_id = self.get_instruction_id(&instruction);
                    
                    if self.debug_mode {
                        println!("DEBUG: Alloca {} at offset {}", instr_id, offset);
                    }
                    
                    self.alloca_map.insert(instr_id, offset);
                }
            }
        }
        self.current_stack_offset = offset;
        offset
    }

    /// 计算函数需要的栈空间
    fn calculate_stack_size(&self, function: &FunctionValue) -> i32 {
        // 简单的栈空间计算，实际应该更复杂
        let mut size = 0;
        for basic_block in function.get_basic_blocks() {
            for instruction in basic_block.get_instructions() {
                if instruction.get_opcode() == inkwell::values::InstructionOpcode::Alloca {
                    size += 4; // 假设每个变量4字节
                }
            }
        }
        size
    }

    /// 生成基本块的汇编代码
    fn generate_basic_block(&mut self, basic_block: &BasicBlock) -> Result<String, String> {
        let mut assembly = String::new();
        
        // 生成基本块标签（除了 entry 块）
        let block_name = basic_block.get_name().to_str().unwrap_or("unknown");
        if block_name != "entry" && !block_name.is_empty() {
            assembly.push_str(&format!(".L{}:\n", block_name));
        }
        
        // 生成基本块中的每条指令
        for instruction in basic_block.get_instructions() {
            assembly.push_str(&self.generate_instruction(&instruction)?);
        }
        
        Ok(assembly)
    }

    /// 生成单条指令的汇编代码
    fn generate_instruction(&mut self, instruction: &InstructionValue) -> Result<String, String> {
        let opcode = instruction.get_opcode();
        // 收集操作数到向量中，过滤掉None值
        let operands: Vec<_> = instruction.get_operands()
            .filter_map(|opt| opt)
            .collect();
        
        if self.debug_mode {
            println!("DEBUG: Processing instruction: {:?} with {} operands", opcode, operands.len());
        }

        match opcode {
            inkwell::values::InstructionOpcode::Alloca => {
                self.generate_alloca_instruction(instruction, &operands)
            }
            inkwell::values::InstructionOpcode::Store => {
                self.generate_store_instruction(instruction, &operands)
            }
            inkwell::values::InstructionOpcode::Load => {
                self.generate_load_instruction(instruction, &operands)
            }
            inkwell::values::InstructionOpcode::Add => {
                self.generate_add_instruction(instruction, &operands)
            }
            inkwell::values::InstructionOpcode::Sub => {
                self.generate_sub_instruction(instruction, &operands)
            }
            inkwell::values::InstructionOpcode::Mul => {
                self.generate_mul_instruction(instruction, &operands)
            }
            inkwell::values::InstructionOpcode::SDiv => {
                self.generate_sdiv_instruction(instruction, &operands)
            }
            inkwell::values::InstructionOpcode::UDiv => {
                self.generate_udiv_instruction(instruction, &operands)
            }
            inkwell::values::InstructionOpcode::ICmp => {
                self.generate_icmp_instruction(instruction, &operands)
            }
            inkwell::values::InstructionOpcode::And => {
                self.generate_and_instruction(instruction, &operands)
            }
            inkwell::values::InstructionOpcode::Or => {
                self.generate_or_instruction(instruction, &operands)
            }
            inkwell::values::InstructionOpcode::Xor => {
                self.generate_xor_instruction(instruction, &operands)
            }
            inkwell::values::InstructionOpcode::Br => {
                self.generate_br_instruction(instruction, &operands)
            }
            inkwell::values::InstructionOpcode::Call => {
                self.generate_call_instruction(instruction, &operands)
            }
            inkwell::values::InstructionOpcode::Return => {
                self.generate_return_instruction(instruction, &operands)
            }
            _ => {
                Ok(format!("\t# TODO: Implement {:?}\n", opcode))
            }
        }
    }

    /// 生成Alloca指令
    fn generate_alloca_instruction(&mut self, _instruction: &InstructionValue, _operands: &[inkwell::Either<BasicValueEnum, BasicBlock>]) -> Result<String, String> {
        // Alloca 指令在分析阶段已经处理，这里不生成任何代码
        // 栈空间已经在函数序言中分配
        Ok(String::new())
    }

    /// 生成Store指令
    fn generate_store_instruction(&mut self, _instruction: &InstructionValue, operands: &[inkwell::Either<BasicValueEnum, BasicBlock>]) -> Result<String, String> {
        if operands.len() < 2 {
            return Err("Store instruction needs at least 2 operands".to_string());
        }
        
            let src_value = operands[0].left().unwrap();
        let dst_ptr = operands[1].left().unwrap();
        
        let mut assembly = String::new();
        
        // 获取目标地址（应该是 alloca 指令的结果）
        let dst_location = self.get_value_location(&dst_ptr)?;
        
        // 处理源操作数
            if let BasicValueEnum::IntValue(int_val) = src_value {
                if int_val.is_const() {
                // 立即数：加载到临时寄存器再存储
                    if let Some(const_val) = int_val.get_zero_extended_constant() {
                    assembly.push_str(&format!("\tmov\tw8, #{}\n", const_val));
                    assembly.push_str(&format!("\tstr\tw8, {}\n", dst_location));
                    return Ok(assembly);
                }
            }
        }
        
        // 非立即数：从 value_map 中查找
        let src_location = self.get_value_location(&src_value)?;
        
        // 如果源是内存位置，先加载到寄存器
        if src_location.starts_with("[") {
            assembly.push_str(&format!("\tldr\tw8, {}\n", src_location));
            assembly.push_str(&format!("\tstr\tw8, {}\n", dst_location));
        } else {
            assembly.push_str(&format!("\tstr\t{}, {}\n", src_location, dst_location));
        }
        
        Ok(assembly)
    }

    /// 生成Load指令
    fn generate_load_instruction(&mut self, instruction: &InstructionValue, operands: &[inkwell::Either<BasicValueEnum, BasicBlock>]) -> Result<String, String> {
        if operands.is_empty() {
            return Err("Load instruction needs at least 1 operand".to_string());
        }
        
        let src_ptr = operands[0].left().unwrap();
        
        // 获取源地址（应该是 alloca 指令的结果）
        let src_location = self.get_value_location(&src_ptr)?;
        
        // 分配一个临时寄存器
        let dest_reg = self.allocate_temp_register();
        let assembly = format!("\tldr\t{}, {}\n", dest_reg, src_location);
        
        // 记录 load 指令的结果寄存器
        let instr_id = self.get_instruction_id(instruction);
        self.value_map.insert(instr_id, dest_reg);
        
        Ok(assembly)
    }

    /// 生成Add指令
    fn generate_add_instruction(&mut self, instruction: &InstructionValue, operands: &[inkwell::Either<BasicValueEnum, BasicBlock>]) -> Result<String, String> {
        if operands.len() < 2 {
            return Err("Add instruction needs at least 2 operands".to_string());
        }
        
            let src1_value = operands[0].left().unwrap();
            let src2_value = operands[1].left().unwrap();
        
        let mut assembly = String::new();
        
        // 获取操作数位置
        let src1_loc = self.get_value_location(&src1_value)?;
        let src2_loc = self.get_value_location(&src2_value)?;
        
        // 使用不同的寄存器避免覆盖
        let reg1 = "w10";
        let reg2 = "w11";
        
        // 加载第一个操作数
        if src1_loc.starts_with("[") {
            assembly.push_str(&format!("\tldr\t{}, {}\n", reg1, src1_loc));
        } else if src1_loc.starts_with("#") {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg1, src1_loc));
        } else {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg1, src1_loc));
        }
        
        // 加载第二个操作数
        if src2_loc.starts_with("[") {
            assembly.push_str(&format!("\tldr\t{}, {}\n", reg2, src2_loc));
        } else if src2_loc.starts_with("#") {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg2, src2_loc));
        } else {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg2, src2_loc));
        }
        
        // 执行加法，使用临时寄存器保存结果
        let result_reg = self.allocate_temp_register();
        assembly.push_str(&format!("\tadd\t{}, {}, {}\n", result_reg, reg1, reg2));
        
        // 记录结果
        let instr_id = self.get_instruction_id(instruction);
        self.value_map.insert(instr_id, result_reg);
        
        Ok(assembly)
    }

    /// 生成Sub指令
    fn generate_sub_instruction(&mut self, instruction: &InstructionValue, operands: &[inkwell::Either<BasicValueEnum, BasicBlock>]) -> Result<String, String> {
        if operands.len() < 2 {
            return Err("Sub instruction needs at least 2 operands".to_string());
        }
        
            let src1_value = operands[0].left().unwrap();
            let src2_value = operands[1].left().unwrap();
        
        let mut assembly = String::new();
        
        // 获取操作数位置
        let src1_loc = self.get_value_location(&src1_value)?;
        let src2_loc = self.get_value_location(&src2_value)?;
        
        // 加载操作数到寄存器
        let reg1 = "w10";
        let reg2 = "w11";
        
        if src1_loc.starts_with("[") {
            assembly.push_str(&format!("\tldr\t{}, {}\n", reg1, src1_loc));
        } else if src1_loc.starts_with("#") {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg1, src1_loc));
        } else {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg1, src1_loc));
        }
        
        if src2_loc.starts_with("[") {
            assembly.push_str(&format!("\tldr\t{}, {}\n", reg2, src2_loc));
        } else if src2_loc.starts_with("#") {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg2, src2_loc));
        } else {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg2, src2_loc));
        }
        
        // 执行减法，使用临时寄存器保存结果
        let result_reg = self.allocate_temp_register();
        assembly.push_str(&format!("\tsub\t{}, {}, {}\n", result_reg, reg1, reg2));
        
        // 记录结果
        let instr_id = self.get_instruction_id(instruction);
        self.value_map.insert(instr_id, result_reg);
        
        Ok(assembly)
    }

    /// 生成Mul指令
    fn generate_mul_instruction(&mut self, instruction: &InstructionValue, operands: &[inkwell::Either<BasicValueEnum, BasicBlock>]) -> Result<String, String> {
        if operands.len() < 2 {
            return Err("Mul instruction needs at least 2 operands".to_string());
        }
        
        let src1_value = operands[0].left().unwrap();
        let src2_value = operands[1].left().unwrap();
        
        let mut assembly = String::new();
        
        // 获取操作数位置
        let src1_loc = self.get_value_location(&src1_value)?;
        let src2_loc = self.get_value_location(&src2_value)?;
        
        // 加载操作数到寄存器
        let reg1 = "w10";
        let reg2 = "w11";
        
        if src1_loc.starts_with("[") {
            assembly.push_str(&format!("\tldr\t{}, {}\n", reg1, src1_loc));
        } else if src1_loc.starts_with("#") {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg1, src1_loc));
        } else {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg1, src1_loc));
        }
        
        if src2_loc.starts_with("[") {
            assembly.push_str(&format!("\tldr\t{}, {}\n", reg2, src2_loc));
        } else if src2_loc.starts_with("#") {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg2, src2_loc));
        } else {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg2, src2_loc));
        }
        
        // 执行乘法，使用临时寄存器保存结果
        let result_reg = self.allocate_temp_register();
        assembly.push_str(&format!("\tmul\t{}, {}, {}\n", result_reg, reg1, reg2));
        
        // 记录结果
        let instr_id = self.get_instruction_id(instruction);
        self.value_map.insert(instr_id, result_reg);
        
        Ok(assembly)
    }

    /// 生成SDiv指令（有符号除法）
    fn generate_sdiv_instruction(&mut self, instruction: &InstructionValue, operands: &[inkwell::Either<BasicValueEnum, BasicBlock>]) -> Result<String, String> {
        if operands.len() < 2 {
            return Err("SDiv instruction needs at least 2 operands".to_string());
        }
        
        let src1_value = operands[0].left().unwrap();
        let src2_value = operands[1].left().unwrap();
        
        let mut assembly = String::new();
        
        // 获取操作数位置
        let src1_loc = self.get_value_location(&src1_value)?;
        let src2_loc = self.get_value_location(&src2_value)?;
        
        // 加载操作数到寄存器
        let reg1 = "w10";
        let reg2 = "w11";
        
        if src1_loc.starts_with("[") {
            assembly.push_str(&format!("\tldr\t{}, {}\n", reg1, src1_loc));
        } else if src1_loc.starts_with("#") {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg1, src1_loc));
        } else {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg1, src1_loc));
        }
        
        if src2_loc.starts_with("[") {
            assembly.push_str(&format!("\tldr\t{}, {}\n", reg2, src2_loc));
        } else if src2_loc.starts_with("#") {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg2, src2_loc));
        } else {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg2, src2_loc));
        }
        
        // 执行有符号除法，使用临时寄存器保存结果
        let result_reg = self.allocate_temp_register();
        assembly.push_str(&format!("\tsdiv\t{}, {}, {}\n", result_reg, reg1, reg2));
        
        // 记录结果
        let instr_id = self.get_instruction_id(instruction);
        self.value_map.insert(instr_id, result_reg);
        
        Ok(assembly)
    }

    /// 生成UDiv指令（无符号除法）
    fn generate_udiv_instruction(&mut self, instruction: &InstructionValue, operands: &[inkwell::Either<BasicValueEnum, BasicBlock>]) -> Result<String, String> {
        if operands.len() < 2 {
            return Err("UDiv instruction needs at least 2 operands".to_string());
        }
        
        let src1_value = operands[0].left().unwrap();
        let src2_value = operands[1].left().unwrap();
        
        let mut assembly = String::new();
        
        // 获取操作数位置
        let src1_loc = self.get_value_location(&src1_value)?;
        let src2_loc = self.get_value_location(&src2_value)?;
        
        // 加载操作数到寄存器
        let reg1 = "w10";
        let reg2 = "w11";
        
        if src1_loc.starts_with("[") {
            assembly.push_str(&format!("\tldr\t{}, {}\n", reg1, src1_loc));
        } else if src1_loc.starts_with("#") {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg1, src1_loc));
        } else {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg1, src1_loc));
        }
        
        if src2_loc.starts_with("[") {
            assembly.push_str(&format!("\tldr\t{}, {}\n", reg2, src2_loc));
        } else if src2_loc.starts_with("#") {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg2, src2_loc));
        } else {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg2, src2_loc));
        }
        
        // 执行无符号除法，使用临时寄存器保存结果
        let result_reg = self.allocate_temp_register();
        assembly.push_str(&format!("\tudiv\t{}, {}, {}\n", result_reg, reg1, reg2));
        
        // 记录结果
        let instr_id = self.get_instruction_id(instruction);
        self.value_map.insert(instr_id, result_reg);
        
        Ok(assembly)
    }

    /// 生成ICmp指令（整数比较）
    fn generate_icmp_instruction(&mut self, instruction: &InstructionValue, operands: &[inkwell::Either<BasicValueEnum, BasicBlock>]) -> Result<String, String> {
        if operands.len() < 2 {
            return Err("ICmp instruction needs at least 2 operands".to_string());
        }
        
        let src1_value = operands[0].left().unwrap();
        let src2_value = operands[1].left().unwrap();
        
        let mut assembly = String::new();
        
        // 获取操作数位置
        let src1_loc = self.get_value_location(&src1_value)?;
        let src2_loc = self.get_value_location(&src2_value)?;
        
        // 加载操作数到寄存器
        let reg1 = "w10";
        let reg2 = "w11";
        
        if src1_loc.starts_with("[") {
            assembly.push_str(&format!("\tldr\t{}, {}\n", reg1, src1_loc));
        } else if src1_loc.starts_with("#") {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg1, src1_loc));
        } else {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg1, src1_loc));
        }
        
        if src2_loc.starts_with("[") {
            assembly.push_str(&format!("\tldr\t{}, {}\n", reg2, src2_loc));
        } else if src2_loc.starts_with("#") {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg2, src2_loc));
        } else {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg2, src2_loc));
        }
        
        // 执行比较并设置条件码
        assembly.push_str(&format!("\tcmp\t{}, {}\n", reg1, reg2));
        
        // 获取比较谓词并生成相应的 cset 指令
        let predicate = instruction.get_icmp_predicate().ok_or("ICmp missing predicate")?;
        let condition = match predicate {
            inkwell::IntPredicate::EQ => "eq",   // equal
            inkwell::IntPredicate::NE => "ne",   // not equal
            inkwell::IntPredicate::SGT => "gt",  // signed greater than
            inkwell::IntPredicate::SGE => "ge",  // signed greater or equal
            inkwell::IntPredicate::SLT => "lt",  // signed less than
            inkwell::IntPredicate::SLE => "le",  // signed less or equal
            inkwell::IntPredicate::UGT => "hi",  // unsigned higher
            inkwell::IntPredicate::UGE => "hs",  // unsigned higher or same
            inkwell::IntPredicate::ULT => "lo",  // unsigned lower
            inkwell::IntPredicate::ULE => "ls",  // unsigned lower or same
        };
        
        // 根据条件设置结果寄存器（0 或 1），使用临时寄存器
        let result_reg = self.allocate_temp_register();
        assembly.push_str(&format!("\tcset\t{}, {}\n", result_reg, condition));
        
        // 记录结果
        let instr_id = self.get_instruction_id(instruction);
        self.value_map.insert(instr_id, result_reg);
        
        Ok(assembly)
    }

    /// 生成And指令（位与/逻辑与）
    fn generate_and_instruction(&mut self, instruction: &InstructionValue, operands: &[inkwell::Either<BasicValueEnum, BasicBlock>]) -> Result<String, String> {
        if operands.len() < 2 {
            return Err("And instruction needs at least 2 operands".to_string());
        }
        
        let src1_value = operands[0].left().unwrap();
        let src2_value = operands[1].left().unwrap();
        
        let mut assembly = String::new();
        
        // 获取操作数位置
        let src1_loc = self.get_value_location(&src1_value)?;
        let src2_loc = self.get_value_location(&src2_value)?;
        
        // 加载操作数到寄存器
        let reg1 = "w10";
        let reg2 = "w11";
        
        if src1_loc.starts_with("[") {
            assembly.push_str(&format!("\tldr\t{}, {}\n", reg1, src1_loc));
        } else if src1_loc.starts_with("#") {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg1, src1_loc));
        } else {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg1, src1_loc));
        }
        
        if src2_loc.starts_with("[") {
            assembly.push_str(&format!("\tldr\t{}, {}\n", reg2, src2_loc));
        } else if src2_loc.starts_with("#") {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg2, src2_loc));
        } else {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg2, src2_loc));
        }
        
        // 执行位与操作，使用临时寄存器保存结果
        let result_reg = self.allocate_temp_register();
        assembly.push_str(&format!("\tand\t{}, {}, {}\n", result_reg, reg1, reg2));
        
        // 记录结果
        let instr_id = self.get_instruction_id(instruction);
        self.value_map.insert(instr_id, result_reg);
        
        Ok(assembly)
    }

    /// 生成Or指令（位或/逻辑或）
    fn generate_or_instruction(&mut self, instruction: &InstructionValue, operands: &[inkwell::Either<BasicValueEnum, BasicBlock>]) -> Result<String, String> {
        if operands.len() < 2 {
            return Err("Or instruction needs at least 2 operands".to_string());
        }
        
        let src1_value = operands[0].left().unwrap();
        let src2_value = operands[1].left().unwrap();
        
        let mut assembly = String::new();
        
        // 获取操作数位置
        let src1_loc = self.get_value_location(&src1_value)?;
        let src2_loc = self.get_value_location(&src2_value)?;
        
        // 加载操作数到寄存器
        let reg1 = "w10";
        let reg2 = "w11";
        
        if src1_loc.starts_with("[") {
            assembly.push_str(&format!("\tldr\t{}, {}\n", reg1, src1_loc));
        } else if src1_loc.starts_with("#") {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg1, src1_loc));
        } else {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg1, src1_loc));
        }
        
        if src2_loc.starts_with("[") {
            assembly.push_str(&format!("\tldr\t{}, {}\n", reg2, src2_loc));
        } else if src2_loc.starts_with("#") {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg2, src2_loc));
        } else {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg2, src2_loc));
        }
        
        // 执行位或操作，使用临时寄存器保存结果
        let result_reg = self.allocate_temp_register();
        assembly.push_str(&format!("\torr\t{}, {}, {}\n", result_reg, reg1, reg2));
        
        // 记录结果
        let instr_id = self.get_instruction_id(instruction);
        self.value_map.insert(instr_id, result_reg);
        
        Ok(assembly)
    }

    /// 生成Xor指令（异或，用于 Not 运算）
    fn generate_xor_instruction(&mut self, instruction: &InstructionValue, operands: &[inkwell::Either<BasicValueEnum, BasicBlock>]) -> Result<String, String> {
        if operands.len() < 2 {
            return Err("Xor instruction needs at least 2 operands".to_string());
        }
        
            let src1_value = operands[0].left().unwrap();
            let src2_value = operands[1].left().unwrap();
        
        let mut assembly = String::new();
        
        // 获取操作数位置
        let src1_loc = self.get_value_location(&src1_value)?;
        let src2_loc = self.get_value_location(&src2_value)?;
        
        // 加载操作数到寄存器
        let reg1 = "w10";
        let reg2 = "w11";
        
        if src1_loc.starts_with("[") {
            assembly.push_str(&format!("\tldr\t{}, {}\n", reg1, src1_loc));
        } else if src1_loc.starts_with("#") {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg1, src1_loc));
        } else {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg1, src1_loc));
        }
        
        if src2_loc.starts_with("[") {
            assembly.push_str(&format!("\tldr\t{}, {}\n", reg2, src2_loc));
        } else if src2_loc.starts_with("#") {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg2, src2_loc));
        } else {
            assembly.push_str(&format!("\tmov\t{}, {}\n", reg2, src2_loc));
        }
        
        // 执行异或操作，使用临时寄存器保存结果
        let result_reg = self.allocate_temp_register();
        assembly.push_str(&format!("\teor\t{}, {}, {}\n", result_reg, reg1, reg2));
        
        // 记录结果
        let instr_id = self.get_instruction_id(instruction);
        self.value_map.insert(instr_id, result_reg);
        
        Ok(assembly)
    }

    /// 生成Call指令（函数调用）
    fn generate_call_instruction(&mut self, instruction: &InstructionValue, operands: &[inkwell::Either<BasicValueEnum, BasicBlock>]) -> Result<String, String> {
        let mut assembly = String::new();
        
        // Call 指令的最后一个操作数是被调用的函数，前面的是参数
        // 注意：实际参数数量可能少于 operands.len()，因为最后一个是函数指针
        
        if operands.is_empty() {
            return Err("Call instruction needs at least a function operand".to_string());
        }
        
        // 获取函数名（通过 instruction 本身获取）
        let call_str = instruction.print_to_string().to_string();
        let func_name = if let Some(start) = call_str.find("@") {
            let end = call_str[start..].find("(").unwrap_or(call_str.len() - start);
            call_str[start+1..start+end].to_string()
        } else {
            "unknown".to_string()
        };
        
        if self.debug_mode {
            println!("DEBUG: Call to function: {}", func_name);
            println!("DEBUG: Call has {} operands", operands.len());
        }
        
        // ARM64 AAPCS64 调用约定：
        // - 前8个整数参数通过 x0-x7 (w0-w7) 传递
        // - 返回值通过 x0 (w0) 返回
        
        // Call 指令的操作数：最后一个是函数，前面的是参数
        let param_count = if operands.len() > 0 { operands.len() - 1 } else { 0 };
        
        if self.debug_mode {
            println!("DEBUG:   Detected {} parameters", param_count);
        }
        
        // 加载参数到寄存器 w0-w7
        for i in 0..param_count.min(8) {
            if let inkwell::Either::Left(param_value) = &operands[i] {
                let param_reg = format!("w{}", i);
                
                // 跳过函数指针，只处理实际参数
                if let BasicValueEnum::PointerValue(ptr) = param_value {
                    if ptr.is_const() {
                        // 这可能是函数指针，跳过
                        if self.debug_mode {
                            println!("DEBUG:   operand[{}] is function pointer, skipping", i);
                        }
                        continue;
                    }
                }
                
                let param_loc = self.get_value_location(param_value)?;
                
                if self.debug_mode {
                    println!("DEBUG:   param[{}] = {} → {}", i, param_loc, param_reg);
                }
                
                // 加载参数到参数寄存器
                if param_loc.starts_with("[") {
                    assembly.push_str(&format!("\tldr\t{}, {}\n", param_reg, param_loc));
                } else if param_loc.starts_with("#") {
                    assembly.push_str(&format!("\tmov\t{}, {}\n", param_reg, param_loc));
        } else {
                    assembly.push_str(&format!("\tmov\t{}, {}\n", param_reg, param_loc));
                }
            }
        }
        
        // 调用函数
        assembly.push_str(&format!("\tbl\t{}\n", func_name));
        
        // 函数返回值在 w0 中，保存到临时寄存器
        let result_reg = self.allocate_temp_register();
        assembly.push_str(&format!("\tmov\t{}, w0\n", result_reg));
        
        // 记录返回值位置
        let instr_id = self.get_instruction_id(instruction);
        self.value_map.insert(instr_id, result_reg);
        
        Ok(assembly)
    }

    /// 生成Br指令（分支）
    fn generate_br_instruction(&mut self, _instruction: &InstructionValue, operands: &[inkwell::Either<BasicValueEnum, BasicBlock>]) -> Result<String, String> {
        let mut assembly = String::new();
        
        if self.debug_mode {
            println!("DEBUG: Br instruction with {} operands", operands.len());
            for (i, op) in operands.iter().enumerate() {
                match op {
                    inkwell::Either::Left(_) => println!("DEBUG:   operand[{}] = value", i),
                    inkwell::Either::Right(bb) => println!("DEBUG:   operand[{}] = block {}", i, bb.get_name().to_str().unwrap_or("?")),
                }
            }
        }
        
        if operands.len() == 1 {
            // 无条件分支
            if let inkwell::Either::Right(target_block) = &operands[0] {
                let block_name = target_block.get_name().to_str().unwrap_or("unknown");
                assembly.push_str(&format!("\tb\t.L{}\n", block_name));
            }
        } else if operands.len() == 3 {
            // 条件分支: br i1 %cond, label %true_block, label %false_block
            // 需要找出哪个是条件，哪些是基本块
            let mut cond_value = None;
            let mut blocks = vec![];
            
            for op in operands.iter() {
                match op {
                    inkwell::Either::Left(val) => cond_value = Some(*val),
                    inkwell::Either::Right(bb) => blocks.push(bb),
                }
            }
            
            if blocks.len() != 2 {
                return Err(format!("Conditional branch needs 2 blocks, found {}", blocks.len()));
            }
            
            let cond_value = cond_value.ok_or("Conditional branch missing condition")?;
            
            // LLVM br 指令的操作数顺序: operands[0]=cond, operands[1]=false, operands[2]=true
            // 所以 blocks[0] 是 false 分支，blocks[1] 是 true 分支
            let false_block = blocks[0];
            let true_block = blocks[1];
            
            let true_name = true_block.get_name().to_str().unwrap_or("unknown");
            let false_name = false_block.get_name().to_str().unwrap_or("unknown");
            
            if self.debug_mode {
                println!("DEBUG: Conditional branch: true={}, false={}", true_name, false_name);
            }
            
            // 获取条件值
            let cond_loc = self.get_value_location(&cond_value)?;
            
            // 加载条件到寄存器
            if cond_loc.starts_with("[") {
                assembly.push_str(&format!("\tldr\tw10, {}\n", cond_loc));
            } else if cond_loc.starts_with("#") {
                assembly.push_str(&format!("\tmov\tw10, {}\n", cond_loc));
            } else {
                assembly.push_str(&format!("\tmov\tw10, {}\n", cond_loc));
            }
            
            // 比较条件值是否为真（非零）
            assembly.push_str("\tcmp\tw10, #0\n");
            
            // 条件分支：不为0跳到 true_block
            assembly.push_str(&format!("\tb.ne\t.L{}\n", true_name));
            
            // 为0（假）则跳转到 false 分支
            assembly.push_str(&format!("\tb\t.L{}\n", false_name));
        }
        
        Ok(assembly)
    }

    /// 获取值的存储位置（栈地址或寄存器）
    fn get_value_location(&self, value: &BasicValueEnum) -> Result<String, String> {
        let value_id = self.get_value_id(value);
        
        if self.debug_mode {
            println!("DEBUG: get_value_location for value_id: {}", value_id);
        }
        
        // 1. 检查是否是 alloca 指令的结果（指针）
        if let BasicValueEnum::PointerValue(_) = value {
            // 尝试在 alloca_map 中查找
            if let Some(offset) = self.alloca_map.get(&value_id) {
                if self.debug_mode {
                    println!("DEBUG: Found in alloca_map: [sp, #{}]", offset);
                }
                return Ok(format!("[sp, #{}]", offset));
            }
            
            if self.debug_mode {
                println!("DEBUG: Not found in alloca_map. Available keys:");
                for key in self.alloca_map.keys() {
                    println!("DEBUG:   - {}", key);
                }
            }
        }
        
        // 2. 检查是否是指令的计算结果（在 value_map 中）
        if let Some(reg) = self.value_map.get(&value_id) {
            if self.debug_mode {
                println!("DEBUG: Found in value_map: {}", reg);
            }
            return Ok(reg.clone());
        }
        
        // 3. 检查是否是立即数
        if let BasicValueEnum::IntValue(int_val) = value {
                if int_val.is_const() {
                    if let Some(const_val) = int_val.get_zero_extended_constant() {
                    if self.debug_mode {
                        println!("DEBUG: Found constant: #{}", const_val);
                    }
                    return Ok(format!("#{}", const_val));
                }
            }
        }
        
        // 4. 未找到，返回错误
        Err(format!("Cannot find location for value: {:?}", value))
    }

    /// 生成Return指令
    fn generate_return_instruction(&mut self, _instruction: &InstructionValue, operands: &[inkwell::Either<BasicValueEnum, BasicBlock>]) -> Result<String, String> {
        let mut assembly = String::new();
        
        if !operands.is_empty() {
            let return_value = operands[0].left().unwrap();
            
            // 获取返回值的位置
            let value_location = self.get_value_location(&return_value)?;
            
            // 将返回值加载到 w0 寄存器
            if value_location.starts_with("[") {
                // 从内存加载
                assembly.push_str(&format!("\tldr\tw0, {}\n", value_location));
            } else if value_location.starts_with("#") {
                // 立即数
                assembly.push_str(&format!("\tmov\tw0, {}\n", value_location));
        } else {
                // 寄存器
                assembly.push_str(&format!("\tmov\tw0, {}\n", value_location));
            }
        }
        
        // 添加函数尾声
        // 恢复栈指针（如果有局部变量）
        if self.current_stack_offset > 0 {
            let aligned_stack_size = ((self.current_stack_offset + 15) / 16) * 16;
            assembly.push_str(&format!("\tadd\tsp, sp, #{}\n", aligned_stack_size));
        }
        
        // 恢复帧指针和返回地址
        assembly.push_str("\tldp\tx29, x30, [sp], #16\n");
        assembly.push_str("\tret\n");
        
        Ok(assembly)
    }
}
