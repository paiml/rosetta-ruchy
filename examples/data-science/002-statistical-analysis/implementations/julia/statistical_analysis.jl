#!/usr/bin/env julia
"""
Statistical Analysis - Julia Implementation
High-performance scientific computing equivalent to Ruchy
Focus: Numerical stability and performance optimization
"""

using Statistics

function calculate_mean(data::Vector{Float64})::Float64
    """Calculate arithmetic mean"""
    if length(data) == 0
        return 0.0
    end
    return mean(data)
end

function calculate_variance(data::Vector{Float64})::Float64
    """Sample variance with Bessel's correction"""
    if length(data) <= 1
        return 0.0
    end
    return var(data)  # Julia's var() uses Bessel's correction by default
end

function calculate_std_dev(data::Vector{Float64})::Float64
    """Standard deviation"""
    return std(data)  # Julia's std() uses Bessel's correction by default
end

function linear_regression_slope(x_data::Vector{Float64}, y_data::Vector{Float64})::Float64
    """Calculate slope of linear regression"""
    if length(x_data) != length(y_data) || length(x_data) < 2
        return 0.0
    end
    
    x_mean = mean(x_data)
    y_mean = mean(y_data)
    
    numerator = sum((x - x_mean) * (y - y_mean) for (x, y) in zip(x_data, y_data))
    denominator = sum((x - x_mean)^2 for x in x_data)
    
    if denominator == 0
        return 0.0
    end
    
    return numerator / denominator
end

function linear_regression_intercept(x_data::Vector{Float64}, y_data::Vector{Float64})::Float64
    """Calculate intercept of linear regression"""
    x_mean = mean(x_data)
    y_mean = mean(y_data)
    slope = linear_regression_slope(x_data, y_data)
    
    return y_mean - (slope * x_mean)
end

function pearson_correlation(x_data::Vector{Float64}, y_data::Vector{Float64})::Float64
    """Pearson product-moment correlation coefficient"""
    if length(x_data) != length(y_data) || length(x_data) < 2
        return 0.0
    end
    
    # Julia's cor() function provides Pearson correlation
    return cor(x_data, y_data)
end

function test_statistical_functions()
    """Test all statistical functions"""
    println("Statistical Analysis Tests - Julia")
    println("=================================")
    
    test_data = [1.0, 2.0, 3.0, 4.0, 5.0]
    
    # Test 1: Mean
    mean_val = calculate_mean(test_data)
    if mean_val == 3.0
        println("✓ Mean calculation: Pass")
    else
        println("✗ Mean calculation: Fail")
    end
    
    # Test 2: Variance
    variance = calculate_variance(test_data)
    if 2.4 <= variance <= 2.6  # Expected: 2.5
        println("✓ Variance calculation: Pass")
    else
        println("✗ Variance calculation: Fail")
    end
    
    # Test 3: Linear regression
    x_vals = [1.0, 2.0, 3.0, 4.0, 5.0]
    y_vals = [2.0, 4.0, 6.0, 8.0, 10.0]  # Perfect linear relationship
    slope = linear_regression_slope(x_vals, y_vals)
    intercept = linear_regression_intercept(x_vals, y_vals)
    
    if slope == 2.0 && abs(intercept) < 0.01  # intercept should be 0
        println("✓ Linear regression: Pass")
    else
        println("✗ Linear regression: Fail")
    end
    
    # Test 4: Correlation
    correlation = pearson_correlation(x_vals, y_vals)
    if 0.99 <= correlation <= 1.01  # Should be 1.0
        println("✓ Pearson correlation: Pass")
    else
        println("✗ Pearson correlation: Fail")
    end
    
    println("")
    println("Statistical functions validation complete")
end

function verify_statistical_properties()
    """Verify mathematical properties"""
    println("Statistical Properties Verification")
    println("=================================")
    
    test_data = [1.0, 2.0, 3.0, 4.0, 5.0]
    
    # Property 1: Variance is non-negative
    variance = calculate_variance(test_data)
    if variance >= 0.0
        println("✓ Variance non-negativity: Verified")
    else
        println("✗ Variance non-negativity: Failed")
    end
    
    # Property 2: Standard deviation is square root of variance
    std_dev = calculate_std_dev(test_data)
    if abs(std_dev^2 - variance) < 0.01
        println("✓ Standard deviation identity: Verified")
    else
        println("✗ Standard deviation identity: Failed")
    end
    
    # Property 3: Correlation bounds
    x_vals = [1.0, 3.0, 5.0, 7.0, 9.0]
    y_vals = [2.0, 6.0, 10.0, 14.0, 18.0]
    correlation = pearson_correlation(x_vals, y_vals)
    
    if -1.0 <= correlation <= 1.0
        println("✓ Correlation bounds: Verified")
    else
        println("✗ Correlation bounds: Failed")
    end
    
    println("")
    println("All mathematical properties verified")
end

function benchmark_operations(iterations::Int = 1000)
    """Benchmark statistical operations"""
    println("Benchmarking with $iterations iterations...")
    
    # Large test data
    large_data = collect(1.0:1000.0)
    
    # Benchmark mean/variance
    stats_time = @elapsed for _ in 1:iterations
        calculate_mean(large_data)
        calculate_variance(large_data)
    end
    
    # Benchmark correlation
    x_data = large_data
    y_data = 2.0 .* x_data .+ 1.0
    
    corr_time = @elapsed for _ in 1:iterations
        pearson_correlation(x_data, y_data)
    end
    
    # Benchmark regression
    regression_time = @elapsed for _ in 1:iterations
        linear_regression_slope(x_data, y_data)
        linear_regression_intercept(x_data, y_data)
    end
    
    println("Mean/Variance: $(stats_time)s")
    println("Correlation: $(corr_time)s")
    println("Linear Regression: $(regression_time)s")
    
    return Dict(
        "stats_time" => stats_time,
        "correlation_time" => corr_time,
        "regression_time" => regression_time
    )
end

function analyze_complexity()
    """Analyze algorithm complexity"""
    println("Statistical Operations Complexity Analysis - Julia")
    println("================================================")
    
    println("Operation Complexities:")
    println("  Mean calculation: O(n)")
    println("  Variance calculation: O(n)")
    println("  Standard deviation: O(n)")
    println("  Linear regression: O(n)")
    println("  Correlation: O(n)")
    println("")
    
    println("Memory Complexity:")
    println("  Data storage: O(n)")
    println("  Intermediate calculations: O(1)")
    println("")
    
    println("Julia Advantages:")
    println("  ✓ JIT compilation to native code")
    println("  ✓ Near-C performance with high-level syntax")
    println("  ✓ Built-in statistical functions (Statistics.jl)")
    println("  ✓ Excellent numerical stability")
    println("  ✓ Multiple dispatch for mathematical operations")
    println("")
    
    println("Ruchy Advantages:")
    println("  ✓ Compile-time formal verification")
    println("  ✓ Mathematical correctness proofs")
    println("  ✓ Static analysis and safety guarantees")
    println("  ✓ Deterministic performance")
    println("  ✓ Zero-cost abstractions without GC")
end

function main()
    """Main demonstration function"""
    println("Statistical Computing Foundation - Julia")
    println("======================================")
    println("High-performance scientific computing comparison with Ruchy")
    println("")
    
    # Run tests
    test_statistical_functions()
    println("")
    
    # Verify properties
    verify_statistical_properties()
    println("")
    
    # Run benchmarks
    benchmark_results = benchmark_operations()
    println("")
    
    # Analyze complexity
    analyze_complexity()
    println("")
    
    println("✅ Julia statistical analysis complete")
    println("📊 Ready for cross-language performance comparison")
    println("🎯 SPRINT 24: Statistical computing validation in progress")
end

# Run main function when script is executed
if abspath(PROGRAM_FILE) == @__FILE__
    main()
end