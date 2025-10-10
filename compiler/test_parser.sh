#!/bin/bash

echo "=== Parser 功能测试 ==="
echo

# 测试1: 简单函数
echo "测试1: 简单函数声明"
echo "输入: int main() { return 42; }"
echo "期望输出: FunctionDeclaration with return statement"
echo

# 测试2: 算术表达式
echo "测试2: 算术表达式"
echo "输入: 1 + 2 * 3;"
echo "期望输出: BinaryExpression(Add, 1, BinaryExpression(Multiply, 2, 3))"
echo

# 测试3: 带参数的函数
echo "测试3: 带参数的函数"
echo "输入: int add(int x, int y) { return x + y; }"
echo "期望输出: FunctionDeclaration with parameters and binary expression"
echo

# 运行测试
echo "运行测试..."
cd compiler
cargo test -- --nocapture

echo
echo "=== 测试完成 ==="


