#!/bin/bash

echo "=== 测试 基础语义分析 (test_basic.c) ==="

echo "测试文件内容:"
cat test_basic.c
echo

echo "使用自研 parser 生成语义分析..."
/app/compiler/target/debug/compiler test_basic.c --emit semantic > output/RBF_test_basic_semantic.txt
if [ $? -eq 0 ]; then
    echo "✓ 自研 parser 成功"
else
    echo "✗ 自研 parser 失败"
    exit 1
fi

echo "使用 Bisheng clang 生成语义分析..."
clang -fsyntax-only test_basic.c > output/bisheng_test_basic_semantic.txt 2>&1
if [ $? -eq 0 ]; then
    echo "✓ Bisheng clang 成功"
else
    echo "✗ Bisheng clang 失败"
fi

echo "比对语义分析..."
if diff output/RBF_test_basic_semantic.txt output/bisheng_test_basic_semantic.txt > output/test_basic_semantic.diff; then
    echo "✓ 语义分析完全一致"
else
    echo "⚠ 语义分析有差异，diff 结果已保存到 output/test_basic_semantic.diff"
    echo "差异预览:"
    head -20 output/test_basic_semantic.diff
fi
