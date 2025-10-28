#!/usr/bin/env bash

echo "=== 对比测试: 自研编译器 vs bisheng clang ==="
echo ""

# 确保output目录存在
mkdir -p output

# 测试文件列表
TEST_FILES=(
    "test01_simple.c"
    "test02_variables.c"
    "test03_functions.c"
    "test04_if.c"
    "test05_errors.c"
    "test06_logic.c"
    "test07_complex_logic.c"
)

# 遍历测试文件
for test_file in "${TEST_FILES[@]}"; do
    echo "=== 测试文件: $test_file ==="
    
    # 显示测试文件内容
    echo "文件内容:"
    cat "$test_file"
    echo ""
    
    # 使用自研编译器
    echo "自研编译器输出:"
    cd ../..
    cargo run -- "tests/semantic/$test_file" --emit semantic 2>/dev/null > "tests/semantic/output/RBF_${test_file%.c}.txt"
    cat "tests/semantic/output/RBF_${test_file%.c}.txt"
    echo ""
    
    # 使用bisheng clang
    echo "bisheng clang输出:"
    clang -fsyntax-only "tests/semantic/$test_file" 2>&1 > "tests/semantic/output/bisheng_${test_file%.c}.txt"
    cat "tests/semantic/output/bisheng_${test_file%.c}.txt"
    echo ""
    
    # 对比结果
    if diff "tests/semantic/output/RBF_${test_file%.c}.txt" "tests/semantic/output/bisheng_${test_file%.c}.txt" > "tests/semantic/output/${test_file%.c}.diff"; then
        echo "✓ 结果一致"
    else
        echo "⚠ 结果有差异"
        echo "差异:"
        cat "tests/semantic/output/${test_file%.c}.diff"
    fi
    
    echo "----------------------------------------"
    echo ""
done

echo "=== 对比测试完成 ==="
