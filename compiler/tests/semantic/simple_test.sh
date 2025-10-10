#!/bin/bash

# 简单semantic测试脚本
# 参考lexer/simple_test.sh的格式

echo "=== Semantic Analysis Test ==="

# 设置路径
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
COMPILER_DIR="$(dirname "$SCRIPT_DIR")"
COMPILER_DIR="$(dirname "$COMPILER_DIR")"
OUTPUT_DIR="$SCRIPT_DIR/output"

# 确保输出目录存在
mkdir -p "$OUTPUT_DIR"

# 测试文件
TEST_FILE="$SCRIPT_DIR/test_final.c"
RBF_OUTPUT="$OUTPUT_DIR/RBF_semantic.txt"
BISHENG_OUTPUT="$OUTPUT_DIR/bisheng_semantic.txt"
DIFF_OUTPUT="$OUTPUT_DIR/semantic.diff"

echo "测试文件: $TEST_FILE"
echo "RBF输出: $RBF_OUTPUT"
echo "Bisheng输出: $BISHENG_OUTPUT"

# 运行RBF编译器
echo "运行RBF编译器..."
cd "$COMPILER_DIR"
# 只捕获语义错误输出，过滤掉编译警告
cargo run --quiet -- --emit semantic "$TEST_FILE" 2>/dev/null > "$RBF_OUTPUT"

# 运行bisheng编译器获取semantic输出
echo "运行bisheng编译器..."
# 使用clang -fsyntax-only来获取semantic错误
clang -fsyntax-only "$TEST_FILE" > "$BISHENG_OUTPUT" 2>&1

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
