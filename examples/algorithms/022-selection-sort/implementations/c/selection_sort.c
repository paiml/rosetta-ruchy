/**
 * Selection Sort Algorithm
 *
 * Part of rosetta-ruchy Tier 0 implementations for transpiler validation
 * Sister project: decy (https://github.com/paiml/decy)
 *
 * Time Complexity: O(n²)
 * Space Complexity: O(1)
 */

#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include <string.h>

/**
 * Swap two integers
 */
void swap(int *a, int *b) {
    int temp = *a;
    *a = *b;
    *b = temp;
}

/**
 * Selection sort implementation
 * Sorts array in-place in ascending order
 */
void selection_sort(int arr[], int size) {
    for (int i = 0; i < size - 1; i++) {
        int min_idx = i;

        // Find minimum element in unsorted portion
        for (int j = i + 1; j < size; j++) {
            if (arr[j] < arr[min_idx]) {
                min_idx = j;
            }
        }

        // Swap minimum with first unsorted element
        if (min_idx != i) {
            swap(&arr[i], &arr[min_idx]);
        }
    }
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
    printf("Running Selection Sort tests...\n");

    // Test 1: Basic array
    int arr1[] = {64, 25, 12, 22, 11};
    int size1 = sizeof(arr1) / sizeof(arr1[0]);
    selection_sort(arr1, size1);
    assert(is_sorted(arr1, size1));
    assert(arr1[0] == 11 && arr1[1] == 12 && arr1[2] == 22);

    // Test 2: Already sorted
    int arr2[] = {1, 2, 3, 4, 5};
    int size2 = sizeof(arr2) / sizeof(arr2[0]);
    selection_sort(arr2, size2);
    assert(is_sorted(arr2, size2));

    // Test 3: Reverse sorted
    int arr3[] = {5, 4, 3, 2, 1};
    int size3 = sizeof(arr3) / sizeof(arr3[0]);
    selection_sort(arr3, size3);
    assert(is_sorted(arr3, size3));
    assert(arr3[0] == 1 && arr3[4] == 5);

    // Test 4: Single element
    int arr4[] = {42};
    selection_sort(arr4, 1);
    assert(arr4[0] == 42);

    // Test 5: Two elements
    int arr5[] = {2, 1};
    selection_sort(arr5, 2);
    assert(arr5[0] == 1 && arr5[1] == 2);

    // Test 6: Duplicates
    int arr6[] = {3, 1, 4, 1, 5, 9, 2, 6};
    int size6 = sizeof(arr6) / sizeof(arr6[0]);
    selection_sort(arr6, size6);
    assert(is_sorted(arr6, size6));
    assert(arr6[0] == 1 && arr6[1] == 1);

    // Test 7: All same elements
    int arr7[] = {7, 7, 7, 7, 7};
    int size7 = sizeof(arr7) / sizeof(arr7[0]);
    selection_sort(arr7, size7);
    assert(is_sorted(arr7, size7));

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
        printf("\nExample: %s 64 25 12 22 11\n", argv[0]);
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
    }

    printf("Before: ");
    print_array(arr, size);

    selection_sort(arr, size);

    printf("After:  ");
    print_array(arr, size);

    free(arr);
    return 0;
}
