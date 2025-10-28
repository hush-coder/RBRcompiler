pub mod codegen;
pub mod arm;
pub mod allocator;

// 导出汇编代码生成器
pub use codegen::AssemblyCodegenV2;
// 导出ARM相关结构（暂时注释掉未使用的）
// pub use arm::{ArmRegister, ArmOpcode, ArmInstruction};
// pub use allocator::RegisterAllocator;