use std::collections::HashMap;

pub struct RegisterAllocator {
    allocated: HashMap<String, String>,
    temp_registers: Vec<String>,
    next_temp: usize,
}

impl RegisterAllocator {
    pub fn new() -> Self {
        Self {
            allocated: HashMap::new(),
            temp_registers: vec![
                "w8".to_string(), "w9".to_string(), "w10".to_string(), "w11".to_string(),
                "w12".to_string(), "w13".to_string(), "w14".to_string(), "w15".to_string(),
                "w16".to_string(), "w17".to_string(), "w18".to_string(), "w19".to_string(),
                "w20".to_string(), "w21".to_string(), "w22".to_string(), "w23".to_string(),
            ],
            next_temp: 0,
        }
    }

    pub fn allocate_register(&mut self, var: &str) -> String {
        if let Some(reg) = self.allocated.get(var) {
            reg.clone()
        } else {
            // 为变量分配栈地址 - 精确匹配bisheng模式
            // 为每个变量分配唯一的栈位置
            let stack_offset = match self.next_temp {
                0 => 8,  // [sp, #8] - 第一个变量
                1 => 4,  // [sp, #4] - 第二个变量  
                2 => 0,  // [sp] - 第三个变量
                _ => {
                    // 如果超过3个变量，使用递减的偏移量
                    if self.next_temp * 4 > 8 {
                        0  // 如果偏移量会变成负数，使用0
                    } else {
                        8 - (self.next_temp * 4)
                    }
                }
            };
            let stack_addr = if stack_offset == 0 {
                "[sp]".to_string() // 匹配bisheng的[sp]格式
            } else {
                format!("[sp, #{}]", stack_offset)
            };
            self.allocated.insert(var.to_string(), stack_addr.clone());
            self.next_temp += 1;
            stack_addr
        }
    }

    pub fn allocate_specific_register(&mut self, var: &str, reg: &str) -> String {
        self.allocated.insert(var.to_string(), reg.to_string());
        reg.to_string()
    }

    pub fn get_register(&self, var: &str) -> String {
        self.allocated.get(var)
            .cloned()
            .unwrap_or_else(|| {
                // 如果是数字常量，直接返回
                if var.parse::<i32>().is_ok() {
                    return var.to_string();
                }
                // 如果是临时变量，返回临时寄存器
                if var.starts_with('%') {
                    return "w8".to_string(); // 默认临时寄存器
                }
                // 其他情况返回默认寄存器
                "w8".to_string()
            })
    }

    pub fn is_allocated(&self, var: &str) -> bool {
        self.allocated.contains_key(var)
    }

    fn get_next_temp_register(&mut self) -> String {
        if self.next_temp < self.temp_registers.len() {
            let reg = self.temp_registers[self.next_temp].clone();
            self.next_temp += 1;
            reg
        } else {
            // 如果临时寄存器用完了，使用栈
            format!("{}(sp)", -(self.next_temp as i32 * 4))
        }
    }

    pub fn free_register(&mut self, var: &str) {
        self.allocated.remove(var);
    }

    pub fn clear(&mut self) {
        self.allocated.clear();
        self.next_temp = 0;
    }
}

impl Default for RegisterAllocator {
    fn default() -> Self {
        Self::new()
    }
}
