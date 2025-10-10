use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum IRInstruction {
    // 算术指令
    Add { dst: String, src1: String, src2: String },
    Sub { dst: String, src1: String, src2: String },
    Mul { dst: String, src1: String, src2: String },
    Div { dst: String, src1: String, src2: String },
    
    // 比较指令
    Cmp { dst: String, src1: String, src2: String },
    CmpEq { dst: String, src1: String, src2: String },
    CmpNe { dst: String, src1: String, src2: String },
    CmpLt { dst: String, src1: String, src2: String },
    CmpLe { dst: String, src1: String, src2: String },
    CmpGt { dst: String, src1: String, src2: String },
    CmpGe { dst: String, src1: String, src2: String },
    
    // 跳转指令
    Jump { target: String },
    JumpIf { condition: String, target: String },
    JumpIfNot { condition: String, target: String },
    
    // 内存操作
    Load { dst: String, addr: String },
    Store { addr: String, src: String },
    Alloca { dst: String, size: u32 },
    
    // 函数调用
    Call { dst: Option<String>, func: String, args: Vec<String> },
    Ret { value: Option<String> },
    
    // 移动指令
    Move { dst: String, src: String },
    
    // 常量加载
    LoadConst { dst: String, value: i64 },
    LoadString { dst: String, value: String },
    
    // 标签
    Label { name: String },
    
    // 空操作
    Nop,
}

impl fmt::Display for IRInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IRInstruction::Add { dst, src1, src2 } => {
                write!(f, "  {} = add {} {}", dst, src1, src2)
            }
            IRInstruction::Sub { dst, src1, src2 } => {
                write!(f, "  {} = sub {} {}", dst, src1, src2)
            }
            IRInstruction::Mul { dst, src1, src2 } => {
                write!(f, "  {} = mul {} {}", dst, src1, src2)
            }
            IRInstruction::Div { dst, src1, src2 } => {
                write!(f, "  {} = div {} {}", dst, src1, src2)
            }
            IRInstruction::Cmp { dst, src1, src2 } => {
                write!(f, "  {} = cmp {} {}", dst, src1, src2)
            }
            IRInstruction::CmpEq { dst, src1, src2 } => {
                write!(f, "  {} = cmpeq {} {}", dst, src1, src2)
            }
            IRInstruction::CmpNe { dst, src1, src2 } => {
                write!(f, "  {} = cmpne {} {}", dst, src1, src2)
            }
            IRInstruction::CmpLt { dst, src1, src2 } => {
                write!(f, "  {} = cmplt {} {}", dst, src1, src2)
            }
            IRInstruction::CmpLe { dst, src1, src2 } => {
                write!(f, "  {} = cmple {} {}", dst, src1, src2)
            }
            IRInstruction::CmpGt { dst, src1, src2 } => {
                write!(f, "  {} = cmpgt {} {}", dst, src1, src2)
            }
            IRInstruction::CmpGe { dst, src1, src2 } => {
                write!(f, "  {} = cmpge {} {}", dst, src1, src2)
            }
            IRInstruction::Jump { target } => {
                write!(f, "  jump {}", target)
            }
            IRInstruction::JumpIf { condition, target } => {
                write!(f, "  jumpif {} {}", condition, target)
            }
            IRInstruction::JumpIfNot { condition, target } => {
                write!(f, "  jumpifnot {} {}", condition, target)
            }
            IRInstruction::Load { dst, addr } => {
                write!(f, "  {} = load {}", dst, addr)
            }
            IRInstruction::Store { addr, src } => {
                write!(f, "  store {} {}", addr, src)
            }
            IRInstruction::Alloca { dst, size } => {
                write!(f, "  {} = alloca {}", dst, size)
            }
            IRInstruction::Call { dst, func, args } => {
                let args_str = args.join(", ");
                if let Some(dst) = dst {
                    write!(f, "  {} = call {} ({})", dst, func, args_str)
                } else {
                    write!(f, "  call {} ({})", func, args_str)
                }
            }
            IRInstruction::Ret { value } => {
                if let Some(value) = value {
                    write!(f, "  ret {}", value)
                } else {
                    write!(f, "  ret")
                }
            }
            IRInstruction::Move { dst, src } => {
                write!(f, "  {} = move {}", dst, src)
            }
            IRInstruction::LoadConst { dst, value } => {
                write!(f, "  {} = const {}", dst, value)
            }
            IRInstruction::LoadString { dst, value } => {
                write!(f, "  {} = string \"{}\"", dst, value)
            }
            IRInstruction::Label { name } => {
                write!(f, "{}:", name)
            }
            IRInstruction::Nop => {
                write!(f, "  nop")
            }
        }
    }
} 