#!/usr/bin/env bash

echo "=== 测试2: 变量声明和赋值 ==="
echo ""

# 显示测试文件内容
echo "测试文件内容:"
cat test02_variables.c
echo ""

# 运行语义分析
echo "运行语义分析..."
cd ../..
cargo run -- tests/semantic/test02_variables.c --emit semantic 2>/dev/null

echo ""
echo "测试完成！"
