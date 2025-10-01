#!/usr/bin/env bash
set -euo pipefail

# 设置颜色输出
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== 简单RISC-V汇编生成测试 ===${NC}"

# 获取脚本所在目录
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEST_FILE="$SCRIPT_DIR/test_simple.c"

# 创建输出目录
OUTPUT_DIR="$SCRIPT_DIR/output"
mkdir -p "$OUTPUT_DIR"

echo "测试文件: $TEST_FILE"
echo "输出目录: $OUTPUT_DIR"

# 显示测试用例内容
echo -e "${BLUE}=== 测试用例内容 ===${NC}"
cat "$TEST_FILE"

echo ""

        # 使用自研编译器生成ARM汇编
        echo -e "${BLUE}=== 使用自研编译器生成ARM汇编 ===${NC}"
        cargo run --manifest-path "$SCRIPT_DIR/../../Cargo.toml" -- --emit-arm "$TEST_FILE" 2>/dev/null > "$OUTPUT_DIR/RBF_arm.s" || {
            echo "错误: 自研编译器生成ARM汇编失败"
            exit 1
        }

echo -e "输出成功，在$OUTPUT_DIR/RBF_arm.s"

# 使用bisheng编译器生成ARM汇编
echo -e "${BLUE}=== 使用bisheng编译器生成ARM汇编 ===${NC}"
clang -S -target aarch64-unknown-linux-gnu "$TEST_FILE" -o "$OUTPUT_DIR/bisheng_arm.s" 2>/dev/null || {
    echo "警告: bisheng生成ARM汇编失败，使用x86_64汇编作为参考"
    clang -S "$TEST_FILE" -o "$OUTPUT_DIR/bisheng_arm.s" 2>/dev/null
}

echo -e "输出成功，在$OUTPUT_DIR/bisheng_arm.s"

# 显示生成的汇编代码
echo -e "${BLUE}=== Bisheng生成的汇编代码 ===${NC}"
cat "$OUTPUT_DIR/bisheng_arm.s"

echo ""
echo -e "${BLUE}=== 自研编译器生成的汇编代码 ===${NC}"
cat "$OUTPUT_DIR/RBF_arm.s"

# 比较输出
echo -e "${BLUE}=== 比较输出 ===${NC}"
if diff -u "$OUTPUT_DIR/bisheng_arm.s" "$OUTPUT_DIR/RBF_arm.s" > "$OUTPUT_DIR/arm.diff"; then
    echo -e "${GREEN}✅ 输出完全匹配！${NC}"
else
    echo -e "${YELLOW}⚠️ 输出有差异，差异如下:${NC}"
    cat "$OUTPUT_DIR/arm.diff"
fi

echo ""
echo -e "${GREEN}✓ 测试完成！${NC}"
