#!/usr/bin/env python3
"""
Fibonacci Benchmark - Python Implementation
Performance comparison for Ruchy benchmarks
"""

import sys
import time
import json
from typing import List, Dict, Any

def fibonacci_recursive(n: int) -> int:
    """Recursive fibonacci implementation"""
    if n <= 1:
        return n
    return fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)

def fibonacci_iterative(n: int) -> int:
    """Iterative fibonacci implementation"""
    if n <= 1:
        return n
    
    prev, curr = 0, 1
    for _ in range(2, n + 1):
        prev, curr = curr, prev + curr
    
    return curr

def fibonacci_memoized(n: int, memo: Dict[int, int] = None) -> int:
    """Memoized fibonacci for better recursive performance"""
    if memo is None:
        memo = {}
    
    if n <= 1:
        return n
    
    if n not in memo:
        memo[n] = fibonacci_memoized(n - 1, memo) + fibonacci_memoized(n - 2, memo)
    
    return memo[n]

def benchmark_fibonacci(iterations: int, n: int, method: str = "iterative") -> float:
    """Benchmark fibonacci with specified method"""
    
    # Warmup phase
    warmup = iterations // 10
    for _ in range(warmup):
        if method == "recursive" and n <= 20:
            fibonacci_recursive(n)
        elif method == "memoized":
            fibonacci_memoized(n)
        else:
            fibonacci_iterative(n)
    
    # Benchmark phase
    start_time = time.perf_counter()
    
    for _ in range(iterations):
        if method == "recursive" and n <= 20:
            fibonacci_recursive(n)
        elif method == "memoized":
            fibonacci_memoized(n, {})  # Fresh memo for each iteration
        else:
            fibonacci_iterative(n)
    
    elapsed = time.perf_counter() - start_time
    return (elapsed / iterations) * 1_000_000  # Convert to microseconds

def main():
    # Parse command line arguments
    iterations = 1000
    if len(sys.argv) > 2 and sys.argv[1] == "--iterations":
        try:
            iterations = int(sys.argv[2])
        except ValueError:
            pass
    
    results = {
        "algorithm": "fibonacci",
        "language": "python",
        "iterations": iterations,
        "results": []
    }
    
    test_sizes = [5, 10, 15, 20, 25, 30, 35, 40]
    
    for n in test_sizes:
        # Benchmark iterative version
        iter_time = benchmark_fibonacci(iterations, n, "iterative")
        
        # Benchmark recursive version (only for small n)
        rec_time = -1.0
        if n <= 20:
            rec_time = benchmark_fibonacci(iterations, n, "recursive")
        
        # Benchmark memoized version
        memo_time = benchmark_fibonacci(iterations, n, "memoized")
        
        results["results"].append({
            "n": n,
            "iterative_time_us": round(iter_time, 3),
            "recursive_time_us": round(rec_time, 3),
            "memoized_time_us": round(memo_time, 3)
        })
    
    print(json.dumps(results, indent=2))

if __name__ == "__main__":
    main()