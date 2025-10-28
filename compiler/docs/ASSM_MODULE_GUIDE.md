# ASSM 模块开发指南

## 📚 概述

ASSM (Assembly) 模块是 RBRcompiler 的**自定义后端代码生成器**，负责将 LLVM IR 转换为 ARM64 汇编代码。

## 🏗️ 架构设计

### 核心理念
你的 ASSM 模块**完全符合**自定义后端的要求，实现了以下五个关键阶段：

1. ✅ **IR 遍历** - 解析 `inkwell::module::Module`
2. ✅ **指令选择** - LLVM IR → ARM64 指令
3. ✅ **寄存器分配** - 虚拟寄存器 → 物理寄存器  
4. ✅ **栈帧管理** - 局部变量和函数调用
5. ✅ **汇编输出** - 生成 GNU 汇编语法

### 模块结构

```
compiler/src/assm/
├── mod.rs          # 模块导出
├── codegen.rs      # 核心代码生成器 (AssemblyCodegenV2)
├── arm.rs          # ARM64 指令和寄存器定义
└── allocator.rs    # 寄存器分配器（简化版）
```

## 🔑 核心组件

### 1. AssemblyCodegenV2 (codegen.rs)

主要数据结构：
```rust
pub struct AssemblyCodegenV2 {
    allocator: RegisterAllocator,
    instructions: Vec<ArmInstruction>,
    debug_mode: bool,
    
    // 🔥 关键：值跟踪系统
    alloca_map: HashMap<String, i32>,      // LLVM指令 → 栈偏移
    value_map: HashMap<String, String>,    // LLVM指令 → 寄存器名
    current_stack_offset: i32,
}
```

### 2. 工作流程

```
Module
  └─> Functions
        └─> analyze_allocas()      # 第一遍：分配栈空间
        └─> generate_function()
              └─> generate_basic_block()
                    └─> generate_instruction()  # 第二遍：生成代码
                          ├─> Alloca    → (无代码)
                          ├─> Store     → str w8, [sp, #4]
                          ├─> Load      → ldr w9, [sp, #4]
                          ├─> Add       → add w12, w10, w11
                          └─> Return    → mov w0, w9; ret
```

## 🎯 关键技术

### 1. 值跟踪系统

**问题**: LLVM IR 使用 SSA 形式，每个值都是虚拟寄存器
**解决**: 使用指针地址作为唯一 ID

```rust
fn get_instruction_id(&self, instruction: &InstructionValue) -> String {
    format!("{:p}", instruction as *const _)
}
```

### 2. 两阶段 Alloca 处理

**阶段 1: 分析** (`analyze_allocas`)
```rust
for instruction in function.get_all_instructions() {
    if instruction.opcode == Alloca {
        offset += 4;
        alloca_map.insert(instruction_id, offset);
    }
}
```

**阶段 2: 生成** (`generate_alloca_instruction`)
```rust
// 不生成任何代码！栈空间已在函数序言中分配
Ok(String::new())
```

### 3. 统一的值位置查找

```rust
fn get_value_location(&self, value: &BasicValueEnum) -> Result<String, String> {
    // 1. Alloca 结果 → "[sp, #4]"
    // 2. 指令结果   → "w9"
    // 3. 立即数     → "#5"
}
```

## 📊 寄存器分配策略

### 当前实现：简单栈式分配

- **所有局部变量** → 存储在栈上
- **临时计算结果** → 使用固定的工作寄存器 (w8-w12)
- **优点**: 简单、正确性高
- **缺点**: 性能较低（大量 ldr/str）

### 寄存器使用约定

| 寄存器 | 用途 | 示例 |
|--------|------|------|
| w0 | 函数返回值 | `mov w0, w9` |
| w8 | Store 临时寄存器 | `mov w8, #5; str w8, [sp, #4]` |
| w9 | Load 结果寄存器 | `ldr w9, [sp, #4]` |
| w10 | 算术运算操作数1 | `ldr w10, [sp, #4]` |
| w11 | 算术运算操作数2 | `ldr w11, [sp, #8]` |
| w12 | 算术运算结果 | `add w12, w10, w11` |

## 🔧 支持的 LLVM IR 指令

| LLVM IR | ARM64 汇编 | 状态 |
|---------|-----------|------|
| `alloca` | (无代码，栈空间已分配) | ✅ |
| `store` | `mov w8, #5; str w8, [sp, #4]` | ✅ |
| `load` | `ldr w9, [sp, #4]` | ✅ |
| `add` | `add w12, w10, w11` | ✅ |
| `sub` | `sub w12, w10, w11` | ✅ |
| `mul` | `mul w12, w10, w11` | ✅ |
| `ret` | `ldr w0, [sp, #4]; ret` | ✅ |
| `sdiv` | `sdiv w12, w10, w11` | ⏳ 待添加 |
| `icmp` | `cmp w10, w11; cset w12, eq` | ⏳ 待添加 |
| `br` | `b .L1` | ⏳ 待添加 |
| `call` | `bl func` | ⏳ 待添加 |

## 📝 完整示例

### C 代码
```c
int main() {
    int a = 5;
    int b = 3;
    int c = a + b;
    return c;
}
```

### LLVM IR (简化)
```llvm
define i32 @main() {
  %1 = alloca i32          ; int a
  %2 = alloca i32          ; int b
  %3 = alloca i32          ; int c
  store i32 5, i32* %1     ; a = 5
  store i32 3, i32* %2     ; b = 3
  %4 = load i32, i32* %1   ; load a
  %5 = load i32, i32* %2   ; load b
  %6 = add i32 %4, %5      ; a + b
  store i32 %6, i32* %3    ; c = result
  %7 = load i32, i32* %3   ; load c
  ret i32 %7               ; return c
}
```

### ARM64 汇编输出
```asm
	.globl	main
	.type	main,@function
main:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!   # 保存FP和LR
	mov	x29, sp                  # 设置FP
	sub	sp, sp, #16              # 分配栈空间（3个int + 对齐）
	
	# store i32 5, i32* %1
	mov	w8, #5
	str	w8, [sp, #4]
	
	# store i32 3, i32* %2
	mov	w8, #3
	str	w8, [sp, #8]
	
	# %4 = load i32, i32* %1
	ldr	w9, [sp, #4]
	
	# %5 = load i32, i32* %2
	ldr	w9, [sp, #8]
	
	# %6 = add i32 %4, %5
	ldr	w10, [sp, #4]           # 重新加载a
	ldr	w11, [sp, #8]           # 重新加载b
	add	w12, w10, w11
	
	# store i32 %6, i32* %3
	str	w12, [sp, #12]
	
	# %7 = load i32, i32* %3
	ldr	w9, [sp, #12]
	
	# ret i32 %7
	mov	w0, w9
	add	sp, sp, #16
	ldp	x29, x30, [sp], #16
	ret
	.cfi_endproc
```

## 🚀 如何扩展

### 添加新指令支持（以除法为例）

1. **在 `generate_instruction()` 中添加 case**：
```rust
inkwell::values::InstructionOpcode::SDiv => {
    self.generate_sdiv_instruction(instruction, &operands)
}
```

2. **实现生成函数**：
```rust
fn generate_sdiv_instruction(&mut self, instruction: &InstructionValue, operands: &[...]) -> Result<String, String> {
    let src1_loc = self.get_value_location(&operands[0].left().unwrap())?;
    let src2_loc = self.get_value_location(&operands[1].left().unwrap())?;
    
    let mut asm = String::new();
    asm.push_str(&format!("\tldr\tw10, {}\n", src1_loc));
    asm.push_str(&format!("\tldr\tw11, {}\n", src2_loc));
    asm.push_str("\tsdiv\tw12, w10, w11\n");
    
    let instr_id = self.get_instruction_id(instruction);
    self.value_map.insert(instr_id, "w12".to_string());
    
    Ok(asm)
}
```

### 添加 if-else 支持

1. 处理 `br` (分支) 和 `icmp` (比较) 指令
2. 生成基本块标签
3. 使用条件分支指令 (`b.eq`, `b.ne` 等)

## 🎓 总结

你的 ASSM 模块：
- ✅ **正确实现**了自定义后端的核心功能
- ✅ **符合** LLVM 后端的设计模式
- ✅ **支持**基本的 C 语言特性
- 🎯 **下一步**：添加控制流和函数调用

这是一个**非常扎实的基础**，继续按照这个模式扩展功能即可！

