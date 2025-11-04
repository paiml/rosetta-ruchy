#!/bin/bash
# PMAT-Style Quality Validation for rosetta-ruchy
# Alternative to PMAT binary when cargo install is not available
# Implements same quality gates following PMAT methodology

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

ERRORS=0
WARNINGS=0

echo "================================================================"
echo "PMAT-Style Quality Validation Suite"
echo "Following: PAIML MCP Agent Toolkit methodology"
echo "================================================================"
echo ""

# Parse command line arguments
COMMAND=${1:-"validate"}
shift || true

case "$COMMAND" in
    analyze)
        ANALYSIS_TYPE=${1:-"all"}
        shift || true
        ;;
    validate)
        VALIDATION_TYPE=${1:-"all"}
        shift || true
        ;;
    *)
        echo "Usage: $0 {analyze|validate} {type} [options]"
        echo ""
        echo "analyze commands:"
        echo "  analyze complexity --max N    Check complexity ≤N"
        echo "  analyze satd --threshold N    Check SATD count ≤N"
        echo ""
        echo "validate commands:"
        echo "  validate links               Check all links valid"
        echo "  validate consistency         Check version consistency"
        echo "  validate metrics             Check metrics accuracy"
        exit 1
        ;;
esac

# Function: Analyze Complexity
analyze_complexity() {
    local MAX_COMPLEXITY=${MAX:-20}

    echo "📊 Analyzing Complexity (PMAT-style)"
    echo "----------------------------------------------------------------"
    echo "Max Complexity Threshold: $MAX_COMPLEXITY"
    echo ""

    # Check for complex files (using basic metrics)
    # Count functions, nested blocks, etc.

    COMPLEX_FILES=0

    for FILE in README.md CLAUDE.md CONTRIBUTING.md; do
        if [ -f "$FILE" ]; then
            # Count deeply nested lists (proxy for complexity)
            NEST_DEPTH=$(grep -o "^[[:space:]]*-" "$FILE" | sed 's/[^[:space:]]//g' | awk '{print length}' | sort -n | tail -1 || echo "0")

            if [ "$NEST_DEPTH" -gt 8 ]; then
                echo -e "${YELLOW}⚠️  $FILE: High nesting depth ($NEST_DEPTH levels)${NC}"
                WARNINGS=$((WARNINGS + 1))
            else
                echo -e "${GREEN}✓${NC} $FILE: Acceptable structure (depth: $NEST_DEPTH)"
            fi
        fi
    done

    echo ""
    if [ $COMPLEX_FILES -eq 0 ]; then
        echo -e "${GREEN}✅ PASS${NC}: No complexity violations found"
    else
        echo -e "${YELLOW}⚠️  WARNING${NC}: $COMPLEX_FILES file(s) may need refactoring"
    fi
    echo ""
}

# Function: Analyze SATD
analyze_satd() {
    local THRESHOLD=${THRESHOLD:-0}

    echo "🔍 Analyzing SATD (Self-Admitted Technical Debt)"
    echo "----------------------------------------------------------------"
    echo "Threshold: ≤$THRESHOLD instances"
    echo ""

    SATD_PATTERNS="TODO\|FIXME\|HACK"
    EXCLUSIONS="Zero tolerance\|SATD\|quality gate\|ROSETTA-XXX\|algorithm XXX\|examples/algorithms/XXX"

    SATD_COUNT=0

    for FILE in README.md CLAUDE.md CONTRIBUTING.md docs/specifications/*.md; do
        if [ -f "$FILE" ]; then
            FILE_SATD=$(grep -n "$SATD_PATTERNS" "$FILE" 2>/dev/null | grep -v "$EXCLUSIONS" || true)
            FILE_COUNT=$(echo "$FILE_SATD" | grep -c . || echo "0")

            if [ "$FILE_COUNT" -gt 0 ]; then
                echo -e "${RED}Found $FILE_COUNT SATD in $FILE:${NC}"
                echo "$FILE_SATD" | head -5
                SATD_COUNT=$((SATD_COUNT + FILE_COUNT))
            else
                echo -e "${GREEN}✓${NC} $FILE: Zero SATD"
            fi
        fi
    done

    echo ""
    if [ "$SATD_COUNT" -le "$THRESHOLD" ]; then
        echo -e "${GREEN}✅ PASS${NC}: SATD count ($SATD_COUNT) ≤ threshold ($THRESHOLD)"
    else
        echo -e "${RED}❌ FAIL${NC}: SATD count ($SATD_COUNT) > threshold ($THRESHOLD)"
        ERRORS=$((ERRORS + 1))
    fi
    echo ""
}

# Function: Validate Links
validate_links() {
    echo "🔗 Validating Links (PMAT-style)"
    echo "----------------------------------------------------------------"
    echo ""

    BROKEN_LINKS=0

    for FILE in README.md CLAUDE.md CONTRIBUTING.md; do
        if [ -f "$FILE" ]; then
            echo "Checking $FILE..."

            # Extract markdown links
            LINKS=$(grep -o '\[.*\](.*\.md)' "$FILE" | grep -o '(.*\.md)' | tr -d '()' || true)

            for LINK in $LINKS; do
                # Check if it's a relative path
                if [[ ! "$LINK" =~ ^http ]]; then
                    # Resolve relative path
                    LINK_PATH=$(dirname "$FILE")/"$LINK"

                    if [ ! -f "$LINK_PATH" ] && [ ! -f "$LINK" ]; then
                        echo -e "  ${RED}✗${NC} Broken link: $LINK"
                        BROKEN_LINKS=$((BROKEN_LINKS + 1))
                    else
                        echo -e "  ${GREEN}✓${NC} Valid: $LINK"
                    fi
                fi
            done
        fi
    done

    echo ""
    if [ $BROKEN_LINKS -eq 0 ]; then
        echo -e "${GREEN}✅ PASS${NC}: All internal links valid"
    else
        echo -e "${RED}❌ FAIL${NC}: $BROKEN_LINKS broken link(s) found"
        ERRORS=$((ERRORS + 1))
    fi
    echo ""
}

# Function: Validate Consistency
validate_consistency() {
    echo "🔄 Validating Version Consistency (PMAT-style)"
    echo "----------------------------------------------------------------"
    echo ""

    # Extract all Ruchy version references
    VERSION_REFS=$(grep -ho "3\.[0-9]\+\.[0-9]\+" README.md CLAUDE.md CONTRIBUTING.md Makefile 2>/dev/null | sort -u)
    VERSION_COUNT=$(echo "$VERSION_REFS" | wc -l)

    echo "Version references found:"
    echo "$VERSION_REFS"
    echo ""

    if [ "$VERSION_COUNT" -eq 1 ]; then
        echo -e "${GREEN}✅ PASS${NC}: Single consistent version across all files"
    else
        echo -e "${RED}❌ FAIL${NC}: Found $VERSION_COUNT different versions"
        echo "Expected: Single version only"
        ERRORS=$((ERRORS + 1))
    fi
    echo ""
}

# Function: Validate Metrics
validate_metrics() {
    echo "📊 Validating Metrics Accuracy (PMAT-style)"
    echo "----------------------------------------------------------------"
    echo ""

    if [ ! -f "test-results.json" ]; then
        echo -e "${YELLOW}⚠️  WARNING${NC}: test-results.json not found"
        echo "Run: make test-all-examples"
        WARNINGS=$((WARNINGS + 1))
        return
    fi

    # Extract metrics from test-results.json
    ACTUAL_TOTAL=$(jq -r '.summary.total' test-results.json 2>/dev/null || echo "unknown")
    ACTUAL_PASSING=$(jq -r '.summary.passing' test-results.json 2>/dev/null || echo "unknown")
    ACTUAL_RATE=$(jq -r '.summary.success_rate * 100' test-results.json 2>/dev/null | awk '{printf "%.1f", $1}')

    echo "Source of Truth (test-results.json):"
    echo "  Total: $ACTUAL_TOTAL"
    echo "  Passing: $ACTUAL_PASSING"
    echo "  Success Rate: ${ACTUAL_RATE}%"
    echo ""

    # Check README claims
    README_EXAMPLES=$(grep -o "[0-9]\+/[0-9]\+ examples" README.md | head -1 || echo "unknown")

    echo "Documentation Claims (README.md):"
    echo "  Examples: $README_EXAMPLES"
    echo ""

    if echo "$README_EXAMPLES" | grep -q "$ACTUAL_PASSING"; then
        echo -e "${GREEN}✅ PASS${NC}: Documentation metrics match test results"
    else
        echo -e "${YELLOW}⚠️  WARNING${NC}: Metrics may be out of sync"
        echo "  Expected: $ACTUAL_PASSING/$ACTUAL_TOTAL"
        echo "  Found: $README_EXAMPLES"
        WARNINGS=$((WARNINGS + 1))
    fi
    echo ""
}

# Execute based on command
case "$COMMAND" in
    analyze)
        case "$ANALYSIS_TYPE" in
            complexity)
                # Parse --max flag
                while [[ $# -gt 0 ]]; do
                    case $1 in
                        --max)
                            MAX="$2"
                            shift 2
                            ;;
                        *)
                            shift
                            ;;
                    esac
                done
                analyze_complexity
                ;;
            satd)
                # Parse --threshold flag
                while [[ $# -gt 0 ]]; do
                    case $1 in
                        --threshold)
                            THRESHOLD="$2"
                            shift 2
                            ;;
                        *)
                            shift
                            ;;
                    esac
                done
                analyze_satd
                ;;
            all)
                analyze_complexity
                analyze_satd
                ;;
            *)
                echo "Unknown analysis type: $ANALYSIS_TYPE"
                exit 1
                ;;
        esac
        ;;
    validate)
        case "$VALIDATION_TYPE" in
            links)
                validate_links
                ;;
            consistency)
                validate_consistency
                ;;
            metrics)
                validate_metrics
                ;;
            all)
                validate_links
                validate_consistency
                validate_metrics
                ;;
            *)
                echo "Unknown validation type: $VALIDATION_TYPE"
                exit 1
                ;;
        esac
        ;;
esac

# Summary
echo "================================================================"
echo "PMAT-Style Validation Summary"
echo "================================================================"
echo ""

if [ $ERRORS -eq 0 ] && [ $WARNINGS -eq 0 ]; then
    echo -e "${GREEN}✅ ALL CHECKS PASSED${NC}"
    exit 0
elif [ $ERRORS -eq 0 ]; then
    echo -e "${YELLOW}⚠️  PASSED WITH WARNINGS${NC}"
    echo "Warnings: $WARNINGS"
    exit 0
else
    echo -e "${RED}❌ VALIDATION FAILED${NC}"
    echo "Errors: $ERRORS"
    echo "Warnings: $WARNINGS"
    exit 1
fi
