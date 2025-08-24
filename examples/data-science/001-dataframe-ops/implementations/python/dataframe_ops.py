#!/usr/bin/env python3
"""
DataFrame Operations - Python/pandas Implementation
Equivalent functionality to Ruchy implementation for performance comparison
Focus: Basic data processing operations matching Ruchy capabilities
"""

import time
from typing import List

def create_test_data() -> List[int]:
    """Create test data matching Ruchy implementation"""
    return [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

def calculate_sum(data: List[int]) -> int:
    """Calculate sum of values"""
    return sum(data)

def calculate_mean(data: List[int]) -> int:
    """Calculate mean of values (integer division to match Ruchy)"""
    if len(data) == 0:
        return 0
    return sum(data) // len(data)

def filter_above_threshold(data: List[int], threshold: int) -> List[int]:
    """Filter values above threshold"""
    return [x for x in data if x > threshold]

def sort_values(data: List[int]) -> List[int]:
    """Sort values in ascending order"""
    return sorted(data)

def find_maximum(data: List[int]) -> int:
    """Find maximum value"""
    return max(data) if data else 0

def find_minimum(data: List[int]) -> int:
    """Find minimum value"""
    return min(data) if data else 0

def count_consecutive_groups(data: List[int]) -> List[int]:
    """Group consecutive values and count occurrences"""
    if not data:
        return []
    
    result = []
    current_count = 1
    current_value = data[0]
    
    for i in range(1, len(data)):
        if data[i] == current_value:
            current_count += 1
        else:
            result.append(current_count)
            current_value = data[i]
            current_count = 1
    
    result.append(current_count)
    return result

def test_data_operations():
    """Test all basic data operations"""
    print("Basic Data Operations Tests - Python/pandas")
    print("==========================================")
    
    test_data = create_test_data()
    
    # Test 1: Sum calculation
    sum_result = calculate_sum(test_data)
    if sum_result == 55:
        print("✓ Sum calculation: Pass")
    else:
        print("✗ Sum calculation: Fail")
    
    # Test 2: Mean calculation
    mean_result = calculate_mean(test_data)
    if mean_result == 5:
        print("✓ Mean calculation: Pass")
    else:
        print("✗ Mean calculation: Fail")
    
    # Test 3: Filtering
    filtered = filter_above_threshold(test_data, 5)
    if len(filtered) == 5:
        print("✓ Filtering operations: Pass")
    else:
        print("✗ Filtering operations: Fail")
    
    # Test 4: Sorting
    test_unsorted = [5, 2, 8, 1, 9]
    sorted_data = sort_values(test_unsorted)
    if sorted_data[0] == 1 and sorted_data[4] == 9:
        print("✓ Sorting operations: Pass")
    else:
        print("✗ Sorting operations: Fail")
    
    # Test 5: Maximum/Minimum
    max_val = find_maximum(test_data)
    min_val = find_minimum(test_data)
    if max_val == 10 and min_val == 1:
        print("✓ Min/Max operations: Pass")
    else:
        print("✗ Min/Max operations: Fail")
    
    # Test 6: Group counting
    grouped_data = [1, 1, 2, 2, 2, 3]
    counts = count_consecutive_groups(grouped_data)
    if len(counts) == 3 and counts[0] == 2 and counts[1] == 3 and counts[2] == 1:
        print("✓ Group counting: Pass")
    else:
        print("✗ Group counting: Fail")
    
    print("")
    print("Basic data operations validation complete")

def benchmark_operations(iterations: int = 10000):
    """Benchmark operations for performance comparison"""
    print(f"Benchmarking with {iterations} iterations...")
    
    # Generate larger test data
    large_data = list(range(1000))
    
    # Benchmark sum calculation
    start_time = time.perf_counter()
    for _ in range(iterations):
        calculate_sum(large_data)
    sum_time = time.perf_counter() - start_time
    
    # Benchmark sorting
    start_time = time.perf_counter()
    for _ in range(iterations):
        sort_values(large_data.copy())
    sort_time = time.perf_counter() - start_time
    
    # Benchmark filtering
    start_time = time.perf_counter()
    for _ in range(iterations):
        filter_above_threshold(large_data, 500)
    filter_time = time.perf_counter() - start_time
    
    print(f"Sum calculation: {sum_time:.4f}s")
    print(f"Sorting: {sort_time:.4f}s") 
    print(f"Filtering: {filter_time:.4f}s")
    
    return {
        'sum_time': sum_time,
        'sort_time': sort_time,
        'filter_time': filter_time
    }

def analyze_complexity():
    """Analyze complexity of operations"""
    print("Data Operations Complexity Analysis - Python")
    print("===========================================")
    
    print("Operation Complexities:")
    print("  Sum/Mean calculation: O(n)")
    print("  Filtering: O(n)")
    print("  Sorting (Timsort): O(n log n)")
    print("  Min/Max finding: O(n)")
    print("  Group counting: O(n)")
    print("")
    
    print("Memory Complexity:")
    print("  Data storage: O(n)")
    print("  Operation results: O(k) where k ≤ n")
    print("")
    
    print("Python Advantages:")
    print("  ✓ Simple, readable syntax")
    print("  ✓ Rich ecosystem and libraries")
    print("  ✓ Dynamic typing flexibility")
    print("  ✓ Rapid prototyping")
    print("  ✓ Interactive development")
    print("")
    
    print("Ruchy Advantages:")
    print("  ✓ Compile-time type safety")
    print("  ✓ Formal verification capabilities")
    print("  ✓ Zero-cost abstractions")
    print("  ✓ Predictable performance")
    print("  ✓ Memory safety guarantees")

def main():
    """Main demonstration function"""
    print("Data Operations Foundation - Python")
    print("===================================")
    print("Performance comparison with Ruchy implementation")
    print("")
    
    # Run comprehensive test suite
    test_data_operations()
    print("")
    
    # Run benchmarks
    benchmark_results = benchmark_operations()
    print("")
    
    # Analyze complexity
    analyze_complexity()
    print("")
    
    print("✅ Python implementation complete")
    print("📊 Ready for performance comparison with Ruchy")
    print("🎯 SPRINT 23: Cross-language validation in progress")

if __name__ == "__main__":
    main()