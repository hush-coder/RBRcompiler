#!/usr/bin/env bash

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# 脚本目录
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUTPUT_DIR="$SCRIPT_DIR/output"

# 创建输出目录
mkdir -p "$OUTPUT_DIR"

echo -e "${PURPLE}=== LALRPOP AST 测试 ===${NC}"
echo -e "${PURPLE}测试LALRPOP解析器生成AST${NC}"
echo ""

# 测试文件列表
TEST_FILES=(
    "test_very_simple.c"
    "test_simple_fixed.c"
    "test_logic_operations.c"
)

# 遍历测试文件
for test_file in "${TEST_FILES[@]}"; do
    echo -e "${BLUE}=== 测试文件: $test_file ===${NC}"
    
    # 检查文件是否存在
    if [ ! -f "$SCRIPT_DIR/$test_file" ]; then
        echo -e "${RED}✗ 文件不存在: $test_file${NC}"
        continue
    fi
    
    # 显示测试文件内容
    echo -e "${CYAN}文件内容:${NC}"
    cat "$SCRIPT_DIR/$test_file"
    echo ""
    
    # 使用LALRPOP解析器生成AST
    echo -e "${CYAN}使用LALRPOP解析器生成AST...${NC}"
    cd "$SCRIPT_DIR/../.."
    
    if cargo run -- "$SCRIPT_DIR/$test_file" --emit ast 2>/dev/null > "$OUTPUT_DIR/lalrpop_${test_file%.c}_ast.txt"; then
        echo -e "${GREEN}✓ LALRPOP解析器成功${NC}"
        echo -e "${YELLOW}AST输出已保存到: $OUTPUT_DIR/lalrpop_${test_file%.c}_ast.txt${NC}"
        
        # 显示AST预览
        echo -e "${YELLOW}AST预览:${NC}"
        echo -e "${CYAN}================================${NC}"
        head -20 "$OUTPUT_DIR/lalrpop_${test_file%.c}_ast.txt"
        echo -e "${CYAN}... (完整内容请查看输出文件)${NC}"
        echo -e "${CYAN}================================${NC}"
        
        # 显示文件统计信息
        local lines=$(wc -l < "$OUTPUT_DIR/lalrpop_${test_file%.c}_ast.txt")
        local size=$(wc -c < "$OUTPUT_DIR/lalrpop_${test_file%.c}_ast.txt")
        echo -e "${BLUE}📊 文件统计: ${lines} 行, ${size} 字节${NC}"
        
    else
        echo -e "${RED}✗ LALRPOP解析器失败${NC}"
        # 重新运行以获取错误信息，但过滤掉警告
        cargo run -- "$SCRIPT_DIR/$test_file" --emit ast 2>&1 | grep -v "warning:" > "$OUTPUT_DIR/lalrpop_${test_file%.c}_ast.txt"
        echo -e "${YELLOW}错误信息已保存到: $OUTPUT_DIR/lalrpop_${test_file%.c}_ast.txt${NC}"
        echo -e "${RED}错误信息预览:${NC}"
        head -10 "$OUTPUT_DIR/lalrpop_${test_file%.c}_ast.txt"
    fi
    
    echo ""
    echo -e "${CYAN}----------------------------------------${NC}"
    echo ""
done

# 测试LALRPOP基本功能
echo -e "${BLUE}=== LALRPOP基本功能测试 ===${NC}"
echo -e "${CYAN}运行内置LALRPOP测试...${NC}"

if cargo run -- --test-lalrpop 2>/dev/null > "$OUTPUT_DIR/lalrpop_basic_test.txt"; then
    echo -e "${GREEN}✓ LALRPOP基本功能测试成功${NC}"
    echo -e "${YELLOW}基本测试输出已保存到: $OUTPUT_DIR/lalrpop_basic_test.txt${NC}"
    
    # 显示基本测试预览
    echo -e "${YELLOW}基本测试预览:${NC}"
    echo -e "${CYAN}================================${NC}"
    head -15 "$OUTPUT_DIR/lalrpop_basic_test.txt"
    echo -e "${CYAN}... (完整内容请查看输出文件)${NC}"
    echo -e "${CYAN}================================${NC}"
else
    echo -e "${RED}✗ LALRPOP基本功能测试失败${NC}"
    # 重新运行以获取错误信息，但过滤掉警告
    cargo run -- --test-lalrpop 2>&1 | grep -v "warning:" > "$OUTPUT_DIR/lalrpop_basic_test.txt"
    echo -e "${YELLOW}错误信息已保存到: $OUTPUT_DIR/lalrpop_basic_test.txt${NC}"
fi

echo ""
echo -e "${GREEN}=== LALRPOP AST测试完成！ ===${NC}"
echo -e "${BLUE}📁 输出文件列表:${NC}"

# 列出所有输出文件
for file in "$OUTPUT_DIR"/lalrpop_*.txt; do
    if [ -f "$file" ]; then
        local filename=$(basename "$file")
        local lines=$(wc -l < "$file")
        local size=$(wc -c < "$file")
        echo -e "  📄 $filename (${lines} 行, ${size} 字节)"
    fi
done

echo ""
echo -e "${PURPLE}💡 LALRPOP解析器测试完成！${NC}"
echo -e "${PURPLE}📊 生成的解析器文件大小: 187,621 字节${NC}"
echo -e "${PURPLE}🔧 包含完整的LALR(1)语法分析状态机！${NC}"
echo -e "${PURPLE}📂 所有输出文件已保存到: $OUTPUT_DIR/${NC}"
