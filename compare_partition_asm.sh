#!/usr/bin/env bash
# 比较 partition 函数在不同优化级别下的汇编代码大小

set -eo pipefail

# 颜色设置
GREEN='\033[0;32m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}=== Partition函数汇编代码对比 (O0 vs O1 vs O2 vs O3) ===${NC}"
echo ""

# 设置BiSheng环境
export BISHENG_HOME=${BISHENG_HOME:-/opt/compiler/BiShengCompiler-4.2.0.2-aarch64-linux}
export PATH="$BISHENG_HOME/bin:$PATH"
export LD_LIBRARY_PATH="${BISHENG_HOME}/lib:${BISHENG_HOME}/lib/aarch64-unknown-linux-gnu:${LD_LIBRARY_PATH}"

# 输入文件
INPUT_FILE="partition_pure.c"
if [ ! -f "$INPUT_FILE" ]; then
    echo -e "${RED}错误: 文件 '$INPUT_FILE' 不存在${NC}"
    exit 1
fi

# 输出目录
OUTPUT_DIR="partition_asm_compare"
mkdir -p "$OUTPUT_DIR"

echo -e "${BLUE}=== 生成汇编代码 ===${NC}"
echo ""

# 优化级别列表
OPTS=("O0" "O1" "O2" "O3")

# 生成汇编并统计
for OPT in "${OPTS[@]}"; do
    ASM_FILE="$OUTPUT_DIR/partition_${OPT}.s"
    
    echo -e "${CYAN}生成 -$OPT 汇编...${NC}"
    
    if clang -S -$OPT "$INPUT_FILE" -o "$ASM_FILE" 2>/dev/null; then
        # 文件大小
        FILE_SIZE=$(stat -c%s "$ASM_FILE" 2>/dev/null || stat -f%z "$ASM_FILE" 2>/dev/null || echo "0")
        
        # 总行数
        TOTAL_LINES=$(wc -l < "$ASM_FILE")
        
        # 实际指令行数（排除注释、空行、指令）
        INSTRUCTION_LINES=$(grep -v "^\s*\." "$ASM_FILE" | grep -v "^\s*$" | grep -v "^\s*//" | wc -l)
        
        # partition 函数的指令数
        PARTITION_INSTRUCTIONS=$(sed -n '/^partition:/,/^\.Lfunc_end/p' "$ASM_FILE" | grep -v "^\s*\." | grep -v "^\s*$" | grep -v "^\s*//" | wc -l)
        
        echo -e "  ${GREEN}✓${NC} 文件大小: ${FILE_SIZE} 字节"
        echo -e "  ${GREEN}✓${NC} 总行数: ${TOTAL_LINES}"
        echo -e "  ${GREEN}✓${NC} 实际指令: ${INSTRUCTION_LINES} 行"
        echo -e "  ${GREEN}✓${NC} partition函数指令: ${PARTITION_INSTRUCTIONS} 行"
        
        # 保存到变量
        eval "${OPT}_SIZE=$FILE_SIZE"
        eval "${OPT}_TOTAL=$TOTAL_LINES"
        eval "${OPT}_INST=$INSTRUCTION_LINES"
        eval "${OPT}_PART=$PARTITION_INSTRUCTIONS"
        
    else
        echo -e "  ${RED}✗ 生成失败${NC}"
        eval "${OPT}_SIZE=0"
        eval "${OPT}_TOTAL=0"
        eval "${OPT}_INST=0"
        eval "${OPT}_PART=0"
    fi
    echo ""
done

echo -e "${BLUE}=== 汇编代码对比总结 ===${NC}"
echo ""

# 文件大小对比
echo -e "${CYAN}📄 文件大小对比:${NC}"
printf "%-12s %-15s %-15s %-15s\n" "优化级别" "文件大小(字节)" "相对大小" "变化"
printf "%-12s %-15s %-15s %-15s\n" "------------" "---------------" "---------------" "---------------"

BASE_SIZE=$O0_SIZE
for OPT in "${OPTS[@]}"; do
    SIZE_VAR="${OPT}_SIZE"
    SIZE=${!SIZE_VAR}
    
    if [ $BASE_SIZE -gt 0 ]; then
        PERCENT=$((SIZE * 100 / BASE_SIZE))
        CHANGE=$((100 - PERCENT))
        
        if [ $CHANGE -eq 0 ]; then
            printf "%-12s %-15s %-15s %-15s\n" "-$OPT" "$SIZE" "100%" "基准"
        elif [ $CHANGE -gt 0 ]; then
            printf "%-12s %-15s %-15s %-15s\n" "-$OPT" "$SIZE" "${PERCENT}%" "减少${CHANGE}%"
        else
            INCREASE=$((-CHANGE))
            printf "%-12s %-15s %-15s %-15s\n" "-$OPT" "$SIZE" "${PERCENT}%" "增加${INCREASE}%"
        fi
    else
        printf "%-12s %-15s %-15s %-15s\n" "-$OPT" "$SIZE" "N/A" "N/A"
    fi
done
echo ""

# partition 函数指令数对比
echo -e "${CYAN}🎯 partition函数指令数对比:${NC}"
printf "%-12s %-20s %-15s %-15s\n" "优化级别" "指令数" "相对数量" "优化率"
printf "%-12s %-20s %-15s %-15s\n" "------------" "--------------------" "---------------" "---------------"

BASE_PART=$O0_PART
for OPT in "${OPTS[@]}"; do
    PART_VAR="${OPT}_PART"
    PART=${!PART_VAR}
    
    if [ $BASE_PART -gt 0 ]; then
        PERCENT=$((PART * 100 / BASE_PART))
        REDUCTION=$((100 - PERCENT))
        
        if [ $REDUCTION -eq 0 ]; then
            printf "%-12s %-20s %-15s %-15s\n" "-$OPT" "$PART" "100%" "0%"
        else
            printf "%-12s %-20s %-15s %-15s\n" "-$OPT" "$PART" "${PERCENT}%" "${REDUCTION}%"
        fi
    else
        printf "%-12s %-20s %-15s %-15s\n" "-$OPT" "$PART" "N/A" "N/A"
    fi
done
echo ""

# 总指令数对比
echo -e "${CYAN}📊 总指令数对比:${NC}"
printf "%-12s %-20s %-15s %-15s\n" "优化级别" "总指令数" "相对数量" "优化率"
printf "%-12s %-20s %-15s %-15s\n" "------------" "--------------------" "---------------" "---------------"

BASE_INST=$O0_INST
for OPT in "${OPTS[@]}"; do
    INST_VAR="${OPT}_INST"
    INST=${!INST_VAR}
    
    if [ $BASE_INST -gt 0 ]; then
        PERCENT=$((INST * 100 / BASE_INST))
        REDUCTION=$((100 - PERCENT))
        
        if [ $REDUCTION -eq 0 ]; then
            printf "%-12s %-20s %-15s %-15s\n" "-$OPT" "$INST" "100%" "0%"
        else
            printf "%-12s %-20s %-15s %-15s\n" "-$OPT" "$INST" "${PERCENT}%" "${REDUCTION}%"
        fi
    else
        printf "%-12s %-20s %-15s %-15s\n" "-$OPT" "$INST" "N/A" "N/A"
    fi
done
echo ""

echo -e "${GREEN}=== 对比完成 ===${NC}"
echo "汇编文件保存在: $OUTPUT_DIR"
echo ""
echo "查看具体汇编代码:"
echo "  cat $OUTPUT_DIR/partition_O0.s"
echo "  cat $OUTPUT_DIR/partition_O2.s"
echo ""
echo "查看 partition 函数差异:"
echo "  diff $OUTPUT_DIR/partition_O0.s $OUTPUT_DIR/partition_O2.s"

