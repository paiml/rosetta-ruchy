#!/usr/bin/env bash
#
# Radix Sort Algorithm (LSD - Least Significant Digit)
#
# Part of rosetta-ruchy Tier 0 implementations for transpiler validation
# Sister project: bashrs (https://github.com/paiml/bashrs)
#
# Time Complexity: O(d * (n + k)) where d is digits, k is radix (base)
# Space Complexity: O(n + k)
#
# Note: This implementation works with non-negative integers only
#

set -euo pipefail

RADIX=10  # Base-10 radix sort

# Find maximum element in array
find_max() {
    local -n fm_arr=$1
    local max=${fm_arr[0]}

    for elem in "${fm_arr[@]}"; do
        if (( elem > max )); then
            max=$elem
        fi
    done

    echo "$max"
}

# Counting sort by specific digit position (exp = 1, 10, 100, ...)
counting_sort_by_digit() {
    local -n csd_arr=$1
    local exp=$2
    local size=${#csd_arr[@]}

    # Create output and count arrays
    local -a output
    local -a count

    # Initialize count array
    local i=0
    while [[ $i -lt $RADIX ]]; do
        count[i]=0
        i=$((i + 1))
    done

    # Count occurrences of each digit
    i=0
    while [[ $i -lt $size ]]; do
        local digit=$(( (csd_arr[i] / exp) % RADIX ))
        count[digit]=$((count[digit] + 1))
        i=$((i + 1))
    done

    # Convert counts to cumulative counts (for stable sorting)
    i=1
    while [[ $i -lt $RADIX ]]; do
        count[i]=$((count[i] + count[i - 1]))
        i=$((i + 1))
    done

    # Build output array from right to left (for stability)
    i=$((size - 1))
    while [[ $i -ge 0 ]]; do
        local digit=$(( (csd_arr[i] / exp) % RADIX ))
        count[digit]=$((count[digit] - 1))
        output[count[digit]]=${csd_arr[i]}
        i=$((i - 1))
    done

    # Copy output array back to original
    i=0
    while [[ $i -lt $size ]]; do
        csd_arr[i]=${output[i]}
        i=$((i + 1))
    done
}

# LSD Radix Sort for non-negative integers
radix_sort() {
    local -n rs_arr=$1
    local size=${#rs_arr[@]}

    if (( size <= 1 )); then
        return
    fi

    # Find maximum to determine number of digits
    local max
    max=$(find_max "$1")

    # Sort by each digit position (LSD to MSD)
    local exp=1
    while [[ $((max / exp)) -gt 0 ]]; do
        counting_sort_by_digit "$1" "$exp"
        exp=$((exp * RADIX))
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
    echo "Running Radix Sort tests..."
    local failed=0

    # Test 1: Basic array
    local arr1=(170 45 75 90 802 24 2 66)
    radix_sort arr1
    if ! is_sorted arr1; then
        echo "FAIL: Test 1 - array not sorted" >&2
        failed=$((failed + 1))
    fi
    if [[ "${arr1[0]}" != "2" ]] || [[ "${arr1[7]}" != "802" ]]; then
        echo "FAIL: Test 1 - incorrect values" >&2
        failed=$((failed + 1))
    fi

    # Test 2: Already sorted
    # shellcheck disable=SC2034
    local arr2=(1 2 3 4 5)
    radix_sort arr2
    if ! is_sorted arr2; then
        echo "FAIL: Test 2 - already sorted" >&2
        failed=$((failed + 1))
    fi

    # Test 3: Reverse sorted
    local arr3=(9 8 7 6 5 4 3 2 1 0)
    radix_sort arr3
    if ! is_sorted arr3; then
        echo "FAIL: Test 3 - reverse sorted" >&2
        failed=$((failed + 1))
    fi
    if [[ "${arr3[0]}" != "0" ]] || [[ "${arr3[9]}" != "9" ]]; then
        echo "FAIL: Test 3 - incorrect values" >&2
        failed=$((failed + 1))
    fi

    # Test 4: Single element
    local arr4=(42)
    radix_sort arr4
    if [[ "${arr4[0]}" != "42" ]]; then
        echo "FAIL: Test 4 - single element" >&2
        failed=$((failed + 1))
    fi

    # Test 5: Two elements
    local arr5=(10 5)
    radix_sort arr5
    if [[ "${arr5[0]}" != "5" ]] || [[ "${arr5[1]}" != "10" ]]; then
        echo "FAIL: Test 5 - two elements" >&2
        failed=$((failed + 1))
    fi

    # Test 6: All same elements
    # shellcheck disable=SC2034
    local arr6=(7 7 7 7 7)
    radix_sort arr6
    if ! is_sorted arr6; then
        echo "FAIL: Test 6 - all same" >&2
        failed=$((failed + 1))
    fi

    # Test 7: With duplicates and varying digit counts
    local arr7=(329 457 657 839 436 720 355)
    radix_sort arr7
    if ! is_sorted arr7; then
        echo "FAIL: Test 7 - with duplicates" >&2
        failed=$((failed + 1))
    fi
    if [[ "${arr7[0]}" != "329" ]] || [[ "${arr7[6]}" != "839" ]]; then
        echo "FAIL: Test 7 - incorrect values" >&2
        failed=$((failed + 1))
    fi

    # Test 8: Including zeros
    local arr8=(100 0 50 0 25 0 75)
    radix_sort arr8
    if ! is_sorted arr8; then
        echo "FAIL: Test 8 - with zeros" >&2
        failed=$((failed + 1))
    fi
    if [[ "${arr8[0]}" != "0" ]] || [[ "${arr8[1]}" != "0" ]] || [[ "${arr8[2]}" != "0" ]]; then
        echo "FAIL: Test 8 - incorrect values" >&2
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
    sort <num1> <num2> ...   Sort non-negative integers
    test                     Run test suite
    help                     Show this help

Examples:
    $0 sort 170 45 75 90 802 24 2 66
    $0 test

Note: Only works with non-negative integers

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

            # Validate non-negative integers
            for num in "$@"; do
                if (( num < 0 )); then
                    echo "Error: Radix sort only works with non-negative integers" >&2
                    exit 1
                fi
            done

            local arr=("$@")
            echo -n "Before: "
            print_array arr
            radix_sort arr
            echo -n "After:  "
            print_array arr
            ;;
        help|--help|-h)
            print_usage
            ;;
        *)
            # Assume numbers for sorting
            # Validate non-negative integers
            for num in "$command" "$@"; do
                if (( num < 0 )); then
                    echo "Error: Radix sort only works with non-negative integers" >&2
                    exit 1
                fi
            done

            # shellcheck disable=SC2034
            local arr=("$command" "$@")
            echo -n "Before: "
            print_array arr
            radix_sort arr
            echo -n "After:  "
            print_array arr
            ;;
    esac
}

# Run main if executed (not sourced)
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
