#!/bin/bash
# Documentation Test Suite
# Validates documentation accuracy following paragraph-by-paragraph spec
# PMAT-style quality validation

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

ERRORS=0
WARNINGS=0

echo "================================================================"
echo "Documentation Quality Validation Suite"
echo "Following: docs/specifications/paragraph-by-paragraph-spec.md"
echo "================================================================"
echo ""

# Test 1: Version Consistency
echo "📋 Test 1: Version Consistency Across Documents"
echo "----------------------------------------------------------------"

RUCHY_VERSION=$(ruchy --version 2>/dev/null | grep -o "3\.[0-9]\+\.[0-9]\+" | head -1 || echo "unknown")
echo "Current Ruchy version: $RUCHY_VERSION"

# Find all version references in core docs
VERSION_REFS=$(grep -ho "3\.[0-9]\+\.[0-9]\+" \
    README.md \
    CLAUDE.md \
    CONTRIBUTING.md \
    Makefile 2>/dev/null | sort -u)

VERSION_COUNT=$(echo "$VERSION_REFS" | wc -l)

echo "Version references found in core docs:"
echo "$VERSION_REFS"
echo ""

if [ "$VERSION_COUNT" -eq 1 ]; then
    echo -e "${GREEN}✅ PASS${NC}: Single consistent Ruchy version across all documentation"
else
    echo -e "${RED}❌ FAIL${NC}: Found $VERSION_COUNT different Ruchy versions in documentation"
    echo "Expected: Single version only"
    echo "Actual: Multiple versions"
    ERRORS=$((ERRORS + 1))
fi
echo ""

# Test 2: Metrics Accuracy (vs INTEGRATION.md)
echo "📊 Test 2: Metrics Accuracy vs INTEGRATION.md"
echo "----------------------------------------------------------------"

if [ -f "test-results.json" ]; then
    ACTUAL_TOTAL=$(jq -r '.summary.total' test-results.json 2>/dev/null || echo "0")
    ACTUAL_PASSING=$(jq -r '.summary.passing' test-results.json 2>/dev/null || echo "0")
    ACTUAL_RATE=$(jq -r '.summary.success_rate * 100' test-results.json 2>/dev/null | awk '{printf "%.1f", $1}')

    echo "test-results.json metrics:"
    echo "  Total: $ACTUAL_TOTAL"
    echo "  Passing: $ACTUAL_PASSING"
    echo "  Success Rate: ${ACTUAL_RATE}%"
    echo ""

    # Check README claims match
    README_TOTAL=$(grep -o "[0-9]\+/[0-9]\+ examples" README.md | head -1 | grep -o "[0-9]\+/[0-9]\+" || echo "unknown")
    README_RATE=$(grep -o "[0-9]\+\.[0-9]\+%" README.md | head -1 | grep -o "[0-9]\+\.[0-9]\+" || echo "unknown")

    echo "README.md claims:"
    echo "  Examples: $README_TOTAL"
    echo "  Rate: ${README_RATE}%"
    echo ""

    # Validate total count
    if echo "$README_TOTAL" | grep -q "$ACTUAL_TOTAL"; then
        echo -e "${GREEN}✅ PASS${NC}: Example count matches ($ACTUAL_TOTAL total)"
    else
        echo -e "${RED}❌ FAIL${NC}: Example count mismatch"
        echo "  Expected in README: $ACTUAL_TOTAL"
        echo "  Found in README: $README_TOTAL"
        ERRORS=$((ERRORS + 1))
    fi

    # Validate success rate
    if [ "$README_RATE" = "$ACTUAL_RATE" ] || [ "$README_RATE" = "100.0" ] && [ "$ACTUAL_RATE" = "100.0" ]; then
        echo -e "${GREEN}✅ PASS${NC}: Success rate matches (${ACTUAL_RATE}%)"
    else
        echo -e "${YELLOW}⚠️  WARNING${NC}: Success rate discrepancy"
        echo "  test-results.json: ${ACTUAL_RATE}%"
        echo "  README.md: ${README_RATE}%"
        WARNINGS=$((WARNINGS + 1))
    fi
else
    echo -e "${YELLOW}⚠️  WARNING${NC}: test-results.json not found, skipping metrics validation"
    echo "Run: make test-all-examples"
    WARNINGS=$((WARNINGS + 1))
fi
echo ""

# Test 3: SATD Detection (Self-Admitted Technical Debt)
echo "🔍 Test 3: SATD Detection (Zero Tolerance)"
echo "----------------------------------------------------------------"

SATD_PATTERNS="TODO\|FIXME\|HACK"
# Exclude policy documentation, examples, placeholders, and instructional text
EXCLUSIONS="Zero tolerance\|SATD\|quality gate\|Not yet\|No.*support\|ROSETTA-XXX\|algorithm XXX\|XXX-algorithm\|examples/algorithms/XXX\|feat(algorithms): Implement XXX\|XXXms (Ruchy)\|algorithm-XXX"

# Check core documentation files
SATD_FOUND=0
for FILE in README.md CLAUDE.md CONTRIBUTING.md; do
    if [ -f "$FILE" ]; then
        # Find SATD but exclude documentation about SATD policy
        SATD_IN_FILE=$(grep -n "$SATD_PATTERNS" "$FILE" | grep -v "$EXCLUSIONS" || true)
        if [ -n "$SATD_IN_FILE" ]; then
            echo -e "${RED}Found SATD in $FILE:${NC}"
            echo "$SATD_IN_FILE"
            SATD_FOUND=$((SATD_FOUND + 1))
        fi
    fi
done

if [ $SATD_FOUND -eq 0 ]; then
    echo -e "${GREEN}✅ PASS${NC}: Zero SATD comments in core documentation"
else
    echo -e "${RED}❌ FAIL${NC}: Found SATD comments in $SATD_FOUND file(s)"
    echo "Policy: Zero tolerance for TODO/FIXME/HACK in production documentation"
    ERRORS=$((ERRORS + 1))
fi
echo ""

# Test 4: Required Files Exist
echo "📁 Test 4: Required Files Exist"
echo "----------------------------------------------------------------"

REQUIRED_FILES=(
    "README.md"
    "CLAUDE.md"
    "CONTRIBUTING.md"
    "INTEGRATION.md"
    "Makefile"
    "roadmap.yaml"
    "docs/specifications/paragraph-by-paragraph-spec.md"
)

MISSING_FILES=0
for FILE in "${REQUIRED_FILES[@]}"; do
    if [ -f "$FILE" ]; then
        echo -e "${GREEN}✓${NC} $FILE"
    else
        echo -e "${RED}✗${NC} $FILE (MISSING)"
        MISSING_FILES=$((MISSING_FILES + 1))
    fi
done

echo ""
if [ $MISSING_FILES -eq 0 ]; then
    echo -e "${GREEN}✅ PASS${NC}: All required files present"
else
    echo -e "${RED}❌ FAIL${NC}: $MISSING_FILES required file(s) missing"
    ERRORS=$((ERRORS + 1))
fi
echo ""

# Test 5: Documentation Structure
echo "🏗️  Test 5: Documentation Structure Validation"
echo "----------------------------------------------------------------"

# Check README has essential sections
README_SECTIONS=(
    "Project Architecture"
    "Quality Gates"
    "Ruchy Formal Verification"
    "Current Status"
    "Quick Start"
)

MISSING_SECTIONS=0
for SECTION in "${README_SECTIONS[@]}"; do
    if grep -q "$SECTION" README.md 2>/dev/null; then
        echo -e "${GREEN}✓${NC} Section: $SECTION"
    else
        echo -e "${RED}✗${NC} Section missing: $SECTION"
        MISSING_SECTIONS=$((MISSING_SECTIONS + 1))
    fi
done

echo ""
if [ $MISSING_SECTIONS -eq 0 ]; then
    echo -e "${GREEN}✅ PASS${NC}: README.md has all essential sections"
else
    echo -e "${RED}❌ FAIL${NC}: $MISSING_SECTIONS essential section(s) missing from README"
    ERRORS=$((ERRORS + 1))
fi
echo ""

# Test 6: Makefile Version Requirements
echo "⚙️  Test 6: Makefile Version Requirements"
echo "----------------------------------------------------------------"

if [ -f "Makefile" ]; then
    MAKEFILE_VERSION=$(grep "REQUIRED_VERSION :=" Makefile | grep -o "3\.[0-9]\+\.[0-9]\+" || echo "unknown")
    echo "Makefile REQUIRED_VERSION: $MAKEFILE_VERSION"
    echo "Current Ruchy version: $RUCHY_VERSION"
    echo ""

    if [ "$MAKEFILE_VERSION" = "$RUCHY_VERSION" ]; then
        echo -e "${GREEN}✅ PASS${NC}: Makefile REQUIRED_VERSION matches installed Ruchy"
    else
        echo -e "${YELLOW}⚠️  WARNING${NC}: Version mismatch"
        echo "  Makefile requires: $MAKEFILE_VERSION"
        echo "  Installed version: $RUCHY_VERSION"
        WARNINGS=$((WARNINGS + 1))
    fi
else
    echo -e "${RED}❌ FAIL${NC}: Makefile not found"
    ERRORS=$((ERRORS + 1))
fi
echo ""

# Test 7: Integration Status Freshness
echo "🕐 Test 7: INTEGRATION.md Freshness"
echo "----------------------------------------------------------------"

if [ -f "INTEGRATION.md" ]; then
    INTEGRATION_DATE=$(grep "Last Updated" INTEGRATION.md | grep -o "[0-9]\{4\}-[0-9]\{2\}-[0-9]\{2\}" | head -1 || echo "unknown")
    echo "INTEGRATION.md last updated: $INTEGRATION_DATE"

    # Check if within last 7 days
    if [ "$INTEGRATION_DATE" != "unknown" ]; then
        DAYS_OLD=$(( ($(date +%s) - $(date -d "$INTEGRATION_DATE" +%s)) / 86400 ))
        echo "Age: $DAYS_OLD days"
        echo ""

        if [ $DAYS_OLD -le 7 ]; then
            echo -e "${GREEN}✅ PASS${NC}: INTEGRATION.md is fresh (≤7 days old)"
        elif [ $DAYS_OLD -le 30 ]; then
            echo -e "${YELLOW}⚠️  WARNING${NC}: INTEGRATION.md is $DAYS_OLD days old"
            echo "Consider running: make update-integration"
            WARNINGS=$((WARNINGS + 1))
        else
            echo -e "${RED}❌ FAIL${NC}: INTEGRATION.md is stale (>30 days old)"
            echo "MUST run: make update-integration"
            ERRORS=$((ERRORS + 1))
        fi
    else
        echo -e "${YELLOW}⚠️  WARNING${NC}: Could not determine INTEGRATION.md age"
        WARNINGS=$((WARNINGS + 1))
    fi
else
    echo -e "${RED}❌ FAIL${NC}: INTEGRATION.md not found"
    ERRORS=$((ERRORS + 1))
fi
echo ""

# Summary
echo "================================================================"
echo "Test Suite Summary"
echo "================================================================"
echo ""

TOTAL_TESTS=7
PASSED=$((TOTAL_TESTS - ERRORS))

echo "Tests Run: $TOTAL_TESTS"
echo -e "Passed: ${GREEN}$PASSED${NC}"
echo -e "Failed: ${RED}$ERRORS${NC}"
echo -e "Warnings: ${YELLOW}$WARNINGS${NC}"
echo ""

if [ $ERRORS -eq 0 ]; then
    echo -e "${GREEN}✅ ALL TESTS PASSED${NC}"
    echo ""
    echo "Documentation quality validated successfully!"
    echo "All files follow paragraph-by-paragraph-spec.md requirements."
    exit 0
else
    echo -e "${RED}❌ TESTS FAILED${NC}"
    echo ""
    echo "Please fix the errors above before committing."
    echo "Refer to: docs/specifications/paragraph-by-paragraph-spec.md"
    exit 1
fi
