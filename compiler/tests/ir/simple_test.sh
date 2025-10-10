#!/bin/bash

# 简单IR测试脚本
# 参考lexer/simple_test.sh的格式

echo "=== IR Generation Test ==="

# 设置路径
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
COMPILER_DIR="$(dirname "$SCRIPT_DIR")"
COMPILER_DIR="$(dirname "$COMPILER_DIR")"
OUTPUT_DIR="$SCRIPT_DIR/output"

# 确保输出目录存在
mkdir -p "$OUTPUT_DIR"

# 测试文件
TEST_FILE="$SCRIPT_DIR/test_simple.c"
RBF_OUTPUT="$OUTPUT_DIR/RBF_ir.txt"
BISHENG_OUTPUT="$OUTPUT_DIR/bisheng_ir.txt"
DIFF_OUTPUT="$OUTPUT_DIR/ir.diff"

echo "测试文件: $TEST_FILE"
echo "RBF输出: $RBF_OUTPUT"
echo "Bisheng输出: $BISHENG_OUTPUT"

# 运行RBF编译器
echo "运行RBF编译器..."
cd "$COMPILER_DIR"
cargo run -- --emit-ir "$TEST_FILE" 2>/dev/null > "$RBF_OUTPUT"

# 运行bisheng编译器获取IR输出
echo "运行bisheng编译器..."
# 使用clang -S -emit-llvm来获取LLVM IR
clang -S -emit-llvm "$TEST_FILE" -o "$BISHENG_OUTPUT" 2>/dev/null

# 比较输出
echo "比较输出..."
if diff -u "$BISHENG_OUTPUT" "$RBF_OUTPUT" > "$DIFF_OUTPUT"; then
    echo "✅ 输出完全匹配！"
    echo "Bisheng输出:"
    cat "$BISHENG_OUTPUT"
    echo ""
    echo "RBF输出:"
    cat "$RBF_OUTPUT"
else
    echo "❌ 输出不匹配，差异如下:"
    cat "$DIFF_OUTPUT"
fi

echo "=== 测试完成 ==="
