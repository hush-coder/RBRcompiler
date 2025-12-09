#!/usr/bin/env bash
set -euo pipefail

# 批量 AST 测试脚本（tests/final）
# 输入: a-.c a.c a+.c
# 输出: tests/final/ast_output

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${BLUE}=== Final AST Batch Test ===${NC}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
COMPILER_DIR="$(dirname "$SCRIPT_DIR")"
COMPILER_DIR="$(dirname "$COMPILER_DIR")"
OUTPUT_DIR="$SCRIPT_DIR/ast_output"

mkdir -p "$OUTPUT_DIR"

FILES=("a-.c" "a.c" "a+.c")

echo "输出目录: $OUTPUT_DIR"

for f in "${FILES[@]}"; do
    TEST_FILE="$SCRIPT_DIR/$f"
    BASE_RAW="$(basename "$f" .c)"
    SANITIZED="$(echo "$BASE_RAW" | sed 's/+/plus/g; s/-/minus/g; s/[^A-Za-z0-9._]/_/g')"

    RBF_OUT="$OUTPUT_DIR/RBF_ast_${SANITIZED}.txt"
    RBF_ERR="$OUTPUT_DIR/RBF_ast_${SANITIZED}.err"
    BISHENG_OUT="$OUTPUT_DIR/bisheng_ast_${SANITIZED}.txt"
    BISHENG_ERR="$OUTPUT_DIR/bisheng_ast_${SANITIZED}.err"
    DIFF_OUT="$OUTPUT_DIR/ast_${SANITIZED}.diff"
    COMP_OUT="$OUTPUT_DIR/${SANITIZED}_ast_comparison.txt"

    echo -e "\n${BLUE}--- 测试: $f (${SANITIZED}) ---${NC}"

    if [ ! -f "$TEST_FILE" ]; then
        echo -e "${YELLOW}警告: 测试文件不存在: $TEST_FILE，跳过${NC}"
        continue
    fi

    # 运行自研编译器生成 AST
    echo -e "${BLUE}运行: 自研编译器 -> $RBF_OUT${NC}"
    (cd "$COMPILER_DIR" && ./target/debug/compiler --emit ast "$TEST_FILE" > "$RBF_OUT" 2> "$RBF_ERR") || true

    # 使用 clang/bisheng 生成 AST（如果可用，用 clang -Xclang -ast-dump?）
    # 这里用 clang 的 -fsyntax-only + -Xclang -ast-dump 可输出 clang AST 到 stderr，尝试捕获到文件
    echo -e "${BLUE}尝试: Bisheng/clang AST -> $BISHENG_OUT${NC}"
    # 首先尝试一个常见命令（可能输出到 stderr）
    clang -fsyntax-only -Xclang -ast-dump "$TEST_FILE" > "$BISHENG_OUT" 2> "$BISHENG_ERR" || true

    # 有些 clang 会把输出写到 stderr，若 .txt 为空但 .err 有内容则复制 .err 到 .txt
    if [ ! -s "$BISHENG_OUT" ] && [ -s "$BISHENG_ERR" ]; then
        cp "$BISHENG_ERR" "$BISHENG_OUT"
    fi

    # 生成 diff 与综合对比文件
    if [ -s "$RBF_OUT" ] && [ -s "$BISHENG_OUT" ]; then
        diff -u "$BISHENG_OUT" "$RBF_OUT" > "$DIFF_OUT" || true
    else
        # 若任一方为空，确保 DIFF_OUT 存在（便于 CI 检查）并写入提示
        echo "(无法比较: 缺少输出文件)" > "$DIFF_OUT"
    fi

    {
        echo "===== Bisheng AST: $(basename "$BISHENG_OUT") ====="
        cat "$BISHENG_OUT" 2>/dev/null || echo "(无文件或读取失败)"
        echo
        echo "===== RBF AST: $(basename "$RBF_OUT") ====="
        cat "$RBF_OUT" 2>/dev/null || echo "(无文件或读取失败)"
        echo
        echo "===== Unified Diff: $(basename "$DIFF_OUT") ====="
        if [ -s "$DIFF_OUT" ]; then
            cat "$DIFF_OUT"
        else
            echo "(无差异或无法生成差异)"
        fi
    } > "$COMP_OUT"

    echo -e "输出: $COMP_OUT"
done

echo -e "\n${GREEN}✓ AST 批量测试完成，查看 $OUTPUT_DIR 下的结果${NC}"
