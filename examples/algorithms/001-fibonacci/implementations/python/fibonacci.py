#!/usr/bin/env python3
"""
Fibonacci sequence implementations in Python
Ergonomics baseline for Rosetta Ruchy benchmarks
"""

import sys
import time
from functools import lru_cache
from typing import Dict, Union

try:
    import numpy as np
    HAS_NUMPY = True
except ImportError:
    HAS_NUMPY = False


def fib_recursive(n: int) -> int:
    """Recursive Fibonacci (exponential complexity)"""
    if n <= 1:
        return n
    return fib_recursive(n - 1) + fib_recursive(n - 2)


def fib_iterative(n: int) -> int:
    """Iterative Fibonacci (linear complexity)"""
    if n <= 1:
        return n
    
    prev, curr = 0, 1
    for _ in range(2, n + 1):
        prev, curr = curr, prev + curr
    
    return curr


def fib_memoized(n: int, cache: Dict[int, int] = None) -> int:
    """Memoized Fibonacci (linear complexity with caching)"""
    if cache is None:
        cache = {}
    
    if n in cache:
        return cache[n]
    
    if n <= 1:
        result = n
    else:
        result = fib_memoized(n - 1, cache) + fib_memoized(n - 2, cache)
    
    cache[n] = result
    return result


@lru_cache(maxsize=None)
def fib_lru_cached(n: int) -> int:
    """LRU-cached Fibonacci using Python's built-in decorator"""
    if n <= 1:
        return n
    return fib_lru_cached(n - 1) + fib_lru_cached(n - 2)


def fib_matrix(n: int) -> int:
    """Matrix multiplication Fibonacci (logarithmic complexity)"""
    if n == 0:
        return 0
    if n == 1:
        return 1
    
    def matrix_mult(a, b):
        """Multiply two 2x2 matrices"""
        return [
            [a[0][0] * b[0][0] + a[0][1] * b[1][0],
             a[0][0] * b[0][1] + a[0][1] * b[1][1]],
            [a[1][0] * b[0][0] + a[1][1] * b[1][0],
             a[1][0] * b[0][1] + a[1][1] * b[1][1]]
        ]
    
    def matrix_pow(mat, p):
        """Compute matrix to the power p"""
        if p == 1:
            return mat
        
        if p % 2 == 0:
            half = matrix_pow(mat, p // 2)
            return matrix_mult(half, half)
        else:
            return matrix_mult(mat, matrix_pow(mat, p - 1))
    
    base_matrix = [[1, 1], [1, 0]]
    result = matrix_pow(base_matrix, n)
    return result[0][1]


def fib_matrix_numpy(n: int) -> int:
    """Matrix Fibonacci using NumPy for better performance"""
    if not HAS_NUMPY:
        # Fallback to regular matrix if NumPy not available
        return fib_matrix(n)
    
    if n <= 1:
        return n
    
    def matrix_pow_np(mat, p):
        """Fast matrix power using NumPy"""
        result = np.eye(2, dtype=object)
        base = mat.copy()
        
        while p > 0:
            if p % 2 == 1:
                result = result @ base
            base = base @ base
            p //= 2
        
        return result
    
    base_matrix = np.array([[1, 1], [1, 0]], dtype=object)
    result = matrix_pow_np(base_matrix, n)
    return int(result[0][1])


def fib_generator(max_n: int):
    """Generator-based Fibonacci for memory efficiency"""
    a, b = 0, 1
    for i in range(max_n + 1):
        if i == 0:
            yield 0
        elif i == 1:
            yield 1
        else:
            a, b = b, a + b
            yield b


def fib_tail_recursive(n: int) -> int:
    """Tail-recursive Fibonacci (Python doesn't optimize tail recursion)"""
    def fib_tail_helper(n, prev=0, curr=1):
        if n == 0:
            return prev
        return fib_tail_helper(n - 1, curr, prev + curr)
    
    return fib_tail_helper(n)


class FibonacciBenchmark:
    """Benchmark runner for Fibonacci implementations"""
    
    def __init__(self):
        self.implementations = {
            'recursive': fib_recursive,
            'iterative': fib_iterative,
            'memoized': fib_memoized,
            'lru_cached': fib_lru_cached,
            'matrix': fib_matrix,
            'matrix_numpy': fib_matrix_numpy,
            'tail_recursive': fib_tail_recursive,
        }
    
    def run(self, n: int, variant: str = 'iterative') -> tuple:
        """Run a specific Fibonacci implementation and return result with timing"""
        if variant not in self.implementations:
            raise ValueError(f"Unknown variant: {variant}")
        
        func = self.implementations[variant]
        
        # Clear cache for lru_cached variant
        if variant == 'lru_cached':
            fib_lru_cached.cache_clear()
        
        start = time.perf_counter_ns()
        result = func(n)
        duration_ns = time.perf_counter_ns() - start
        
        return result, duration_ns
    
    def benchmark_all(self, n: int) -> dict:
        """Benchmark all implementations for a given n"""
        results = {}
        
        for variant, func in self.implementations.items():
            # Skip recursive for large n
            if variant == 'recursive' and n > 35:
                continue
            
            try:
                result, duration_ns = self.run(n, variant)
                results[variant] = {
                    'result': result,
                    'time_ns': duration_ns,
                    'time_ms': duration_ns / 1_000_000
                }
            except RecursionError:
                results[variant] = {'error': 'RecursionError'}
        
        return results


def main():
    """Command-line interface"""
    if len(sys.argv) < 2:
        print(f"Usage: {sys.argv[0]} <n> [variant]")
        print("Variants: recursive, iterative, memoized, lru_cached, matrix, matrix_numpy, tail_recursive")
        sys.exit(1)
    
    n = int(sys.argv[1])
    variant = sys.argv[2] if len(sys.argv) > 2 else 'iterative'
    
    benchmark = FibonacciBenchmark()
    
    try:
        result, duration_ns = benchmark.run(n, variant)
        print(f"fib({n}) = {result}")
        print(f"Time: {duration_ns} ns ({duration_ns / 1_000_000:.3f} ms)")
    except ValueError as e:
        print(f"Error: {e}")
        sys.exit(1)


if __name__ == '__main__':
    main()