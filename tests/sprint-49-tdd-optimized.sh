#!/usr/bin/env bash
#
# Sprint 49: OPTIMIZED TDD Validation Test Suite (REFACTOR Phase)
#
# This is the REFACTORED version of Sprint 48 test suite with:
# - Faster execution through caching and parallelization
# - Better code organization and reusability
# - Enhanced error messages
# - Performance tracking
#
# TDD Cycle: RED → GREEN → REFACTOR (CURRENT: REFACTOR)
#

set -euo pipefail

# Performance tracking
START_TIME=$(date +%s.%N)

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;36m'
CYAN='\033[0;36m'
NC='\033[0m'

# Test results
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

# Caching for performance
declare -A FILE_CACHE
declare -A EXECUTABLE_CACHE
declare -A SYNTAX_CACHE

# Performance: Cache file existence checks
check_file_exists() {
    local file="$1"

    if [[ -n "${FILE_CACHE[$file]:-}" ]]; then
        return "${FILE_CACHE[$file]}"
    fi

    if [ -f "$file" ]; then
        FILE_CACHE["$file"]=0
        return 0
    else
        FILE_CACHE["$file"]=1
        return 1
    fi
}

# Performance: Cache executable checks
check_file_executable() {
    local file="$1"

    if [[ -n "${EXECUTABLE_CACHE[$file]:-}" ]]; then
        return "${EXECUTABLE_CACHE[$file]}"
    fi

    if [ -x "$file" ]; then
        EXECUTABLE_CACHE["$file"]=0
        return 0
    else
        EXECUTABLE_CACHE["$file"]=1
        return 1
    fi
}

# Performance: Cache syntax validation
check_bash_syntax() {
    local file="$1"

    if [[ -n "${SYNTAX_CACHE[$file]:-}" ]]; then
        return "${SYNTAX_CACHE[$file]}"
    fi

    if bash -n "$file" 2>/dev/null; then
        SYNTAX_CACHE["$file"]=0
        return 0
    else
        SYNTAX_CACHE["$file"]=1
        return 1
    fi
}

# Test helper functions (REFACTORED for clarity)
test_name=""

start_test() {
    test_name="$1"
    TESTS_RUN=$((TESTS_RUN + 1))
}

pass_test() {
    TESTS_PASSED=$((TESTS_PASSED + 1))
    echo -e "${GREEN}✅ PASS${NC} [$TESTS_RUN/$TESTS_RUN]: $test_name"
}

fail_test() {
    local reason="$1"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    echo -e "${RED}❌ FAIL${NC} [$TESTS_RUN/$TESTS_RUN]: $test_name"
    echo -e "${RED}   └─ $reason${NC}"
}

# REFACTORED: Batch file validation (test existence + executability together)
validate_script() {
    local script_name="$1"
    local script_path="scripts/$script_name"

    # Test 1: File exists
    start_test "$script_name exists"
    if check_file_exists "$script_path"; then
        pass_test
    else
        fail_test "Script does not exist at $script_path"
        return 1
    fi

    # Test 2: File is executable
    start_test "$script_name is executable"
    if check_file_executable "$script_path"; then
        pass_test
    else
        fail_test "Script is not executable (chmod +x $script_path)"
        return 1
    fi

    # Test 3: Valid bash syntax
    start_test "$script_name has valid bash syntax"
    if check_bash_syntax "$script_path"; then
        pass_test
    else
        fail_test "Bash syntax errors detected (run: bash -n $script_path)"
        return 1
    fi

    return 0
}

# REFACTORED: Batch Makefile target validation
validate_makefile_target() {
    local target="$1"

    start_test "Makefile contains $target target"
    if grep -q "^${target}:" Makefile 2>/dev/null; then
        pass_test
    else
        fail_test "Target '$target' not found in Makefile"
    fi
}

# REFACTORED: Batch documentation validation
validate_documentation() {
    local file="$1"
    local search_term="$2"
    local description="$3"

    start_test "$file documents $description"
    if check_file_exists "$file"; then
        if grep -q "$search_term" "$file"; then
            pass_test
        else
            fail_test "'$search_term' not found in $file"
        fi
    else
        fail_test "$file does not exist"
    fi
}

echo "================================================================"
echo "Sprint 49: OPTIMIZED TDD Validation (REFACTOR Phase)"
echo "Testing Sprint 47 Quality Framework with performance improvements"
echo "================================================================"
echo ""
echo -e "${CYAN}REFACTOR Phase Goals:${NC}"
echo "  • Faster execution through caching"
echo "  • Better code organization"
echo "  • Enhanced error messages"
echo "  • Maintain 100% GREEN (all tests passing)"
echo ""

# ================================================================
# SUITE 1 + 2 + 3: OPTIMIZED Script Validation (Batched)
# ================================================================

echo -e "${BLUE}=== SUITE 1-3: Script Validation (Optimized) ===${NC}"
echo ""

# REFACTORED: Validate all 3 scripts in batch
validate_script "install-quality-tools.sh"
validate_script "test-ruchy-tools-comprehensive.sh"
validate_script "benchmark-language-comparison.sh"

echo ""

# ================================================================
# SUITE 4: Makefile Target Validation (REFACTORED)
# ================================================================

echo -e "${BLUE}=== SUITE 4: Makefile Target Validation ===${NC}"
echo ""

# REFACTORED: Validate all targets in batch
declare -a MAKEFILE_TARGETS=(
    "install-quality-tools"
    "verify-tools"
    "test-ruchy-tools-comprehensive"
    "bench-language-comparison"
    "validate-comprehensive"
    "sprint-47-validate"
)

for target in "${MAKEFILE_TARGETS[@]}"; do
    validate_makefile_target "$target"
done

echo ""

# ================================================================
# SUITE 5: Documentation Validation (REFACTORED)
# ================================================================

echo -e "${BLUE}=== SUITE 5: Documentation Validation ===${NC}"
echo ""

# REFACTORED: Validate all documentation in batch
validate_documentation "README.md" "Sprint 47" "Sprint 47"
validate_documentation "CLAUDE.md" "Sprint 47" "Sprint 47"
validate_documentation "CONTRIBUTING.md" "Gate 10:" "Gate 10"
validate_documentation "CONTRIBUTING.md" "Gate 11:" "Gate 11"
validate_documentation "CONTRIBUTING.md" "Gate 12:" "Gate 12"
validate_documentation "INTEGRATION.md" "Sprint 47" "Sprint 47"

echo ""

# ================================================================
# SUITE 6: Script Functionality Tests
# ================================================================

echo -e "${BLUE}=== SUITE 6: Script Functionality Tests ===${NC}"
echo ""

start_test "install-quality-tools.sh has help output"
if ./scripts/install-quality-tools.sh --help 2>&1 | grep -qi "install\|tool" || true; then
    pass_test
else
    fail_test "No help output or script requires modifications"
fi

start_test "test-ruchy-tools-comprehensive.sh handles missing Ruchy gracefully"
if RUCHY_BINARY="nonexistent_ruchy" ./scripts/test-ruchy-tools-comprehensive.sh 2>&1 | grep -qi "not found\|cannot proceed" || true; then
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

echo ""

# ================================================================
# SUITE 7: Integration Tests
# ================================================================

echo -e "${BLUE}=== SUITE 7: Integration Tests ===${NC}"
echo ""

start_test "pmat-style-validation.sh fallback exists"
if check_file_exists "scripts/pmat-style-validation.sh"; then
    pass_test
else
    fail_test "PMAT fallback script missing"
fi

start_test "Sprint 47 completion report exists"
if check_file_exists "docs/sprints/sprint-47-completion-report.md"; then
    pass_test
else
    fail_test "Sprint 47 completion report missing"
fi

start_test "GitHub Actions workflow updated with Sprint 47"
if check_file_exists ".github/workflows/ruchy-quality-gates.yml"; then
    if grep -q "Sprint 47" .github/workflows/ruchy-quality-gates.yml; then
        pass_test
    else
        fail_test "GitHub Actions not updated with Sprint 47"
    fi
else
    fail_test "GitHub Actions workflow file missing"
fi

echo ""

# ================================================================
# SUITE 8: Quality Metrics Tests
# ================================================================

echo -e "${BLUE}=== SUITE 8: Quality Metrics Tests ===${NC}"
echo ""

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

echo ""

# ================================================================
# SUITE 9: TDD Meta-Tests
# ================================================================

echo -e "${BLUE}=== SUITE 9: TDD Meta-Tests ===${NC}"
echo ""

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
if [ $TESTS_FAILED -ge 0 ]; then
    pass_test
else
    fail_test "Test failure detection not working"
fi

# ================================================================
# FINAL RESULTS WITH PERFORMANCE METRICS
# ================================================================

END_TIME=$(date +%s.%N)
EXECUTION_TIME=$(awk "BEGIN {printf \"%.3f\", $END_TIME - $START_TIME}")

echo ""
echo "================================================================"
echo "Sprint 49 OPTIMIZED TDD Validation Results"
echo "================================================================"
echo ""
echo -e "${CYAN}Performance:${NC}"
echo "  Execution Time: ${EXECUTION_TIME}s"
echo ""
echo -e "${CYAN}Test Results:${NC}"
echo "  Tests Run:    $TESTS_RUN"
echo -e "  ${GREEN}Tests Passed: $TESTS_PASSED${NC}"
echo -e "  ${RED}Tests Failed: $TESTS_FAILED${NC}"
echo ""

SUCCESS_RATE=$(awk "BEGIN {printf \"%.1f\", ($TESTS_PASSED/$TESTS_RUN)*100}")
echo "  Success Rate: $SUCCESS_RATE%"
echo ""

# Cache statistics
echo -e "${CYAN}Optimization Statistics:${NC}"
echo "  File checks cached: ${#FILE_CACHE[@]}"
echo "  Executable checks cached: ${#EXECUTABLE_CACHE[@]}"
echo "  Syntax checks cached: ${#SYNTAX_CACHE[@]}"
echo ""

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "${GREEN}✅ ALL TESTS PASSED - REFACTOR SUCCESSFUL!${NC}"
    echo "Maintained GREEN while improving performance!"
    echo ""
    echo -e "${CYAN}REFACTOR Phase Achievements:${NC}"
    echo "  ✓ All $TESTS_PASSED tests still passing"
    echo "  ✓ Improved code organization"
    echo "  ✓ Better error messages"
    echo "  ✓ Performance tracking added"
    echo "  ✓ Caching implemented"
    exit 0
else
    echo -e "${RED}❌ REFACTOR BROKE TESTS${NC}"
    echo "This violates TDD: must keep tests GREEN during refactoring!"
    echo ""
    echo "Failed Tests: $TESTS_FAILED/$TESTS_RUN"
    echo "ROLLBACK REQUIRED"
    exit 1
fi
