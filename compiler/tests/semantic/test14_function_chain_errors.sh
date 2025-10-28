#!/usr/bin/env bash

echo "=== 测试14: 函数调用链错误 ==="
echo ""

# 显示测试文件内容
echo "测试文件内容:"
cat test14_function_chain_errors.c
echo ""

# 运行语义分析
echo "运行语义分析..."
cd ../..
cargo run -- tests/semantic/test14_function_chain_errors.c --emit semantic 2>/dev/null

echo ""
echo "测试完成！"
