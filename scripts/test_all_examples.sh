#!/bin/bash
# test_all_examples.sh - Comprehensive test aggregation for rosetta-ruchy
# Inspired by ruchy-book's extract-examples.ts but adapted for rosetta-ruchy
#
# Purpose: Test ALL Ruchy examples across algorithms, data-science, and advanced-ai
# Output: JSON report, console summary, and optional INTEGRATION.md update

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get Ruchy version
RUCHY_VERSION=$(ruchy --version 2>/dev/null || echo "unknown")
TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

echo -e "${BLUE}🧪 Rosetta-Ruchy Comprehensive Test Suite${NC}"
echo "=========================================="
echo ""
echo "Ruchy Version: $RUCHY_VERSION"
echo "Timestamp: $TIMESTAMP"
echo ""

# Initialize counters
TOTAL=0
PASSED=0
FAILED=0

# Category counters
declare -A CATEGORY_TOTAL
declare -A CATEGORY_PASSED

# Error counters
declare -A ERROR_TYPES

# Results array (for JSON)
RESULTS_JSON="["

# Test a single Ruchy file
test_ruchy_file() {
    local file_path="$1"
    local category="$2"
    local example_name="$3"
    local filename=$(basename "$file_path")

    printf "   Testing: %-40s" "$filename"

    # Run ruchy check
    if ruchy check "$file_path" &>/dev/null; then
        echo -e "${GREEN}✅ PASS${NC}"
        ((PASSED++))
        ((CATEGORY_PASSED[$category]++))

        # Get quality score if available
        SCORE=$(ruchy score "$file_path" 2>/dev/null | grep "Score:" | awk '{print $2}' | cut -d'/' -f1 || echo "0")

        # Add to JSON results
        if [ "$RESULTS_JSON" != "[" ]; then
            RESULTS_JSON+=","
        fi
        RESULTS_JSON+="{\"path\":\"$file_path\",\"category\":\"$category\",\"name\":\"$example_name\",\"passed\":true,\"score\":$SCORE}"
    else
        echo -e "${RED}❌ FAIL${NC}"
        ((FAILED++))

        # Categorize error
        ERROR_TYPE="SyntaxError"
        ((ERROR_TYPES[$ERROR_TYPE]++))

        # Add to JSON results
        if [ "$RESULTS_JSON" != "[" ]; then
            RESULTS_JSON+=","
        fi
        RESULTS_JSON+="{\"path\":\"$file_path\",\"category\":\"$category\",\"name\":\"$example_name\",\"passed\":false,\"error\":\"$ERROR_TYPE\"}"
    fi

    ((TOTAL++))
    ((CATEGORY_TOTAL[$category]++))
}

# Test an example directory
test_example() {
    local category_path="$1"
    local example_name="$2"
    local category=$(basename "$category_path")

    local ruchy_impl_dir="${category_path}/${example_name}/implementations/ruchy"

    if [ ! -d "$ruchy_impl_dir" ]; then
        return
    fi

    # Find all .ruchy files (excluding test_*, build*, benchmark*)
    for file in "$ruchy_impl_dir"/*.ruchy; do
        if [ -f "$file" ]; then
            local filename=$(basename "$file")
            # Skip test, build, and benchmark files
            if [[ ! "$filename" =~ ^test_ ]] && [[ ! "$filename" =~ ^build ]] && [[ ! "$filename" =~ ^benchmark ]]; then
                test_ruchy_file "$file" "$category" "$example_name"
            fi
        fi
    done
}

# Test a category
test_category() {
    local category_path="$1"
    local category=$(basename "$category_path")

    echo -e "${BLUE}📂 Testing category: $category_path${NC}"

    if [ ! -d "$category_path" ]; then
        echo -e "   ${YELLOW}⚠️  Directory not found: $category_path${NC}"
        return
    fi

    # Initialize category counters
    CATEGORY_TOTAL[$category]=0
    CATEGORY_PASSED[$category]=0

    # Find all example directories
    for example_dir in "$category_path"/*/; do
        if [ -d "$example_dir" ]; then
            local example_name=$(basename "$example_dir")
            test_example "$category_path" "$example_name"
        fi
    done
}

# Test all categories
test_category "examples/algorithms"
test_category "examples/data-science"
test_category "examples/advanced-ai"

# Close JSON array
RESULTS_JSON+="]"

# Calculate success rate
if [ $TOTAL -gt 0 ]; then
    SUCCESS_RATE=$(awk "BEGIN {printf \"%.1f\", ($PASSED/$TOTAL)*100}")
else
    SUCCESS_RATE="0.0"
fi

# Print summary
echo ""
echo -e "${BLUE}📊 TEST SUMMARY${NC}"
echo "=========================================="
echo "Ruchy Version: $RUCHY_VERSION"
echo "Timestamp:     $TIMESTAMP"
echo ""
echo "Total Examples:  $TOTAL"
echo -e "✅ Passed:       $PASSED (${SUCCESS_RATE}%)"
echo -e "❌ Failed:       $FAILED"
echo ""

# Category breakdown
echo "By Category:"
for category in "${!CATEGORY_TOTAL[@]}"; do
    total=${CATEGORY_TOTAL[$category]}
    passed=${CATEGORY_PASSED[$category]}
    if [ $total -gt 0 ]; then
        rate=$(awk "BEGIN {printf \"%.1f\", ($passed/$total)*100}")
        echo "  $category: $passed/$total ($rate%)"
    fi
done
echo ""

# Error breakdown
if [ ${#ERROR_TYPES[@]} -gt 0 ]; then
    echo "Error Categories:"
    for error in "${!ERROR_TYPES[@]}"; do
        echo "  $error: ${ERROR_TYPES[$error]}"
    done
    echo ""
fi

echo "=========================================="

# Generate JSON report
cat > test-results.json << EOF
{
  "summary": {
    "total": $TOTAL,
    "passed": $PASSED,
    "failed": $FAILED,
    "success_rate": $SUCCESS_RATE,
    "ruchy_version": "$RUCHY_VERSION",
    "timestamp": "$TIMESTAMP"
  },
  "by_category": {
EOF

# Add category stats
first=true
for category in "${!CATEGORY_TOTAL[@]}"; do
    total=${CATEGORY_TOTAL[$category]}
    passed=${CATEGORY_PASSED[$category]}
    if [ $total -gt 0 ]; then
        rate=$(awk "BEGIN {printf \"%.1f\", ($passed/$total)*100}")
        if [ "$first" = false ]; then
            echo "    ," >> test-results.json
        fi
        first=false
        echo -n "    \"$category\": {\"total\": $total, \"passed\": $passed, \"success_rate\": $rate}" >> test-results.json
    fi
done

cat >> test-results.json << EOF

  },
  "by_error": {
EOF

# Add error stats
first=true
for error in "${!ERROR_TYPES[@]}"; do
    if [ "$first" = false ]; then
        echo "    ," >> test-results.json
    fi
    first=false
    echo -n "    \"$error\": ${ERROR_TYPES[$error]}" >> test-results.json
done

cat >> test-results.json << EOF

  },
  "results": $RESULTS_JSON
}
EOF

echo -e "${GREEN}📄 Detailed report written to: test-results.json${NC}"

# Check for --update-integration flag
if [[ "$*" == *"--update-integration"* ]]; then
    echo ""
    echo -e "${BLUE}📝 Updating INTEGRATION.md...${NC}"

    # Prepend new entry to version history
    # This is a simplified version - full implementation would parse and update the table
    echo "✅ INTEGRATION.md update requested (manual update required)"
fi

# Exit with error if any tests failed
if [ $FAILED -gt 0 ]; then
    exit 1
fi
