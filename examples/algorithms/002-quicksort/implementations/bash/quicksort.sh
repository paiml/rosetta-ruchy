#!/usr/bin/env bash
#
# Quicksort Algorithm
#
# Part of rosetta-ruchy Tier 0 implementations for transpiler validation
# Sister project: bashrs (https://github.com/paiml/bashrs)
#
# Time Complexity: O(n log n) average, O(n²) worst case
# Space Complexity: O(log n) for recursion stack
#

set -euo pipefail

# In-place quicksort implementation using Lomuto partition scheme
quicksort() {
    local -n qs_arr=$1
    local size=${#qs_arr[@]}

    if (( size > 1 )); then
        quicksort_range "$1" 0 $((size - 1))
    fi
}

# Global variable for partition result (avoids subshell issues)
_PARTITION_INDEX=0

# Quicksort for a range within the array
quicksort_range() {
    # shellcheck disable=SC2034
    local -n qsr_arr=$1
    local low=$2
    local high=$3

    if (( low < high )); then
        partition "$1" "$low" "$high"
        local pivot_index=$_PARTITION_INDEX

        if (( pivot_index > 0 )); then
            quicksort_range "$1" "$low" $((pivot_index - 1))
        fi

        if (( pivot_index + 1 <= high )); then
            quicksort_range "$1" $((pivot_index + 1)) "$high"
        fi
    fi
}

# Lomuto partition scheme
# Sets _PARTITION_INDEX as output
partition() {
    local -n part_arr=$1
    local low=$2
    local high=$3

    local pivot=${part_arr[$high]}
    local i=$low

    local j=$low
    while (( j < high )); do
        if (( part_arr[j] <= pivot )); then
            # Swap part_arr[i] and part_arr[j]
            local temp=${part_arr[i]}
            part_arr[i]=${part_arr[j]}
            part_arr[j]=$temp
            i=$((i + 1))
        fi
        j=$((j + 1))
    done

    # Swap part_arr[i] and part_arr[high]
    local temp=${part_arr[i]}
    part_arr[i]=${part_arr[high]}
    part_arr[high]=$temp

    _PARTITION_INDEX=$i
}

# Check if array is sorted
is_sorted() {
    # shellcheck disable=SC2178
    local -n array_ref=$1
    local size=${#array_ref[@]}

    local i=0
    while (( i < size - 1 )); do
        if (( array_ref[i] > array_ref[i+1] )); then
            return 1
        fi
        i=$((i + 1))
    done
    return 0
}

# Print array
print_array() {
    # shellcheck disable=SC2178
    local -n array_ref=$1
    echo -n "["
    local first=1
    for elem in "${array_ref[@]}"; do
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
    echo "Running Quicksort tests..."
    local failed=0

    # Test 1: Empty array
    local arr1=()
    quicksort arr1
    if [[ ${#arr1[@]} -ne 0 ]]; then
        echo "FAIL: Test 1 - empty array size changed" >&2
        failed=$((failed + 1))
    fi

    # Test 2: Single element
    local arr2=(42)
    quicksort arr2
    if [[ "${arr2[0]}" != "42" ]]; then
        echo "FAIL: Test 2 - single element" >&2
        failed=$((failed + 1))
    fi

    # Test 3: Basic array with duplicates
    local arr3=(3 1 4 1 5 9 2 6)
    quicksort arr3
    if ! is_sorted arr3; then
        echo "FAIL: Test 3 - array not sorted" >&2
        failed=$((failed + 1))
    fi
    if [[ "${arr3[0]}" != "1" ]] || [[ "${arr3[7]}" != "9" ]]; then
        echo "FAIL: Test 3 - incorrect values" >&2
        failed=$((failed + 1))
    fi

    # Test 4: Reverse sorted
    local arr4=(5 4 3 2 1)
    quicksort arr4
    if ! is_sorted arr4; then
        echo "FAIL: Test 4 - reverse sorted" >&2
        failed=$((failed + 1))
    fi
    if [[ "${arr4[0]}" != "1" ]] || [[ "${arr4[4]}" != "5" ]]; then
        echo "FAIL: Test 4 - incorrect values" >&2
        failed=$((failed + 1))
    fi

    # Test 5: Already sorted
    # shellcheck disable=SC2034
    local arr5=(1 2 3 4 5)
    quicksort arr5
    if ! is_sorted arr5; then
        echo "FAIL: Test 5 - already sorted" >&2
        failed=$((failed + 1))
    fi

    # Test 6: All same elements
    # shellcheck disable=SC2034
    local arr6=(5 5 5 5 5)
    quicksort arr6
    if ! is_sorted arr6; then
        echo "FAIL: Test 6 - all same" >&2
        failed=$((failed + 1))
    fi

    # Test 7: Two elements
    local arr7=(2 1)
    quicksort arr7
    if [[ "${arr7[0]}" != "1" ]] || [[ "${arr7[1]}" != "2" ]]; then
        echo "FAIL: Test 7 - two elements" >&2
        failed=$((failed + 1))
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
    $0 sort 3 1 4 1 5 9 2 6
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
            quicksort arr
            echo -n "After:  "
            print_array arr
            ;;
        help|--help|-h)
            print_usage
            ;;
        *)
            # Assume numbers for sorting
            # shellcheck disable=SC2034
            local arr=("$command" "$@")
            echo -n "Before: "
            print_array arr
            quicksort arr
            echo -n "After:  "
            print_array arr
            ;;
    esac
}

# Run main if executed (not sourced)
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
