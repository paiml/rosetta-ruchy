#!/bin/bash
# Comprehensive Book Validation
# Validates ALL documentation files in the repository
# Following: docs/specifications/paragraph-by-paragraph-spec.md

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

ERRORS=0
WARNINGS=0
CHECKED=0

echo "================================================================"
echo "Comprehensive Book Validation - ALL Documentation Files"
echo "================================================================"
echo ""

# Get current Ruchy version
CURRENT_VERSION=$(grep "REQUIRED_VERSION :=" Makefile | grep -o "3\.[0-9]\+\.[0-9]\+" || echo "3.88.0")
echo "📌 Current Ruchy Version: $CURRENT_VERSION"
echo ""

# Find all markdown files
echo "📚 Finding all markdown files..."
MARKDOWN_FILES=$(find . -name "*.md" -type f ! -path "./.git/*" ! -path "./node_modules/*" ! -path "./target/*" | sort)
TOTAL_FILES=$(echo "$MARKDOWN_FILES" | wc -l)

echo "Found: $TOTAL_FILES markdown files"
echo ""

# Create validation report
REPORT_FILE="book-validation-report.md"
echo "# Comprehensive Book Validation Report" > "$REPORT_FILE"
echo "" >> "$REPORT_FILE"
echo "**Date**: $(date -u +"%Y-%m-%d %H:%M:%S UTC")" >> "$REPORT_FILE"
echo "**Ruchy Version**: $CURRENT_VERSION" >> "$REPORT_FILE"
echo "**Total Files**: $TOTAL_FILES" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

# Categorize files by tier
echo "## File Categories" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

# Tier 1 (P0): Core documentation
TIER1_FILES=(
    "./README.md"
    "./CLAUDE.md"
    "./CONTRIBUTING.md"
    "./INTEGRATION.md"
    "./Makefile"
)

# Tier 2 (P1): Specifications
TIER2_PATTERN="./docs/specifications/"

# Tier 3 (P2): Reports and summaries
TIER3_PATTERNS="REPORT\|SUMMARY\|SCIENTIFIC_REPORT"

echo "=== TIER 1 (P0): Core Documentation ==="
echo "### Tier 1 (P0): Core Documentation" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

for FILE in "${TIER1_FILES[@]}"; do
    if [ -f "$FILE" ]; then
        echo -e "${CYAN}Checking:${NC} $FILE"
        CHECKED=$((CHECKED + 1))

        # Check for old version references
        OLD_VERSIONS=$(grep -n "3\.79\.0\|3\.77\.0\|3\.62\|1\.89\.0" "$FILE" 2>/dev/null || true)

        if [ -n "$OLD_VERSIONS" ]; then
            echo -e "  ${YELLOW}⚠️  Found outdated version references${NC}"
            echo "- ⚠️ **$FILE**: Outdated version references found" >> "$REPORT_FILE"
            WARNINGS=$((WARNINGS + 1))
        else
            echo -e "  ${GREEN}✓${NC} Version references current"
            echo "- ✅ **$FILE**: Up to date" >> "$REPORT_FILE"
        fi
    fi
done

echo ""
echo "### Tier 2 (P1): Specifications" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

echo "=== TIER 2 (P1): Specifications ==="
TIER2_COUNT=0
TIER2_OUTDATED=0

for FILE in $MARKDOWN_FILES; do
    if [[ "$FILE" == *"$TIER2_PATTERN"* ]]; then
        TIER2_COUNT=$((TIER2_COUNT + 1))
        CHECKED=$((CHECKED + 1))

        # Check for old versions
        OLD_VERSIONS=$(grep -n "3\.79\.0\|3\.77\.0\|3\.62\|1\.89\.0" "$FILE" 2>/dev/null || true)

        if [ -n "$OLD_VERSIONS" ]; then
            echo -e "${YELLOW}⚠️${NC} $FILE"
            TIER2_OUTDATED=$((TIER2_OUTDATED + 1))
            WARNINGS=$((WARNINGS + 1))
        else
            echo -e "${GREEN}✓${NC} $FILE"
        fi
    fi
done

echo "  Checked: $TIER2_COUNT files, Outdated: $TIER2_OUTDATED"
echo "**Checked**: $TIER2_COUNT files, **Outdated**: $TIER2_OUTDATED" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

echo ""
echo "### Tier 3 (P2): Reports and Summaries" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

echo "=== TIER 3 (P2): Reports and Summaries ==="
TIER3_COUNT=0
TIER3_OUTDATED=0

for FILE in $MARKDOWN_FILES; do
    if echo "$FILE" | grep -q "$TIER3_PATTERNS"; then
        TIER3_COUNT=$((TIER3_COUNT + 1))
        CHECKED=$((CHECKED + 1))

        # Check for old versions (less strict for historical docs)
        OLD_VERSIONS=$(grep -n "3\.79\.0\|3\.77\.0" "$FILE" 2>/dev/null || true)

        if [ -n "$OLD_VERSIONS" ]; then
            TIER3_OUTDATED=$((TIER3_OUTDATED + 1))
            WARNINGS=$((WARNINGS + 1))
        fi
    fi
done

echo "  Checked: $TIER3_COUNT files, Outdated: $TIER3_OUTDATED"
echo "**Checked**: $TIER3_COUNT files, **Outdated**: $TIER3_OUTDATED" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

echo ""
echo "=== VERSION CONSISTENCY ANALYSIS ==="
echo "## Version Consistency Analysis" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

# Find all unique version references
ALL_VERSIONS=$(grep -rho "3\.[0-9]\+\.[0-9]\+" . --include="*.md" 2>/dev/null | sort -u)
VERSION_COUNT=$(echo "$ALL_VERSIONS" | wc -l)

echo "Unique Ruchy versions found in documentation:"
echo "$ALL_VERSIONS"
echo ""

echo "### Unique Versions Found" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"
echo '```' >> "$REPORT_FILE"
echo "$ALL_VERSIONS" >> "$REPORT_FILE"
echo '```' >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

if [ "$VERSION_COUNT" -eq 1 ]; then
    echo -e "${GREEN}✅ EXCELLENT${NC}: Single version ($CURRENT_VERSION) across all documentation"
    echo "**Status**: ✅ **EXCELLENT** - Single version across all documentation" >> "$REPORT_FILE"
elif [ "$VERSION_COUNT" -le 3 ]; then
    echo -e "${YELLOW}⚠️  ACCEPTABLE${NC}: $VERSION_COUNT versions found (mostly historical)"
    echo "**Status**: ⚠️ **ACCEPTABLE** - $VERSION_COUNT versions found (likely historical references)" >> "$REPORT_FILE"
    WARNINGS=$((WARNINGS + 1))
else
    echo -e "${RED}❌ NEEDS CLEANUP${NC}: $VERSION_COUNT versions found"
    echo "**Status**: ❌ **NEEDS CLEANUP** - $VERSION_COUNT versions found" >> "$REPORT_FILE"
    ERRORS=$((ERRORS + 1))
fi

echo ""
echo "" >> "$REPORT_FILE"

# SATD Analysis
echo "=== SATD ANALYSIS (Self-Admitted Technical Debt) ==="
echo "## SATD Analysis" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

SATD_PATTERNS="TODO\|FIXME\|HACK"
SATD_EXCLUSIONS="Zero tolerance\|SATD\|quality gate\|ROSETTA-XXX\|algorithm XXX"

SATD_COUNT=0
SATD_FILES=()

for FILE in $MARKDOWN_FILES; do
    if [ -f "$FILE" ]; then
        FILE_SATD=$(grep -n "$SATD_PATTERNS" "$FILE" 2>/dev/null | grep -v "$SATD_EXCLUSIONS" || true)

        if [ -n "$FILE_SATD" ]; then
            COUNT=$(echo "$FILE_SATD" | wc -l)
            SATD_COUNT=$((SATD_COUNT + COUNT))
            SATD_FILES+=("$FILE")

            # Only show first 3 for brevity
            if [ ${#SATD_FILES[@]} -le 3 ]; then
                echo -e "${YELLOW}Found SATD in:${NC} $FILE ($COUNT instances)"
            fi
        fi
    fi
done

echo ""
echo "Total SATD instances: $SATD_COUNT in ${#SATD_FILES[@]} files"
echo "**Total SATD**: $SATD_COUNT instances in ${#SATD_FILES[@]} files" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

if [ $SATD_COUNT -eq 0 ]; then
    echo -e "${GREEN}✅ EXCELLENT${NC}: Zero SATD in documentation"
    echo "**Status**: ✅ **EXCELLENT** - Zero SATD" >> "$REPORT_FILE"
elif [ $SATD_COUNT -le 10 ]; then
    echo -e "${YELLOW}⚠️  ACCEPTABLE${NC}: Low SATD count (policy documentation allowed)"
    echo "**Status**: ⚠️ **ACCEPTABLE** - Low SATD count" >> "$REPORT_FILE"
    WARNINGS=$((WARNINGS + 1))
else
    echo -e "${RED}❌ NEEDS CLEANUP${NC}: High SATD count"
    echo "**Status**: ❌ **NEEDS CLEANUP** - High SATD count" >> "$REPORT_FILE"
    ERRORS=$((ERRORS + 1))
fi

echo ""
echo "" >> "$REPORT_FILE"

# Broken Links Analysis (basic check)
echo "=== BROKEN LINKS ANALYSIS ==="
echo "## Broken Links Analysis" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

BROKEN_LINKS=0

# Sample check on Tier 1 files
for FILE in "${TIER1_FILES[@]}"; do
    if [ -f "$FILE" ]; then
        # Extract markdown links
        LINKS=$(grep -o '\[.*\](.*\.md)' "$FILE" | grep -o '(.*\.md)' | tr -d '()' || true)

        for LINK in $LINKS; do
            if [[ ! "$LINK" =~ ^http ]]; then
                LINK_PATH=$(dirname "$FILE")/"$LINK"

                if [ ! -f "$LINK_PATH" ] && [ ! -f "$LINK" ]; then
                    BROKEN_LINKS=$((BROKEN_LINKS + 1))
                fi
            fi
        done
    fi
done

echo "Broken links found (Tier 1 only): $BROKEN_LINKS"
echo "**Broken Links** (Tier 1 files): $BROKEN_LINKS" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

if [ $BROKEN_LINKS -eq 0 ]; then
    echo -e "${GREEN}✅ GOOD${NC}: No broken links in core documentation"
    echo "**Status**: ✅ **GOOD** - No broken links" >> "$REPORT_FILE"
else
    echo -e "${YELLOW}⚠️  WARNING${NC}: $BROKEN_LINKS broken link(s) found"
    echo "**Status**: ⚠️ **WARNING** - Broken links found" >> "$REPORT_FILE"
    WARNINGS=$((WARNINGS + 1))
fi

echo ""
echo "" >> "$REPORT_FILE"

# Summary
echo "================================================================"
echo "Validation Summary"
echo "================================================================"
echo ""

echo "## Summary" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

echo "Files Checked: $CHECKED / $TOTAL_FILES"
echo "Errors: $ERRORS"
echo "Warnings: $WARNINGS"

echo "| Metric | Value |" >> "$REPORT_FILE"
echo "|--------|-------|" >> "$REPORT_FILE"
echo "| **Files Checked** | $CHECKED / $TOTAL_FILES |" >> "$REPORT_FILE"
echo "| **Errors** | $ERRORS |" >> "$REPORT_FILE"
echo "| **Warnings** | $WARNINGS |" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

# Quality Score
TOTAL_CHECKS=$((ERRORS + WARNINGS))
if [ $TOTAL_CHECKS -eq 0 ]; then
    SCORE="A+ (Perfect)"
    COLOR=$GREEN
elif [ $ERRORS -eq 0 ] && [ $WARNINGS -le 5 ]; then
    SCORE="A (Excellent)"
    COLOR=$GREEN
elif [ $ERRORS -eq 0 ] && [ $WARNINGS -le 15 ]; then
    SCORE="B (Good)"
    COLOR=$YELLOW
elif [ $ERRORS -le 3 ]; then
    SCORE="C (Needs Work)"
    COLOR=$YELLOW
else
    SCORE="D (Needs Significant Cleanup)"
    COLOR=$RED
fi

echo ""
echo -e "${COLOR}Overall Quality Score: $SCORE${NC}"
echo "**Overall Quality Score**: $SCORE" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

# Recommendations
echo ""
echo "=== RECOMMENDATIONS ==="
echo "## Recommendations" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

if [ $ERRORS -gt 0 ] || [ $WARNINGS -gt 10 ]; then
    echo "1. Update Tier 1 (P0) files first - highest priority"
    echo "2. Review and update Tier 2 (P1) specifications"
    echo "3. Historical documents (Tier 3) can maintain old versions for context"
    echo "4. Run: make test-docs for automated validation"

    echo "1. **Update Tier 1 (P0) files first** - highest priority" >> "$REPORT_FILE"
    echo "2. **Review and update Tier 2 (P1) specifications**" >> "$REPORT_FILE"
    echo "3. **Historical documents (Tier 3)** can maintain old versions for context" >> "$REPORT_FILE"
    echo "4. **Run automated validation**: \`make test-docs\`" >> "$REPORT_FILE"
else
    echo "✅ Documentation is in excellent shape!"
    echo "   Continue maintaining with automated quality gates"

    echo "✅ **Documentation is in excellent shape!**" >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
    echo "Continue maintaining with automated quality gates." >> "$REPORT_FILE"
fi

echo ""
echo "" >> "$REPORT_FILE"
echo "---" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"
echo "**Generated by**: scripts/validate-entire-book.sh" >> "$REPORT_FILE"
echo "**Specification**: docs/specifications/paragraph-by-paragraph-spec.md" >> "$REPORT_FILE"

echo ""
echo "📄 Report saved to: $REPORT_FILE"
echo ""

# Exit code
if [ $ERRORS -gt 0 ]; then
    echo -e "${RED}❌ VALIDATION FAILED${NC}"
    echo "Run automated validation: make test-docs"
    exit 1
else
    echo -e "${GREEN}✅ VALIDATION PASSED${NC}"
    echo "Documentation quality is acceptable"
    exit 0
fi
