#!/usr/bin/env bash
# BiSheng编译器分区函数测试脚本 - O0 vs O1 vs O2 对比

set -eo pipefail

# 设置颜色输出
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== BiSheng编译器优化级别对比测试 (O0 vs O1 vs O2) ===${NC}"

# 设置BiSheng环境
export BISHENG_HOME=${BISHENG_HOME:-/opt/compiler/BiShengCompiler-4.2.0.2-aarch64-linux}
export PATH="$BISHENG_HOME/bin:$PATH"
export LD_LIBRARY_PATH="${BISHENG_HOME}/lib:${BISHENG_HOME}/lib/aarch64-unknown-linux-gnu:${LD_LIBRARY_PATH}"

echo "BiSheng环境设置:"
echo "BISHENG_HOME: $BISHENG_HOME"
echo ""

# 检查输入文件
INPUT_FILE="test_partition.c"
if [ ! -f "$INPUT_FILE" ]; then
    echo -e "${RED}错误: 文件 '$INPUT_FILE' 不存在${NC}"
    exit 1
fi

# 显示系统信息
echo -e "${BLUE}=== 系统信息 ===${NC}"
echo "系统架构: $(uname -m)"
echo "操作系统: $(uname -s)"
echo ""

# 检查BiSheng编译器
echo -e "${BLUE}=== 检查BiSheng编译器 ===${NC}"
if command -v clang >/dev/null 2>&1; then
    echo -e "${GREEN}✓ BiSheng clang编译器可用${NC}"
    clang --version | head -1
else
    echo -e "${RED}✗ BiSheng clang编译器不可用${NC}"
    exit 1
fi
echo ""

# 创建输出目录
OUTPUT_DIR="partition_output"
mkdir -p "$OUTPUT_DIR"

# 1. 编译程序 - O0 (无优化)
echo -e "${BLUE}=== 编译程序 (O0 无优化) ===${NC}"
O0_EXE="$OUTPUT_DIR/partition_o0"
if clang -O0 "$INPUT_FILE" -o "$O0_EXE" 2>/dev/null; then
    echo -e "${GREEN}✓ O0 编译成功${NC}"
else
    echo -e "${RED}✗ O0 编译失败${NC}"
    exit 1
fi
echo ""

# 2. 编译程序 - O1 (基本优化)
echo -e "${BLUE}=== 编译程序 (O1 基本优化) ===${NC}"
O1_EXE="$OUTPUT_DIR/partition_o1"
if clang -O1 "$INPUT_FILE" -o "$O1_EXE" 2>/dev/null; then
    echo -e "${GREEN}✓ O1 编译成功${NC}"
else
    echo -e "${RED}✗ O1 编译失败${NC}"
    exit 1
fi
echo ""

# 3. 编译程序 - O2 (标准优化)
echo -e "${BLUE}=== 编译程序 (O2 标准优化) ===${NC}"
O2_EXE="$OUTPUT_DIR/partition_o2"
if clang -O2 "$INPUT_FILE" -o "$O2_EXE" 2>/dev/null; then
    echo -e "${GREEN}✓ O2 编译成功${NC}"
else
    echo -e "${RED}✗ O2 编译失败${NC}"
    exit 1
fi
echo ""

# 4. 生成汇编代码
echo -e "${BLUE}=== 生成汇编代码 ===${NC}"
O0_ASM="$OUTPUT_DIR/partition_o0.s"
O1_ASM="$OUTPUT_DIR/partition_o1.s"
O2_ASM="$OUTPUT_DIR/partition_o2.s"

# 生成 O0 汇编
echo "生成 O0 汇编..."
if clang -S -O0 "$INPUT_FILE" -o "$O0_ASM" 2>/dev/null; then
    O0_ASM_SIZE=$(stat -c%s "$O0_ASM" 2>/dev/null || stat -f%z "$O0_ASM" 2>/dev/null || echo "0")
    O0_ASM_LINES=$(grep -v "^\s*\." "$O0_ASM" | grep -v "^\s*$" | grep -v "^\s*//" | wc -l 2>/dev/null || echo "0")
    echo -e "${GREEN}✓ O0 汇编生成成功${NC} - $O0_ASM_SIZE 字节, $O0_ASM_LINES 指令行"
else
    echo -e "${RED}✗ O0 汇编生成失败${NC}"
fi

# 生成 O1 汇编
echo "生成 O1 汇编..."
if clang -S -O1 "$INPUT_FILE" -o "$O1_ASM" 2>/dev/null; then
    O1_ASM_SIZE=$(stat -c%s "$O1_ASM" 2>/dev/null || stat -f%z "$O1_ASM" 2>/dev/null || echo "0")
    O1_ASM_LINES=$(grep -v "^\s*\." "$O1_ASM" | grep -v "^\s*$" | grep -v "^\s*//" | wc -l 2>/dev/null || echo "0")
    echo -e "${GREEN}✓ O1 汇编生成成功${NC} - $O1_ASM_SIZE 字节, $O1_ASM_LINES 指令行"
else
    echo -e "${RED}✗ O1 汇编生成失败${NC}"
fi

# 生成 O2 汇编
echo "生成 O2 汇编..."
if clang -S -O2 "$INPUT_FILE" -o "$O2_ASM" 2>/dev/null; then
    O2_ASM_SIZE=$(stat -c%s "$O2_ASM" 2>/dev/null || stat -f%z "$O2_ASM" 2>/dev/null || echo "0")
    O2_ASM_LINES=$(grep -v "^\s*\." "$O2_ASM" | grep -v "^\s*$" | grep -v "^\s*//" | wc -l 2>/dev/null || echo "0")
    echo -e "${GREEN}✓ O2 汇编生成成功${NC} - $O2_ASM_SIZE 字节, $O2_ASM_LINES 指令行"
else
    echo -e "${RED}✗ O2 汇编生成失败${NC}"
fi
echo ""

# 5. 性能测试 - 运行并显示程序内部计时
echo -e "${BLUE}=== 性能测试 (partition 函数执行时间) ===${NC}"
echo ""

# O0 性能测试
echo -e "${CYAN}O0 (无优化):${NC}"
set +e
O0_OUTPUT=$("$O0_EXE" 2>&1)
O0_EXIT=$?
echo "$O0_OUTPUT"
O0_TIME=$(echo "$O0_OUTPUT" | grep "平均执行时间" | grep -oE '[0-9]+\.[0-9]+')
set -e
echo ""

# O1 性能测试  
echo -e "${CYAN}O1 (基本优化):${NC}"
set +e
O1_OUTPUT=$("$O1_EXE" 2>&1)
O1_EXIT=$?
echo "$O1_OUTPUT"
O1_TIME=$(echo "$O1_OUTPUT" | grep "平均执行时间" | grep -oE '[0-9]+\.[0-9]+')
set -e
echo ""

# O2 性能测试
echo -e "${CYAN}O2 (标准优化):${NC}"
set +e
O2_OUTPUT=$("$O2_EXE" 2>&1)
O2_EXIT=$?
echo "$O2_OUTPUT"
O2_TIME=$(echo "$O2_OUTPUT" | grep "平均执行时间" | grep -oE '[0-9]+\.[0-9]+')
set -e
echo ""

# 6. 性能对比
echo -e "${BLUE}=== 性能对比总结 ===${NC}"
echo ""

# 运行时间对比
echo -e "${CYAN}⚡ partition函数平均执行时间 (100次):${NC}"
printf "%-15s %-20s %-15s %-15s\n" "优化级别" "执行时间(毫秒)" "相对速度" "性能提升"
printf "%-15s %-20s %-15s %-15s\n" "---------------" "--------------------" "---------------" "---------------"

if [ -n "$O0_TIME" ] && [ -n "$O1_TIME" ] && [ -n "$O2_TIME" ]; then
    printf "%-15s %-20s %-15s %-15s\n" "O0 (无优化)" "$O0_TIME" "100%" "基准"
    
    # 使用 awk 计算浮点数
    O1_SPEEDUP=$(awk "BEGIN {printf \"%.1f\", ($O0_TIME - $O1_TIME) / $O0_TIME * 100}")
    O2_SPEEDUP=$(awk "BEGIN {printf \"%.1f\", ($O0_TIME - $O2_TIME) / $O0_TIME * 100}")
    O1_PERCENT=$(awk "BEGIN {printf \"%.1f\", $O1_TIME / $O0_TIME * 100}")
    O2_PERCENT=$(awk "BEGIN {printf \"%.1f\", $O2_TIME / $O0_TIME * 100}")
    
    printf "%-15s %-20s %-15s %-15s\n" "O1 (基本)" "$O1_TIME" "${O1_PERCENT}%" "快${O1_SPEEDUP}%"
    printf "%-15s %-20s %-15s %-15s\n" "O2 (标准)" "$O2_TIME" "${O2_PERCENT}%" "快${O2_SPEEDUP}%"
else
    echo "无法提取时间数据"
    echo "O0_TIME=$O0_TIME, O1_TIME=$O1_TIME, O2_TIME=$O2_TIME"
fi
echo ""

# 汇编代码对比
echo -e "${CYAN}📄 汇编代码对比:${NC}"
printf "%-15s %-15s %-15s %-15s\n" "优化级别" "文件大小" "指令行数" "指令减少"
printf "%-15s %-15s %-15s %-15s\n" "---------------" "---------------" "---------------" "---------------"
printf "%-15s %-15s %-15s %-15s\n" "O0 (无优化)" "$O0_ASM_SIZE" "$O0_ASM_LINES" "0%"

if [ "$O0_ASM_LINES" -gt 0 ]; then
    O1_REDUCTION=$((100 - O1_ASM_LINES * 100 / O0_ASM_LINES))
    printf "%-15s %-15s %-15s %-15s\n" "O1 (基本)" "$O1_ASM_SIZE" "$O1_ASM_LINES" "${O1_REDUCTION}%"
    
    O2_REDUCTION=$((100 - O2_ASM_LINES * 100 / O0_ASM_LINES))
    printf "%-15s %-15s %-15s %-15s\n" "O2 (标准)" "$O2_ASM_SIZE" "$O2_ASM_LINES" "${O2_REDUCTION}%"
fi
echo ""

# 7. 总结
echo -e "${GREEN}=== 测试完成 ===${NC}"
echo "输出目录: $OUTPUT_DIR"
echo ""
echo "生成的文件:"
ls -lh "$OUTPUT_DIR" | grep partition
echo ""
echo "💡 关键发现:"
echo "  - 指令数减少越多，代码越精简"
echo "  - 运行时间越短，性能越好"
echo "  - O1 通常是性能和编译时间的最佳平衡"
