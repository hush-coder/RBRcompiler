#!/bin/bash
# 测试单个文件

if [ -z "$1" ]; then
    echo "用法: $0 <test_file.c>"
    exit 1
fi

TEST_FILE="test_cases/$1"
COMPILER="../../target/debug/compiler"

if [ ! -f "$TEST_FILE" ]; then
    echo "错误: 文件 $TEST_FILE 不存在"
    exit 1
fi

echo "=== 测试: $1 ==="
echo ""

echo "📝 C 代码:"
cat "$TEST_FILE"
echo ""

echo "🔧 生成 LLVM IR:"
"$COMPILER" "$TEST_FILE" --emit-ir
echo ""

echo "🔧 生成汇编:"
"$COMPILER" "$TEST_FILE" --emit-assm -o /tmp/test.s
echo ""

echo "📄 生成的汇编:"
cat /tmp/test.s
echo ""

echo "🔧 编译并运行:"
if clang /tmp/test.s -o /tmp/test 2>/dev/null; then
    set +e
    /tmp/test >/dev/null 2>&1
    result=$?
    set -e
    echo "✅ 我们的编译器 → 返回值: $result"
else
    echo "❌ 编译失败"
fi

if clang "$TEST_FILE" -o /tmp/test_ref 2>/dev/null; then
    set +e
    /tmp/test_ref >/dev/null 2>&1
    ref_result=$?
    set -e
    echo "✅ BiSheng编译器 → 返回值: $ref_result"
else
    echo "❌ BiSheng编译失败"
fi

if [ "$result" = "$ref_result" ]; then
    echo ""
    echo "🎉 结果一致！"
else
    echo ""
    echo "❌ 结果不一致"
fi

