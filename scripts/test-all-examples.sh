#!/usr/bin/env bash
# Test All Ruchy Examples - Comprehensive Test Suite
# Generates test-results.json for INTEGRATION.md updates

set -euo pipefail

# Configuration
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUTPUT_FILE="${PROJECT_ROOT}/test-results.json"
TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
RUCHY_VERSION=$(ruchy --version 2>/dev/null | awk '{print $2}' || echo "unknown")

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Counters
declare -A CATEGORY_TOTAL
declare -A CATEGORY_PASSING
declare -A CATEGORY_FAILING
declare -a RESULTS

echo -e "${BLUE}════════════════════════════════════════════${NC}"
echo -e "${BLUE}  Rosetta Ruchy - Test All Examples${NC}"
echo -e "${BLUE}════════════════════════════════════════════${NC}"
echo -e "Ruchy Version: ${GREEN}${RUCHY_VERSION}${NC}"
echo -e "Timestamp: ${TIMESTAMP}"
echo

# Find all .ruchy files
find_ruchy_files() {
    local category=$1
    find "${PROJECT_ROOT}/examples/${category}" -name "*.ruchy" 2>/dev/null || true
}

# Test a single .ruchy file
test_file() {
    local file=$1
    local category=$2
    local relative_path="${file#${PROJECT_ROOT}/}"

    echo -n "Testing ${relative_path} ... "

    # Run ruchy check
    if ruchy check "${file}" > /dev/null 2>&1; then
        echo -e "${GREEN}✓ PASS${NC}"
        CATEGORY_PASSING[$category]=$((${CATEGORY_PASSING[$category]:-0} + 1))

        # Store result
        RESULTS+=("{\"file\": \"${relative_path}\", \"category\": \"${category}\", \"status\": \"pass\", \"error\": null}")
        return 0
    else
        local error=$(ruchy check "${file}" 2>&1 | head -n 5 | sed 's/"/\\"/g' | tr '\n' ' ')
        echo -e "${RED}✗ FAIL${NC}"
        CATEGORY_FAILING[$category]=$((${CATEGORY_FAILING[$category]:-0} + 1))

        # Store result
        RESULTS+=("{\"file\": \"${relative_path}\", \"category\": \"${category}\", \"status\": \"fail\", \"error\": \"${error}\"}")
        return 1
    fi
}

# Test all files in a category
test_category() {
    local category=$1

    echo -e "${YELLOW}─────────────────────────────────────────────${NC}"
    echo -e "${YELLOW}Category: ${category}${NC}"
    echo -e "${YELLOW}─────────────────────────────────────────────${NC}"

    CATEGORY_TOTAL[$category]=0
    CATEGORY_PASSING[$category]=0
    CATEGORY_FAILING[$category]=0

    while IFS= read -r file; do
        if [[ -n "$file" ]]; then
            CATEGORY_TOTAL[$category]=$((${CATEGORY_TOTAL[$category]} + 1))
            test_file "$file" "$category" || true
        fi
    done < <(find_ruchy_files "$category")

    local passing=${CATEGORY_PASSING[$category]:-0}
    local total=${CATEGORY_TOTAL[$category]:-0}
    local rate=0

    if [[ $total -gt 0 ]]; then
        rate=$(awk "BEGIN {printf \"%.1f\", (${passing} / ${total}) * 100}")
    fi

    echo
    echo -e "Category Summary: ${GREEN}${passing}${NC}/${total} passing (${rate}%)"
    echo
}

# Main execution
main() {
    # Test all categories
    for category in algorithms data-science advanced-ai; do
        if [[ -d "${PROJECT_ROOT}/examples/${category}" ]]; then
            test_category "$category"
        fi
    done

    # Calculate totals
    local total_passing=0
    local total_files=0

    for category in "${!CATEGORY_TOTAL[@]}"; do
        total_files=$((total_files + ${CATEGORY_TOTAL[$category]}))
        total_passing=$((total_passing + ${CATEGORY_PASSING[$category]:-0}))
    done

    local success_rate=0
    if [[ $total_files -gt 0 ]]; then
        success_rate=$(awk "BEGIN {printf \"%.3f\", (${total_passing} / ${total_files})}")
    fi

    # Print summary
    echo -e "${BLUE}════════════════════════════════════════════${NC}"
    echo -e "${BLUE}  Final Summary${NC}"
    echo -e "${BLUE}════════════════════════════════════════════${NC}"
    echo -e "Total Examples: ${total_files}"
    echo -e "Passing: ${GREEN}${total_passing}${NC}"
    echo -e "Failing: ${RED}$((total_files - total_passing))${NC}"
    echo -e "Success Rate: ${GREEN}${success_rate}${NC} ($(awk "BEGIN {printf \"%.1f\", ${success_rate} * 100}")%)"
    echo

    # Category breakdown
    echo -e "${YELLOW}Category Breakdown:${NC}"
    for category in "${!CATEGORY_TOTAL[@]}"; do
        local passing=${CATEGORY_PASSING[$category]:-0}
        local total=${CATEGORY_TOTAL[$category]:-0}
        local rate=0

        if [[ $total -gt 0 ]]; then
            rate=$(awk "BEGIN {printf \"%.3f\", (${passing} / ${total})}")
        fi

        echo -e "  ${category}: ${passing}/${total} (${rate})"
    done
    echo

    # Generate JSON output
    cat > "${OUTPUT_FILE}" <<EOF
{
  "timestamp": "${TIMESTAMP}",
  "ruchy_version": "${RUCHY_VERSION}",
  "summary": {
    "total_examples": ${total_files},
    "passing": ${total_passing},
    "failing": $((total_files - total_passing)),
    "success_rate": ${success_rate}
  },
  "by_category": {
$(for category in "${!CATEGORY_TOTAL[@]}"; do
    local passing=${CATEGORY_PASSING[$category]:-0}
    local total=${CATEGORY_TOTAL[$category]:-0}
    local rate=0

    if [[ $total -gt 0 ]]; then
        rate=$(awk "BEGIN {printf \"%.3f\", (${passing} / ${total})}")
    fi

    echo "    \"${category}\": {"
    echo "      \"total\": ${total},"
    echo "      \"passing\": ${passing},"
    echo "      \"failing\": $((total - passing)),"
    echo "      \"rate\": ${rate}"
    echo "    },"
done | sed '$ s/,$//')
  },
  "results": [
$(IFS=,; echo "${RESULTS[*]}")
  ]
}
EOF

    echo -e "${GREEN}✓ Results saved to: ${OUTPUT_FILE}${NC}"
    echo

    # Check if regression threshold met
    if (( $(awk "BEGIN {print (${success_rate} >= 0.85) ? 1 : 0}") )); then
        echo -e "${GREEN}✓ Success rate meets threshold (≥85%)${NC}"
        exit 0
    else
        echo -e "${RED}✗ Success rate below threshold (<85%)${NC}"
        exit 1
    fi
}

# Run main
main "$@"
