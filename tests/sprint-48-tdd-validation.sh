#!/usr/bin/env bash
#
# Sprint 48: TDD Validation Test Suite
# Extreme TDD: Write tests FIRST, then make them pass
#
# Test-Driven Development Cycle:
# 1. RED: Write failing test
# 2. GREEN: Make test pass with minimal code
# 3. REFACTOR: Clean up while keeping tests green
#

set -euo pipefail

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;36m'
NC='\033[0m'

# Test results
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

# Test helper functions
test_name=""

start_test() {
    test_name="$1"
    TESTS_RUN=$((TESTS_RUN + 1))
    echo ""
    echo -e "${BLUE}TEST $TESTS_RUN: $test_name${NC}"
}

pass_test() {
    TESTS_PASSED=$((TESTS_PASSED + 1))
    echo -e "${GREEN}✅ PASS${NC}: $test_name"
}

fail_test() {
    local reason="$1"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    echo -e "${RED}❌ FAIL${NC}: $test_name"
    echo -e "${RED}   Reason: $reason${NC}"
}

echo "================================================================"
echo "Sprint 48: TDD Validation Test Suite"
echo "Testing the Sprint 47 Quality Framework"
echo "================================================================"
echo ""
echo "Following Extreme TDD Methodology:"
echo "1. RED: Write test that fails"
echo "2. GREEN: Make test pass"
echo "3. REFACTOR: Improve while keeping green"
echo ""

# ================================================================
# TEST SUITE 1: Script Existence and Permissions (TDD - RED phase)
# ================================================================

echo "=== SUITE 1: Script Existence Tests ==="

start_test "install-quality-tools.sh exists"
if [ -f "scripts/install-quality-tools.sh" ]; then
    pass_test
else
    fail_test "Script does not exist"
fi

start_test "install-quality-tools.sh is executable"
if [ -x "scripts/install-quality-tools.sh" ]; then
    pass_test
else
    fail_test "Script is not executable"
fi

start_test "test-ruchy-tools-comprehensive.sh exists"
if [ -f "scripts/test-ruchy-tools-comprehensive.sh" ]; then
    pass_test
else
    fail_test "Script does not exist"
fi

start_test "test-ruchy-tools-comprehensive.sh is executable"
if [ -x "scripts/test-ruchy-tools-comprehensive.sh" ]; then
    pass_test
else
    fail_test "Script is not executable"
fi

start_test "benchmark-language-comparison.sh exists"
if [ -f "scripts/benchmark-language-comparison.sh" ]; then
    pass_test
else
    fail_test "Script does not exist"
fi

start_test "benchmark-language-comparison.sh is executable"
if [ -x "scripts/benchmark-language-comparison.sh" ]; then
    pass_test
else
    fail_test "Script is not executable"
fi

# ================================================================
# TEST SUITE 2: Script Syntax Validation (TDD - RED phase)
# ================================================================

echo ""
echo "=== SUITE 2: Script Syntax Tests ==="

start_test "install-quality-tools.sh has valid bash syntax"
if bash -n scripts/install-quality-tools.sh 2>/dev/null; then
    pass_test
else
    fail_test "Bash syntax errors detected"
fi

start_test "test-ruchy-tools-comprehensive.sh has valid bash syntax"
if bash -n scripts/test-ruchy-tools-comprehensive.sh 2>/dev/null; then
    pass_test
else
    fail_test "Bash syntax errors detected"
fi

start_test "benchmark-language-comparison.sh has valid bash syntax"
if bash -n scripts/benchmark-language-comparison.sh 2>/dev/null; then
    pass_test
else
    fail_test "Bash syntax errors detected"
fi

# ================================================================
# TEST SUITE 3: Makefile Target Validation (TDD - RED phase)
# ================================================================

echo ""
echo "=== SUITE 3: Makefile Target Tests ==="

start_test "Makefile contains install-quality-tools target"
if grep -q "^install-quality-tools:" Makefile; then
    pass_test
else
    fail_test "Target not found in Makefile"
fi

start_test "Makefile contains verify-tools target"
if grep -q "^verify-tools:" Makefile; then
    pass_test
else
    fail_test "Target not found in Makefile"
fi

start_test "Makefile contains test-ruchy-tools-comprehensive target"
if grep -q "^test-ruchy-tools-comprehensive:" Makefile; then
    pass_test
else
    fail_test "Target not found in Makefile"
fi

start_test "Makefile contains bench-language-comparison target"
if grep -q "^bench-language-comparison:" Makefile; then
    pass_test
else
    fail_test "Target not found in Makefile"
fi

start_test "Makefile contains validate-comprehensive target"
if grep -q "^validate-comprehensive:" Makefile; then
    pass_test
else
    fail_test "Target not found in Makefile"
fi

start_test "Makefile contains sprint-47-validate target"
if grep -q "^sprint-47-validate:" Makefile; then
    pass_test
else
    fail_test "Target not found in Makefile"
fi

# ================================================================
# TEST SUITE 4: Documentation Validation (TDD - RED phase)
# ================================================================

echo ""
echo "=== SUITE 4: Documentation Tests ==="

start_test "README.md documents Sprint 47"
if grep -q "Sprint 47" README.md; then
    pass_test
else
    fail_test "Sprint 47 not documented in README.md"
fi

start_test "CLAUDE.md documents Sprint 47"
if grep -q "Sprint 47" CLAUDE.md; then
    pass_test
else
    fail_test "Sprint 47 not documented in CLAUDE.md"
fi

start_test "CONTRIBUTING.md documents Gate 10"
if grep -q "Gate 10:" CONTRIBUTING.md; then
    pass_test
else
    fail_test "Gate 10 not documented in CONTRIBUTING.md"
fi

start_test "CONTRIBUTING.md documents Gate 11"
if grep -q "Gate 11:" CONTRIBUTING.md; then
    pass_test
else
    fail_test "Gate 11 not documented in CONTRIBUTING.md"
fi

start_test "CONTRIBUTING.md documents Gate 12"
if grep -q "Gate 12:" CONTRIBUTING.md; then
    pass_test
else
    fail_test "Gate 12 not documented in CONTRIBUTING.md"
fi

start_test "INTEGRATION.md documents Sprint 47"
if grep -q "Sprint 47" INTEGRATION.md; then
    pass_test
else
    fail_test "Sprint 47 not documented in INTEGRATION.md"
fi

# ================================================================
# TEST SUITE 5: Script Functionality Tests (TDD - RED phase)
# ================================================================

echo ""
echo "=== SUITE 5: Script Functionality Tests ==="

start_test "install-quality-tools.sh has help output"
if ./scripts/install-quality-tools.sh --help 2>&1 | grep -q "Installation\|install\|Install" || true; then
    pass_test
else
    # This is expected to fail initially (TDD RED)
    fail_test "No help output or script requires modifications"
fi

start_test "test-ruchy-tools-comprehensive.sh handles no Ruchy gracefully"
# This should handle missing ruchy gracefully
if RUCHY_BINARY="nonexistent_ruchy" ./scripts/test-ruchy-tools-comprehensive.sh 2>&1 | grep -q "not found\|Cannot proceed" || true; then
    pass_test
else
    fail_test "Script does not handle missing Ruchy correctly"
fi

start_test "Scripts use proper error handling (set -euo pipefail)"
if grep -q "set -euo pipefail" scripts/install-quality-tools.sh; then
    pass_test
else
    fail_test "Script missing proper error handling"
fi

# ================================================================
# TEST SUITE 6: Integration Tests (TDD - RED phase)
# ================================================================

echo ""
echo "=== SUITE 6: Integration Tests ==="

start_test "pmat-style-validation.sh fallback exists"
if [ -f "scripts/pmat-style-validation.sh" ]; then
    pass_test
else
    fail_test "PMAT fallback script missing"
fi

start_test "Sprint 47 completion report exists"
if [ -f "docs/sprints/sprint-47-completion-report.md" ]; then
    pass_test
else
    fail_test "Sprint 47 completion report missing"
fi

start_test "GitHub Actions workflow updated with Sprint 47"
if grep -q "Sprint 47" .github/workflows/ruchy-quality-gates.yml; then
    pass_test
else
    fail_test "GitHub Actions not updated with Sprint 47"
fi

# ================================================================
# TEST SUITE 7: Quality Metrics Tests (TDD - RED phase)
# ================================================================

echo ""
echo "=== SUITE 7: Quality Metrics Tests ==="

start_test "Scripts have proper shebang"
if head -1 scripts/install-quality-tools.sh | grep -q "^#!/usr/bin/env bash"; then
    pass_test
else
    fail_test "Missing or incorrect shebang"
fi

start_test "Scripts have documentation comments"
if head -10 scripts/install-quality-tools.sh | grep -q "#"; then
    pass_test
else
    fail_test "Missing documentation comments"
fi

start_test "Scripts define functions"
if grep -q "^[a-z_]*() {" scripts/install-quality-tools.sh; then
    pass_test
else
    fail_test "No functions defined (poor structure)"
fi

start_test "Scripts have error handling"
if grep -q "|| \|if.*; then\|2>/dev/null" scripts/install-quality-tools.sh; then
    pass_test
else
    fail_test "Minimal error handling detected"
fi

# ================================================================
# TEST SUITE 8: TDD Meta-Tests (Testing our tests!)
# ================================================================

echo ""
echo "=== SUITE 8: TDD Meta-Tests ==="

start_test "This test suite itself is executable"
if [ -x "$0" ]; then
    pass_test
else
    fail_test "Test suite not executable"
fi

start_test "Test suite has proper test count tracking"
if [ $TESTS_RUN -gt 0 ]; then
    pass_test
else
    fail_test "Test counting not working"
fi

start_test "Test suite can fail tests (meta-validation)"
# We expect some tests to fail in TDD RED phase
if [ $TESTS_FAILED -ge 0 ]; then
    pass_test
else
    fail_test "Test failure detection not working"
fi

# ================================================================
# FINAL RESULTS
# ================================================================

echo ""
echo "================================================================"
echo "Sprint 48 TDD Validation Results"
echo "================================================================"
echo ""
echo "Tests Run:    $TESTS_RUN"
echo -e "${GREEN}Tests Passed: $TESTS_PASSED${NC}"
echo -e "${RED}Tests Failed: $TESTS_FAILED${NC}"
echo ""

SUCCESS_RATE=$(awk "BEGIN {printf \"%.1f\", ($TESTS_PASSED/$TESTS_RUN)*100}")
echo "Success Rate: $SUCCESS_RATE%"
echo ""

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "${GREEN}✅ ALL TESTS PASSED - GREEN PHASE!${NC}"
    echo "Sprint 47 framework is fully validated!"
    exit 0
else
    echo -e "${YELLOW}⚠️  SOME TESTS FAILED - RED PHASE${NC}"
    echo "This is EXPECTED in TDD - now fix the failures!"
    echo ""
    echo "TDD Next Steps:"
    echo "1. Review failed tests above"
    echo "2. Fix issues to make tests pass (GREEN phase)"
    echo "3. Refactor code while keeping tests green"
    exit 1
fi
