#!/usr/bin/env bash
#
# Selection Sort Algorithm
#
# Part of rosetta-ruchy Tier 0 implementations for transpiler validation
# Sister project: bashrs (https://github.com/paiml/bashrs)
#
# Time Complexity: O(n²)
# Space Complexity: O(1)
#

set -euo pipefail

# Selection sort implementation
# Sorts array in-place in ascending order
selection_sort() {
    local -n arr=$1
    local size=${#arr[@]}

    for (( i=0; i<size-1; i++ )); do
        local min_idx=$i

        # Find minimum element in unsorted portion
        for (( j=i+1; j<size; j++ )); do
            if (( arr[j] < arr[min_idx] )); then
                min_idx=$j
            fi
        done

        # Swap minimum with first unsorted element
        if (( min_idx != i )); then
            local temp=${arr[i]}
            arr[i]=${arr[min_idx]}
            arr[min_idx]=$temp
        fi
    done
}

# Check if array is sorted
is_sorted() {
    # shellcheck disable=SC2178
    local -n arr=$1
    local size=${#arr[@]}

    for (( i=0; i<size-1; i++ )); do
        if (( arr[i] > arr[i+1] )); then
            return 1
        fi
    done
    return 0
}

# Print array
print_array() {
    # shellcheck disable=SC2178
    local -n arr=$1
    echo -n "["
    local first=1
    for elem in "${arr[@]}"; do
        if (( first )); then
            echo -n "$elem"
            first=0
        else
            echo -n ", $elem"
        fi
    done
    echo "]"
}

# Run test suite
run_tests() {
    echo "Running Selection Sort tests..."
    local failed=0

    # Test 1: Basic array
    local arr1=(64 25 12 22 11)
    selection_sort arr1
    if ! is_sorted arr1; then
        echo "FAIL: Test 1 - array not sorted" >&2
        ((failed++))
    fi
    if [[ "${arr1[0]}" != "11" ]] || [[ "${arr1[1]}" != "12" ]]; then
        echo "FAIL: Test 1 - incorrect values" >&2
        ((failed++))
    fi

    # Test 2: Already sorted
    # shellcheck disable=SC2034
    local arr2=(1 2 3 4 5)
    selection_sort arr2
    if ! is_sorted arr2; then
        echo "FAIL: Test 2 - already sorted array" >&2
        ((failed++))
    fi

    # Test 3: Reverse sorted
    local arr3=(5 4 3 2 1)
    selection_sort arr3
    if ! is_sorted arr3; then
        echo "FAIL: Test 3 - reverse sorted" >&2
        ((failed++))
    fi
    if [[ "${arr3[0]}" != "1" ]] || [[ "${arr3[4]}" != "5" ]]; then
        echo "FAIL: Test 3 - incorrect values" >&2
        ((failed++))
    fi

    # Test 4: Single element
    local arr4=(42)
    selection_sort arr4
    if [[ "${arr4[0]}" != "42" ]]; then
        echo "FAIL: Test 4 - single element" >&2
        ((failed++))
    fi

    # Test 5: Two elements
    local arr5=(2 1)
    selection_sort arr5
    if [[ "${arr5[0]}" != "1" ]] || [[ "${arr5[1]}" != "2" ]]; then
        echo "FAIL: Test 5 - two elements" >&2
        ((failed++))
    fi

    # Test 6: Duplicates
    local arr6=(3 1 4 1 5 9 2 6)
    selection_sort arr6
    if ! is_sorted arr6; then
        echo "FAIL: Test 6 - duplicates" >&2
        ((failed++))
    fi
    if [[ "${arr6[0]}" != "1" ]] || [[ "${arr6[1]}" != "1" ]]; then
        echo "FAIL: Test 6 - duplicate values" >&2
        ((failed++))
    fi

    # Test 7: All same elements
    # shellcheck disable=SC2034
    local arr7=(7 7 7 7 7)
    selection_sort arr7
    if ! is_sorted arr7; then
        echo "FAIL: Test 7 - all same" >&2
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

# Print usage
print_usage() {
    cat <<EOF
Usage: $0 <command> [args]

Commands:
    sort <num1> <num2> ...   Sort numbers
    test                     Run test suite
    help                     Show this help

Examples:
    $0 sort 64 25 12 22 11
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
        sort)
            if (( $# < 1 )); then
                echo "Error: sort requires at least one number" >&2
                exit 1
            fi
            local arr=("$@")
            echo -n "Before: "
            print_array arr
            selection_sort arr
            echo -n "After:  "
            print_array arr
            ;;
        help|--help|-h)
            print_usage
            ;;
        *)
            # Assume numbers for sorting
            local arr=("$command" "$@")
            echo -n "Before: "
            print_array arr
            selection_sort arr
            echo -n "After:  "
            print_array arr
            ;;
    esac
}

# Run main if executed (not sourced)
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
