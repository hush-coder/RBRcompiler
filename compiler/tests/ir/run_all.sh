#!/bin/bash

# IR生成测试脚本
# 参考parser/run_all.sh的格式

echo "=== IR Generation Tests ==="

# 设置路径
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
COMPILER_DIR="$(dirname "$SCRIPT_DIR")"
COMPILER_DIR="$(dirname "$COMPILER_DIR")"
OUTPUT_DIR="$SCRIPT_DIR/output"

# 确保输出目录存在
mkdir -p "$OUTPUT_DIR"

# 测试文件列表
TEST_FILES=(
    "test_simple.c"
    "test_functions.c" 
    "test_expressions.c"
    "test_control_flow.c"
)

echo "开始运行IR生成测试..."

for test_file in "${TEST_FILES[@]}"; do
    echo ""
    echo "=== 测试文件: $test_file ==="
    
    TEST_PATH="$SCRIPT_DIR/$test_file"
    RBF_OUTPUT="$OUTPUT_DIR/RBF_${test_file%.c}_ir.txt"
    BISHENG_OUTPUT="$OUTPUT_DIR/bisheng_${test_file%.c}_ir.txt"
    DIFF_OUTPUT="$OUTPUT_DIR/${test_file%.c}_ir.diff"
    
    echo "测试文件: $TEST_PATH"
    echo "RBF输出: $RBF_OUTPUT"
    echo "Bisheng输出: $BISHENG_OUTPUT"
    
    # 运行RBF编译器
    echo "运行RBF编译器..."
    cd "$COMPILER_DIR"
    cargo run -- --emit-ir "$TEST_PATH" > "$RBF_OUTPUT" 2>&1
    
    # 运行bisheng编译器获取IR输出
    echo "运行bisheng编译器..."
    clang -S -emit-llvm "$TEST_PATH" -o "$BISHENG_OUTPUT" 2>/dev/null
    
    # 比较输出
    echo "比较输出..."
    if diff -u "$BISHENG_OUTPUT" "$RBF_OUTPUT" > "$DIFF_OUTPUT"; then
        echo "✅ $test_file - 输出完全匹配！"
        echo "Bisheng输出:"
        cat "$BISHENG_OUTPUT"
        echo ""
        echo "RBF输出:"
        cat "$RBF_OUTPUT"
    else
        echo "❌ $test_file - 输出不匹配，差异如下:"
        cat "$DIFF_OUTPUT"
    fi
    
    echo "---"
done

echo ""
echo "=== 所有测试完成 ==="
echo "输出文件位置: $OUTPUT_DIR"
