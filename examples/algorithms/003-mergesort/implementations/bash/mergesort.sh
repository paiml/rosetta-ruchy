#!/usr/bin/env bash
#
# Merge Sort Algorithm
#
# Part of rosetta-ruchy Tier 0 implementations for transpiler validation
# Sister project: bashrs (https://github.com/paiml/bashrs)
#
# Time Complexity: O(n log n)
# Space Complexity: O(n)
#

set -euo pipefail

# Merge two sorted subarrays arr[left..mid] and arr[mid+1..right]
merge() {
    local -n merge_arr=$1
    local left=$2
    local mid=$3
    local right=$4

    local left_size=$((mid - left + 1))
    local right_size=$((right - mid))

    # Create temporary arrays
    local -a left_arr
    local -a right_arr

    # Copy data to temporary arrays
    local i=0
    while (( i < left_size )); do
        left_arr[i]=${merge_arr[left + i]}
        i=$((i + 1))
    done

    local j=0
    while (( j < right_size )); do
        right_arr[j]=${merge_arr[mid + 1 + j]}
        j=$((j + 1))
    done

    # Merge the temporary arrays back into arr[left..right]
    i=0
    j=0
    local k=$left

    while [[ $i -lt $left_size && $j -lt $right_size ]]; do
        if (( left_arr[i] <= right_arr[j] )); then
            merge_arr[k]=${left_arr[i]}
            i=$((i + 1))
        else
            merge_arr[k]=${right_arr[j]}
            j=$((j + 1))
        fi
        k=$((k + 1))
    done

    # Copy remaining elements of left_arr, if any
    while [[ $i -lt $left_size ]]; do
        merge_arr[k]=${left_arr[i]}
        i=$((i + 1))
        k=$((k + 1))
    done

    # Copy remaining elements of right_arr, if any
    while [[ $j -lt $right_size ]]; do
        merge_arr[k]=${right_arr[j]}
        j=$((j + 1))
        k=$((k + 1))
    done
}

# Recursive mergesort implementation
mergesort_recursive() {
    # shellcheck disable=SC2034
    local -n msr_arr=$1
    local left=$2
    local right=$3

    if (( left < right )); then
        local mid=$((left + (right - left) / 2))

        # Sort first and second halves
        mergesort_recursive "$1" "$left" "$mid"
        mergesort_recursive "$1" $((mid + 1)) "$right"

        # Merge the sorted halves
        merge "$1" "$left" "$mid" "$right"
    fi
}

# Merge sort main entry point
mergesort() {
    local -n ms_arr=$1
    local size=${#ms_arr[@]}

    if (( size > 1 )); then
        mergesort_recursive "$1" 0 $((size - 1))
    fi
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
    echo "Running Merge Sort tests..."
    local failed=0

    # Test 1: Basic array
    local arr1=(64 34 25 12 22 11 90 88)
    mergesort arr1
    if ! is_sorted arr1; then
        echo "FAIL: Test 1 - array not sorted" >&2
        failed=$((failed + 1))
    fi
    if [[ "${arr1[0]}" != "11" ]] || [[ "${arr1[7]}" != "90" ]]; then
        echo "FAIL: Test 1 - incorrect values" >&2
        failed=$((failed + 1))
    fi

    # Test 2: Already sorted
    # shellcheck disable=SC2034
    local arr2=(1 2 3 4 5)
    mergesort arr2
    if ! is_sorted arr2; then
        echo "FAIL: Test 2 - already sorted" >&2
        failed=$((failed + 1))
    fi

    # Test 3: Reverse sorted
    local arr3=(5 4 3 2 1)
    mergesort arr3
    if ! is_sorted arr3; then
        echo "FAIL: Test 3 - reverse sorted" >&2
        failed=$((failed + 1))
    fi
    if [[ "${arr3[0]}" != "1" ]] || [[ "${arr3[4]}" != "5" ]]; then
        echo "FAIL: Test 3 - incorrect values" >&2
        failed=$((failed + 1))
    fi

    # Test 4: Single element
    local arr4=(42)
    mergesort arr4
    if [[ "${arr4[0]}" != "42" ]]; then
        echo "FAIL: Test 4 - single element" >&2
        failed=$((failed + 1))
    fi

    # Test 5: Two elements
    local arr5=(2 1)
    mergesort arr5
    if [[ "${arr5[0]}" != "1" ]] || [[ "${arr5[1]}" != "2" ]]; then
        echo "FAIL: Test 5 - two elements" >&2
        failed=$((failed + 1))
    fi

    # Test 6: All same elements
    # shellcheck disable=SC2034
    local arr6=(7 7 7 7 7)
    mergesort arr6
    if ! is_sorted arr6; then
        echo "FAIL: Test 6 - all same" >&2
        failed=$((failed + 1))
    fi

    # Test 7: With duplicates
    local arr7=(3 1 4 1 5 9 2 6)
    mergesort arr7
    if ! is_sorted arr7; then
        echo "FAIL: Test 7 - with duplicates" >&2
        failed=$((failed + 1))
    fi
    if [[ "${arr7[0]}" != "1" ]] || [[ "${arr7[1]}" != "1" ]]; then
        echo "FAIL: Test 7 - incorrect values" >&2
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
    $0 sort 64 34 25 12 22 11 90 88
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
            mergesort arr
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
            mergesort arr
            echo -n "After:  "
            print_array arr
            ;;
    esac
}

# Run main if executed (not sourced)
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
