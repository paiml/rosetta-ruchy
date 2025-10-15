#!/usr/bin/env bash
#
# Fibonacci Number Calculator
#
# Part of rosetta-ruchy Tier 0 implementations for transpiler validation
# Sister project: bashrs (https://github.com/paiml/bashrs)
#
# Quality Gates:
# - shellcheck (zero warnings/errors)
# - POSIX compliance where possible
# - set -euo pipefail (strict mode)
# - BATS testing framework
#

set -euo pipefail

# Recursive Fibonacci (exponential complexity)
# Time: O(2^n), Space: O(n) call stack
fibonacci_recursive() {
    local n=$1

    if (( n < 0 )); then
        echo "Error: Fibonacci not defined for negative numbers" >&2
        return 1
    fi

    if (( n <= 1 )); then
        echo "$n"
        return 0
    fi

    local n1 n2
    n1=$(fibonacci_recursive $((n - 1)))
    n2=$(fibonacci_recursive $((n - 2)))

    echo $((n1 + n2))
}

# Iterative Fibonacci (linear complexity)
# Time: O(n), Space: O(1)
fibonacci_iterative() {
    local n=$1

    if (( n < 0 )); then
        echo "Error: Fibonacci not defined for negative numbers" >&2
        return 1
    fi

    if (( n <= 1 )); then
        echo "$n"
        return 0
    fi

    local a=0 b=1
    for (( i=2; i<=n; i++ )); do
        local temp=$((a + b))
        a=$b
        b=$temp
    done

    echo "$b"
}

# Memoized Fibonacci using associative array
# Time: O(n), Space: O(n)
declare -A FIB_CACHE

fibonacci_memoized() {
    local n=$1

    if (( n < 0 )); then
        echo "Error: Fibonacci not defined for negative numbers" >&2
        return 1
    fi

    if (( n <= 1 )); then
        echo "$n"
        return 0
    fi

    # Check cache
    if [[ -n "${FIB_CACHE[$n]:-}" ]]; then
        echo "${FIB_CACHE[$n]}"
        return 0
    fi

    # Calculate and cache
    local n1 n2
    n1=$(fibonacci_memoized $((n - 1)))
    n2=$(fibonacci_memoized $((n - 2)))
    local result=$((n1 + n2))

    FIB_CACHE[$n]=$result
    echo "$result"
}

# Run test suite
run_tests() {
    echo "Running Fibonacci tests..."
    local failed=0

    # Test cases: n -> expected
    local test_cases=(
        "0:0"
        "1:1"
        "2:1"
        "3:2"
        "4:3"
        "5:5"
        "10:55"
        "20:6765"
    )

    # Test iterative implementation
    echo "Testing iterative..."
    for test in "${test_cases[@]}"; do
        IFS=':' read -r n expected <<< "$test"
        result=$(fibonacci_iterative "$n")

        if [[ "$result" != "$expected" ]]; then
            echo "  FAIL: fib($n) = $result, expected $expected" >&2
            ((failed++))
        fi
    done

    # Test memoized implementation
    echo "Testing memoized..."
    for test in "${test_cases[@]}"; do
        IFS=':' read -r n expected <<< "$test"
        result=$(fibonacci_memoized "$n")

        if [[ "$result" != "$expected" ]]; then
            echo "  FAIL: fib($n) = $result, expected $expected" >&2
            ((failed++))
        fi
    done

    # Test recursive implementation (limited to small values)
    echo "Testing recursive (small values only)..."
    for test in "0:0" "1:1" "2:1" "3:2" "4:3" "5:5" "10:55"; do
        IFS=':' read -r n expected <<< "$test"
        result=$(fibonacci_recursive "$n")

        if [[ "$result" != "$expected" ]]; then
            echo "  FAIL: fib($n) = $result, expected $expected" >&2
            ((failed++))
        fi
    done

    # Test error handling
    if fibonacci_iterative -1 2>/dev/null; then
        echo "  FAIL: Should have errored on negative input" >&2
        ((failed++))
    fi

    if (( failed == 0 )); then
        echo "✓ All tests passed"
        return 0
    else
        echo "✗ $failed test(s) failed" >&2
        return 1
    fi
}

# Run benchmark
run_benchmark() {
    local n=${1:-30}

    echo "Bash Fibonacci Benchmarks"
    echo "========================="

    # Iterative benchmark
    echo -n "Iterative fib($n): "
    local start end duration
    start=$(date +%s%N)
    result=$(fibonacci_iterative "$n")
    end=$(date +%s%N)
    duration=$((end - start))
    echo "$result (${duration}ns)"

    # Memoized benchmark
    echo -n "Memoized fib($n): "
    start=$(date +%s%N)
    result=$(fibonacci_memoized "$n")
    end=$(date +%s%N)
    duration=$((end - start))
    echo "$result (${duration}ns)"
}

# Print usage
print_usage() {
    cat <<EOF
Usage: $0 <command> [args]

Commands:
    <n>             Calculate fibonacci(n) using iterative method
    test            Run test suite
    benchmark [n]   Run benchmarks (default n=30)
    help            Show this help message

Variants:
    recursive <n>   Use recursive implementation (slow for n>20)
    iterative <n>   Use iterative implementation (default)
    memoized <n>    Use memoized implementation

Examples:
    $0 10
    $0 test
    $0 benchmark 40
    $0 iterative 20

EOF
}

# Main entry point
main() {
    if (( $# == 0 )); then
        print_usage
        exit 1
    fi

    local command=$1
    shift || true

    case "$command" in
        test)
            run_tests
            ;;
        benchmark)
            run_benchmark "${1:-30}"
            ;;
        help|--help|-h)
            print_usage
            ;;
        recursive)
            if (( $# == 0 )); then
                echo "Error: Missing argument n" >&2
                exit 1
            fi
            fibonacci_recursive "$1"
            ;;
        iterative)
            if (( $# == 0 )); then
                echo "Error: Missing argument n" >&2
                exit 1
            fi
            fibonacci_iterative "$1"
            ;;
        memoized)
            if (( $# == 0 )); then
                echo "Error: Missing argument n" >&2
                exit 1
            fi
            fibonacci_memoized "$1"
            ;;
        [0-9]*)
            # Numeric argument - calculate fibonacci
            fibonacci_iterative "$command"
            ;;
        *)
            echo "Error: Unknown command: $command" >&2
            print_usage
            exit 1
            ;;
    esac
}

# Run main if executed (not sourced)
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
