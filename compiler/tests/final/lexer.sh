#!/usr/bin/env bash
set -euo pipefail

# 词法测试脚本 —— 输入: tests/final/a-.c a.c a+.c
# 输出目录: tests/final/lexer_output

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${BLUE}=== Final 词法分析器测试 ===${NC}"

# 脚本所在目录（tests/final）
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
COMPILER_DIR="$(dirname "$SCRIPT_DIR")"
COMPILER_DIR="$(dirname "$COMPILER_DIR")"

# 指定需要测试的源文件（相对于 SCRIPT_DIR）
FILES=("a-.c" "a.c" "a+.c")

OUTPUT_DIR="$SCRIPT_DIR/lexer_output"
mkdir -p "$OUTPUT_DIR"

echo "输出目录: $OUTPUT_DIR"

for f in "${FILES[@]}"; do
    SRC="$SCRIPT_DIR/$f"
    if [ ! -f "$SRC" ]; then
        echo -e "${YELLOW}警告: 测试文件不存在: $SRC，跳过${NC}"
        continue
    fi

    base="$(basename "$SRC")"
    base_noext="${base%.c}"
    sanitized="$(echo "$base_noext" | sed 's/+/plus/g; s/-/minus/g; s/[^A-Za-z0-9._]/_/g')"

    echo -e "\n${BLUE}--- 处理: $SRC -> $sanitized ${NC}"

    # 使用本地可执行编译器生成 token 流（已存在可执行文件时使用）
    echo -e "${BLUE}生成: 自研编译器 token -> RBF_${sanitized}_tokens.txt${NC}"
    (cd "$COMPILER_DIR" && ./target/debug/compiler --emit tokens "$SRC" > "$OUTPUT_DIR/RBF_${sanitized}_tokens.txt" 2> "$OUTPUT_DIR/RBF_${sanitized}_tokens.err") || {
        echo -e "${YELLOW}错误: 自研编译器生成 token 失败（见 .err）${NC}"
    }

    # 使用 clang（Bisheng clang）生成 token 流
    echo -e "${BLUE}生成: Bisheng clang token -> bisheng_${sanitized}_tokens.txt${NC}"
    clang -Xclang -dump-tokens -fsyntax-only "$SRC" > "$OUTPUT_DIR/bisheng_${sanitized}_tokens.txt" 2> "$OUTPUT_DIR/bisheng_${sanitized}_tokens.err" || {
        echo -e "${YELLOW}警告: clang 生成 token 失败或不可用（见 .err）${NC}"
    }

    # 有些 clang 版本会把 -Xclang -dump-tokens 的输出写到 stderr，若发生则把 .err 复制到 .txt 以便比较
    if [ ! -s "$OUTPUT_DIR/bisheng_${sanitized}_tokens.txt" ] && [ -s "$OUTPUT_DIR/bisheng_${sanitized}_tokens.err" ]; then
        cp "$OUTPUT_DIR/bisheng_${sanitized}_tokens.err" "$OUTPUT_DIR/bisheng_${sanitized}_tokens.txt"
    fi

    # 比较（如果都成功生成）并生成对比文件
    DIFF_FILE="$OUTPUT_DIR/${sanitized}.diff"
    COMP_FILE="$OUTPUT_DIR/${sanitized}_comparison.txt"
    if [ -s "$OUTPUT_DIR/RBF_${sanitized}_tokens.txt" ] && [ -s "$OUTPUT_DIR/bisheng_${sanitized}_tokens.txt" ]; then
        diff -u "$OUTPUT_DIR/bisheng_${sanitized}_tokens.txt" "$OUTPUT_DIR/RBF_${sanitized}_tokens.txt" > "$DIFF_FILE" || true
        echo -e "比较输出: $DIFF_FILE"

        # 生成一个包含双方输出与 diff 的对比文件，便于查看
        {
            echo "===== Bisheng tokens: bisheng_${sanitized}_tokens.txt ====="
            cat "$OUTPUT_DIR/bisheng_${sanitized}_tokens.txt" || true
            echo
            echo "===== RBF tokens: RBF_${sanitized}_tokens.txt ====="
            cat "$OUTPUT_DIR/RBF_${sanitized}_tokens.txt" || true
            echo
            echo "===== Unified Diff: ${sanitized}.diff ====="
            if [ -s "$DIFF_FILE" ]; then
                cat "$DIFF_FILE"
            else
                echo "(无差异)"
            fi
        } > "$COMP_FILE"

        echo -e "对比文件: $COMP_FILE"
    else
        echo -e "${YELLOW}跳过比较: 缺少某方输出文件${NC}"
    fi
done

echo -e "\n${GREEN}✓ 词法测试完成，查看 $OUTPUT_DIR 下的结果${NC}"
