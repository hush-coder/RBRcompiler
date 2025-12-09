#!/bin/bash

# 批量 IR 测试脚本
# 针对 tests/final 下的 a.c, a-.c, a+.c

echo "=== Final IR Batch Test ==="

# 设置路径
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
COMPILER_DIR="$(dirname "$SCRIPT_DIR")"
COMPILER_DIR="$(dirname "$COMPILER_DIR")"
OUTPUT_DIR="$SCRIPT_DIR/ir_output"

mkdir -p "$OUTPUT_DIR"

# 要测试的文件（相对于 SCRIPT_DIR）
FILES=("a.c" "a-.c" "a+.c")

echo "输出目录: $OUTPUT_DIR"

for f in "${FILES[@]}"; do
    TEST_FILE="$SCRIPT_DIR/$f"
    BASE="$(basename "$f" .c)"
    RBF_OUTPUT="$OUTPUT_DIR/RBF_ir_${BASE}.txt"
    BISHENG_OUTPUT="$OUTPUT_DIR/bisheng_ir_${BASE}.ll"
    DIFF_OUTPUT="$OUTPUT_DIR/ir_${BASE}.diff"
    RBF_ERR="$OUTPUT_DIR/rbf_${BASE}.err"
    CLANG_ERR="$OUTPUT_DIR/clang_${BASE}.err"

    echo "\n--- 测试: $f ---"
    echo "测试文件: $TEST_FILE"
    echo "RBF 输出: $RBF_OUTPUT"
    echo "Bisheng 输出: $BISHENG_OUTPUT"

    # 运行 RBF 编译器（将 stderr 保存为单独文件，stdout 写入 RBF_OUTPUT）
    cd "$COMPILER_DIR"
    ./target/debug/compiler --emit-ir "$TEST_FILE" > "$RBF_OUTPUT" 2> "$RBF_ERR"
    if [ $? -ne 0 ]; then
        echo "[错误] RBF 编译器对 $f 生成 IR 失败，查看 $RBF_ERR"
    fi

    # 使用 clang 生成 LLVM IR
    clang -S -emit-llvm "$TEST_FILE" -o "$BISHENG_OUTPUT" 2> "$CLANG_ERR"
    if [ $? -ne 0 ]; then
        echo "[错误] clang 对 $f 生成 IR 失败，查看 $CLANG_ERR"
    fi

    # 比较
    if [ -f "$BISHENG_OUTPUT" ] && [ -f "$RBF_OUTPUT" ]; then
        if diff -u "$BISHENG_OUTPUT" "$RBF_OUTPUT" > "$DIFF_OUTPUT"; then
            echo "✅ $f: 输出完全匹配"
        else
            echo "❌ $f: 输出不匹配，差异保存在 $DIFF_OUTPUT"
        fi
    else
        echo "⚠️ $f: 无法比较（可能缺少输出文件）"
    fi
done

echo "=== 全部测试完成 ==="