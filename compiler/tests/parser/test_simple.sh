#!/usr/bin/env bash

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 脚本目录
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUTPUT_DIR="$SCRIPT_DIR/output"

# 创建输出目录
mkdir -p "$OUTPUT_DIR"

# 测试文件
TEST_FILE="test_simple.c"

echo -e "${BLUE}=== 简单语法测试 ===${NC}"
echo -e "${BLUE}测试文件: $TEST_FILE${NC}"

# 显示测试文件内容
echo -e "${BLUE}测试文件内容:${NC}"
cat "$SCRIPT_DIR/$TEST_FILE"
echo ""

# 使用自研 parser 生成 AST
echo -e "${BLUE}使用自研 parser 生成 AST...${NC}"
if cargo run --manifest-path "$SCRIPT_DIR/../../Cargo.toml" -- "$SCRIPT_DIR/$TEST_FILE" --ast-output "$OUTPUT_DIR/RBF_${TEST_FILE%.c}_ast.txt" --ast-bisheng 2>&1; then
    echo -e "${GREEN}✓ 自研 parser 成功${NC}"
    
    # 使用 Bisheng clang 生成 AST
    echo -e "${BLUE}使用 Bisheng clang 生成 AST...${NC}"
    if clang -Xclang -ast-dump -fsyntax-only "$SCRIPT_DIR/$TEST_FILE" > "$OUTPUT_DIR/bisheng_${TEST_FILE%.c}_ast.txt" 2>&1; then
        echo -e "${GREEN}✓ Bisheng clang 成功${NC}"
        
        # 比对 AST
        echo -e "${BLUE}比对 AST...${NC}"
        if diff "$OUTPUT_DIR/RBF_${TEST_FILE%.c}_ast.txt" "$OUTPUT_DIR/bisheng_${TEST_FILE%.c}_ast.txt" > "$OUTPUT_DIR/${TEST_FILE%.c}_ast.diff"; then
            echo -e "${GREEN}✓ AST 完全一致！${NC}"
        else
            echo -e "${YELLOW}⚠ AST 有差异，diff 结果已保存到 $OUTPUT_DIR/${TEST_FILE%.c}_ast.diff${NC}"
            echo -e "${YELLOW}差异预览:${NC}"
            head -20 "$OUTPUT_DIR/${TEST_FILE%.c}_ast.diff"
        fi
        
        # 显示文件大小对比
        echo -e "${BLUE}文件大小对比:${NC}"
        echo -e "自研 parser AST: $(wc -l < "$OUTPUT_DIR/RBF_${TEST_FILE%.c}_ast.txt") 行"
        echo -e "Bisheng clang AST: $(wc -l < "$OUTPUT_DIR/bisheng_${TEST_FILE%.c}_ast.txt") 行"
        
    else
        echo -e "${RED}✗ Bisheng clang 失败${NC}"
        echo -e "${RED}错误信息:${NC}"
        tail -10 "$OUTPUT_DIR/bisheng_${TEST_FILE%.c}_ast.txt"
    fi
else
    echo -e "${RED}✗ 自研 parser 失败${NC}"
fi

echo -e "\n${GREEN}=== 测试完成！ ===${NC}"
echo -e "${BLUE}输出文件:${NC}"
echo -e "  自研 parser AST: $OUTPUT_DIR/RBF_${TEST_FILE%.c}_ast.txt"
echo -e "  Bisheng clang AST: $OUTPUT_DIR/bisheng_${TEST_FILE%.c}_ast.txt"
echo -e "  差异文件: $OUTPUT_DIR/${TEST_FILE%.c}_ast.diff"
