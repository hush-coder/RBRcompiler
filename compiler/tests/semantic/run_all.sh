#!/bin/bash

echo "=== 运行所有语义分析测试 ==="

# 确保output目录存在
mkdir -p output

# 运行所有测试
echo "1. 运行基础语义分析测试..."
./test_basic.sh
echo

echo "2. 运行函数语义分析测试..."
./test_functions.sh
echo

echo "3. 运行控制流语义分析测试..."
./test_control_flow.sh
echo

echo "4. 运行表达式语义分析测试..."
./test_expressions.sh
echo

echo "5. 运行错误语义分析测试..."
./test_errors.sh
echo

echo "6. 运行简单语义分析测试..."
./test_simple.sh
echo

echo "=== 所有语义分析测试完成 ==="
echo "查看 output/ 目录中的结果文件"
