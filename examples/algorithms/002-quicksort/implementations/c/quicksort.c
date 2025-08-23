#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <stdbool.h>

// Function prototypes
void quicksort_inplace(int arr[], int size);
void quicksort_range(int arr[], int low, int high);
int partition(int arr[], int low, int high);

int* quicksort_functional(int arr[], int size);
int* merge_arrays(int* arr1, int size1, int* arr2, int size2, int* arr3, int size3);

void quicksort_three_way(int arr[], int size);
void three_way_partition_sort(int arr[], int low, int high);

bool is_sorted(int arr[], int size);
void print_array(int arr[], int size);
void copy_array(int dest[], int src[], int size);

// Comparison function for qsort
int compare_ints(const void *a, const void *b) {
    int ia = *(const int*)a;
    int ib = *(const int*)b;
    return (ia > ib) - (ia < ib);
}

// In-place quicksort implementation
void quicksort_inplace(int arr[], int size) {
    if (size > 1) {
        quicksort_range(arr, 0, size - 1);
    }
}

void quicksort_range(int arr[], int low, int high) {
    if (low < high) {
        int pivot_index = partition(arr, low, high);
        
        if (pivot_index > 0) {
            quicksort_range(arr, low, pivot_index - 1);
        }
        if (pivot_index + 1 <= high) {
            quicksort_range(arr, pivot_index + 1, high);
        }
    }
}

int partition(int arr[], int low, int high) {
    int pivot = arr[high];
    int i = low;
    
    for (int j = low; j < high; j++) {
        if (arr[j] <= pivot) {
            // Swap arr[i] and arr[j]
            int temp = arr[i];
            arr[i] = arr[j];
            arr[j] = temp;
            i++;
        }
    }
    
    // Swap arr[i] and arr[high]
    int temp = arr[i];
    arr[i] = arr[high];
    arr[high] = temp;
    
    return i;
}

// Functional quicksort implementation
int* quicksort_functional(int arr[], int size) {
    if (size <= 1) {
        int* result = malloc(size * sizeof(int));
        if (result == NULL) return NULL;
        memcpy(result, arr, size * sizeof(int));
        return result;
    }
    
    int pivot = arr[size / 2];
    
    // Count elements in each partition
    int less_count = 0, equal_count = 0, greater_count = 0;
    for (int i = 0; i < size; i++) {
        if (arr[i] < pivot) less_count++;
        else if (arr[i] == pivot) equal_count++;
        else greater_count++;
    }
    
    // Allocate arrays for partitions
    int* less = malloc(less_count * sizeof(int));
    int* equal = malloc(equal_count * sizeof(int));
    int* greater = malloc(greater_count * sizeof(int));
    
    if ((less_count > 0 && less == NULL) || 
        (equal_count > 0 && equal == NULL) || 
        (greater_count > 0 && greater == NULL)) {
        free(less);
        free(equal);
        free(greater);
        return NULL;
    }
    
    // Fill partitions
    int less_idx = 0, equal_idx = 0, greater_idx = 0;
    for (int i = 0; i < size; i++) {
        if (arr[i] < pivot) {
            less[less_idx++] = arr[i];
        } else if (arr[i] == pivot) {
            equal[equal_idx++] = arr[i];
        } else {
            greater[greater_idx++] = arr[i];
        }
    }
    
    // Recursively sort partitions
    int* sorted_less = quicksort_functional(less, less_count);
    int* sorted_greater = quicksort_functional(greater, greater_count);
    
    // Merge results
    int* result = merge_arrays(sorted_less, less_count, equal, equal_count, 
                              sorted_greater, greater_count);
    
    // Cleanup
    free(less);
    free(equal);
    free(greater);
    if (sorted_less != less) free(sorted_less);
    if (sorted_greater != greater) free(sorted_greater);
    
    return result;
}

int* merge_arrays(int* arr1, int size1, int* arr2, int size2, int* arr3, int size3) {
    int total_size = size1 + size2 + size3;
    int* result = malloc(total_size * sizeof(int));
    if (result == NULL) return NULL;
    
    int idx = 0;
    
    // Copy first array
    for (int i = 0; i < size1; i++) {
        result[idx++] = arr1[i];
    }
    
    // Copy second array
    for (int i = 0; i < size2; i++) {
        result[idx++] = arr2[i];
    }
    
    // Copy third array
    for (int i = 0; i < size3; i++) {
        result[idx++] = arr3[i];
    }
    
    return result;
}

// Three-way quicksort implementation
void quicksort_three_way(int arr[], int size) {
    if (size <= 1) return;
    three_way_partition_sort(arr, 0, size - 1);
}

void three_way_partition_sort(int arr[], int low, int high) {
    if (low >= high) return;
    
    int pivot = arr[low];
    int lt = low;
    int gt = high;
    int i = low + 1;
    
    while (i <= gt) {
        if (arr[i] < pivot) {
            // Swap arr[i] and arr[lt]
            int temp = arr[i];
            arr[i] = arr[lt];
            arr[lt] = temp;
            lt++;
            i++;
        } else if (arr[i] > pivot) {
            // Swap arr[i] and arr[gt]
            int temp = arr[i];
            arr[i] = arr[gt];
            arr[gt] = temp;
            gt--;
        } else {
            i++;
        }
    }
    
    if (lt > 0) {
        three_way_partition_sort(arr, low, lt - 1);
    }
    three_way_partition_sort(arr, gt + 1, high);
}

// Utility functions
bool is_sorted(int arr[], int size) {
    for (int i = 0; i < size - 1; i++) {
        if (arr[i] > arr[i + 1]) {
            return false;
        }
    }
    return true;
}

void print_array(int arr[], int size) {
    printf("[");
    for (int i = 0; i < size; i++) {
        printf("%d", arr[i]);
        if (i < size - 1) printf(", ");
    }
    printf("]");
}

void copy_array(int dest[], int src[], int size) {
    for (int i = 0; i < size; i++) {
        dest[i] = src[i];
    }
}

// Test function
void run_tests() {
    printf("Running Quicksort Tests...\n\n");
    
    // Test cases
    int test1[] = {};
    int test2[] = {42};
    int test3[] = {3, 1, 4, 1, 5, 9, 2, 6};
    int test4[] = {5, 4, 3, 2, 1};
    int test5[] = {1, 2, 3, 4, 5};
    int test6[] = {5, 5, 5, 5, 5};
    
    int* tests[] = {test1, test2, test3, test4, test5, test6};
    int sizes[] = {0, 1, 8, 5, 5, 5};
    
    for (int t = 0; t < 6; t++) {
        printf("Test case %d: ", t + 1);
        print_array(tests[t], sizes[t]);
        printf("\n");
        
        if (sizes[t] > 0) {
            // In-place test
            int* arr1 = malloc(sizes[t] * sizeof(int));
            copy_array(arr1, tests[t], sizes[t]);
            quicksort_inplace(arr1, sizes[t]);
            printf("  In-place:    ");
            print_array(arr1, sizes[t]);
            printf("\n");
            free(arr1);
            
            // Functional test
            int* arr2 = quicksort_functional(tests[t], sizes[t]);
            printf("  Functional:  ");
            print_array(arr2, sizes[t]);
            printf("\n");
            free(arr2);
            
            // Three-way test
            int* arr3 = malloc(sizes[t] * sizeof(int));
            copy_array(arr3, tests[t], sizes[t]);
            quicksort_three_way(arr3, sizes[t]);
            printf("  Three-way:   ");
            print_array(arr3, sizes[t]);
            printf("\n");
            free(arr3);
        } else {
            printf("  (Empty array - skipping detailed tests)\n");
        }
        
        printf("\n");
    }
}

// Performance benchmark
void benchmark() {
    printf("Performance demonstration with large array:\n");
    
    const int size = 10000;
    int* large_array = malloc(size * sizeof(int));
    
    // Generate test data
    srand(42);
    for (int i = 0; i < size; i++) {
        large_array[i] = (i * 37 + 11) % 1000;
    }
    
    printf("  Array size: %d\n", size);
    
    // Benchmark in-place quicksort
    int* arr_copy = malloc(size * sizeof(int));
    copy_array(arr_copy, large_array, size);
    
    clock_t start = clock();
    quicksort_inplace(arr_copy, size);
    clock_t end = clock();
    double duration = ((double)(end - start)) / CLOCKS_PER_SEC;
    printf("  In-place time: %.4f seconds\n", duration);
    free(arr_copy);
    
    // Benchmark functional quicksort
    start = clock();
    int* sorted_functional = quicksort_functional(large_array, size);
    end = clock();
    duration = ((double)(end - start)) / CLOCKS_PER_SEC;
    printf("  Functional time: %.4f seconds\n", duration);
    free(sorted_functional);
    
    // Benchmark standard library qsort
    arr_copy = malloc(size * sizeof(int));
    copy_array(arr_copy, large_array, size);
    
    start = clock();
    qsort(arr_copy, size, sizeof(int), compare_ints);
    end = clock();
    duration = ((double)(end - start)) / CLOCKS_PER_SEC;
    printf("  qsort time: %.4f seconds\n", duration);
    free(arr_copy);
    
    free(large_array);
}

int main() {
    printf("Quicksort Implementations Demo\n\n");
    
    run_tests();
    benchmark();
    
    return 0;
}