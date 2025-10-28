use std::collections::{HashMap, HashSet};
use inkwell::values::BasicValueEnum;
use super::arm::ArmRegister;

/// 寄存器分配器
/// 负责管理虚拟寄存器到物理寄存器的映射
pub struct RegisterAllocator {
    /// 跟踪虚拟寄存器到物理寄存器的映射
    current_mapping: HashMap<String, ArmRegister>,
    /// 跟踪寄存器何时被占用
    is_busy: HashSet<ArmRegister>,
    /// 跟踪溢出到栈上的变量及其栈偏移量
    spilled_vars: HashMap<String, i32>,
    /// 可用的Caller-Saved寄存器列表
    available_regs: Vec<ArmRegister>,
    /// 当前栈偏移量
    stack_offset: i32,
    /// 临时寄存器计数器
    temp_counter: u32,
}

impl RegisterAllocator {
    /// 创建新的寄存器分配器
    pub fn new() -> Self {
        Self {
            current_mapping: HashMap::new(),
            is_busy: HashSet::new(),
            spilled_vars: HashMap::new(),
            available_regs: vec![
                ArmRegister::X0, ArmRegister::X1, ArmRegister::X2, ArmRegister::X3,
                ArmRegister::X4, ArmRegister::X5, ArmRegister::X6, ArmRegister::X7,
                ArmRegister::X8, ArmRegister::X9, ArmRegister::X10, ArmRegister::X11,
                ArmRegister::X12, ArmRegister::X13, ArmRegister::X14, ArmRegister::X15,
            ],
            stack_offset: 0,
            temp_counter: 0,
        }
    }

    /// 为LLVM值分配寄存器
    pub fn allocate_register_for_value(&mut self, value: &BasicValueEnum) -> ArmRegister {
        let value_id = self.get_value_id(value);
        
        // 检查是否已经分配了寄存器
        if let Some(reg) = self.current_mapping.get(&value_id) {
            return *reg;
        }
        
        // 分配新寄存器
        let reg = self.allocate_available_register();
        self.current_mapping.insert(value_id, reg);
        reg
    }

    /// 获取LLVM值的唯一标识符
    fn get_value_id(&mut self, value: &BasicValueEnum) -> String {
        match value {
            BasicValueEnum::IntValue(int_val) => {
                if int_val.is_const() {
                    if let Some(const_val) = int_val.get_zero_extended_constant() {
                        return format!("const_{}", const_val);
                    }
                }
                // 使用简单的计数器来生成唯一ID
                self.temp_counter += 1;
                format!("int_{}", self.temp_counter)
            }
            BasicValueEnum::PointerValue(_ptr_val) => {
                self.temp_counter += 1;
                format!("ptr_{}", self.temp_counter)
            }
            _ => {
                self.temp_counter += 1;
                format!("val_{}", self.temp_counter)
            }
        }
    }

    /// 分配一个可用的寄存器
    pub fn allocate_available_register(&mut self) -> ArmRegister {
        // 寻找第一个可用的寄存器
        for &reg in &self.available_regs {
            if !self.is_busy.contains(&reg) {
                self.is_busy.insert(reg);
                return reg;
            }
        }
        
        // 如果没有可用寄存器，选择第一个寄存器进行溢出
        let reg_to_spill = self.available_regs[0];
        self.spill_register(reg_to_spill);
        self.is_busy.insert(reg_to_spill);
        reg_to_spill
    }

    /// 将寄存器内容溢出到栈上
    fn spill_register(&mut self, reg: ArmRegister) {
        // 找到使用这个寄存器的值
        let value_to_spill = self.current_mapping.iter()
            .find(|(_, &r)| r == reg)
            .map(|(k, _)| k.clone());
        
        if let Some(value_id) = value_to_spill {
            // 分配栈空间
            self.stack_offset += 4; // 假设32位整数
            self.spilled_vars.insert(value_id.clone(), self.stack_offset);
            
            // 从当前映射中移除
            self.current_mapping.remove(&value_id);
        }
    }

    /// 获取值的存储位置（寄存器或栈偏移）
    pub fn get_value_location(&self, value: &BasicValueEnum) -> String {
        let value_id = self.get_value_id_for_lookup(value);
        
        // 检查是否在寄存器中
        if let Some(reg) = self.current_mapping.get(&value_id) {
            return reg.to_32bit();
        }
        
        // 检查是否在栈上
        if let Some(offset) = self.spilled_vars.get(&value_id) {
            return format!("[sp, #{}]", offset);
        }
        
        // 如果都没找到，返回一个临时寄存器
        "w0".to_string()
    }

    /// 获取值的唯一标识符（用于查找）
    pub fn get_value_id_for_lookup(&self, value: &BasicValueEnum) -> String {
        match value {
            BasicValueEnum::IntValue(int_val) => {
                if int_val.is_const() {
                    if let Some(const_val) = int_val.get_zero_extended_constant() {
                        return format!("const_{}", const_val);
                    }
                }
                // 使用简单的计数器作为唯一标识符
                format!("int_{}", self.temp_counter)
            }
            BasicValueEnum::PointerValue(_ptr_val) => {
                format!("ptr_{}", self.temp_counter)
            }
            _ => {
                format!("val_{}", self.temp_counter)
            }
        }
    }

    /// 释放寄存器
    pub fn release_register(&mut self, reg: ArmRegister) {
        self.is_busy.remove(&reg);
    }

    /// 获取栈偏移量
    pub fn get_stack_offset(&self) -> i32 {
        self.stack_offset
    }

    /// 分配栈空间
    pub fn allocate_stack_space(&mut self, size: i32) -> i32 {
        self.stack_offset += size;
        self.stack_offset
    }

    /// 获取下一个指令ID
    pub fn get_next_instruction_id(&mut self) -> u32 {
        self.temp_counter += 1;
        self.temp_counter
    }

    /// 注册指令结果
    pub fn register_instruction_result(&mut self, instruction_id: &str, reg: ArmRegister) {
        self.current_mapping.insert(instruction_id.to_string(), reg);
    }

    /// 获取值的寄存器
    pub fn get_register_for_value(&self, value_id: &str) -> Option<ArmRegister> {
        self.current_mapping.get(value_id).copied()
    }

    /// 调试：输出当前状态
    pub fn debug_state(&self) -> String {
        let mut debug = String::new();
        debug.push_str("# DEBUG: Register Allocator State\n");
        debug.push_str(&format!("# Stack offset: {}\n", self.stack_offset));
        debug.push_str("# Current mappings:\n");
        for (value_id, reg) in &self.current_mapping {
            debug.push_str(&format!("#   {} -> {}\n", value_id, reg.to_32bit()));
        }
        debug.push_str("# Spilled variables:\n");
        for (value_id, offset) in &self.spilled_vars {
            debug.push_str(&format!("#   {} -> [sp, #{}]\n", value_id, offset));
        }
        debug
    }
}
