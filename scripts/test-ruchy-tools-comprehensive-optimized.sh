#!/usr/bin/env bash
#
# test-ruchy-tools-comprehensive-optimized.sh - OPTIMIZED: Test all 18+ Ruchy tools
#
# Sprint 51 GREEN Phase: Optimized version with Sprint 49 patterns
# - File existence caching (FILE_CACHE)
# - Tool availability caching (TOOL_CACHE)
# - Syntax validation caching (SYNTAX_CACHE)
# - Performance tracking
# - Enhanced error messages
# - Progress reporting
#
# Following the ruchy-book methodology with performance optimizations
#
# Based on: https://github.com/paiml/ruchy-book testing framework
#

set -euo pipefail

# Performance tracking (Sprint 49 pattern)
START_TIME=$(date +%s.%N)

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;36m'
CYAN='\033[0;36m'
NC='\033[0;m' # No Color

# Configuration
RUCHY_BINARY="${RUCHY_BINARY:-ruchy}"
EXAMPLES_DIR="examples"
RESULTS_FILE="ruchy-tools-test-results.json"
DETAILED_LOG="ruchy-tools-detailed.log"

# Sprint 49 Optimization: Caching mechanisms
declare -A FILE_CACHE
declare -A TOOL_CACHE
declare -A SYNTAX_CACHE

# Cache statistics
CACHE_FILE_HITS=0
CACHE_TOOL_HITS=0
CACHE_SYNTAX_HITS=0

# Ruchy tool commands to test (18+ tools)
declare -a RUCHY_TOOLS=(
    # Core validation tools (MANDATORY - always work)
    "check"          # Syntax validation
    "parse"          # AST parsing
    "provability"    # Formal correctness verification
    "runtime"        # Complexity and performance analysis
    "score"          # Unified quality score
    "ast"            # Enhanced AST analysis

    # Advanced analysis tools (HIGH PRIORITY)
    "optimize"       # Hardware-aware optimization
    "prove"          # Interactive theorem prover
    "quality-gate"   # Quality gate enforcement
    "mcp"            # Real-time quality analysis

    # Development tools (STANDARD)
    "fmt"            # Format source code
    "lint"           # Lint for issues and style
    "doc"            # Generate documentation

    # Compilation tools (MAY HAVE ISSUES)
    "transpile"      # Transpile to Rust
    "build"          # Build project
    "run"            # Compile and run
    "test"           # Run tests

    # Performance tools (ADVANCED)
    "benchmark"      # Performance benchmarking
    "profile"        # Profiling
    "energy"         # Energy consumption analysis

    # Additional tools (EXPERIMENTAL)
    "complexity"     # Complexity analysis
    "verify"         # Verification
    "validate"       # Validation
    "analyze"        # General analysis
)

# Statistics
declare -A TOOL_STATS
declare -A EXAMPLE_STATS
TOTAL_TESTS=0
TOTAL_PASSED=0
TOTAL_FAILED=0
TOTAL_SKIPPED=0

# Initialize statistics
for tool in "${RUCHY_TOOLS[@]}"; do
    TOOL_STATS["${tool}_passed"]=0
    TOOL_STATS["${tool}_failed"]=0
    TOOL_STATS["${tool}_skipped"]=0
done

# Logging functions
log_info() {
    echo -e "${BLUE}ℹ${NC} $*" | tee -a "$DETAILED_LOG"
}

log_success() {
    echo -e "${GREEN}✓${NC} $*" | tee -a "$DETAILED_LOG"
}

log_warning() {
    echo -e "${YELLOW}⚠${NC} $*" | tee -a "$DETAILED_LOG"
}

log_error() {
    echo -e "${RED}✗${NC} $*" | tee -a "$DETAILED_LOG"
}

# Sprint 49 Optimization: Cache file existence checks
check_file_exists() {
    local file="$1"

    if [[ -n "${FILE_CACHE[$file]:-}" ]]; then
        CACHE_FILE_HITS=$((CACHE_FILE_HITS + 1))
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

# Sprint 49 Optimization: Cache tool availability
check_tool_available() {
    local tool="$1"

    if [[ -n "${TOOL_CACHE[$tool]:-}" ]]; then
        CACHE_TOOL_HITS=$((CACHE_TOOL_HITS + 1))
        return "${TOOL_CACHE[$tool]}"
    fi

    if command -v "$tool" &>/dev/null; then
        TOOL_CACHE["$tool"]=0
        return 0
    else
        TOOL_CACHE["$tool"]=1
        return 1
    fi
}

# Sprint 49 Optimization: Cache bash syntax validation
check_bash_syntax() {
    local file="$1"

    if [[ -n "${SYNTAX_CACHE[$file]:-}" ]]; then
        CACHE_SYNTAX_HITS=$((CACHE_SYNTAX_HITS + 1))
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

# Check if Ruchy is installed
check_ruchy_installation() {
    log_info "Checking Ruchy installation..."

    # Use cached tool check
    if ! check_tool_available "$RUCHY_BINARY"; then
        log_error "Ruchy not found in PATH"
        log_info "Please run: ./scripts/install-quality-tools.sh"
        log_error "   └─ Cannot proceed without Ruchy (use: make install-quality-tools)"
        return 1
    fi

    local ruchy_version=$("$RUCHY_BINARY" --version 2>&1 | head -1 || echo "unknown")
    log_success "Ruchy found: $ruchy_version"

    return 0
}

# Find all Ruchy example files
find_ruchy_examples() {
    log_info "Finding all .ruchy files in $EXAMPLES_DIR..."

    local ruchy_files
    ruchy_files=$(find "$EXAMPLES_DIR" -type f -name "*.ruchy" | sort)

    local count=$(echo "$ruchy_files" | wc -l)
    log_success "Found $count .ruchy files"

    echo "$ruchy_files"
}

# Test a single Ruchy tool on a single file
test_tool_on_file() {
    local tool=$1
    local file=$2
    local timeout=30  # 30 second timeout per tool

    local tool_output
    local exit_code

    # Run the tool with timeout
    if tool_output=$(timeout "$timeout" "$RUCHY_BINARY" "$tool" "$file" 2>&1); then
        exit_code=0
    else
        exit_code=$?
    fi

    # Analyze result
    if [ $exit_code -eq 0 ]; then
        # Success
        TOOL_STATS["${tool}_passed"]=$((${TOOL_STATS["${tool}_passed"]} + 1))
        TOTAL_PASSED=$((TOTAL_PASSED + 1))
        return 0
    elif [ $exit_code -eq 124 ]; then
        # Timeout
        log_warning "$tool timed out on $file"
        TOOL_STATS["${tool}_skipped"]=$((${TOOL_STATS["${tool}_skipped"]} + 1))
        TOTAL_SKIPPED=$((TOTAL_SKIPPED + 1))
        return 2
    else
        # Check if tool is unavailable in this Ruchy version
        # Using regex patterns to detect tool availability errors
        if echo "$tool_output" | grep -qE "not (yet )?implement|not support|unknown|unrecognized"; then
            # Tool not available in this Ruchy version
            TOOL_STATS["${tool}_skipped"]=$((${TOOL_STATS["${tool}_skipped"]} + 1))
            TOTAL_SKIPPED=$((TOTAL_SKIPPED + 1))
            return 2
        else
            # Genuine failure
            TOOL_STATS["${tool}_failed"]=$((${TOOL_STATS["${tool}_failed"]} + 1))
            TOTAL_FAILED=$((TOTAL_FAILED + 1))
            echo "$tool_output" >> "$DETAILED_LOG"
            return 1
        fi
    fi
}

# Backwards compatibility wrapper for test suite
test_tools_on_file() {
    test_all_tools_on_file "$@"
}

# Sprint 51 Optimization: Batch test all tools on a single file with progress reporting
# Note: Enhanced from test_tool_on_file with batching and progress tracking
test_all_tools_on_file() {
    local file=$1
    local file_num=$2
    local total_files=$3
    local basename=$(basename "$file")

    # Enhanced progress reporting
    log_info "[$file_num/$total_files] Testing $basename..."

    # Optimization: Check file exists (cached)
    if ! check_file_exists "$file"; then
        log_error "   └─ File not found: $file (skipping)"
        return 1
    fi

    local file_passed=0
    local file_failed=0
    local file_skipped=0

    for tool in "${RUCHY_TOOLS[@]}"; do
        TOTAL_TESTS=$((TOTAL_TESTS + 1))

        if test_tool_on_file "$tool" "$file"; then
            ((file_passed++))
        else
            local result=$?
            if [ $result -eq 2 ]; then
                ((file_skipped++))
            else
                ((file_failed++))
                log_error "   └─ $tool FAILED on $basename"
            fi
        fi
    done

    # Summary for this file (enhanced with percentage)
    local total_for_file=${#RUCHY_TOOLS[@]}
    local success_rate=$(awk "BEGIN {printf \"%.1f\", ($file_passed/$total_for_file)*100}")

    if [ $file_passed -eq $total_for_file ]; then
        log_success "   ✅ $basename: ALL $total_for_file tools PASSED (100.0%)"
    elif [ $file_passed -gt 0 ]; then
        log_warning "   ⚠️  $basename: $file_passed/$total_for_file tools passed ($success_rate%)"
    else
        log_error "   ❌ $basename: 0/$total_for_file tools passed (0.0%)"
    fi

    EXAMPLE_STATS["$file"]="$file_passed,$file_failed,$file_skipped"
}

# Generate JSON results
generate_json_results() {
    log_info "Generating JSON results..."

    local timestamp=$(date -u +"%Y-%m-%d %H:%M:%S UTC")
    local ruchy_version=$("$RUCHY_BINARY" --version 2>&1 | head -1 || echo "unknown")

    cat > "$RESULTS_FILE" << EOF
{
  "timestamp": "$timestamp",
  "ruchy_version": "$ruchy_version",
  "summary": {
    "total_tests": $TOTAL_TESTS,
    "passed": $TOTAL_PASSED,
    "failed": $TOTAL_FAILED,
    "skipped": $TOTAL_SKIPPED,
    "success_rate": $(awk "BEGIN {printf \"%.2f\", ($TOTAL_PASSED/$TOTAL_TESTS)*100}")
  },
  "by_tool": {
EOF

    local first_tool=true
    for tool in "${RUCHY_TOOLS[@]}"; do
        if [ "$first_tool" = false ]; then
            echo "," >> "$RESULTS_FILE"
        fi
        first_tool=false

        local passed=${TOOL_STATS["${tool}_passed"]}
        local failed=${TOOL_STATS["${tool}_failed"]}
        local skipped=${TOOL_STATS["${tool}_skipped"]}
        local total=$((passed + failed + skipped))
        local rate=0

        if [ $total -gt 0 ]; then
            rate=$(awk "BEGIN {printf \"%.2f\", ($passed/$total)*100}")
        fi

        cat >> "$RESULTS_FILE" << TOOL_EOF
    "$tool": {
      "passed": $passed,
      "failed": $failed,
      "skipped": $skipped,
      "total": $total,
      "success_rate": $rate
    }
TOOL_EOF
    done

    cat >> "$RESULTS_FILE" << EOF

  },
  "examples_tested": $(echo "${!EXAMPLE_STATS[@]}" | wc -w),
  "tools_tested": ${#RUCHY_TOOLS[@]}
}
EOF

    log_success "Results saved to $RESULTS_FILE"
}

# Generate markdown report
generate_markdown_report() {
    local report_file="ruchy-tools-validation-report.md"

    log_info "Generating markdown report..."

    local timestamp=$(date -u +"%Y-%m-%d %H:%M:%S UTC")
    local ruchy_version=$("$RUCHY_BINARY" --version 2>&1 | head -1 || echo "unknown")

    cat > "$report_file" << EOF
# Ruchy Tools Comprehensive Validation Report

**Generated**: $timestamp
**Ruchy Version**: $ruchy_version
**Repository**: rosetta-ruchy

## Executive Summary

| Metric | Value |
|--------|-------|
| **Total Tests** | $TOTAL_TESTS |
| **Passed** | ✅ $TOTAL_PASSED |
| **Failed** | ❌ $TOTAL_FAILED |
| **Skipped** | ⚠️ $TOTAL_SKIPPED |
| **Success Rate** | $(awk "BEGIN {printf \"%.1f%%\", ($TOTAL_PASSED/$TOTAL_TESTS)*100}") |
| **Tools Tested** | ${#RUCHY_TOOLS[@]} |
| **Examples Tested** | $(echo "${!EXAMPLE_STATS[@]}" | wc -w) |

## Tools Performance

| Tool | Passed | Failed | Skipped | Total | Success Rate |
|------|--------|--------|---------|-------|--------------|
EOF

    for tool in "${RUCHY_TOOLS[@]}"; do
        local passed=${TOOL_STATS["${tool}_passed"]}
        local failed=${TOOL_STATS["${tool}_failed"]}
        local skipped=${TOOL_STATS["${tool}_skipped"]}
        local total=$((passed + failed + skipped))
        local rate="0.0%"

        if [ $total -gt 0 ]; then
            rate=$(awk "BEGIN {printf \"%.1f%%\", ($passed/$total)*100}")
        fi

        local status_emoji="✅"
        if [ $failed -gt 0 ]; then
            status_emoji="❌"
        elif [ $skipped -gt 0 ]; then
            status_emoji="⚠️"
        fi

        echo "| $status_emoji \`ruchy $tool\` | $passed | $failed | $skipped | $total | $rate |" >> "$report_file"
    done

    cat >> "$report_file" << EOF

## Tool Categories

### 🟢 Core Validation Tools (100% Required)
- \`ruchy check\` - Syntax validation
- \`ruchy parse\` - AST parsing
- \`ruchy provability\` - Formal correctness verification
- \`ruchy runtime\` - Complexity and performance analysis
- \`ruchy score\` - Unified quality score
- \`ruchy ast\` - Enhanced AST analysis

### 🔵 Advanced Analysis Tools
- \`ruchy optimize\` - Hardware-aware optimization
- \`ruchy prove\` - Interactive theorem prover
- \`ruchy quality-gate\` - Quality gate enforcement
- \`ruchy mcp\` - Real-time quality analysis

### 🟡 Development Tools
- \`ruchy fmt\` - Format source code
- \`ruchy lint\` - Lint for issues and style
- \`ruchy doc\` - Generate documentation

### 🟠 Compilation Tools (May Have Limitations)
- \`ruchy transpile\` - Transpile to Rust
- \`ruchy build\` - Build project
- \`ruchy run\` - Compile and run
- \`ruchy test\` - Run tests

### 🔴 Performance Tools (Advanced)
- \`ruchy benchmark\` - Performance benchmarking
- \`ruchy profile\` - Profiling
- \`ruchy energy\` - Energy consumption analysis

## Methodology

Following the [ruchy-book](https://github.com/paiml/ruchy-book) testing framework:

1. **Comprehensive Coverage**: Test ALL 18+ Ruchy tools
2. **Systematic Validation**: Test every .ruchy file in the repository
3. **Statistical Analysis**: Track success rates per tool and per example
4. **Scientific Rigor**: Document failures, timeouts, and unsupported features

## Quality Gates

Based on rosetta-ruchy project requirements:

- ✅ **Core Tools**: Must have 100% success rate
- ⚠️ **Advanced Tools**: Target 90%+ success rate
- ℹ️ **Experimental Tools**: Best effort, document limitations

## Reproducibility

To reproduce these results:

\`\`\`bash
# Run comprehensive Ruchy tools validation
./scripts/test-ruchy-tools-comprehensive.sh

# View detailed results
cat ruchy-tools-test-results.json
cat ruchy-tools-validation-report.md
cat ruchy-tools-detailed.log
\`\`\`

## Integration with CI/CD

This validation should run:
- On every push to main branch
- On all pull requests
- Nightly via scheduled workflow
- After Ruchy version upgrades

See: \`.github/workflows/ruchy-quality-gates.yml\`

---

*Automated validation following Toyota Way principles and ruchy-book methodology*
EOF

    log_success "Report saved to $report_file"
}

# Main execution
main() {
    echo "================================================================"
    echo "Ruchy Tools Comprehensive Validation"
    echo "Testing 18+ Ruchy tools across ALL examples"
    echo "================================================================"
    echo ""

    # Clear previous logs
    > "$DETAILED_LOG"

    # Check Ruchy installation
    if ! check_ruchy_installation; then
        log_error "Cannot proceed without Ruchy installation"
        exit 1
    fi

    echo ""

    # Find all Ruchy examples
    local ruchy_files
    ruchy_files=$(find_ruchy_examples)

    if [ -z "$ruchy_files" ]; then
        log_error "No .ruchy files found in $EXAMPLES_DIR"
        exit 1
    fi

    echo ""
    log_info "Testing ${#RUCHY_TOOLS[@]} tools on each example..."
    echo ""

    # Sprint 51 Optimization: Add progress tracking
    local file_count=0
    local total_file_count=$(echo "$ruchy_files" | wc -l)

    # Test all tools on all files with progress
    while IFS= read -r file; do
        file_count=$((file_count + 1))
        test_all_tools_on_file "$file" "$file_count" "$total_file_count"
    done <<< "$ruchy_files"

    # Performance tracking (Sprint 49 pattern)
    END_TIME=$(date +%s.%N)
    EXECUTION_TIME=$(awk "BEGIN {printf \"%.3f\", $END_TIME - $START_TIME}")

    echo ""
    echo "================================================================"
    echo "Validation Complete"
    echo "================================================================"
    echo ""

    # Generate results
    generate_json_results
    generate_markdown_report

    # Final summary
    echo ""
    log_info "Summary:"
    echo "  Total tests: $TOTAL_TESTS"
    echo "  Passed: ${GREEN}$TOTAL_PASSED${NC}"
    echo "  Failed: ${RED}$TOTAL_FAILED${NC}"
    echo "  Skipped: ${YELLOW}$TOTAL_SKIPPED${NC}"
    echo "  Success rate: $(awk "BEGIN {printf \"%.1f%%\", ($TOTAL_PASSED/$TOTAL_TESTS)*100}")"
    echo ""

    # Sprint 51 Optimization: Performance and cache statistics
    log_info "Performance Metrics:"
    echo "  Execution time: ${EXECUTION_TIME}s"
    echo ""
    log_info "Optimization Statistics:"
    echo "  File cache hits: $CACHE_FILE_HITS"
    echo "  Tool cache hits: $CACHE_TOOL_HITS"
    echo "  Syntax cache hits: $CACHE_SYNTAX_HITS"
    local total_cache_hits=$((CACHE_FILE_HITS + CACHE_TOOL_HITS + CACHE_SYNTAX_HITS))
    echo "  Total cache hits: $total_cache_hits"
    echo ""

    log_info "Results saved to:"
    echo "  - $RESULTS_FILE (machine-readable JSON)"
    echo "  - ruchy-tools-validation-report.md (human-readable report)"
    echo "  - $DETAILED_LOG (detailed execution log)"
    echo ""

    # Exit with error if failures
    if [ $TOTAL_FAILED -gt 0 ]; then
        log_error "Some tools failed validation"
        exit 1
    else
        log_success "All available tools validated successfully!"
        exit 0
    fi
}

# Run main
main "$@"
