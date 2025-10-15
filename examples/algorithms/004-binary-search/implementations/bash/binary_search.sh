#!/usr/bin/env bash
#
# Binary Search Algorithm
#
# Part of rosetta-ruchy Tier 0 implementations for transpiler validation
# Sister project: bashrs (https://github.com/paiml/bashrs)
#
# Time Complexity: O(log n)
# Space Complexity: O(1) iterative, O(log n) recursive
#

set -euo pipefail

# Iterative binary search
# Args: target, array elements...
# Returns: index (0-based) or -1 if not found
binary_search_iterative() {
    local target=$1
    shift
    local arr=("$@")
    local size=${#arr[@]}

    local left=0
    local right=$((size - 1))

    while (( left <= right )); do
        local mid=$(( left + (right - left) / 2 ))

        if (( arr[mid] == target )); then
            echo "$mid"
            return 0
        elif (( arr[mid] < target )); then
            left=$((mid + 1))
        else
            right=$((mid - 1))
        fi
    done

    echo "-1"
    return 1
}

# Recursive binary search helper
binary_search_recursive_helper() {
    local target=$1
    local left=$2
    local right=$3
    shift 3
    local arr=("$@")

    if (( left > right )); then
        echo "-1"
        return 1
    fi

    local mid=$(( left + (right - left) / 2 ))

    if (( arr[mid] == target )); then
        echo "$mid"
        return 0
    elif (( arr[mid] < target )); then
        binary_search_recursive_helper "$target" $((mid + 1)) "$right" "${arr[@]}"
    else
        binary_search_recursive_helper "$target" "$left" $((mid - 1)) "${arr[@]}"
    fi
}

# Recursive binary search
# Args: target, array elements...
binary_search_recursive() {
    local target=$1
    shift
    local arr=("$@")
    local size=${#arr[@]}

    binary_search_recursive_helper "$target" 0 $((size - 1)) "${arr[@]}"
}

# Run test suite
run_tests() {
    echo "Running Binary Search tests..."
    local failed=0

    # Test case 1: Simple array
    local arr=(1 3 5 7 9 11 13 15 17 19)

    # Test iterative
    result=$(binary_search_iterative 7 "${arr[@]}")
    [[ "$result" == "3" ]] || { echo "FAIL: Expected 3, got $result"; ((failed++)); }

    result=$(binary_search_iterative 1 "${arr[@]}")
    [[ "$result" == "0" ]] || { echo "FAIL: Expected 0, got $result"; ((failed++)); }

    result=$(binary_search_iterative 19 "${arr[@]}")
    [[ "$result" == "9" ]] || { echo "FAIL: Expected 9, got $result"; ((failed++)); }

    result=$(binary_search_iterative 20 "${arr[@]}") || true
    [[ "$result" == "-1" ]] || { echo "FAIL: Expected -1, got $result"; ((failed++)); }

    result=$(binary_search_iterative 8 "${arr[@]}") || true
    [[ "$result" == "-1" ]] || { echo "FAIL: Expected -1 (not found), got $result"; ((failed++)); }

    # Test recursive
    result=$(binary_search_recursive 7 "${arr[@]}")
    [[ "$result" == "3" ]] || { echo "FAIL (recursive): Expected 3, got $result"; ((failed++)); }

    result=$(binary_search_recursive 19 "${arr[@]}")
    [[ "$result" == "9" ]] || { echo "FAIL (recursive): Expected 9, got $result"; ((failed++)); }

    # Test edge cases
    result=$(binary_search_iterative 42 42)
    [[ "$result" == "0" ]] || { echo "FAIL (single element): Expected 0, got $result"; ((failed++)); }

    result=$(binary_search_iterative 10 10 20)
    [[ "$result" == "0" ]] || { echo "FAIL (two elements): Expected 0, got $result"; ((failed++)); }

    result=$(binary_search_iterative 20 10 20)
    [[ "$result" == "1" ]] || { echo "FAIL (two elements): Expected 1, got $result"; ((failed++)); }

    if (( failed == 0 )); then
        echo "✓ All tests passed"
        return 0
    else
        echo "✗ $failed test(s) failed" >&2
        return 1
    fi
}

# Print usage
print_usage() {
    cat <<EOF
Usage: $0 <command> [args]

Commands:
    search <target> <elem1> <elem2> ...   Search for target in array
    test                                   Run test suite
    help                                   Show this help

Examples:
    $0 search 7 1 3 5 7 9 11 13
    $0 test

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
        search)
            if (( $# < 2 )); then
                echo "Error: search requires at least target and one array element" >&2
                exit 1
            fi
            result=$(binary_search_iterative "$@")
            if [[ "$result" == "-1" ]]; then
                echo "Not found"
                exit 1
            else
                echo "Found at index: $result"
            fi
            ;;
        recursive)
            if (( $# < 2 )); then
                echo "Error: recursive requires at least target and one array element" >&2
                exit 1
            fi
            result=$(binary_search_recursive "$@")
            if [[ "$result" == "-1" ]]; then
                echo "Not found"
                exit 1
            else
                echo "Found at index: $result"
            fi
            ;;
        help|--help|-h)
            print_usage
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
