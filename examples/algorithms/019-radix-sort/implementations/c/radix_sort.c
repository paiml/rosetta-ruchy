/**
 * Radix Sort Algorithm (LSD - Least Significant Digit)
 *
 * Part of rosetta-ruchy Tier 0 implementations for transpiler validation
 * Sister project: decy (https://github.com/paiml/decy)
 *
 * Time Complexity: O(d * (n + k)) where d is digits, k is radix (base)
 * Space Complexity: O(n + k)
 *
 * Note: This implementation works with non-negative integers only
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

#define RADIX 10  // Base-10 radix sort

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
 * Counting sort by specific digit position (exp = 1, 10, 100, ...)
 */
void counting_sort_by_digit(int arr[], int size, int exp) {
    int *output = malloc(size * sizeof(int));
    int count[RADIX] = {0};

    if (output == NULL) {
        fprintf(stderr, "Error: Memory allocation failed\n");
        return;
    }

    // Count occurrences of each digit
    for (int i = 0; i < size; i++) {
        int digit = (arr[i] / exp) % RADIX;
        count[digit]++;
    }

    // Convert counts to cumulative counts (for stable sorting)
    for (int i = 1; i < RADIX; i++) {
        count[i] += count[i - 1];
    }

    // Build output array from right to left (for stability)
    for (int i = size - 1; i >= 0; i--) {
        int digit = (arr[i] / exp) % RADIX;
        count[digit]--;
        output[count[digit]] = arr[i];
    }

    // Copy output array back to original
    memcpy(arr, output, size * sizeof(int));

    free(output);
}

/**
 * LSD Radix Sort for non-negative integers
 */
void radix_sort(int arr[], int size) {
    if (size <= 1) {
        return;
    }

    // Find maximum to determine number of digits
    int max = find_max(arr, size);

    // Sort by each digit position (LSD to MSD)
    for (int exp = 1; max / exp > 0; exp *= RADIX) {
        counting_sort_by_digit(arr, size, exp);
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
    printf("Running Radix Sort tests...\n");

    // Test 1: Basic array
    int arr1[] = {170, 45, 75, 90, 802, 24, 2, 66};
    int size1 = sizeof(arr1) / sizeof(arr1[0]);
    radix_sort(arr1, size1);
    assert(is_sorted(arr1, size1));
    assert(arr1[0] == 2 && arr1[7] == 802);

    // Test 2: Already sorted
    int arr2[] = {1, 2, 3, 4, 5};
    int size2 = sizeof(arr2) / sizeof(arr2[0]);
    radix_sort(arr2, size2);
    assert(is_sorted(arr2, size2));

    // Test 3: Reverse sorted
    int arr3[] = {9, 8, 7, 6, 5, 4, 3, 2, 1, 0};
    int size3 = sizeof(arr3) / sizeof(arr3[0]);
    radix_sort(arr3, size3);
    assert(is_sorted(arr3, size3));
    assert(arr3[0] == 0 && arr3[9] == 9);

    // Test 4: Single element
    int arr4[] = {42};
    radix_sort(arr4, 1);
    assert(arr4[0] == 42);

    // Test 5: Two elements
    int arr5[] = {10, 5};
    int size5 = sizeof(arr5) / sizeof(arr5[0]);
    radix_sort(arr5, size5);
    assert(arr5[0] == 5 && arr5[1] == 10);

    // Test 6: All same elements
    int arr6[] = {7, 7, 7, 7, 7};
    int size6 = sizeof(arr6) / sizeof(arr6[0]);
    radix_sort(arr6, size6);
    assert(is_sorted(arr6, size6));

    // Test 7: With duplicates and varying digit counts
    int arr7[] = {329, 457, 657, 839, 436, 720, 355};
    int size7 = sizeof(arr7) / sizeof(arr7[0]);
    radix_sort(arr7, size7);
    assert(is_sorted(arr7, size7));
    assert(arr7[0] == 329 && arr7[6] == 839);

    // Test 8: Including zeros
    int arr8[] = {100, 0, 50, 0, 25, 0, 75};
    int size8 = sizeof(arr8) / sizeof(arr8[0]);
    radix_sort(arr8, size8);
    assert(is_sorted(arr8, size8));
    assert(arr8[0] == 0 && arr8[1] == 0 && arr8[2] == 0);

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
        printf("\nExample: %s 170 45 75 90 802 24 2 66\n", argv[0]);
        printf("\nNote: Only works with non-negative integers\n");
        return 1;
    }

    int size = argc - 1;
    int *arr = malloc(size * sizeof(int));

    if (arr == NULL) {
        fprintf(stderr, "Error: Memory allocation failed\n");
        return 1;
    }

    // Parse and validate array elements
    for (int i = 0; i < size; i++) {
        arr[i] = atoi(argv[i + 1]);
        if (arr[i] < 0) {
            fprintf(stderr, "Error: Radix sort only works with non-negative integers\n");
            free(arr);
            return 1;
        }
    }

    printf("Before: ");
    print_array(arr, size);

    radix_sort(arr, size);

    printf("After:  ");
    print_array(arr, size);

    free(arr);
    return 0;
}
