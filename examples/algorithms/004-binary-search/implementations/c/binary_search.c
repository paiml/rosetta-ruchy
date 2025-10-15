/**
 * Binary Search Algorithm
 *
 * Part of rosetta-ruchy Tier 0 implementations for transpiler validation
 * Sister project: decy (https://github.com/paiml/decy)
 *
 * Time Complexity: O(log n)
 * Space Complexity: O(1)
 */

#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include <string.h>

/**
 * Binary search implementation
 * Returns index of target in sorted array, or -1 if not found
 */
int binary_search(const int arr[], int size, int target) {
    int left = 0;
    int right = size - 1;

    while (left <= right) {
        int mid = left + (right - left) / 2;

        if (arr[mid] == target) {
            return mid;
        } else if (arr[mid] < target) {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    return -1;  // Not found
}

/**
 * Recursive binary search implementation
 */
int binary_search_recursive_helper(const int arr[], int left, int right, int target) {
    if (left > right) {
        return -1;
    }

    int mid = left + (right - left) / 2;

    if (arr[mid] == target) {
        return mid;
    } else if (arr[mid] < target) {
        return binary_search_recursive_helper(arr, mid + 1, right, target);
    } else {
        return binary_search_recursive_helper(arr, left, mid - 1, target);
    }
}

int binary_search_recursive(const int arr[], int size, int target) {
    return binary_search_recursive_helper(arr, 0, size - 1, target);
}

/**
 * Run test suite
 */
void run_tests(void) {
    printf("Running Binary Search tests...\n");

    // Test case 1: Simple array
    int arr1[] = {1, 3, 5, 7, 9, 11, 13, 15, 17, 19};
    int size1 = sizeof(arr1) / sizeof(arr1[0]);

    assert(binary_search(arr1, size1, 7) == 3);
    assert(binary_search(arr1, size1, 1) == 0);
    assert(binary_search(arr1, size1, 19) == 9);
    assert(binary_search(arr1, size1, 9) == 4);
    assert(binary_search(arr1, size1, 20) == -1);
    assert(binary_search(arr1, size1, 0) == -1);
    assert(binary_search(arr1, size1, 8) == -1);

    // Test recursive version
    assert(binary_search_recursive(arr1, size1, 7) == 3);
    assert(binary_search_recursive(arr1, size1, 1) == 0);
    assert(binary_search_recursive(arr1, size1, 19) == 9);
    assert(binary_search_recursive(arr1, size1, 20) == -1);

    // Test case 2: Single element
    int arr2[] = {42};
    assert(binary_search(arr2, 1, 42) == 0);
    assert(binary_search(arr2, 1, 43) == -1);

    // Test case 3: Two elements
    int arr3[] = {10, 20};
    assert(binary_search(arr3, 2, 10) == 0);
    assert(binary_search(arr3, 2, 20) == 1);
    assert(binary_search(arr3, 2, 15) == -1);

    // Test case 4: Large array
    int arr4[1000];
    for (int i = 0; i < 1000; i++) {
        arr4[i] = i * 2;  // 0, 2, 4, 6, ...
    }
    assert(binary_search(arr4, 1000, 500) == 250);
    assert(binary_search(arr4, 1000, 0) == 0);
    assert(binary_search(arr4, 1000, 1998) == 999);
    assert(binary_search(arr4, 1000, 501) == -1);

    printf("✓ All tests passed\n");
}

/**
 * Main entry point
 */
int main(int argc, char *argv[]) {
    // If "test" argument, run tests
    if (argc == 2 && strcmp(argv[1], "test") == 0) {
        run_tests();
        return 0;
    }

    // Otherwise, expect array elements and target
    if (argc < 3) {
        printf("Usage: %s <target> <element1> <element2> ...\n", argv[0]);
        printf("       %s test\n", argv[0]);
        printf("\nExample: %s 7 1 3 5 7 9 11 13\n", argv[0]);
        return 1;
    }

    int target = atoi(argv[1]);
    int size = argc - 2;
    int *arr = malloc(size * sizeof(int));

    if (arr == NULL) {
        fprintf(stderr, "Error: Memory allocation failed\n");
        return 1;
    }

    // Parse array elements
    for (int i = 0; i < size; i++) {
        arr[i] = atoi(argv[i + 2]);
    }

    // Perform search
    int result = binary_search(arr, size, target);

    if (result != -1) {
        printf("Found at index: %d\n", result);
    } else {
        printf("Not found\n");
    }

    free(arr);
    return 0;
}
