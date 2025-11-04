#!/usr/bin/env bash
#
# benchmark-language-comparison.sh - EXACT language comparison benchmarks
#
# Following ruchy-book methodology for comprehensive language performance comparison.
# Compares Ruchy against Rust, Python, Go, JavaScript, Julia, R, and other languages
# across all implemented algorithms and examples.
#
# Based on: https://github.com/paiml/ruchy-book benchmark framework
#

set -euo pipefail

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;36m'
CYAN='\033[0;36m'
NC='\033[0;m'

# Configuration
EXAMPLES_DIR="examples"
RESULTS_DIR="benchmark-results"
RESULTS_FILE="$RESULTS_DIR/language-comparison-results.json"
REPORT_FILE="$RESULTS_DIR/language-comparison-report.md"
DETAILED_LOG="$RESULTS_DIR/benchmark-detailed.log"

# Benchmark configuration
WARMUP_ITERATIONS=3
BENCHMARK_ITERATIONS=10
TIMEOUT_SECONDS=300

# Languages to benchmark (Tier 1: Primary languages)
declare -a TIER1_LANGUAGES=(
    "ruchy"      # Ruchy (target language)
    "rust"       # Rust (performance baseline)
    "python"     # Python (ergonomics baseline)
    "javascript" # JavaScript (widespread adoption)
)

# Languages to benchmark (Tier 2: JVM and specialized)
declare -a TIER2_LANGUAGES=(
    "go"         # Go (concurrency)
    "julia"      # Julia (scientific computing)
    "r"          # R (statistical computing)
    "kotlin"     # Kotlin (JVM ecosystem)
    "scala"      # Scala (functional + big data)
)

# All languages combined
ALL_LANGUAGES=("${TIER1_LANGUAGES[@]}" "${TIER2_LANGUAGES[@]}")

# Benchmark metrics
declare -A LANGUAGE_STATS
declare -A ALGORITHM_STATS

# Statistics
TOTAL_BENCHMARKS=0
SUCCESSFUL_BENCHMARKS=0
FAILED_BENCHMARKS=0

# Setup
mkdir -p "$RESULTS_DIR"
> "$DETAILED_LOG"

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

# Check if language runtime is available
check_language_runtime() {
    local lang=$1

    case "$lang" in
        ruchy)
            command -v ruchy &>/dev/null && return 0 || return 1
            ;;
        rust)
            command -v cargo &>/dev/null && return 0 || return 1
            ;;
        python)
            command -v python3 &>/dev/null && return 0 || return 1
            ;;
        javascript)
            command -v node &>/dev/null && return 0 || return 1
            ;;
        go)
            command -v go &>/dev/null && return 0 || return 1
            ;;
        julia)
            command -v julia &>/dev/null && return 0 || return 1
            ;;
        r)
            command -v Rscript &>/dev/null && return 0 || return 1
            ;;
        kotlin)
            command -v kotlinc &>/dev/null && return 0 || return 1
            ;;
        scala)
            command -v scala &>/dev/null && return 0 || return 1
            ;;
        *)
            return 1
            ;;
    esac
}

# Get language version
get_language_version() {
    local lang=$1

    case "$lang" in
        ruchy)
            ruchy --version 2>&1 | head -1 || echo "unknown"
            ;;
        rust)
            cargo --version 2>&1 | head -1 || echo "unknown"
            ;;
        python)
            python3 --version 2>&1 | head -1 || echo "unknown"
            ;;
        javascript)
            node --version 2>&1 | head -1 || echo "unknown"
            ;;
        go)
            go version 2>&1 | head -1 || echo "unknown"
            ;;
        julia)
            julia --version 2>&1 | head -1 || echo "unknown"
            ;;
        r)
            Rscript --version 2>&1 | head -1 || echo "unknown"
            ;;
        kotlin)
            kotlinc -version 2>&1 | head -1 || echo "unknown"
            ;;
        scala)
            scala -version 2>&1 | head -1 || echo "unknown"
            ;;
        *)
            echo "unknown"
            ;;
    esac
}

# Find implementation file for a language in an algorithm directory
find_implementation() {
    local algorithm_dir=$1
    local lang=$2

    local impl_dir="$algorithm_dir/implementations/$lang"

    if [ ! -d "$impl_dir" ]; then
        return 1
    fi

    # Find the main implementation file
    case "$lang" in
        ruchy)
            find "$impl_dir" -name "*.ruchy" -type f | head -1
            ;;
        rust)
            echo "$impl_dir/Cargo.toml"
            ;;
        python)
            find "$impl_dir" -name "*.py" -type f | grep -v test | head -1
            ;;
        javascript)
            find "$impl_dir" -name "*.js" -type f | grep -v test | head -1
            ;;
        go)
            find "$impl_dir" -name "*.go" -type f | grep -v test | head -1
            ;;
        julia)
            find "$impl_dir" -name "*.jl" -type f | head -1
            ;;
        r)
            find "$impl_dir" -name "*.R" -type f | head -1
            ;;
        kotlin)
            find "$impl_dir" -name "*.kt" -type f | head -1
            ;;
        scala)
            find "$impl_dir" -name "*.scala" -type f | head -1
            ;;
        *)
            return 1
            ;;
    esac
}

# Compile/prepare implementation for benchmarking
prepare_implementation() {
    local impl_file=$1
    local lang=$2
    local impl_dir=$(dirname "$impl_file")

    cd "$impl_dir"

    case "$lang" in
        ruchy)
            # Ruchy compilation (if needed)
            timeout "$TIMEOUT_SECONDS" ruchy build "$impl_file" 2>&1 || return 1
            ;;
        rust)
            # Rust compilation with optimizations
            timeout "$TIMEOUT_SECONDS" cargo build --release 2>&1 || return 1
            ;;
        python)
            # Python doesn't need compilation
            return 0
            ;;
        javascript)
            # JavaScript doesn't need compilation
            return 0
            ;;
        go)
            # Go compilation
            timeout "$TIMEOUT_SECONDS" go build -o binary *.go 2>&1 || return 1
            ;;
        julia)
            # Julia doesn't need ahead-of-time compilation
            return 0
            ;;
        r)
            # R doesn't need compilation
            return 0
            ;;
        kotlin)
            # Kotlin compilation
            timeout "$TIMEOUT_SECONDS" kotlinc "$impl_file" -include-runtime -d binary.jar 2>&1 || return 1
            ;;
        scala)
            # Scala compilation
            timeout "$TIMEOUT_SECONDS" scalac "$impl_file" 2>&1 || return 1
            ;;
        *)
            return 1
            ;;
    esac
}

# Run benchmark for a single implementation
run_benchmark() {
    local impl_file=$1
    local lang=$2
    local impl_dir=$(dirname "$impl_file")

    cd "$impl_dir"

    local total_time=0
    local successful_runs=0

    # Warmup runs
    for i in $(seq 1 $WARMUP_ITERATIONS); do
        case "$lang" in
            ruchy)
                timeout "$TIMEOUT_SECONDS" ruchy run "$impl_file" &>/dev/null || continue
                ;;
            rust)
                timeout "$TIMEOUT_SECONDS" cargo run --release &>/dev/null || continue
                ;;
            python)
                timeout "$TIMEOUT_SECONDS" python3 "$impl_file" &>/dev/null || continue
                ;;
            javascript)
                timeout "$TIMEOUT_SECONDS" node "$impl_file" &>/dev/null || continue
                ;;
            go)
                timeout "$TIMEOUT_SECONDS" ./binary &>/dev/null || continue
                ;;
            julia)
                timeout "$TIMEOUT_SECONDS" julia "$impl_file" &>/dev/null || continue
                ;;
            r)
                timeout "$TIMEOUT_SECONDS" Rscript "$impl_file" &>/dev/null || continue
                ;;
            kotlin)
                timeout "$TIMEOUT_SECONDS" java -jar binary.jar &>/dev/null || continue
                ;;
            scala)
                timeout "$TIMEOUT_SECONDS" scala $(basename "$impl_file" .scala) &>/dev/null || continue
                ;;
        esac
    done

    # Benchmark runs
    for i in $(seq 1 $BENCHMARK_ITERATIONS); do
        local start_time=$(date +%s%N)

        local run_success=false
        case "$lang" in
            ruchy)
                if timeout "$TIMEOUT_SECONDS" ruchy run "$impl_file" &>/dev/null; then
                    run_success=true
                fi
                ;;
            rust)
                if timeout "$TIMEOUT_SECONDS" cargo run --release &>/dev/null; then
                    run_success=true
                fi
                ;;
            python)
                if timeout "$TIMEOUT_SECONDS" python3 "$impl_file" &>/dev/null; then
                    run_success=true
                fi
                ;;
            javascript)
                if timeout "$TIMEOUT_SECONDS" node "$impl_file" &>/dev/null; then
                    run_success=true
                fi
                ;;
            go)
                if timeout "$TIMEOUT_SECONDS" ./binary &>/dev/null; then
                    run_success=true
                fi
                ;;
            julia)
                if timeout "$TIMEOUT_SECONDS" julia "$impl_file" &>/dev/null; then
                    run_success=true
                fi
                ;;
            r)
                if timeout "$TIMEOUT_SECONDS" Rscript "$impl_file" &>/dev/null; then
                    run_success=true
                fi
                ;;
            kotlin)
                if timeout "$TIMEOUT_SECONDS" java -jar binary.jar &>/dev/null; then
                    run_success=true
                fi
                ;;
            scala)
                if timeout "$TIMEOUT_SECONDS" scala $(basename "$impl_file" .scala) &>/dev/null; then
                    run_success=true
                fi
                ;;
        esac

        local end_time=$(date +%s%N)

        if [ "$run_success" = true ]; then
            local elapsed=$((end_time - start_time))
            total_time=$((total_time + elapsed))
            ((successful_runs++))
        fi
    done

    if [ $successful_runs -eq 0 ]; then
        return 1
    fi

    # Calculate average time in milliseconds
    local avg_time_ns=$((total_time / successful_runs))
    local avg_time_ms=$(awk "BEGIN {printf \"%.2f\", $avg_time_ns/1000000}")

    echo "$avg_time_ms"
}

# Benchmark an algorithm across all languages
benchmark_algorithm() {
    local algorithm_dir=$1
    local algorithm_name=$(basename "$algorithm_dir")

    log_info "Benchmarking $algorithm_name..."

    declare -A lang_results
    local baseline_time=0

    # Benchmark each language
    for lang in "${ALL_LANGUAGES[@]}"; do
        TOTAL_BENCHMARKS=$((TOTAL_BENCHMARKS + 1))

        # Check if runtime is available
        if ! check_language_runtime "$lang"; then
            log_warning "  $lang: Runtime not available (skipped)"
            continue
        fi

        # Find implementation
        local impl_file
        if ! impl_file=$(find_implementation "$algorithm_dir" "$lang"); then
            log_warning "  $lang: Implementation not found (skipped)"
            continue
        fi

        if [ -z "$impl_file" ]; then
            log_warning "  $lang: Implementation not found (skipped)"
            continue
        fi

        # Prepare (compile) implementation
        log_info "  $lang: Preparing..."
        if ! prepare_implementation "$impl_file" "$lang" > "$DETAILED_LOG" 2>&1; then
            log_error "  $lang: Compilation failed"
            FAILED_BENCHMARKS=$((FAILED_BENCHMARKS + 1))
            continue
        fi

        # Run benchmark
        log_info "  $lang: Running benchmark ($BENCHMARK_ITERATIONS iterations)..."
        local result
        if result=$(run_benchmark "$impl_file" "$lang"); then
            lang_results["$lang"]="$result"
            SUCCESSFUL_BENCHMARKS=$((SUCCESSFUL_BENCHMARKS + 1))
            log_success "  $lang: ${result}ms (avg)"

            # Store Rust as baseline
            if [ "$lang" = "rust" ]; then
                baseline_time="$result"
            fi
        else
            log_error "  $lang: Benchmark failed"
            FAILED_BENCHMARKS=$((FAILED_BENCHMARKS + 1))
            continue
        fi
    done

    # Calculate performance relative to Rust baseline
    if [ -n "$baseline_time" ] && (( $(echo "$baseline_time > 0" | bc -l) )); then
        echo ""
        log_info "Performance vs Rust baseline:"

        for lang in "${!lang_results[@]}"; do
            if [ "$lang" != "rust" ]; then
                local lang_time="${lang_results[$lang]}"
                local relative=$(awk "BEGIN {printf \"%.2f\", ($lang_time/$baseline_time)*100}")

                local status_emoji="✅"
                if (( $(echo "$relative > 120" | bc -l) )); then
                    status_emoji="❌"  # More than 20% slower
                elif (( $(echo "$relative > 105" | bc -l) )); then
                    status_emoji="⚠️"   # 5-20% slower
                fi

                log_info "  $status_emoji $lang: ${relative}% of Rust (${lang_time}ms vs ${baseline_time}ms)"
            fi
        done
    fi

    echo ""
}

# Find all algorithm directories
find_algorithms() {
    find "$EXAMPLES_DIR" -type d -name "implementations" | while read -r impl_dir; do
        dirname "$impl_dir"
    done | sort | uniq
}

# Generate JSON results
generate_json_results() {
    log_info "Generating JSON results..."

    # Implementation would generate structured JSON output
    cat > "$RESULTS_FILE" << EOF
{
  "timestamp": "$(date -u +"%Y-%m-%d %H:%M:%S UTC")",
  "methodology": "ruchy-book compatible benchmark framework",
  "configuration": {
    "warmup_iterations": $WARMUP_ITERATIONS,
    "benchmark_iterations": $BENCHMARK_ITERATIONS,
    "timeout_seconds": $TIMEOUT_SECONDS
  },
  "summary": {
    "total_benchmarks": $TOTAL_BENCHMARKS,
    "successful": $SUCCESSFUL_BENCHMARKS,
    "failed": $FAILED_BENCHMARKS,
    "success_rate": $(awk "BEGIN {printf \"%.2f\", ($SUCCESSFUL_BENCHMARKS/$TOTAL_BENCHMARKS)*100}")
  },
  "languages": {
EOF

    local first=true
    for lang in "${ALL_LANGUAGES[@]}"; do
        if [ "$first" = false ]; then
            echo "," >> "$RESULTS_FILE"
        fi
        first=false

        local version=$(get_language_version "$lang")
        cat >> "$RESULTS_FILE" << LANG_EOF
    "$lang": {
      "version": "$version",
      "available": $(check_language_runtime "$lang" && echo "true" || echo "false")
    }
LANG_EOF
    done

    cat >> "$RESULTS_FILE" << EOF

  }
}
EOF

    log_success "Results saved to $RESULTS_FILE"
}

# Generate markdown report
generate_markdown_report() {
    log_info "Generating markdown report..."

    cat > "$REPORT_FILE" << EOF
# Language Comparison Benchmark Report

**Generated**: $(date -u +"%Y-%m-%d %H:%M:%S UTC")
**Methodology**: ruchy-book compatible benchmark framework
**Repository**: rosetta-ruchy

## Executive Summary

| Metric | Value |
|--------|-------|
| **Total Benchmarks** | $TOTAL_BENCHMARKS |
| **Successful** | ✅ $SUCCESSFUL_BENCHMARKS |
| **Failed** | ❌ $FAILED_BENCHMARKS |
| **Success Rate** | $(awk "BEGIN {printf \"%.1f%%\", ($SUCCESSFUL_BENCHMARKS/$TOTAL_BENCHMARKS)*100}") |

## Language Runtimes

| Language | Version | Available |
|----------|---------|-----------|
EOF

    for lang in "${ALL_LANGUAGES[@]}"; do
        local version=$(get_language_version "$lang")
        local available=$(check_language_runtime "$lang" && echo "✅" || echo "❌")
        echo "| $lang | $version | $available |" >> "$REPORT_FILE"
    done

    cat >> "$REPORT_FILE" << EOF

## Benchmark Configuration

- **Warmup Iterations**: $WARMUP_ITERATIONS
- **Benchmark Iterations**: $BENCHMARK_ITERATIONS
- **Timeout**: $TIMEOUT_SECONDS seconds

## Methodology

Following the [ruchy-book](https://github.com/paiml/ruchy-book) benchmark framework:

1. **Fair Comparison**: Same algorithm across all languages
2. **Statistical Rigor**: Multiple iterations with warmup
3. **Performance Baseline**: Rust as reference implementation
4. **Comprehensive Coverage**: All Tier 1 and Tier 2 languages

## Performance Targets

Based on rosetta-ruchy project goals:

- ✅ **Ruchy vs Rust**: Within 5% (target: performance parity)
- ⚠️ **Ruchy vs Python**: 10-50x faster (expected)
- ℹ️ **Ruchy vs Go**: Comparable performance

## Reproducibility

To reproduce these results:

\`\`\`bash
# Run language comparison benchmarks
./scripts/benchmark-language-comparison.sh

# View results
cat benchmark-results/language-comparison-results.json
cat benchmark-results/language-comparison-report.md
\`\`\`

---

*Automated benchmarking following ruchy-book methodology*
EOF

    log_success "Report saved to $REPORT_FILE"
}

# Main execution
main() {
    echo "================================================================"
    echo "Language Comparison Benchmark Suite"
    echo "EXACT methodology from ruchy-book"
    echo "================================================================"
    echo ""

    log_info "Checking language runtimes..."
    for lang in "${ALL_LANGUAGES[@]}"; do
        if check_language_runtime "$lang"; then
            local version=$(get_language_version "$lang")
            log_success "$lang: $version"
        else
            log_warning "$lang: Not available"
        fi
    done

    echo ""
    log_info "Finding algorithms to benchmark..."

    local algorithms
    algorithms=$(find_algorithms)
    local algorithm_count=$(echo "$algorithms" | wc -l)

    log_success "Found $algorithm_count algorithms"
    echo ""

    # Benchmark all algorithms
    while IFS= read -r algorithm_dir; do
        benchmark_algorithm "$algorithm_dir"
    done <<< "$algorithms"

    # Generate results
    generate_json_results
    generate_markdown_report

    echo ""
    echo "================================================================"
    echo "Benchmark Complete"
    echo "================================================================"
    echo ""
    log_info "Summary:"
    echo "  Total benchmarks: $TOTAL_BENCHMARKS"
    echo "  Successful: ${GREEN}$SUCCESSFUL_BENCHMARKS${NC}"
    echo "  Failed: ${RED}$FAILED_BENCHMARKS${NC}"
    echo ""
    log_info "Results saved to:"
    echo "  - $RESULTS_FILE"
    echo "  - $REPORT_FILE"
    echo "  - $DETAILED_LOG"
    echo ""
}

# Run main
main "$@"
