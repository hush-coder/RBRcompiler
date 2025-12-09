#!/bin/bash

# Bisheng比较测试脚本（已定制）
# 生成我们的汇编和bisheng汇编，并比较差异

set -e

# 配置
COMPILER_PATH="../../target/debug/compiler"
OUTPUT_DIR="AssmOutput"

# 测试文件（直接列出 a-.c, a.c, a+.c）
TEST_FILES=(
    "a-.c"
    "a.c"
    "a+.c"
)

# 创建输出目录
mkdir -p "$OUTPUT_DIR"

echo "🚀 Bisheng比较测试"
echo "=================="

# 检查编译器
if [ ! -f "$COMPILER_PATH" ]; then
    echo "❌ 编译器不存在，正在构建..."
    cd ../../
    cargo build
    cd tests/final/
fi

# 遍历所有测试文件
for TEST_FILE in "${TEST_FILES[@]}"; do
    TEST_NAME=$(basename "$TEST_FILE" .c)
    echo ""
    echo "======================================"
    echo "📝 测试: $TEST_NAME"
    echo "======================================"

    # 1. 生成我们的汇编代码
    OUR_OUTPUT="$OUTPUT_DIR/${TEST_NAME}_our.s"
    echo "🔧 生成我们的汇编..."
    if "$COMPILER_PATH" "$TEST_FILE" --emit-assm -o "$OUR_OUTPUT" 2>&1; then
        echo "✅ 我们的汇编: $OUR_OUTPUT"
    else
        echo "❌ 我们的汇编生成失败"
        exit 1
    fi

    # 2. 生成bisheng汇编代码（使用clang）
    BISHENG_OUTPUT="$OUTPUT_DIR/${TEST_NAME}_bisheng.s"
    echo "🔧 生成Bisheng汇编..."
    if clang -S -O0 "$TEST_FILE" -o "$BISHENG_OUTPUT" 2>/dev/null; then
        echo "✅ Bisheng汇编: $BISHENG_OUTPUT"
    else
        echo "❌ Bisheng汇编生成失败"
        exit 1
    fi

    # 3. 生成diff
    DIFF_FILE="$OUTPUT_DIR/${TEST_NAME}.diff"
    if diff -u "$BISHENG_OUTPUT" "$OUR_OUTPUT" > "$DIFF_FILE" 2>/dev/null; then
        echo "🎉 汇编代码完全匹配！"
        rm "$DIFF_FILE"
    else
        echo "📊 差异统计: 我们 $(wc -l < "$OUR_OUTPUT") 行 vs Bisheng $(wc -l < "$BISHENG_OUTPUT") 行"
    fi

    # 4. 编译并运行测试
    echo ""
    echo "🔧 编译和运行测试:"
    echo "=================="

    # 编译并运行我们的汇编
    OUR_EXE="$OUTPUT_DIR/${TEST_NAME}_our"
    if clang "$OUR_OUTPUT" -o "$OUR_EXE" 2>/dev/null; then
        set +e
        "$OUR_EXE" >/dev/null 2>&1
        OUR_RESULT=$?
        set -e
        echo "✅ 我们的编译器 → 返回值: $OUR_RESULT"
    else
        echo "❌ 我们的汇编编译失败"
        OUR_RESULT="FAIL"
    fi

    # 直接编译 C 文件对比
    C_EXE="$OUTPUT_DIR/${TEST_NAME}_direct"
    if clang "$TEST_FILE" -o "$C_EXE" 2>/dev/null; then
        set +e
        "$C_EXE" >/dev/null 2>&1
        C_RESULT=$?
        set -e
        echo "✅ BiSheng直接编译 → 返回值: $C_RESULT"
    else
        echo "❌ C文件编译失败"
        C_RESULT="FAIL"
    fi

    # 结果对比
    if [ "$OUR_RESULT" = "$C_RESULT" ]; then
        echo "✅ 结果一致！"
    else
        echo "❌ 结果不一致: 我们=$OUR_RESULT, BiSheng=$C_RESULT"
    fi

done  # 结束 for 循环

echo ""
echo "======================================"
echo "🎉 所有测试完成！"
echo "======================================"
