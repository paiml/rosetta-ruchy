#!/usr/bin/env bash
# Dogfood All Ruchy Tools - Heavy Dogfooding Strategy
# Inspired by ruchy-book 15-tool methodology
# Runs comprehensive tool validation on all Ruchy examples

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RESULTS_DIR="${PROJECT_ROOT}/reports/dogfooding"
TIMESTAMP=$(date -u +"%Y-%m-%d %H:%M:%S UTC")
RUCHY_VERSION=$(ruchy --version | awk '{print $2}')

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Create results directory
mkdir -p "${RESULTS_DIR}"

# Parse arguments
MODE="quick" # Default mode
if [[ $# -gt 0 ]]; then
    case "$1" in
        --quick) MODE="quick" ;;
        --quality) MODE="quality" ;;
        --full) MODE="full" ;;
        --comprehensive) MODE="comprehensive" ;;
        *) echo "Usage: $0 [--quick|--quality|--full|--comprehensive]"; exit 1 ;;
    esac
fi

echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}  Rosetta Ruchy - Heavy Dogfooding${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
echo -e "Mode: ${YELLOW}${MODE}${NC}"
echo -e "Ruchy Version: ${GREEN}${RUCHY_VERSION}${NC}"
echo -e "Timestamp: ${TIMESTAMP}"
echo ""

# Find all .ruchy files in validated categories only (algorithms, data-science, advanced-ai)
# Excludes experimental directories: blockchain, compiler, quantum, OS, deep-learning
RUCHY_FILES=$(find "${PROJECT_ROOT}/examples/algorithms" \
                   "${PROJECT_ROOT}/examples/data-science" \
                   "${PROJECT_ROOT}/examples/advanced-ai" \
                   -name "*.ruchy" -type f 2>/dev/null | sort)
TOTAL_FILES=$(echo "$RUCHY_FILES" | wc -l)

echo -e "Total Examples: ${GREEN}${TOTAL_FILES}${NC}"
echo ""

# Initialize counters for each tool
declare -A TOOL_PASS
declare -A TOOL_FAIL
declare -A TOOL_SKIP

# Define tool suites based on mode
QUICK_TOOLS=("check" "lint" "score")
QUALITY_TOOLS=("${QUICK_TOOLS[@]}" "provability" "runtime" "quality-gate" "test")
FULL_TOOLS=("${QUALITY_TOOLS[@]}" "optimize" "ast" "doc")
COMPREHENSIVE_TOOLS=("${FULL_TOOLS[@]}" "prove" "bench" "fmt" "coverage" "mcp")

# Select tools based on mode
case "${MODE}" in
    quick) TOOLS=("${QUICK_TOOLS[@]}") ;;
    quality) TOOLS=("${QUALITY_TOOLS[@]}") ;;
    full) TOOLS=("${FULL_TOOLS[@]}") ;;
    comprehensive) TOOLS=("${COMPREHENSIVE_TOOLS[@]}") ;;
esac

echo -e "${YELLOW}Tools to run (${#TOOLS[@]}):${NC} ${TOOLS[*]}"
echo ""

# Initialize all tool counters
for tool in "${TOOLS[@]}"; do
    TOOL_PASS[$tool]=0
    TOOL_FAIL[$tool]=0
    TOOL_SKIP[$tool]=0
done

# Function to run a tool on a file
run_tool() {
    local tool=$1
    local file=$2

    case "$tool" in
        check)
            ruchy check "$file" &>/dev/null
            ;;
        lint)
            ruchy lint "$file" &>/dev/null
            ;;
        score)
            ruchy score "$file" &>/dev/null
            ;;
        provability)
            ruchy provability "$file" &>/dev/null
            ;;
        runtime)
            ruchy runtime "$file" &>/dev/null
            ;;
        quality-gate)
            ruchy quality-gate "$file" &>/dev/null
            ;;
        test)
            # Skip if no test in filename
            if [[ ! "$file" =~ test ]]; then
                return 2  # Skip code
            fi
            ruchy test "$file" &>/dev/null
            ;;
        optimize)
            ruchy optimize "$file" &>/dev/null
            ;;
        ast)
            ruchy ast "$file" &>/dev/null
            ;;
        doc)
            ruchy doc "$file" --output "${RESULTS_DIR}/docs" &>/dev/null
            ;;
        prove)
            ruchy prove "$file" --batch &>/dev/null
            ;;
        bench)
            # Only benchmark files with "bench" in name
            if [[ ! "$file" =~ bench ]]; then
                return 2  # Skip code
            fi
            timeout 10s ruchy bench "$file" &>/dev/null
            ;;
        fmt)
            ruchy fmt "$file" --check &>/dev/null
            ;;
        coverage)
            # Skip if not a test file
            if [[ ! "$file" =~ test ]]; then
                return 2  # Skip code
            fi
            ruchy coverage "$file" &>/dev/null
            ;;
        mcp)
            # MCP is a server, skip file-level testing
            return 2
            ;;
        *)
            echo "Unknown tool: $tool"
            return 1
            ;;
    esac
}

# Main testing loop
echo -e "${BLUE}─────────────────────────────────────────────────────${NC}"
echo -e "${BLUE}Running Dogfooding Tests${NC}"
echo -e "${BLUE}─────────────────────────────────────────────────────${NC}"
echo ""

FILE_COUNT=0
for file in $RUCHY_FILES; do
    FILE_COUNT=$((FILE_COUNT + 1))
    RELATIVE_PATH="${file#${PROJECT_ROOT}/}"

    # Progress indicator every 10 files
    if (( FILE_COUNT % 10 == 0 )); then
        echo -e "${BLUE}Progress: ${FILE_COUNT}/${TOTAL_FILES} files tested...${NC}"
    fi

    # Test each tool
    for tool in "${TOOLS[@]}"; do
        if run_tool "$tool" "$file"; then
            TOOL_PASS[$tool]=$((${TOOL_PASS[$tool]} + 1))
        else
            EXIT_CODE=$?
            if [[ $EXIT_CODE -eq 2 ]]; then
                # Skipped
                TOOL_SKIP[$tool]=$((${TOOL_SKIP[$tool]} + 1))
            else
                # Failed
                TOOL_FAIL[$tool]=$((${TOOL_FAIL[$tool]} + 1))
            fi
        fi
    done
done

echo ""
echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}  Dogfooding Results Summary${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
echo ""

# Calculate and display results for each tool
TOTAL_PASS=0
TOTAL_FAIL=0
OVERALL_PASS=true

for tool in "${TOOLS[@]}"; do
    PASS=${TOOL_PASS[$tool]}
    FAIL=${TOOL_FAIL[$tool]}
    SKIP=${TOOL_SKIP[$tool]}
    TESTED=$((PASS + FAIL))

    if [[ $TESTED -eq 0 ]]; then
        RATE="N/A"
        GRADE="SKIP"
        COLOR="${YELLOW}"
    else
        RATE=$(awk "BEGIN {printf \"%.1f\", ($PASS / $TESTED) * 100}")

        # Assign grade based on pass rate
        if (( $(echo "$RATE >= 95" | bc -l) )); then
            GRADE="A+"
            COLOR="${GREEN}"
        elif (( $(echo "$RATE >= 90" | bc -l) )); then
            GRADE="A"
            COLOR="${GREEN}"
        elif (( $(echo "$RATE >= 80" | bc -l) )); then
            GRADE="B"
            COLOR="${YELLOW}"
        elif (( $(echo "$RATE >= 70" | bc -l) )); then
            GRADE="C"
            COLOR="${YELLOW}"
        else
            GRADE="F"
            COLOR="${RED}"
            OVERALL_PASS=false
        fi
    fi

    printf "%-15s ${COLOR}%4s / %4s${NC}  (${COLOR}%6s%%${NC})  Grade: ${COLOR}%3s${NC}" \
        "$tool" "$PASS" "$TESTED" "$RATE" "$GRADE"

    if [[ $SKIP -gt 0 ]]; then
        printf "  [Skipped: %d]" "$SKIP"
    fi
    printf "\n"

    TOTAL_PASS=$((TOTAL_PASS + PASS))
    TOTAL_FAIL=$((TOTAL_FAIL + FAIL))
done

echo ""
echo -e "${BLUE}─────────────────────────────────────────────────────${NC}"
TOTAL_TESTS=$((TOTAL_PASS + TOTAL_FAIL))
if [[ $TOTAL_TESTS -eq 0 ]]; then
    OVERALL_RATE="N/A"
else
    OVERALL_RATE=$(awk "BEGIN {printf \"%.1f\", ($TOTAL_PASS / $TOTAL_TESTS) * 100}")
fi

echo -e "Total Tests: ${TOTAL_TESTS}"
echo -e "Passing: ${GREEN}${TOTAL_PASS}${NC}"
echo -e "Failing: ${RED}${TOTAL_FAIL}${NC}"
echo -e "Overall Pass Rate: ${GREEN}${OVERALL_RATE}%${NC}"
echo ""

# Generate JSON report
JSON_REPORT="${RESULTS_DIR}/dogfood-${MODE}-$(date +%Y%m%d-%H%M%S).json"
cat > "$JSON_REPORT" <<EOF
{
  "timestamp": "${TIMESTAMP}",
  "ruchy_version": "${RUCHY_VERSION}",
  "mode": "${MODE}",
  "total_files": ${TOTAL_FILES},
  "total_tests": ${TOTAL_TESTS},
  "total_pass": ${TOTAL_PASS},
  "total_fail": ${TOTAL_FAIL},
  "overall_pass_rate": ${OVERALL_RATE},
  "tools": {
EOF

FIRST=true
for tool in "${TOOLS[@]}"; do
    if [ "$FIRST" = false ]; then
        echo "," >> "$JSON_REPORT"
    fi
    FIRST=false

    PASS=${TOOL_PASS[$tool]}
    FAIL=${TOOL_FAIL[$tool]}
    SKIP=${TOOL_SKIP[$tool]}
    TESTED=$((PASS + FAIL))

    if [[ $TESTED -eq 0 ]]; then
        RATE="null"
    else
        RATE=$(awk "BEGIN {printf \"%.3f\", ($PASS / $TESTED)}")
    fi

    cat >> "$JSON_REPORT" <<EOF
    "${tool}": {
      "pass": ${PASS},
      "fail": ${FAIL},
      "skip": ${SKIP},
      "tested": ${TESTED},
      "pass_rate": ${RATE}
    }
EOF
done

cat >> "$JSON_REPORT" <<EOF

  }
}
EOF

echo -e "📊 Report saved: ${JSON_REPORT}"
echo ""

# Exit with appropriate code
if [ "$OVERALL_PASS" = true ] && [[ $TOTAL_FAIL -eq 0 ]]; then
    echo -e "${GREEN}✅ All dogfooding tests passed!${NC}"
    exit 0
else
    echo -e "${YELLOW}⚠️  Some dogfooding tests failed. Review results above.${NC}"
    exit 1
fi
