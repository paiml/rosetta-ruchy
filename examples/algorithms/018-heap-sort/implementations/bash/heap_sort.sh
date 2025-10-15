#!/usr/bin/env bash
#
# Heap Sort Algorithm
#
# Part of rosetta-ruchy Tier 0 implementations for transpiler validation
# Sister project: bashrs (https://github.com/paiml/bashrs)
#
# Time Complexity: O(n log n)
# Space Complexity: O(1) - in-place sorting
#

set -euo pipefail

# Swap two elements in array
swap() {
    local -n swap_arr=$1
    local i=$2
    local j=$3

    local temp=${swap_arr[i]}
    swap_arr[i]=${swap_arr[j]}
    swap_arr[j]=$temp
}

# Heapify a subtree rooted at index i
heapify() {
    local -n heap_arr=$1
    local heap_size=$2
    local i=$3

    local largest=$i
    local left=$((2 * i + 1))
    local right=$((2 * i + 2))

    # If left child is larger than root
    if [[ $left -lt $heap_size && ${heap_arr[left]} -gt ${heap_arr[largest]} ]]; then
        largest=$left
    fi

    # If right child is larger than largest so far
    if [[ $right -lt $heap_size && ${heap_arr[right]} -gt ${heap_arr[largest]} ]]; then
        largest=$right
    fi

    # If largest is not root
    if [[ $largest -ne $i ]]; then
        swap "$1" "$i" "$largest"

        # Recursively heapify the affected subtree
        heapify "$1" "$heap_size" "$largest"
    fi
}

# Build a max heap from unsorted array
build_max_heap() {
    local -n build_arr=$1
    local size=${#build_arr[@]}

    # Start from last non-leaf node and heapify each node
    local last_non_leaf=$(((size / 2) - 1))

    local i=$last_non_leaf
    while [[ $i -ge 0 ]]; do
        heapify "$1" "$size" "$i"
        i=$((i - 1))
    done
}

# Heap sort main function
heap_sort() {
    local -n hs_arr=$1
    local size=${#hs_arr[@]}

    if (( size <= 1 )); then
        return
    fi

    # Build max heap
    build_max_heap "$1"

    # Extract elements one by one from heap
    local i=$((size - 1))
    while [[ $i -gt 0 ]]; do
        # Move current root to end
        swap "$1" 0 "$i"

        # Call heapify on the reduced heap
        heapify "$1" "$i" 0

        i=$((i - 1))
    done
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
    echo "Running Heap Sort tests..."
    local failed=0

    # Test 1: Basic array
    local arr1=(4 2 7 1 9 3 6 5)
    heap_sort arr1
    if ! is_sorted arr1; then
        echo "FAIL: Test 1 - array not sorted" >&2
        failed=$((failed + 1))
    fi
    if [[ "${arr1[0]}" != "1" ]] || [[ "${arr1[7]}" != "9" ]]; then
        echo "FAIL: Test 1 - incorrect values" >&2
        failed=$((failed + 1))
    fi

    # Test 2: Already sorted
    # shellcheck disable=SC2034
    local arr2=(1 2 3 4 5)
    heap_sort arr2
    if ! is_sorted arr2; then
        echo "FAIL: Test 2 - already sorted" >&2
        failed=$((failed + 1))
    fi

    # Test 3: Reverse sorted
    local arr3=(8 7 6 5 4 3 2 1)
    heap_sort arr3
    if ! is_sorted arr3; then
        echo "FAIL: Test 3 - reverse sorted" >&2
        failed=$((failed + 1))
    fi
    if [[ "${arr3[0]}" != "1" ]] || [[ "${arr3[7]}" != "8" ]]; then
        echo "FAIL: Test 3 - incorrect values" >&2
        failed=$((failed + 1))
    fi

    # Test 4: Single element
    local arr4=(42)
    heap_sort arr4
    if [[ "${arr4[0]}" != "42" ]]; then
        echo "FAIL: Test 4 - single element" >&2
        failed=$((failed + 1))
    fi

    # Test 5: Two elements
    local arr5=(2 1)
    heap_sort arr5
    if [[ "${arr5[0]}" != "1" ]] || [[ "${arr5[1]}" != "2" ]]; then
        echo "FAIL: Test 5 - two elements" >&2
        failed=$((failed + 1))
    fi

    # Test 6: All same elements
    # shellcheck disable=SC2034
    local arr6=(5 5 5 5 5 5 5)
    heap_sort arr6
    if ! is_sorted arr6; then
        echo "FAIL: Test 6 - all same" >&2
        failed=$((failed + 1))
    fi

    # Test 7: With duplicates
    local arr7=(3 1 4 1 5 9 2 6 5 3)
    heap_sort arr7
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
    $0 sort 4 2 7 1 9 3 6 5
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
            heap_sort arr
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
            heap_sort arr
            echo -n "After:  "
            print_array arr
            ;;
    esac
}

# Run main if executed (not sourced)
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
