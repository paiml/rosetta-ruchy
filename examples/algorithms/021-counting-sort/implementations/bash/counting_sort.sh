#!/usr/bin/env bash
#
# Counting Sort Algorithm
#
# Part of rosetta-ruchy Tier 0 implementations for transpiler validation
# Sister project: bashrs (https://github.com/paiml/bashrs)
#
# Time Complexity: O(n + k) where k is range
# Space Complexity: O(k)
#

set -euo pipefail

# Find maximum element in array
find_max() {
    # shellcheck disable=SC2178
    local -n array_ref=$1
    local max=${array_ref[0]}

    for elem in "${array_ref[@]}"; do
        if (( elem > max )); then
            max=$elem
        fi
    done

    echo "$max"
}

# Counting sort implementation
# Sorts array in-place
counting_sort() {
    # shellcheck disable=SC2178
    local -n input_arr=$1
    local size=${#input_arr[@]}

    if (( size <= 1 )); then
        return 0
    fi

    # Find maximum element
    local max=${input_arr[0]}
    for elem in "${input_arr[@]}"; do
        if (( elem > max )); then
            max=$elem
        fi
    done

    # Create sparse count array (Bash is slow with large dense arrays)
    local -A count

    # Store count of each element
    for elem in "${input_arr[@]}"; do
        count[$elem]=$((${count[$elem]:-0} + 1))
    done

    # Get sorted unique values (more efficient in Bash than full range iteration)
    local -a sorted_keys
    mapfile -t sorted_keys < <(printf '%s\n' "${!count[@]}" | sort -n)

    # Reconstruct sorted array
    local idx=0
    for key in "${sorted_keys[@]}"; do
        local cnt=${count[$key]}
        local j=0
        while [[ $j -lt $cnt ]]; do
            input_arr[idx]=$key
            idx=$((idx + 1))
            j=$((j + 1))
        done
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
    echo "Running Counting Sort tests..."
    local failed=0

    # Test 1: Basic array
    local arr1=(4 2 2 8 3 3 1)
    counting_sort arr1
    if ! is_sorted arr1; then
        echo "FAIL: Test 1 - array not sorted" >&2
        ((failed++))
    fi
    if [[ "${arr1[0]}" != "1" ]] || [[ "${arr1[1]}" != "2" ]]; then
        echo "FAIL: Test 1 - incorrect values" >&2
        ((failed++))
    fi

    # Test 2: Already sorted
    # shellcheck disable=SC2034
    local arr2=(1 2 3 4 5)
    counting_sort arr2
    if ! is_sorted arr2; then
        echo "FAIL: Test 2 - already sorted" >&2
        ((failed++))
    fi

    # Test 3: Reverse sorted
    local arr3=(5 4 3 2 1)
    counting_sort arr3
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
    counting_sort arr4
    if [[ "${arr4[0]}" != "42" ]]; then
        echo "FAIL: Test 4 - single element" >&2
        ((failed++))
    fi

    # Test 5: All same elements
    # shellcheck disable=SC2034
    local arr5=(7 7 7 7 7)
    counting_sort arr5
    if ! is_sorted arr5; then
        echo "FAIL: Test 5 - all same" >&2
        ((failed++))
    fi

    # Test 6: With zeros
    local arr6=(0 5 2 0 3 0 1)
    counting_sort arr6
    if ! is_sorted arr6; then
        echo "FAIL: Test 6 - with zeros" >&2
        ((failed++))
    fi
    if [[ "${arr6[0]}" != "0" ]] || [[ "${arr6[1]}" != "0" ]]; then
        echo "FAIL: Test 6 - incorrect values" >&2
        ((failed++))
    fi

    # Test 7: Small range (Bash counting sort is slow, keep ranges small)
    local arr7=(15 5 20 1 10)
    counting_sort arr7
    if ! is_sorted arr7; then
        echo "FAIL: Test 7 - small range" >&2
        ((failed++))
    fi
    if [[ "${arr7[0]}" != "1" ]] || [[ "${arr7[4]}" != "20" ]]; then
        echo "FAIL: Test 7 - incorrect values" >&2
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
    sort <num1> <num2> ...   Sort non-negative integers
    test                     Run test suite
    help                     Show this help

Examples:
    $0 sort 4 2 2 8 3 3 1
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
                    echo "Error: Counting sort only works with non-negative integers" >&2
                    exit 1
                fi
            done

            local arr=("$@")
            echo -n "Before: "
            print_array arr
            counting_sort arr
            echo -n "After:  "
            print_array arr
            ;;
        help|--help|-h)
            print_usage
            ;;
        *)
            # Assume numbers for sorting
            local arr=("$command" "$@")

            # Validate non-negative integers
            for num in "${arr[@]}"; do
                if (( num < 0 )); then
                    echo "Error: Counting sort only works with non-negative integers" >&2
                    exit 1
                fi
            done

            echo -n "Before: "
            print_array arr
            counting_sort arr
            echo -n "After:  "
            print_array arr
            ;;
    esac
}

# Run main if executed (not sourced)
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
