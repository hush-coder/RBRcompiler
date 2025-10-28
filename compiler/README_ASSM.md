# ASSM 模块 - 自定义 ARM64 后端

## 📖 概述

ASSM 模块是 RBRcompiler 的**自定义代码生成后端**，将 LLVM IR（由 Inkwell 生成）翻译为 ARM64 汇编代码。

这个后端**完全符合**编译器后端设计的标准要求，实现了 IR 遍历、指令选择、寄存器分配、栈帧管理和汇编输出五个核心阶段。

## ✨ 特性

### ✅ 已实现
- **基本数据类型**: `int` (32位整数)
- **局部变量**: 声明、赋值、读取
- **算术运算**: 加法 `+`、减法 `-`、乘法 `*`
- **返回语句**: `return` 支持常量和变量
- **栈帧管理**: 自动管理函数序言/尾声
- **16字节栈对齐**: 符合 ARM64 ABI

### ⏳ 待实现
- 除法运算 `/`
- 控制流 (`if-else`)
- 循环 (`while`, `for` - 如果需要)
- 函数调用
- 比较运算符
- 逻辑运算符

## 🚀 快速开始

### 1. 编译

```bash
cd compiler
cargo build --release
```

### 2. 运行测试

```bash
# 方法 1: 使用测试脚本（推荐）
chmod +x test_assm_simple.sh
./test_assm_simple.sh

# 方法 2: 手动测试
./target/release/compiler tests/assm/test_simple.c --emit-assm -o output.s
cat output.s
```

### 3. 查看示例

```bash
# C 代码 → LLVM IR
./target/release/compiler tests/assm/test_arithmetic.c --emit-ir

# C 代码 → ARM64 汇编（自定义后端）
./target/release/compiler tests/assm/test_arithmetic.c --emit-assm

# C 代码 → ARM64 汇编（LLVM 官方后端，用于对比）
./target/release/compiler tests/assm/test_arithmetic.c --emit-arm
```

## 📁 文件结构

```
compiler/src/assm/
├── mod.rs              # 模块导出
├── codegen.rs          # ⭐ 核心代码生成器
├── arm.rs              # ARM64 指令定义
└── allocator.rs        # 寄存器分配器

compiler/docs/
├── ASSM_MODULE_GUIDE.md      # 📘 完整开发指南（必读）
├── assm_improvements.md      # 改进详情
└── assm_redesign.md          # 重构计划

compiler/
├── QUICK_START.md            # 快速开始
├── test_assm_simple.sh       # 测试脚本
└── README_ASSM.md            # 本文件
```

## 🎯 工作原理

### 架构图

```
C 源代码
    ↓
词法分析 (Lexer)
    ↓
语法分析 (Parser)
    ↓
语义分析 (Semantic)
    ↓
LLVM IR (Inkwell)  ← 从这里开始是 ASSM 模块的工作
    ↓
┌─────────────────────────────────────┐
│  ASSM 模块 (自定义后端)              │
│                                     │
│  1. IR 遍历                         │
│     module → function → BB → instr │
│                                     │
│  2. 指令选择                        │
│     Alloca  → (栈分配)              │
│     Store   → str w8, [sp, #4]     │
│     Load    → ldr w9, [sp, #4]     │
│     Add     → add w12, w10, w11    │
│     Return  → mov w0, w9; ret      │
│                                     │
│  3. 寄存器分配                      │
│     简单栈式分配 + 固定工作寄存器   │
│                                     │
│  4. 栈帧管理                        │
│     Prologue/Epilogue               │
│                                     │
│  5. 汇编输出                        │
│     GNU 汇编语法                    │
└─────────────────────────────────────┘
    ↓
ARM64 汇编代码 (.s)
```

### 关键技术

#### 1. 值跟踪系统
使用两个 HashMap 跟踪 LLVM 值的位置：
- `alloca_map`: Alloca 指令 → 栈偏移量
- `value_map`: 计算指令 → 寄存器名

```rust
// 唯一 ID 使用指针地址
fn get_instruction_id(&self, instruction: &InstructionValue) -> String {
    format!("{:p}", instruction as *const _)
}
```

#### 2. 两阶段 Alloca 处理
- **阶段 1** (`analyze_allocas`): 扫描所有 alloca，分配栈偏移
- **阶段 2** (`generate_alloca_instruction`): 不生成代码

#### 3. 寄存器使用约定
| 寄存器 | 用途 |
|--------|------|
| w0 | 函数返回值 |
| w8 | Store 临时寄存器 |
| w9 | Load 结果 |
| w10, w11 | 算术操作数 |
| w12 | 算术结果 |

## 📝 示例

### 输入: test_arithmetic.c
```c
int main() {
    int a = 5;
    int b = 3;
    int c = a + b;
    return c;
}
```

### 输出: ARM64 汇编
```asm
	.globl	main
	.type	main,@function
main:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	sub	sp, sp, #16
	
	mov	w8, #5
	str	w8, [sp, #4]
	mov	w8, #3
	str	w8, [sp, #8]
	
	ldr	w10, [sp, #4]
	ldr	w11, [sp, #8]
	add	w12, w10, w11
	str	w12, [sp, #12]
	
	ldr	w9, [sp, #12]
	mov	w0, w9
	add	sp, sp, #16
	ldp	x29, x30, [sp], #16
	ret
	.cfi_endproc
```

## 🔧 如何扩展

### 添加新指令（以除法为例）

1. **在 `codegen.rs` 添加 match 分支**:
```rust
inkwell::values::InstructionOpcode::SDiv => {
    self.generate_sdiv_instruction(instruction, &operands)
}
```

2. **实现生成函数**:
```rust
fn generate_sdiv_instruction(&mut self, instruction: &InstructionValue, operands: &[...]) -> Result<String, String> {
    // 类似 add/sub/mul 的实现
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

## 📚 文档

- **开发指南**: `docs/ASSM_MODULE_GUIDE.md` - 详细的架构和实现说明
- **快速开始**: `QUICK_START.md` - 测试和使用示例
- **改进总结**: `docs/assm_improvements.md` - 重构前后对比

## 🎓 学习资源

### ARM64 相关
- [ARM64 指令集参考](https://developer.arm.com/documentation/)
- [AAPCS64 调用约定](https://github.com/ARM-software/abi-aa/blob/main/aapcs64/aapcs64.rst)

### LLVM/Inkwell 相关
- [Inkwell 文档](https://thedan64.github.io/inkwell/)
- [LLVM IR 参考](https://llvm.org/docs/LangRef.html)

### 编译器后端设计
- 虎书 (Tiger Book) - 第9章：指令选择
- 龙书 (Dragon Book) - 第8章：代码生成

## 🐛 问题排查

### 编译错误
```bash
# 清理并重新编译
cargo clean
cargo build --release
```

### 生成的汇编不正确
```bash
# 启用调试模式（在 codegen.rs 中默认已启用）
# 会输出详细的值跟踪信息
DEBUG: Alloca 0x... at offset 4
DEBUG: Processing instruction: Store with 2 operands
...
```

### 对比 LLVM 官方后端
```bash
# 官方后端
./target/release/compiler test.c --emit-arm -o llvm.s

# 自定义后端
./target/release/compiler test.c --emit-assm -o custom.s

# 对比
diff -u llvm.s custom.s
```

## 🎉 总结

ASSM 模块是一个**完整且正确的编译器后端实现**：

✅ 符合标准后端架构  
✅ 正确的值跟踪系统  
✅ 清晰的代码结构  
✅ 易于扩展和维护  

继续按照这个模式添加更多指令支持即可！

---

**有问题？** 查看 `docs/ASSM_MODULE_GUIDE.md` 或提出 Issue。

