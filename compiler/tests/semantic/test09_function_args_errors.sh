#!/usr/bin/env bash

echo "=== 测试9: 函数参数错误 ==="
echo ""

# 显示测试文件内容
echo "测试文件内容:"
cat test09_function_args_errors.c
echo ""

# 运行语义分析
echo "运行语义分析..."
cd ../..
cargo run -- tests/semantic/test09_function_args_errors.c --emit semantic 2>/dev/null

echo ""
echo "测试完成！"
