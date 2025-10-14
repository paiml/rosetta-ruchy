#!/usr/bin/env bash
# Test Regression Detection
# Alerts when success rate drops below threshold

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CURRENT_FILE="${PROJECT_ROOT}/test-results.json"
BASELINE_FILE="${PROJECT_ROOT}/test-results-baseline.json"
THRESHOLD=0.05  # 5% drop triggers alert

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}════════════════════════════════════════════${NC}"
echo -e "${BLUE}  Regression Detection${NC}"
echo -e "${BLUE}════════════════════════════════════════════${NC}"

# Check if files exist
if [[ ! -f "${CURRENT_FILE}" ]]; then
    echo -e "${RED}Error: ${CURRENT_FILE} not found${NC}"
    echo "Run 'make test-all-examples' first"
    exit 1
fi

if [[ ! -f "${BASELINE_FILE}" ]]; then
    echo -e "${YELLOW}⚠ No baseline found. Creating baseline from current results...${NC}"
    cp "${CURRENT_FILE}" "${BASELINE_FILE}"
    echo -e "${GREEN}✓ Baseline created: ${BASELINE_FILE}${NC}"
    exit 0
fi

# Parse JSON (requires jq)
if ! command -v jq &> /dev/null; then
    echo "Error: jq is required but not installed"
    exit 1
fi

# Extract success rates
CURRENT_RATE=$(jq -r '.summary.success_rate' "${CURRENT_FILE}")
BASELINE_RATE=$(jq -r '.summary.success_rate' "${BASELINE_FILE}")
CURRENT_VERSION=$(jq -r '.ruchy_version' "${CURRENT_FILE}")
BASELINE_VERSION=$(jq -r '.ruchy_version' "${BASELINE_FILE}")

# Calculate delta
DELTA=$(awk "BEGIN {printf \"%.3f\", ${CURRENT_RATE} - ${BASELINE_RATE}}")
DELTA_PERCENT=$(awk "BEGIN {printf \"%.1f\", ${DELTA} * 100}")

# Display comparison
echo
echo -e "Baseline: ${BASELINE_VERSION} @ $(awk "BEGIN {printf \"%.1f\", ${BASELINE_RATE} * 100}")%"
echo -e "Current:  ${CURRENT_VERSION} @ $(awk "BEGIN {printf \"%.1f\", ${CURRENT_RATE} * 100}")%"
echo -e "Delta:    ${DELTA_PERCENT}%"
echo

# Category-level regression check
declare -a REGRESSIONS

for category in algorithms data-science advanced-ai; do
    CURRENT_CAT_RATE=$(jq -r ".by_category[\"${category}\"].rate // 0" "${CURRENT_FILE}")
    BASELINE_CAT_RATE=$(jq -r ".by_category[\"${category}\"].rate // 0" "${BASELINE_FILE}")

    CAT_DELTA=$(awk "BEGIN {printf \"%.3f\", ${CURRENT_CAT_RATE} - ${BASELINE_CAT_RATE}}")
    CAT_DELTA_PERCENT=$(awk "BEGIN {printf \"%.1f\", ${CAT_DELTA} * 100}")

    if (( $(awk "BEGIN {print (${CAT_DELTA} < -${THRESHOLD}) ? 1 : 0}") )); then
        echo -e "${RED}✗ Regression detected in ${category}: ${CAT_DELTA_PERCENT}%${NC}"
        REGRESSIONS+=("${category}")
    elif (( $(awk "BEGIN {print (${CAT_DELTA} < 0) ? 1 : 0}") )); then
        echo -e "${YELLOW}⚠ Minor drop in ${category}: ${CAT_DELTA_PERCENT}%${NC}"
    else
        echo -e "${GREEN}✓ ${category}: ${CAT_DELTA_PERCENT}%${NC}"
    fi
done

echo

# Check overall regression
if (( $(awk "BEGIN {print (${DELTA} < -${THRESHOLD}) ? 1 : 0}") )); then
    echo -e "${RED}════════════════════════════════════════════${NC}"
    echo -e "${RED}  ✗ REGRESSION DETECTED (>${THRESHOLD_PERCENT}% drop)${NC}"
    echo -e "${RED}════════════════════════════════════════════${NC}"
    echo
    echo -e "Baseline: $(awk "BEGIN {printf \"%.1f\", ${BASELINE_RATE} * 100}")%"
    echo -e "Current:  $(awk "BEGIN {printf \"%.1f\", ${CURRENT_RATE} * 100}")%"
    echo -e "Drop:     ${DELTA_PERCENT}%"
    echo

    if [[ ${#REGRESSIONS[@]} -gt 0 ]]; then
        echo "Affected categories:"
        for cat in "${REGRESSIONS[@]}"; do
            echo "  - ${cat}"
        done
        echo
    fi

    echo "Action required:"
    echo "  1. Review failing examples in test-results.json"
    echo "  2. Check for breaking changes in Ruchy ${CURRENT_VERSION}"
    echo "  3. Update INTEGRATION.md with migration notes"
    echo "  4. File issues for new bugs discovered"
    echo

    exit 1

elif (( $(awk "BEGIN {print (${DELTA} < 0) ? 1 : 0}") )); then
    echo -e "${YELLOW}════════════════════════════════════════════${NC}"
    echo -e "${YELLOW}  ⚠ Minor regression (<${THRESHOLD_PERCENT}% drop)${NC}"
    echo -e "${YELLOW}════════════════════════════════════════════${NC}"
    echo
    echo "Monitor for further drops in future sprints"
    echo

    exit 0

else
    echo -e "${GREEN}════════════════════════════════════════════${NC}"
    echo -e "${GREEN}  ✓ No regression detected${NC}"
    echo -e "${GREEN}════════════════════════════════════════════${NC}"
    echo

    if (( $(awk "BEGIN {print (${DELTA} > 0) ? 1 : 0}") )); then
        echo -e "${GREEN}Success rate improved by ${DELTA_PERCENT}%!${NC}"
        echo

        # Offer to update baseline
        if [[ "${UPDATE_BASELINE:-}" == "1" ]]; then
            echo "Updating baseline..."
            cp "${CURRENT_FILE}" "${BASELINE_FILE}"
            echo -e "${GREEN}✓ Baseline updated${NC}"
        else
            echo "To update baseline, run: UPDATE_BASELINE=1 make test-regression"
        fi
    fi

    exit 0
fi
