#!/usr/bin/env bash
set -euo pipefail

# 设置颜色输出
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== 简单词法分析器测试 ===${NC}"


echo -e "${GREEN}✓ Bisheng clang 可用${NC}"

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

# 使用自研编译器生成token流
echo -e "${BLUE}=== 使用自研编译器生成token流 ===${NC}"
cargo run --manifest-path "$SCRIPT_DIR/../../Cargo.toml" -- --emit tokens "$TEST_FILE" > "$OUTPUT_DIR/RBF_tokens.txt" || {
    echo "错误: 自研编译器生成token流失败"
    exit 1
}

echo -e "输出成功，在$OUTPUT_DIR/RBF_tokens.txt"

# 使用Bisheng clang生成token流
echo -e "${BLUE}=== 使用Bisheng clang生成token流 ===${NC}"
clang -Xclang -dump-tokens -fsyntax-only "$TEST_FILE" > "$OUTPUT_DIR/bisheng_tokens.txt" 2>&1 || {
    echo "错误: Bisheng clang生成token流失败"
    exit 1
}

echo -e "输出成功，在$OUTPUT_DIR/bisheng_tokens.txt"

echo "比对成功"

echo ""
echo -e "${GREEN}✓ 测试完成！${NC}"