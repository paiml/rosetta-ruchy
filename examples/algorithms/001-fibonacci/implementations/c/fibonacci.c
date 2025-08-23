#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <stdint.h>
#include <inttypes.h>

// Maximum supported value for standard uint64_t
#define MAX_FIB_N 93

// Recursive Fibonacci (exponential complexity)
uint64_t fib_recursive(int n) {
    if (n <= 1) return n;
    return fib_recursive(n - 1) + fib_recursive(n - 2);
}

// Iterative Fibonacci (linear complexity)
uint64_t fib_iterative(int n) {
    if (n <= 1) return n;
    
    uint64_t prev = 0, curr = 1;
    for (int i = 2; i <= n; i++) {
        uint64_t next = prev + curr;
        prev = curr;
        curr = next;
    }
    
    return curr;
}

// Memoized Fibonacci using static array
uint64_t fib_memoized(int n) {
    static uint64_t cache[MAX_FIB_N + 1] = {0};
    static int initialized = 0;
    
    if (!initialized) {
        memset(cache, 0, sizeof(cache));
        cache[0] = 0;
        cache[1] = 1;
        initialized = 1;
    }
    
    if (n <= 1) return n;
    
    if (cache[n] != 0 || n == 0) {
        return cache[n];
    }
    
    cache[n] = fib_memoized(n - 1) + fib_memoized(n - 2);
    return cache[n];
}

// Matrix multiplication for 2x2 matrices
void matrix_mult(uint64_t a[2][2], uint64_t b[2][2], uint64_t result[2][2]) {
    uint64_t temp[2][2];
    temp[0][0] = a[0][0] * b[0][0] + a[0][1] * b[1][0];
    temp[0][1] = a[0][0] * b[0][1] + a[0][1] * b[1][1];
    temp[1][0] = a[1][0] * b[0][0] + a[1][1] * b[1][0];
    temp[1][1] = a[1][0] * b[0][1] + a[1][1] * b[1][1];
    
    memcpy(result, temp, sizeof(temp));
}

// Matrix power
void matrix_pow(uint64_t mat[2][2], int n, uint64_t result[2][2]) {
    if (n == 1) {
        memcpy(result, mat, sizeof(uint64_t) * 4);
        return;
    }
    
    if (n % 2 == 0) {
        uint64_t half[2][2];
        matrix_pow(mat, n / 2, half);
        matrix_mult(half, half, result);
    } else {
        uint64_t temp[2][2];
        matrix_pow(mat, n - 1, temp);
        matrix_mult(mat, temp, result);
    }
}

// Matrix Fibonacci (logarithmic complexity)
uint64_t fib_matrix(int n) {
    if (n == 0) return 0;
    if (n == 1) return 1;
    
    uint64_t base[2][2] = {{1, 1}, {1, 0}};
    uint64_t result[2][2];
    
    matrix_pow(base, n, result);
    return result[0][1];
}

// Tail-recursive helper
uint64_t fib_tail_helper(int n, uint64_t prev, uint64_t curr) {
    if (n == 0) return prev;
    return fib_tail_helper(n - 1, curr, prev + curr);
}

// Tail-recursive Fibonacci
uint64_t fib_tail_recursive(int n) {
    return fib_tail_helper(n, 0, 1);
}

// Optimized iterative with minimal memory
uint64_t fib_optimized(int n) {
    if (n <= 1) return n;
    
    uint64_t a = 0, b = 1;
    for (int i = 2; i <= n; i++) {
        b = a + b;
        a = b - a;
    }
    
    return b;
}

// Benchmark function
double benchmark(const char* name, int n, uint64_t (*func)(int)) {
    clock_t start = clock();
    uint64_t result = func(n);
    clock_t end = clock();
    
    double cpu_time = ((double)(end - start)) / CLOCKS_PER_SEC * 1000000000; // Convert to nanoseconds
    
    printf("%s: fib(%d) = %" PRIu64 ", time = %.0f ns\n", name, n, result, cpu_time);
    return cpu_time;
}

// Test function
void test_implementation(const char* name, uint64_t (*func)(int)) {
    struct {
        int n;
        uint64_t expected;
    } test_cases[] = {
        {0, 0},
        {1, 1},
        {5, 5},
        {10, 55},
        {20, 6765},
        {30, 832040},
        {40, 102334155},
    };
    
    int num_tests = sizeof(test_cases) / sizeof(test_cases[0]);
    int passed = 0;
    
    printf("Testing %s:\n", name);
    for (int i = 0; i < num_tests; i++) {
        // Skip large recursive tests
        if (strcmp(name, "recursive") == 0 && test_cases[i].n > 35) {
            continue;
        }
        
        uint64_t result = func(test_cases[i].n);
        if (result == test_cases[i].expected) {
            passed++;
        } else {
            printf("  FAIL: fib(%d) = %" PRIu64 ", expected %" PRIu64 "\n",
                   test_cases[i].n, result, test_cases[i].expected);
        }
    }
    
    printf("  %d/%d tests passed\n", passed, num_tests);
}

int main(int argc, char* argv[]) {
    if (argc == 2 && strcmp(argv[1], "test") == 0) {
        // Run tests
        test_implementation("recursive", fib_recursive);
        test_implementation("iterative", fib_iterative);
        test_implementation("memoized", fib_memoized);
        test_implementation("matrix", fib_matrix);
        test_implementation("tail_recursive", fib_tail_recursive);
        test_implementation("optimized", fib_optimized);
        return 0;
    }
    
    if (argc == 2 && strcmp(argv[1], "benchmark") == 0) {
        // Run benchmarks
        printf("C Fibonacci Benchmarks\n");
        printf("======================\n");
        
        benchmark("Iterative", 40, fib_iterative);
        benchmark("Memoized", 40, fib_memoized);
        benchmark("Matrix", 40, fib_matrix);
        benchmark("TailRecursive", 40, fib_tail_recursive);
        benchmark("Optimized", 40, fib_optimized);
        
        printf("\nLarge number test:\n");
        benchmark("Iterative", 90, fib_iterative);
        
        return 0;
    }
    
    if (argc < 2) {
        printf("Usage: %s <n> [variant]\n", argv[0]);
        printf("       %s test\n", argv[0]);
        printf("       %s benchmark\n", argv[0]);
        printf("Variants: recursive, iterative, memoized, matrix, tail, optimized\n");
        return 1;
    }
    
    int n = atoi(argv[1]);
    if (n < 0 || n > MAX_FIB_N) {
        fprintf(stderr, "Error: n must be between 0 and %d\n", MAX_FIB_N);
        return 1;
    }
    
    const char* variant = argc > 2 ? argv[2] : "iterative";
    
    if (strcmp(variant, "recursive") == 0) {
        if (n > 40) {
            fprintf(stderr, "Warning: recursive is very slow for n > 40\n");
        }
        benchmark("Recursive", n, fib_recursive);
    } else if (strcmp(variant, "iterative") == 0) {
        benchmark("Iterative", n, fib_iterative);
    } else if (strcmp(variant, "memoized") == 0) {
        benchmark("Memoized", n, fib_memoized);
    } else if (strcmp(variant, "matrix") == 0) {
        benchmark("Matrix", n, fib_matrix);
    } else if (strcmp(variant, "tail") == 0) {
        benchmark("TailRecursive", n, fib_tail_recursive);
    } else if (strcmp(variant, "optimized") == 0) {
        benchmark("Optimized", n, fib_optimized);
    } else {
        fprintf(stderr, "Unknown variant: %s\n", variant);
        return 1;
    }
    
    return 0;
}