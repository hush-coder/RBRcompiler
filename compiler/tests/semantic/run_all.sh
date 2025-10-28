#!/usr/bin/env bash

echo "=== 运行所有语义分析测试 ==="
echo ""

# 确保output目录存在
mkdir -p output

# 运行所有测试
echo "1. 测试1: 最简单的语义分析"
./test01_simple.sh
echo ""

echo "2. 测试2: 变量声明和赋值"
./test02_variables.sh
echo ""

echo "3. 测试3: 函数定义和调用"
./test03_functions.sh
echo ""

echo "4. 测试4: if语句"
./test04_if.sh
echo ""

echo "5. 测试5: 错误检测"
./test05_errors.sh
echo ""

echo "6. 测试6: 逻辑运算和一元运算符"
./test06_logic.sh
echo ""

echo "7. 测试7: 复杂逻辑运算"
./test07_complex_logic.sh
echo ""

echo "8. 测试8: 作用域错误"
./test08_scope_errors.sh
echo ""

echo "9. 测试9: 函数参数错误"
./test09_function_args_errors.sh
echo ""

echo "10. 测试10: 重复定义错误"
./test11_redefinition_errors.sh
echo ""

echo "11. 测试11: 嵌套作用域错误"
./test13_nested_scope_errors.sh
echo ""

echo "12. 测试12: 函数调用链错误"
./test14_function_chain_errors.sh
echo ""

echo "13. 测试13: 综合错误测试"
./test15_comprehensive_errors.sh
echo ""

echo "=== 所有测试完成 ==="
