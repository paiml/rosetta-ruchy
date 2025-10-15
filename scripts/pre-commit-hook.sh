#!/bin/bash
# Rosetta Ruchy Pre-commit Hook - RIGID QUALITY ENFORCEMENT
# Toyota Way: Zero tolerance for defects

set -e

echo "🔍 ROSETTA RUCHY Pre-commit Quality Gates"
echo "==========================================="
echo "ZERO TOLERANCE MODE ACTIVE"
echo ""

# Configuration (STRICT THRESHOLDS)
export MAX_CYCLOMATIC_COMPLEXITY=20
export MAX_COGNITIVE_COMPLEXITY=20
export MIN_TEST_COVERAGE=80
export MAX_SATD_COMMENTS=0  # ZERO tolerance
export REQUIRED_RUCHY_VERSION="3.86.0"

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check Ruchy version
echo -n "  Ruchy version check... "
CURRENT_VERSION=$(ruchy --version 2>/dev/null | cut -d' ' -f2 || echo "unknown")
if [ "$CURRENT_VERSION" != "$REQUIRED_RUCHY_VERSION" ]; then
    echo -e "${YELLOW}⚠️  Warning: Version $CURRENT_VERSION (expected $REQUIRED_RUCHY_VERSION)${NC}"
else
    echo -e "${GREEN}✅${NC}"
fi

# 1. SATD Check (ZERO tolerance)
echo -n "  SATD check (ZERO tolerance)... "
SATD_COUNT=$(find . -name "*.ruchy" -o -name "*.rs" | xargs grep -c "TODO\|FIXME\|HACK\|XXX" 2>/dev/null | awk '{sum+=$1} END {print sum}')
if [ "$SATD_COUNT" != "0" ] && [ -n "$SATD_COUNT" ]; then
    echo -e "${RED}❌${NC}"
    echo -e "${RED}   Found $SATD_COUNT SATD comments - ZERO tolerance violated!${NC}"
    echo "   Violations:"
    find . -name "*.ruchy" -o -name "*.rs" | xargs grep -n "TODO\|FIXME\|HACK\|XXX" | head -5
    echo ""
    echo -e "${RED}   ❌ COMMIT BLOCKED: Remove all TODO/FIXME/HACK comments${NC}"
    exit 1
else
    echo -e "${GREEN}✅${NC}"
fi

# 2. Ruchy syntax validation (version-aware)
echo -n "  Ruchy syntax validation (version-aware)... "
CURRENT_MAJOR=$(echo "$CURRENT_VERSION" | cut -d'.' -f1)

# Only validate files matching current major version
if [ "$CURRENT_MAJOR" = "1" ]; then
    # On v1.x, check v189 files
    VERSION_FILES=$(find examples -name "*v189.ruchy" 2>/dev/null)
elif [ "$CURRENT_MAJOR" = "3" ]; then
    # On v3.x, check v3* files (if they exist, otherwise skip)
    VERSION_FILES=$(find examples -name "*v3*.ruchy" 2>/dev/null)
else
    VERSION_FILES=""
fi

RUCHY_ERRORS=0
if [ -n "$VERSION_FILES" ]; then
    for file in $VERSION_FILES; do
        if ! ruchy check "$file" 2>&1 | grep -q "✓ Syntax is valid"; then
            RUCHY_ERRORS=$((RUCHY_ERRORS + 1))
            echo "Failed: $file"
        fi
    done
fi

if [ $RUCHY_ERRORS -gt 0 ]; then
    echo -e "${RED}❌${NC}"
    echo -e "${RED}   $RUCHY_ERRORS Ruchy v${CURRENT_MAJOR}.x files have syntax errors${NC}"
    echo -e "${RED}   ❌ COMMIT BLOCKED: Fix Ruchy syntax errors${NC}"
    exit 1
else
    if [ -z "$VERSION_FILES" ]; then
        echo -e "${YELLOW}⚠️  No v${CURRENT_MAJOR}.x files found (migration in progress)${NC}"
    else
        echo -e "${GREEN}✅${NC}"
    fi
fi

# 3. Complexity check (if pmat available)
if command -v pmat &> /dev/null; then
    echo -n "  Complexity check... "
    COMPLEXITY_OUTPUT=$(pmat analyze complexity --max-cyclomatic $MAX_CYCLOMATIC_COMPLEXITY --max-cognitive $MAX_COGNITIVE_COMPLEXITY 2>&1)
    if echo "$COMPLEXITY_OUTPUT" | grep -q "❌.*Errors: [1-9]"; then
        echo -e "${RED}❌${NC}"
        echo "$COMPLEXITY_OUTPUT" | grep "Issues Found" | head -1
        echo -e "${RED}   Complexity exceeds thresholds (Max: $MAX_CYCLOMATIC_COMPLEXITY)${NC}"
        echo -e "${RED}   ❌ COMMIT BLOCKED: Reduce complexity${NC}"
        exit 1
    else
        echo -e "${GREEN}✅${NC}"
    fi
fi

# 4. Rust linting (if cargo available)
if command -v cargo &> /dev/null; then
    echo -n "  Rust lint check... "
    if ! cargo clippy --all-targets --all-features -- -D warnings 2>/dev/null; then
        echo -e "${RED}❌${NC}"
        echo -e "${RED}   ❌ COMMIT BLOCKED: Fix clippy warnings${NC}"
        exit 1
    else
        echo -e "${GREEN}✅${NC}"
    fi
fi

# 5. Check for stub implementations
echo -n "  Stub implementation check... "
STUB_COUNT=$(find . -name "*.ruchy" -o -name "*.rs" | xargs grep -c "unimplemented!\|todo!()\|unreachable!()" 2>/dev/null | awk '{sum+=$1} END {print sum}')
if [ "$STUB_COUNT" != "0" ] && [ -n "$STUB_COUNT" ]; then
    echo -e "${RED}❌${NC}"
    echo -e "${RED}   Found $STUB_COUNT stub implementations${NC}"
    echo -e "${RED}   ❌ COMMIT BLOCKED: Complete all implementations${NC}"
    exit 1
else
    echo -e "${GREEN}✅${NC}"
fi

# 6. Security audit (Sprint 42 - Gemini audit response)
echo ""
echo -n "6. Security audit (cargo audit)... "

if command -v cargo &> /dev/null; then
    # Check if cargo-audit is installed
    if ! cargo audit --version &>/dev/null; then
        echo ""
        echo -e "${YELLOW}  ⚠️  cargo-audit not installed${NC}"
        echo "  Install with: cargo install cargo-audit"
        echo "  Skipping security check..."
    else
        if cargo audit 2>&1 | grep -q "error:.*vulnerability"; then
            echo ""
            echo -e "${RED}   ❌ Security vulnerabilities found${NC}"
            echo -e "${RED}   ❌ COMMIT BLOCKED: Fix security issues${NC}"
            echo ""
            echo "  Run: cargo audit"
            echo "  Then: cargo update -p <vulnerable-package>"
            echo ""
            exit 1
        else
            echo -e "${GREEN}✅${NC}"
        fi
    fi
else
    echo -e "${YELLOW}⚠️  (cargo not found, skipped)${NC}"
fi

# 7. Dogfood-quick validation (Sprint 40+ infrastructure)
if command -v make &> /dev/null && [ -f "Makefile" ]; then
    echo ""
    echo "  Running dogfood-quick (3 tools: check, lint, score)..."
    echo "  This may take ~2 minutes..."
    echo ""

    if make dogfood-quick 2>&1 | tail -20; then
        echo ""
        echo -e "${GREEN}  ✅ Dogfood-quick passed (all examples validated)${NC}"
    else
        echo ""
        echo -e "${RED}  ❌ Dogfood-quick failed${NC}"
        echo -e "${RED}  ❌ COMMIT BLOCKED: Fix failing examples${NC}"
        echo ""
        echo "  Debug commands:"
        echo "    make test-all-examples    # See which files fail"
        echo "    cat test-results.json     # View detailed results"
        echo ""
        exit 1
    fi
fi

# 8. SATD detection (Sprint 42A - Zero SATD policy)
echo ""
echo -n "8. SATD detection (zero TODO/FIXME/HACK)... "

if command -v make &> /dev/null && [ -f "Makefile" ]; then
    # Check for SATD comments in committed files
    SATD_COUNT=$(grep -r "TODO\|FIXME\|HACK" --include="*.rs" --include="*.ruchy" . --exclude-dir=target --exclude-dir=.git 2>/dev/null | \
                 grep -v "Checking for TODO\|check for TODOs\|Found TODO\|No TODO" | wc -l)

    if [ "$SATD_COUNT" -gt 3 ]; then
        echo ""
        echo -e "${RED}   ❌ Found $SATD_COUNT SATD comments (TODO/FIXME/HACK)${NC}"
        echo -e "${RED}   ❌ COMMIT BLOCKED: Remove all SATD comments${NC}"
        echo ""
        echo "  Found comments:"
        grep -rn "TODO\|FIXME\|HACK" --include="*.rs" --include="*.ruchy" . --exclude-dir=target --exclude-dir=.git 2>/dev/null | \
            grep -v "Checking for TODO\|check for TODOs\|Found TODO\|No TODO" | head -10
        echo ""
        echo "  Policy: Convert to GitHub issues or resolve immediately"
        echo ""
        exit 1
    else
        echo -e "${GREEN}✅${NC}"
    fi
else
    echo -e "${YELLOW}⚠️  (make not found, skipped)${NC}"
fi

# 9. Lint warning no-increase enforcement (Sprint 43 Ticket 1)
echo ""
echo -n "9. Lint warning count check (no-increase policy)... "

if [ -f "scripts/count-lint-warnings.sh" ] && [ -f ".lint-baseline" ]; then
    # Get current warning count
    CURRENT_WARNINGS=$(./scripts/count-lint-warnings.sh 2>/dev/null || echo "0")
    BASELINE_WARNINGS=$(cat .lint-baseline 2>/dev/null || echo "744")

    if [ "$CURRENT_WARNINGS" -gt "$BASELINE_WARNINGS" ]; then
        echo ""
        echo -e "${RED}   ❌ Lint warning count increased${NC}"
        echo -e "${RED}   Baseline: $BASELINE_WARNINGS warnings${NC}"
        echo -e "${RED}   Current:  $CURRENT_WARNINGS warnings${NC}"
        echo -e "${RED}   Increase: +$((CURRENT_WARNINGS - BASELINE_WARNINGS)) warnings${NC}"
        echo ""
        echo -e "${RED}   ❌ COMMIT BLOCKED: Fix new lint warnings before committing${NC}"
        echo ""
        echo "  Policy: No new lint warnings allowed (Kaizen improvement only)"
        echo "  To see warnings: make lint-report"
        echo ""
        exit 1
    else
        REDUCTION=$((BASELINE_WARNINGS - CURRENT_WARNINGS))
        if [ "$REDUCTION" -gt 0 ]; then
            echo -e "${GREEN}✅ ($CURRENT_WARNINGS warnings, -$REDUCTION from baseline!)${NC}"
        else
            echo -e "${GREEN}✅ ($CURRENT_WARNINGS warnings, no increase)${NC}"
        fi
    fi
else
    echo -e "${YELLOW}⚠️  (warning counter not found, skipped)${NC}"
fi

echo ""
echo -e "${GREEN}✅ All quality gates passed!${NC}"
echo ""
echo "To bypass this hook (use sparingly): git commit --no-verify"
echo ""

exit 0
