/**
 * Merge Sort Algorithm
 *
 * Part of rosetta-ruchy Tier 0 implementations for transpiler validation
 * Sister project: decy (https://github.com/paiml/decy)
 *
 * Time Complexity: O(n log n)
 * Space Complexity: O(n)
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

/**
 * Merge two sorted subarrays arr[left..mid] and arr[mid+1..right]
 */
void merge(int arr[], int left, int mid, int right) {
    int left_size = mid - left + 1;
    int right_size = right - mid;

    // Create temporary arrays
    int *left_arr = malloc(left_size * sizeof(int));
    int *right_arr = malloc(right_size * sizeof(int));

    if (left_arr == NULL || right_arr == NULL) {
        fprintf(stderr, "Error: Memory allocation failed\n");
        free(left_arr);
        free(right_arr);
        return;
    }

    // Copy data to temporary arrays
    for (int i = 0; i < left_size; i++) {
        left_arr[i] = arr[left + i];
    }
    for (int j = 0; j < right_size; j++) {
        right_arr[j] = arr[mid + 1 + j];
    }

    // Merge the temporary arrays back into arr[left..right]
    int i = 0;  // Initial index of left subarray
    int j = 0;  // Initial index of right subarray
    int k = left;  // Initial index of merged array

    while (i < left_size && j < right_size) {
        if (left_arr[i] <= right_arr[j]) {
            arr[k] = left_arr[i];
            i++;
        } else {
            arr[k] = right_arr[j];
            j++;
        }
        k++;
    }

    // Copy remaining elements of left_arr, if any
    while (i < left_size) {
        arr[k] = left_arr[i];
        i++;
        k++;
    }

    // Copy remaining elements of right_arr, if any
    while (j < right_size) {
        arr[k] = right_arr[j];
        j++;
        k++;
    }

    free(left_arr);
    free(right_arr);
}

/**
 * Recursive mergesort implementation
 */
void mergesort_recursive(int arr[], int left, int right) {
    if (left < right) {
        int mid = left + (right - left) / 2;

        // Sort first and second halves
        mergesort_recursive(arr, left, mid);
        mergesort_recursive(arr, mid + 1, right);

        // Merge the sorted halves
        merge(arr, left, mid, right);
    }
}

/**
 * Merge sort main entry point
 */
void mergesort(int arr[], int size) {
    if (size > 1) {
        mergesort_recursive(arr, 0, size - 1);
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
    printf("Running Merge Sort tests...\n");

    // Test 1: Basic array
    int arr1[] = {64, 34, 25, 12, 22, 11, 90, 88};
    int size1 = sizeof(arr1) / sizeof(arr1[0]);
    mergesort(arr1, size1);
    assert(is_sorted(arr1, size1));
    assert(arr1[0] == 11 && arr1[7] == 90);

    // Test 2: Already sorted
    int arr2[] = {1, 2, 3, 4, 5};
    int size2 = sizeof(arr2) / sizeof(arr2[0]);
    mergesort(arr2, size2);
    assert(is_sorted(arr2, size2));

    // Test 3: Reverse sorted
    int arr3[] = {5, 4, 3, 2, 1};
    int size3 = sizeof(arr3) / sizeof(arr3[0]);
    mergesort(arr3, size3);
    assert(is_sorted(arr3, size3));
    assert(arr3[0] == 1 && arr3[4] == 5);

    // Test 4: Single element
    int arr4[] = {42};
    mergesort(arr4, 1);
    assert(arr4[0] == 42);

    // Test 5: Two elements
    int arr5[] = {2, 1};
    int size5 = sizeof(arr5) / sizeof(arr5[0]);
    mergesort(arr5, size5);
    assert(arr5[0] == 1 && arr5[1] == 2);

    // Test 6: All same elements
    int arr6[] = {7, 7, 7, 7, 7};
    int size6 = sizeof(arr6) / sizeof(arr6[0]);
    mergesort(arr6, size6);
    assert(is_sorted(arr6, size6));

    // Test 7: With duplicates
    int arr7[] = {3, 1, 4, 1, 5, 9, 2, 6};
    int size7 = sizeof(arr7) / sizeof(arr7[0]);
    mergesort(arr7, size7);
    assert(is_sorted(arr7, size7));
    assert(arr7[0] == 1 && arr7[1] == 1);

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
        printf("\nExample: %s 64 34 25 12 22 11 90 88\n", argv[0]);
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

    mergesort(arr, size);

    printf("After:  ");
    print_array(arr, size);

    free(arr);
    return 0;
}
