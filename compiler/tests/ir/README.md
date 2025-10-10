# IR (Intermediate Representation) 测试

这个目录包含了RBRcompiler的IR生成测试。

## 测试文件

- `test_simple.c` - 基本变量声明和算术运算
- `test_functions.c` - 函数定义和调用
- `test_expressions.c` - 复杂表达式计算
- `test_control_flow.c` - 控制流语句（if/while）

## 测试脚本

- `simple_test.sh` - 单个测试文件测试
- `run_all.sh` - 运行所有IR测试

## IR指令集

### 算术指令
- `add` - 加法
- `sub` - 减法
- `mul` - 乘法
- `div` - 除法

### 比较指令
- `cmp` - 比较
- `cmpeq` - 等于比较
- `cmpne` - 不等于比较
- `cmplt` - 小于比较
- `cmple` - 小于等于比较
- `cmpgt` - 大于比较
- `cmpge` - 大于等于比较

### 跳转指令
- `jump` - 无条件跳转
- `jumpif` - 条件跳转
- `jumpifnot` - 条件不满足跳转

### 内存操作
- `load` - 加载
- `store` - 存储
- `alloca` - 分配内存

### 函数调用
- `call` - 函数调用
- `ret` - 返回

### 其他指令
- `move` - 移动值
- `const` - 常量加载
- `string` - 字符串加载
- `label` - 标签定义

## 运行测试

```bash
# 运行单个测试
./simple_test.sh

# 运行所有测试
./run_all.sh
```

## 输出格式

IR输出采用三地址码格式，每个指令都有明确的操作数和目标变量。
