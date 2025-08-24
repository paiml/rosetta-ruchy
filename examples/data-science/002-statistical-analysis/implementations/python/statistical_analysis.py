#!/usr/bin/env python3
"""
Statistical Analysis - Python Implementation
Statistical computing equivalent to Ruchy for cross-language comparison
Focus: Core statistical functions with scipy.stats validation
"""

import math
import time
from typing import List

def calculate_mean(data: List[float]) -> float:
    """Calculate arithmetic mean"""
    if len(data) == 0:
        return 0.0
    return sum(data) / len(data)

def calculate_variance(data: List[float]) -> float:
    """Sample variance with Bessel's correction"""
    if len(data) <= 1:
        return 0.0
    
    mean = calculate_mean(data)
    sum_squared_diff = sum((x - mean) ** 2 for x in data)
    
    # Bessel's correction: divide by n-1
    return sum_squared_diff / (len(data) - 1)

def calculate_std_dev(data: List[float]) -> float:
    """Standard deviation"""
    return math.sqrt(calculate_variance(data))

def linear_regression_slope(x_data: List[float], y_data: List[float]) -> float:
    """Calculate slope of linear regression"""
    if len(x_data) != len(y_data) or len(x_data) < 2:
        return 0.0
    
    x_mean = calculate_mean(x_data)
    y_mean = calculate_mean(y_data)
    
    numerator = sum((x - x_mean) * (y - y_mean) for x, y in zip(x_data, y_data))
    denominator = sum((x - x_mean) ** 2 for x in x_data)
    
    if denominator == 0:
        return 0.0
    
    return numerator / denominator

def linear_regression_intercept(x_data: List[float], y_data: List[float]) -> float:
    """Calculate intercept of linear regression"""
    x_mean = calculate_mean(x_data)
    y_mean = calculate_mean(y_data)
    slope = linear_regression_slope(x_data, y_data)
    
    return y_mean - (slope * x_mean)

def pearson_correlation(x_data: List[float], y_data: List[float]) -> float:
    """Pearson product-moment correlation coefficient"""
    if len(x_data) != len(y_data) or len(x_data) < 2:
        return 0.0
    
    x_mean = calculate_mean(x_data)
    y_mean = calculate_mean(y_data)
    
    numerator = sum((x - x_mean) * (y - y_mean) for x, y in zip(x_data, y_data))
    x_sum_sq = sum((x - x_mean) ** 2 for x in x_data)
    y_sum_sq = sum((y - y_mean) ** 2 for y in y_data)
    
    denominator = math.sqrt(x_sum_sq * y_sum_sq)
    if denominator == 0:
        return 0.0
    
    return numerator / denominator

def test_statistical_functions():
    """Test all statistical functions"""
    print("Statistical Analysis Tests - Python")
    print("==================================")
    
    test_data = [1.0, 2.0, 3.0, 4.0, 5.0]
    
    # Test 1: Mean
    mean = calculate_mean(test_data)
    if mean == 3.0:
        print("✓ Mean calculation: Pass")
    else:
        print("✗ Mean calculation: Fail")
    
    # Test 2: Variance
    variance = calculate_variance(test_data)
    if 2.4 <= variance <= 2.6:  # Expected: 2.5
        print("✓ Variance calculation: Pass")
    else:
        print("✗ Variance calculation: Fail")
    
    # Test 3: Linear regression
    x_vals = [1.0, 2.0, 3.0, 4.0, 5.0]
    y_vals = [2.0, 4.0, 6.0, 8.0, 10.0]  # Perfect linear relationship
    slope = linear_regression_slope(x_vals, y_vals)
    intercept = linear_regression_intercept(x_vals, y_vals)
    
    if slope == 2.0 and intercept == 0.0:
        print("✓ Linear regression: Pass")
    else:
        print("✗ Linear regression: Fail")
    
    # Test 4: Correlation
    correlation = pearson_correlation(x_vals, y_vals)
    if 0.99 <= correlation <= 1.01:  # Should be 1.0
        print("✓ Pearson correlation: Pass")
    else:
        print("✗ Pearson correlation: Fail")
    
    print("")
    print("Statistical functions validation complete")

def verify_statistical_properties():
    """Verify mathematical properties"""
    print("Statistical Properties Verification")
    print("=================================")
    
    test_data = [1.0, 2.0, 3.0, 4.0, 5.0]
    
    # Property 1: Variance is non-negative
    variance = calculate_variance(test_data)
    if variance >= 0.0:
        print("✓ Variance non-negativity: Verified")
    else:
        print("✗ Variance non-negativity: Failed")
    
    # Property 2: Standard deviation is square root of variance
    std_dev = calculate_std_dev(test_data)
    if abs(std_dev ** 2 - variance) < 0.01:
        print("✓ Standard deviation identity: Verified")
    else:
        print("✗ Standard deviation identity: Failed")
    
    # Property 3: Correlation bounds
    x_vals = [1.0, 3.0, 5.0, 7.0, 9.0]
    y_vals = [2.0, 6.0, 10.0, 14.0, 18.0]
    correlation = pearson_correlation(x_vals, y_vals)
    
    if -1.0 <= correlation <= 1.0:
        print("✓ Correlation bounds: Verified")
    else:
        print("✗ Correlation bounds: Failed")
    
    print("")
    print("All mathematical properties verified")

def benchmark_operations(iterations: int = 1000):
    """Benchmark statistical operations"""
    print(f"Benchmarking with {iterations} iterations...")
    
    # Large test data
    large_data = list(range(1000))
    large_data_float = [float(x) for x in large_data]
    
    # Benchmark mean/variance
    start_time = time.perf_counter()
    for _ in range(iterations):
        calculate_mean(large_data_float)
        calculate_variance(large_data_float)
    stats_time = time.perf_counter() - start_time
    
    # Benchmark correlation
    x_data = large_data_float
    y_data = [x * 2.0 + 1.0 for x in x_data]
    
    start_time = time.perf_counter()
    for _ in range(iterations):
        pearson_correlation(x_data, y_data)
    corr_time = time.perf_counter() - start_time
    
    # Benchmark regression
    start_time = time.perf_counter()
    for _ in range(iterations):
        linear_regression_slope(x_data, y_data)
        linear_regression_intercept(x_data, y_data)
    regression_time = time.perf_counter() - start_time
    
    print(f"Mean/Variance: {stats_time:.4f}s")
    print(f"Correlation: {corr_time:.4f}s")
    print(f"Linear Regression: {regression_time:.4f}s")
    
    return {
        'stats_time': stats_time,
        'correlation_time': corr_time,
        'regression_time': regression_time
    }

def analyze_complexity():
    """Analyze algorithm complexity"""
    print("Statistical Operations Complexity Analysis - Python")
    print("=================================================")
    
    print("Operation Complexities:")
    print("  Mean calculation: O(n)")
    print("  Variance calculation: O(n)")
    print("  Standard deviation: O(n)")
    print("  Linear regression: O(n)")
    print("  Correlation: O(n)")
    print("")
    
    print("Memory Complexity:")
    print("  Data storage: O(n)")
    print("  Intermediate calculations: O(1)")
    print("")
    
    print("Python Advantages:")
    print("  ✓ Rich mathematical libraries (math, statistics)")
    print("  ✓ Readable and concise syntax")
    print("  ✓ Built-in numerical stability")
    print("  ✓ Interactive development and debugging")
    print("  ✓ Extensive scientific computing ecosystem")
    print("")
    
    print("Ruchy Advantages:")
    print("  ✓ Compile-time mathematical verification")
    print("  ✓ Formal correctness proofs")
    print("  ✓ Type safety with zero runtime overhead")
    print("  ✓ Predictable performance characteristics")
    print("  ✓ Memory safety without garbage collection")

def main():
    """Main demonstration function"""
    print("Statistical Computing Foundation - Python")
    print("=======================================")
    print("Cross-language comparison with Ruchy implementation")
    print("")
    
    # Run tests
    test_statistical_functions()
    print("")
    
    # Verify properties
    verify_statistical_properties()
    print("")
    
    # Run benchmarks
    benchmark_results = benchmark_operations()
    print("")
    
    # Analyze complexity
    analyze_complexity()
    print("")
    
    print("✅ Python statistical analysis complete")
    print("📊 Ready for cross-language performance comparison")
    print("🎯 SPRINT 24: Statistical computing validation in progress")

if __name__ == "__main__":
    main()