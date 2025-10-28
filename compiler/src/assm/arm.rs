/// ARM64 寄存器定义
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArmRegister {
    // 64位通用寄存器
    X0, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15,
    X16, X17, X18, X19, X20, X21, X22, X23, X24, X25, X26, X27, X28, X29, X30,
    // 特殊寄存器
    SP, LR, FP,
}

impl ArmRegister {
    /// 获取32位寄存器名称 (W0-W30)
    pub fn to_32bit(&self) -> String {
        match self {
            ArmRegister::X0 => "w0".to_string(),
            ArmRegister::X1 => "w1".to_string(),
            ArmRegister::X2 => "w2".to_string(),
            ArmRegister::X3 => "w3".to_string(),
            ArmRegister::X4 => "w4".to_string(),
            ArmRegister::X5 => "w5".to_string(),
            ArmRegister::X6 => "w6".to_string(),
            ArmRegister::X7 => "w7".to_string(),
            ArmRegister::X8 => "w8".to_string(),
            ArmRegister::X9 => "w9".to_string(),
            ArmRegister::X10 => "w10".to_string(),
            ArmRegister::X11 => "w11".to_string(),
            ArmRegister::X12 => "w12".to_string(),
            ArmRegister::X13 => "w13".to_string(),
            ArmRegister::X14 => "w14".to_string(),
            ArmRegister::X15 => "w15".to_string(),
            ArmRegister::X16 => "w16".to_string(),
            ArmRegister::X17 => "w17".to_string(),
            ArmRegister::X18 => "w18".to_string(),
            ArmRegister::X19 => "w19".to_string(),
            ArmRegister::X20 => "w20".to_string(),
            ArmRegister::X21 => "w21".to_string(),
            ArmRegister::X22 => "w22".to_string(),
            ArmRegister::X23 => "w23".to_string(),
            ArmRegister::X24 => "w24".to_string(),
            ArmRegister::X25 => "w25".to_string(),
            ArmRegister::X26 => "w26".to_string(),
            ArmRegister::X27 => "w27".to_string(),
            ArmRegister::X28 => "w28".to_string(),
            ArmRegister::X29 => "w29".to_string(),
            ArmRegister::X30 => "w30".to_string(),
            ArmRegister::SP => "sp".to_string(),
            ArmRegister::LR => "lr".to_string(),
            ArmRegister::FP => "fp".to_string(),
        }
    }

    /// 获取64位寄存器名称 (X0-X30)
    pub fn to_64bit(&self) -> String {
        match self {
            ArmRegister::X0 => "x0".to_string(),
            ArmRegister::X1 => "x1".to_string(),
            ArmRegister::X2 => "x2".to_string(),
            ArmRegister::X3 => "x3".to_string(),
            ArmRegister::X4 => "x4".to_string(),
            ArmRegister::X5 => "x5".to_string(),
            ArmRegister::X6 => "x6".to_string(),
            ArmRegister::X7 => "x7".to_string(),
            ArmRegister::X8 => "x8".to_string(),
            ArmRegister::X9 => "x9".to_string(),
            ArmRegister::X10 => "x10".to_string(),
            ArmRegister::X11 => "x11".to_string(),
            ArmRegister::X12 => "x12".to_string(),
            ArmRegister::X13 => "x13".to_string(),
            ArmRegister::X14 => "x14".to_string(),
            ArmRegister::X15 => "x15".to_string(),
            ArmRegister::X16 => "x16".to_string(),
            ArmRegister::X17 => "x17".to_string(),
            ArmRegister::X18 => "x18".to_string(),
            ArmRegister::X19 => "x19".to_string(),
            ArmRegister::X20 => "x20".to_string(),
            ArmRegister::X21 => "x21".to_string(),
            ArmRegister::X22 => "x22".to_string(),
            ArmRegister::X23 => "x23".to_string(),
            ArmRegister::X24 => "x24".to_string(),
            ArmRegister::X25 => "x25".to_string(),
            ArmRegister::X26 => "x26".to_string(),
            ArmRegister::X27 => "x27".to_string(),
            ArmRegister::X28 => "x28".to_string(),
            ArmRegister::X29 => "x29".to_string(),
            ArmRegister::X30 => "x30".to_string(),
            ArmRegister::SP => "sp".to_string(),
            ArmRegister::LR => "lr".to_string(),
            ArmRegister::FP => "fp".to_string(),
        }
    }
}

/// ARM64 指令操作码
#[derive(Debug, Clone)]
pub enum ArmOpcode {
    // 算术运算
    ADD, SUB, MUL, DIV,
    // 内存操作
    LDR, STR, LDP, STP,
    // 移动指令
    MOV,
    // 分支指令
    B, BL, BR, RET,
    // 比较指令
    CMP, CBZ, CBNZ,
}

/// ARM操作数类型
#[derive(Debug, Clone)]
pub enum Operand {
    Register(ArmRegister),
    Immediate(i64),
    MemoryAddress { base: ArmRegister, offset: i32 },
    Label(String),
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Register(reg) => write!(f, "{}", reg.to_32bit()),
            Operand::Immediate(val) => write!(f, "#{}", val),
            Operand::MemoryAddress { base, offset } => {
                if *offset == 0 {
                    write!(f, "[{}]", base.to_32bit())
                } else {
                    write!(f, "[{}, #{}]", base.to_32bit(), offset)
                }
            }
            Operand::Label(label) => write!(f, "{}", label),
        }
    }
}

/// ARM指令结构
#[derive(Debug, Clone)]
pub struct ArmInstruction {
    pub opcode: ArmOpcode,
    pub operands: Vec<Operand>,
    pub comment: Option<String>,
}

/// ARM汇编输出器
pub struct ArmEmitter {
    output: String,
    indent_level: usize,
}

impl ArmEmitter {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            indent_level: 0,
        }
    }

    pub fn emit_instruction(&mut self, instruction: &ArmInstruction) {
        let assembly = instruction.to_assembly();
        self.output.push_str(&format!("{}\n", assembly));
    }

    pub fn emit_label(&mut self, label: &str) {
        self.output.push_str(&format!("{}:\n", label));
    }

    pub fn emit_comment(&mut self, comment: &str) {
        self.output.push_str(&format!("\t# {}\n", comment));
    }

    pub fn emit_directive(&mut self, directive: &str) {
        self.output.push_str(&format!("\t{}\n", directive));
    }

    pub fn get_output(&self) -> &String {
        &self.output
    }
}

impl ArmInstruction {
    /// 将指令转换为汇编字符串
    pub fn to_assembly(&self) -> String {
        let mut result = String::new();
        
        // 添加注释
        if let Some(comment) = &self.comment {
            result.push_str(&format!("\t# {}\n", comment));
        }
        
        // 生成指令
        result.push_str("\t");
        match &self.opcode {
            ArmOpcode::ADD => {
                if self.operands.len() >= 3 {
                    result.push_str(&format!("add\t{}, {}, {}", 
                        self.operands[0].to_string(), 
                        self.operands[1].to_string(), 
                        self.operands[2].to_string()));
                }
            }
            ArmOpcode::SUB => {
                if self.operands.len() >= 3 {
                    result.push_str(&format!("sub\t{}, {}, {}", 
                        self.operands[0].to_string(), 
                        self.operands[1].to_string(), 
                        self.operands[2].to_string()));
                }
            }
            ArmOpcode::MUL => {
                if self.operands.len() >= 3 {
                    result.push_str(&format!("mul\t{}, {}, {}", 
                        self.operands[0].to_string(), 
                        self.operands[1].to_string(), 
                        self.operands[2].to_string()));
                }
            }
            ArmOpcode::DIV => {
                if self.operands.len() >= 3 {
                    result.push_str(&format!("sdiv\t{}, {}, {}", 
                        self.operands[0].to_string(), 
                        self.operands[1].to_string(), 
                        self.operands[2].to_string()));
                }
            }
            ArmOpcode::LDR => {
                if self.operands.len() >= 2 {
                    result.push_str(&format!("ldr\t{}, [{}]", self.operands[0], self.operands[1]));
                }
            }
            ArmOpcode::STR => {
                if self.operands.len() >= 2 {
                    result.push_str(&format!("str\t{}, [{}]", self.operands[0], self.operands[1]));
                }
            }
            ArmOpcode::LDP => {
                if self.operands.len() >= 3 {
                    result.push_str(&format!("ldp\t{}, {}, [{}]", self.operands[0], self.operands[1], self.operands[2]));
                }
            }
            ArmOpcode::STP => {
                if self.operands.len() >= 3 {
                    result.push_str(&format!("stp\t{}, {}, [{}]", self.operands[0], self.operands[1], self.operands[2]));
                }
            }
            ArmOpcode::MOV => {
                if self.operands.len() >= 2 {
                    result.push_str(&format!("mov\t{}, {}", self.operands[0], self.operands[1]));
                }
            }
            ArmOpcode::B => {
                if !self.operands.is_empty() {
                    result.push_str(&format!("b\t{}", self.operands[0]));
                }
            }
            ArmOpcode::BL => {
                if !self.operands.is_empty() {
                    result.push_str(&format!("bl\t{}", self.operands[0]));
                }
            }
            ArmOpcode::BR => {
                if !self.operands.is_empty() {
                    result.push_str(&format!("br\t{}", self.operands[0]));
                }
            }
            ArmOpcode::RET => {
                result.push_str("ret");
            }
            ArmOpcode::CMP => {
                if self.operands.len() >= 2 {
                    result.push_str(&format!("cmp\t{}, {}", self.operands[0], self.operands[1]));
                }
            }
            ArmOpcode::CBZ => {
                if self.operands.len() >= 2 {
                    result.push_str(&format!("cbz\t{}, {}", self.operands[0], self.operands[1]));
                }
            }
            ArmOpcode::CBNZ => {
                if self.operands.len() >= 2 {
                    result.push_str(&format!("cbnz\t{}, {}", self.operands[0], self.operands[1]));
                }
            }
        }
        
        result.push_str("\n");
        result
    }
}
