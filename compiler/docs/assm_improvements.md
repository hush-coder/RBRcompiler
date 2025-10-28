# ASSM 模块改进总结

## 🎯 主要问题与解决方案

### 问题 1: 值跟踪系统混乱 ❌
**原因**: 每次调用 `get_value_id()` 都生成新的 ID，导致同一个 LLVM Value 被识别为不同的值

**解决方案** ✅:
- 使用指针地址作为唯一 ID: `format!("{:p}", value as *const _)`
- 添加两个 HashMap:
  - `alloca_map`: LLVM 指令 → 栈偏移
  - `value_map`: LLVM 指令 → 寄存器名

### 问题 2: Alloca 指令处理错误 ❌
**原来的实现**:
```rust
// 错误：生成无意义的代码
add w0, sp, #4
```

**改进后的实现** ✅:
```rust
// 两阶段处理：
// 1. analyze_allocas: 扫描所有 alloca，分配栈偏移
// 2. generate_alloca_instruction: 不生成任何代码（返回空字符串）
```

### 问题 3: Store/Load 指令使用错误的地址 ❌
**原来的输出**:
```asm
str w1, [w1]  # 错误！地址和值是同一个寄存器
```

**改进后的输出** ✅:
```asm
mov w8, #5        # 立即数加载到临时寄存器
str w8, [sp, #4]  # 直接存储到栈地址
ldr w9, [sp, #4]  # 直接从栈地址加载
```

### 问题 4: Return 指令缺少函数尾声 ❌
**改进后** ✅:
```asm
ldr w0, [sp, #4]          # 加载返回值
add sp, sp, #16           # 恢复栈指针
ldp x29, x30, [sp], #16   # 恢复帧指针和返回地址
ret                       # 返回
```

## 🔧 核心改进

### 1. 新增函数: `get_instruction_id()` 和 `get_value_id()`
```rust
fn get_instruction_id(&self, instruction: &InstructionValue) -> String {
    format!("{:p}", instruction as *const _)
}

fn get_value_id(&self, value: &BasicValueEnum) -> String {
    format!("{:p}", value as *const _)
}
```

### 2. 新增函数: `analyze_allocas()`
在代码生成前，先扫描所有 alloca 指令并分配栈偏移：
```rust
fn analyze_allocas(&mut self, function: &FunctionValue) -> i32 {
    let mut offset = 0;
    for basic_block in function.get_basic_blocks() {
        for instruction in basic_block.get_instructions() {
            if instruction.get_opcode() == Alloca {
                offset += 4;
                self.alloca_map.insert(instr_id, offset);
            }
        }
    }
    self.current_stack_offset = offset;
    offset
}
```

### 3. 核心函数: `get_value_location()`
统一处理所有值的位置查找：
```rust
fn get_value_location(&self, value: &BasicValueEnum) -> Result<String, String> {
    // 1. Alloca 结果 → 返回 "[sp, #offset]"
    if let PointerValue(_) = value {
        if let Some(offset) = self.alloca_map.get(&value_id) {
            return Ok(format!("[sp, #{}]", offset));
        }
    }
    
    // 2. 指令结果 → 返回寄存器名 "w9", "w12" 等
    if let Some(reg) = self.value_map.get(&value_id) {
        return Ok(reg.clone());
    }
    
    // 3. 立即数 → 返回 "#5"
    if let IntValue(int_val) = value {
        if int_val.is_const() {
            return Ok(format!("#{}", const_val));
        }
    }
}
```

## 📊 寄存器使用约定

| 寄存器 | 用途 |
|--------|------|
| w0     | 函数返回值 |
| w8     | Store 指令的临时寄存器 |
| w9     | Load 指令的结果 |
| w10    | 算术指令的第一个操作数 |
| w11    | 算术指令的第二个操作数 |
| w12    | 算术指令的结果 |
| x29    | 帧指针 (FP) |
| x30    | 返回地址 (LR) |
| sp     | 栈指针 |

## 🎯 支持的功能

### ✅ 已支持
1. `return 常量` - 例如: `return 0;`
2. `局部变量声明和赋值` - 例如: `int a = 5;`
3. `变量加载和返回` - 例如: `return a;`
4. `算术运算 (Add, Sub, Mul)` - 例如: `int c = a + b;`

### ⏳ 待支持
1. 除法运算 (Div)
2. If-else 控制流
3. 函数调用
4. 比较运算符
5. 逻辑运算符

## 📝 示例

### test_simple_assign.c
```c
int main() {
    int a = 5;
    return a;
}
```

### 生成的汇编（改进后）
```asm
	.globl	main
	.p2align	2
	.type	main,@function
main:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!  # 保存帧指针和返回地址
	mov	x29, sp                 # 设置新的帧指针
	sub	sp, sp, #16             # 分配16字节栈空间（4字节+对齐）
	mov	w8, #5                  # 加载立即数5
	str	w8, [sp, #4]            # 存储到栈上的变量a
	ldr	w9, [sp, #4]            # 从变量a加载值
	mov	w0, w9                  # 移动到返回值寄存器
	add	sp, sp, #16             # 恢复栈指针
	ldp	x29, x30, [sp], #16     # 恢复帧指针和返回地址
	ret                          # 返回
	.cfi_endproc
```

## 🚀 下一步计划

1. 测试基本功能是否正确
2. 添加除法指令支持
3. 实现 if-else 分支
4. 实现函数调用
5. 优化：减少不必要的 mov 指令

