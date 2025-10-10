#!/usr/bin/env bash
set -euo pipefail

# 彩色输出
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${BLUE}=== parser AST 集成测试 ===${NC}"

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

# 使用自研 parser 生成 AST
echo -e "${BLUE}=== 使用自研 parser 生成 AST ===${NC}"
cargo run --manifest-path "$SCRIPT_DIR/../../Cargo.toml" -- "$TEST_FILE" --ast-output "$OUTPUT_DIR/RBF_ast.txt" --ast-bisheng || {
    echo "错误: 自研 parser 生成 AST 失败"
    exit 1
}
echo -e "输出成功，在$OUTPUT_DIR/RBF_ast.txt"

# 使用 Bisheng clang 生成 AST
echo -e "${BLUE}=== 使用 Bisheng clang 生成 AST ===${NC}"
clang -Xclang -ast-dump -fsyntax-only "$TEST_FILE" > "$OUTPUT_DIR/bisheng_ast.txt" 2>&1 || {
    echo "错误: Bisheng clang 生成 AST 失败"
    exit 1
}
echo -e "输出成功，在$OUTPUT_DIR/bisheng_ast.txt"

# 比对 AST
echo -e "${BLUE}=== AST Diff ===${NC}"
if diff "$OUTPUT_DIR/RBF_ast.txt" "$OUTPUT_DIR/bisheng_ast.txt" > "$OUTPUT_DIR/ast.diff"; then
    echo -e "${GREEN}✓ AST 完全一致！${NC}"
else
    echo -e "${YELLOW}AST 有差异，diff 结果已保存到 $OUTPUT_DIR/ast.diff${NC}"
    exit 1
fi

echo ""
echo -e "${GREEN}✓ parser AST 测试完成！${NC}"