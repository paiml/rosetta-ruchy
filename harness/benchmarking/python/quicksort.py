#!/usr/bin/env python3
"""
QuickSort Benchmark - Python Implementation
O(n log n) average case performance validation
"""

import sys
import time
import json
import random
from typing import List, Any

def quicksort(arr: List[int]) -> List[int]:
    """Standard quicksort implementation"""
    if len(arr) <= 1:
        return arr
    
    pivot = arr[len(arr) // 2]
    left = [x for x in arr if x < pivot]
    middle = [x for x in arr if x == pivot]
    right = [x for x in arr if x > pivot]
    
    return quicksort(left) + middle + quicksort(right)

def quicksort_inplace(arr: List[int], low: int = 0, high: int = None) -> None:
    """In-place quicksort implementation"""
    if high is None:
        high = len(arr) - 1
    
    if low < high:
        pi = partition(arr, low, high)
        quicksort_inplace(arr, low, pi - 1)
        quicksort_inplace(arr, pi + 1, high)

def partition(arr: List[int], low: int, high: int) -> int:
    """Partition helper for in-place quicksort"""
    pivot = arr[high]
    i = low - 1
    
    for j in range(low, high):
        if arr[j] <= pivot:
            i += 1
            arr[i], arr[j] = arr[j], arr[i]
    
    arr[i + 1], arr[high] = arr[high], arr[i + 1]
    return i + 1

def generate_test_array(size: int, pattern: str) -> List[int]:
    """Generate test arrays with different patterns"""
    if pattern == "random":
        arr = list(range(size))
        random.shuffle(arr)
        return arr
    elif pattern == "sorted":
        return list(range(size))
    elif pattern == "reverse":
        return list(range(size - 1, -1, -1))
    elif pattern == "partial":
        arr = list(range(size))
        # Shuffle last quarter
        quarter = size // 4
        tail = arr[-quarter:]
        random.shuffle(tail)
        arr[-quarter:] = tail
        return arr
    else:
        return [0] * size

def benchmark_quicksort(iterations: int, size: int, pattern: str, inplace: bool = True) -> float:
    """Benchmark quicksort with specified parameters"""
    total_time = 0.0
    
    for _ in range(iterations):
        arr = generate_test_array(size, pattern)
        
        start_time = time.perf_counter()
        if inplace:
            quicksort_inplace(arr.copy())
        else:
            sorted_arr = quicksort(arr)
        elapsed = time.perf_counter() - start_time
        
        total_time += elapsed
    
    return (total_time / iterations) * 1_000_000  # Convert to microseconds

def main():
    # Parse command line arguments
    iterations = 1000
    if len(sys.argv) > 2 and sys.argv[1] == "--iterations":
        try:
            iterations = int(sys.argv[2])
        except ValueError:
            pass
    
    results = {
        "algorithm": "quicksort",
        "language": "python",
        "iterations": iterations,
        "results": []
    }
    
    test_sizes = [10, 50, 100, 500, 1000, 5000]
    patterns = ["random", "sorted", "reverse", "partial"]
    
    for size in test_sizes:
        for pattern in patterns:
            avg_time = benchmark_quicksort(iterations, size, pattern)
            
            results["results"].append({
                "size": size,
                "pattern": pattern,
                "avg_time_us": round(avg_time, 3)
            })
    
    print(json.dumps(results, indent=2))

if __name__ == "__main__":
    main()