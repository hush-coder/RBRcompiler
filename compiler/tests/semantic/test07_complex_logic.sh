#!/usr/bin/env bash

echo "=== 测试7: 复杂逻辑运算 ==="
echo ""

# 显示测试文件内容
echo "测试文件内容:"
cat test07_complex_logic.c
echo ""

# 运行语义分析
echo "运行语义分析..."
cd ../..
cargo run -- tests/semantic/test07_complex_logic.c --emit semantic 2>/dev/null

echo ""
echo "测试完成！"

