/**
 * Counting Sort Algorithm
 *
 * Part of rosetta-ruchy Tier 0 implementations for transpiler validation
 * Sister project: decy (https://github.com/paiml/decy)
 *
 * Time Complexity: O(n + k) where k is range
 * Space Complexity: O(k)
 */

#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include <string.h>

/**
 * Find maximum element in array
 */
int find_max(const int arr[], int size) {
    int max = arr[0];
    for (int i = 1; i < size; i++) {
        if (arr[i] > max) {
            max = arr[i];
        }
    }
    return max;
}

/**
 * Counting sort implementation
 * Sorts array of non-negative integers
 */
void counting_sort(int arr[], int size) {
    if (size <= 1) {
        return;
    }

    // Find maximum element to determine range
    int max = find_max(arr, size);

    // Create count array and initialize to 0
    int *count = calloc(max + 1, sizeof(int));
    if (count == NULL) {
        fprintf(stderr, "Error: Memory allocation failed\n");
        return;
    }

    // Store count of each element
    for (int i = 0; i < size; i++) {
        count[arr[i]]++;
    }

    // Modify count array to store cumulative counts
    for (int i = 1; i <= max; i++) {
        count[i] += count[i - 1];
    }

    // Build output array
    int *output = malloc(size * sizeof(int));
    if (output == NULL) {
        fprintf(stderr, "Error: Memory allocation failed\n");
        free(count);
        return;
    }

    // Place elements in sorted order (iterate backwards for stability)
    for (int i = size - 1; i >= 0; i--) {
        output[count[arr[i]] - 1] = arr[i];
        count[arr[i]]--;
    }

    // Copy sorted elements back to original array
    memcpy(arr, output, size * sizeof(int));

    free(count);
    free(output);
}

/**
 * Check if array is sorted
 */
int is_sorted(const int arr[], int size) {
    for (int i = 0; i < size - 1; i++) {
        if (arr[i] > arr[i + 1]) {
            return 0;
        }
    }
    return 1;
}

/**
 * Print array
 */
void print_array(const int arr[], int size) {
    printf("[");
    for (int i = 0; i < size; i++) {
        printf("%d", arr[i]);
        if (i < size - 1) {
            printf(", ");
        }
    }
    printf("]\n");
}

/**
 * Run test suite
 */
void run_tests(void) {
    printf("Running Counting Sort tests...\n");

    // Test 1: Basic array
    int arr1[] = {4, 2, 2, 8, 3, 3, 1};
    int size1 = sizeof(arr1) / sizeof(arr1[0]);
    counting_sort(arr1, size1);
    assert(is_sorted(arr1, size1));
    assert(arr1[0] == 1 && arr1[1] == 2 && arr1[2] == 2);

    // Test 2: Already sorted
    int arr2[] = {1, 2, 3, 4, 5};
    int size2 = sizeof(arr2) / sizeof(arr2[0]);
    counting_sort(arr2, size2);
    assert(is_sorted(arr2, size2));

    // Test 3: Reverse sorted
    int arr3[] = {5, 4, 3, 2, 1};
    int size3 = sizeof(arr3) / sizeof(arr3[0]);
    counting_sort(arr3, size3);
    assert(is_sorted(arr3, size3));
    assert(arr3[0] == 1 && arr3[4] == 5);

    // Test 4: Single element
    int arr4[] = {42};
    counting_sort(arr4, 1);
    assert(arr4[0] == 42);

    // Test 5: All same elements
    int arr5[] = {7, 7, 7, 7, 7};
    int size5 = sizeof(arr5) / sizeof(arr5[0]);
    counting_sort(arr5, size5);
    assert(is_sorted(arr5, size5));

    // Test 6: With zeros
    int arr6[] = {0, 5, 2, 0, 3, 0, 1};
    int size6 = sizeof(arr6) / sizeof(arr6[0]);
    counting_sort(arr6, size6);
    assert(is_sorted(arr6, size6));
    assert(arr6[0] == 0 && arr6[1] == 0 && arr6[2] == 0);

    // Test 7: Large range
    int arr7[] = {100, 5, 200, 1, 50};
    int size7 = sizeof(arr7) / sizeof(arr7[0]);
    counting_sort(arr7, size7);
    assert(is_sorted(arr7, size7));
    assert(arr7[0] == 1 && arr7[4] == 200);

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

    // Otherwise, parse array from command line
    if (argc < 2) {
        printf("Usage: %s <num1> <num2> <num3> ...\n", argv[0]);
        printf("       %s test\n", argv[0]);
        printf("\nExample: %s 4 2 2 8 3 3 1\n", argv[0]);
        printf("\nNote: Only works with non-negative integers\n");
        return 1;
    }

    int size = argc - 1;
    int *arr = malloc(size * sizeof(int));

    if (arr == NULL) {
        fprintf(stderr, "Error: Memory allocation failed\n");
        return 1;
    }

    // Parse array elements
    for (int i = 0; i < size; i++) {
        arr[i] = atoi(argv[i + 1]);
        if (arr[i] < 0) {
            fprintf(stderr, "Error: Counting sort only works with non-negative integers\n");
            free(arr);
            return 1;
        }
    }

    printf("Before: ");
    print_array(arr, size);

    counting_sort(arr, size);

    printf("After:  ");
    print_array(arr, size);

    free(arr);
    return 0;
}
