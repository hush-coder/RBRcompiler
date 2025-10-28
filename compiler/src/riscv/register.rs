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
                "t0".to_string(), "t1".to_string(), "t2".to_string(), "t3".to_string(),
                "t4".to_string(), "t5".to_string(), "t6".to_string(),
                "a0".to_string(), "a1".to_string(), "a2".to_string(), "a3".to_string(),
                "a4".to_string(), "a5".to_string(), "a6".to_string(), "a7".to_string(),
            ],
            next_temp: 0,
        }
    }

    pub fn allocate_register(&mut self, var: &str) -> String {
        if let Some(reg) = self.allocated.get(var) {
            reg.clone()
        } else {
            let reg = self.get_next_temp_register();
            self.allocated.insert(var.to_string(), reg.clone());
            reg
        }
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
                    return "t0".to_string(); // 默认临时寄存器
                }
                // 其他情况返回默认寄存器
                "t0".to_string()
            })
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