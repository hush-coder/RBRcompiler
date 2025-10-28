#!/usr/bin/env bash

echo "=== 测试6: 逻辑运算和一元运算符 ==="
echo ""

# 显示测试文件内容
echo "测试文件内容:"
cat test06_logic.c
echo ""

# 运行语义分析
echo "运行语义分析..."
cd ../..
cargo run -- tests/semantic/test06_logic.c --emit semantic 2>/dev/null

echo ""
echo "测试完成！"
