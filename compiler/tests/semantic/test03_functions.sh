#!/usr/bin/env bash

echo "=== 测试3: 函数定义和调用 ==="
echo ""

# 显示测试文件内容
echo "测试文件内容:"
cat test03_functions.c
echo ""

# 运行语义分析
echo "运行语义分析..."
cd ../..
cargo run -- tests/semantic/test03_functions.c --emit semantic 2>/dev/null

echo ""
echo "测试完成！"
