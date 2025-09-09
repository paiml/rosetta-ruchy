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
export REQUIRED_RUCHY_VERSION="1.89.0"

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

# 2. Ruchy syntax validation (only check v1.89 files)
echo -n "  Ruchy syntax validation (v1.89 files only)... "
RUCHY_ERRORS=0
for file in $(find examples -name "*v189.ruchy" 2>/dev/null); do
    if ! ruchy check "$file" 2>&1 | grep -q "✓ Syntax is valid"; then
        RUCHY_ERRORS=$((RUCHY_ERRORS + 1))
        echo "Failed: $file"
    fi
done
if [ $RUCHY_ERRORS -gt 0 ]; then
    echo -e "${RED}❌${NC}"
    echo -e "${RED}   $RUCHY_ERRORS v1.89 Ruchy files have syntax errors${NC}"
    echo -e "${RED}   ❌ COMMIT BLOCKED: Fix v1.89 Ruchy syntax errors${NC}"
    exit 1
else
    echo -e "${GREEN}✅${NC}"
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

echo ""
echo -e "${GREEN}✅ All quality gates passed!${NC}"
echo ""

exit 0
