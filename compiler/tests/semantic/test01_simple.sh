#!/usr/bin/env bash

echo "=== 测试1: 最简单的语义分析 ==="
echo ""

# 显示测试文件内容
echo "测试文件内容:"
cat test01_simple.c
echo ""

# 运行语义分析
echo "运行语义分析..."
cd ../..
cargo run -- tests/semantic/test01_simple.c --emit semantic 2>/dev/null

echo ""
echo "测试完成！"
