/**
 * Heap Sort Algorithm
 *
 * Part of rosetta-ruchy Tier 0 implementations for transpiler validation
 * Sister project: decy (https://github.com/paiml/decy)
 *
 * Time Complexity: O(n log n)
 * Space Complexity: O(1) - in-place sorting
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

/**
 * Swap two elements in array
 */
void swap(int arr[], int i, int j) {
    int temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}

/**
 * Heapify a subtree rooted at index i
 * heap_size is size of heap
 */
void heapify(int arr[], int heap_size, int i) {
    int largest = i;
    int left = 2 * i + 1;
    int right = 2 * i + 2;

    // If left child is larger than root
    if (left < heap_size && arr[left] > arr[largest]) {
        largest = left;
    }

    // If right child is larger than largest so far
    if (right < heap_size && arr[right] > arr[largest]) {
        largest = right;
    }

    // If largest is not root
    if (largest != i) {
        swap(arr, i, largest);

        // Recursively heapify the affected subtree
        heapify(arr, heap_size, largest);
    }
}

/**
 * Build a max heap from unsorted array
 */
void build_max_heap(int arr[], int size) {
    // Start from last non-leaf node and heapify each node
    int last_non_leaf = (size / 2) - 1;

    for (int i = last_non_leaf; i >= 0; i--) {
        heapify(arr, size, i);
    }
}

/**
 * Heap sort main function
 */
void heap_sort(int arr[], int size) {
    if (size <= 1) {
        return;
    }

    // Build max heap
    build_max_heap(arr, size);

    // Extract elements one by one from heap
    for (int i = size - 1; i > 0; i--) {
        // Move current root to end
        swap(arr, 0, i);

        // Call heapify on the reduced heap
        heapify(arr, i, 0);
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
    printf("Running Heap Sort tests...\n");

    // Test 1: Basic array
    int arr1[] = {4, 2, 7, 1, 9, 3, 6, 5};
    int size1 = sizeof(arr1) / sizeof(arr1[0]);
    heap_sort(arr1, size1);
    assert(is_sorted(arr1, size1));
    assert(arr1[0] == 1 && arr1[7] == 9);

    // Test 2: Already sorted
    int arr2[] = {1, 2, 3, 4, 5};
    int size2 = sizeof(arr2) / sizeof(arr2[0]);
    heap_sort(arr2, size2);
    assert(is_sorted(arr2, size2));

    // Test 3: Reverse sorted
    int arr3[] = {8, 7, 6, 5, 4, 3, 2, 1};
    int size3 = sizeof(arr3) / sizeof(arr3[0]);
    heap_sort(arr3, size3);
    assert(is_sorted(arr3, size3));
    assert(arr3[0] == 1 && arr3[7] == 8);

    // Test 4: Single element
    int arr4[] = {42};
    heap_sort(arr4, 1);
    assert(arr4[0] == 42);

    // Test 5: Two elements
    int arr5[] = {2, 1};
    int size5 = sizeof(arr5) / sizeof(arr5[0]);
    heap_sort(arr5, size5);
    assert(arr5[0] == 1 && arr5[1] == 2);

    // Test 6: All same elements
    int arr6[] = {5, 5, 5, 5, 5, 5, 5};
    int size6 = sizeof(arr6) / sizeof(arr6[0]);
    heap_sort(arr6, size6);
    assert(is_sorted(arr6, size6));

    // Test 7: With duplicates
    int arr7[] = {3, 1, 4, 1, 5, 9, 2, 6, 5, 3};
    int size7 = sizeof(arr7) / sizeof(arr7[0]);
    heap_sort(arr7, size7);
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
        printf("\nExample: %s 4 2 7 1 9 3 6 5\n", argv[0]);
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

    heap_sort(arr, size);

    printf("After:  ");
    print_array(arr, size);

    free(arr);
    return 0;
}
