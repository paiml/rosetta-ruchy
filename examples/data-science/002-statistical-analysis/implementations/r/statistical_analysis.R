#!/usr/bin/env Rscript
# Statistical Analysis - R Implementation
# Statistical computing gold standard equivalent to Ruchy
# Focus: Native R statistical functions with comprehensive analysis

calculate_mean <- function(data) {
    # Calculate arithmetic mean
    if (length(data) == 0) {
        return(0.0)
    }
    return(mean(data))
}

calculate_variance <- function(data) {
    # Sample variance with Bessel's correction
    if (length(data) <= 1) {
        return(0.0)
    }
    return(var(data))  # R's var() uses Bessel's correction by default
}

calculate_std_dev <- function(data) {
    # Standard deviation
    return(sd(data))  # R's sd() uses Bessel's correction by default
}

linear_regression_slope <- function(x_data, y_data) {
    # Calculate slope of linear regression
    if (length(x_data) != length(y_data) || length(x_data) < 2) {
        return(0.0)
    }
    
    x_mean <- mean(x_data)
    y_mean <- mean(y_data)
    
    numerator <- sum((x_data - x_mean) * (y_data - y_mean))
    denominator <- sum((x_data - x_mean)^2)
    
    if (denominator == 0) {
        return(0.0)
    }
    
    return(numerator / denominator)
}

linear_regression_intercept <- function(x_data, y_data) {
    # Calculate intercept of linear regression
    x_mean <- mean(x_data)
    y_mean <- mean(y_data)
    slope <- linear_regression_slope(x_data, y_data)
    
    return(y_mean - (slope * x_mean))
}

pearson_correlation <- function(x_data, y_data) {
    # Pearson product-moment correlation coefficient
    if (length(x_data) != length(y_data) || length(x_data) < 2) {
        return(0.0)
    }
    
    # R's cor() function provides Pearson correlation by default
    return(cor(x_data, y_data))
}

test_statistical_functions <- function() {
    # Test all statistical functions
    cat("Statistical Analysis Tests - R\n")
    cat("==============================\n")
    
    test_data <- c(1.0, 2.0, 3.0, 4.0, 5.0)
    
    # Test 1: Mean
    mean_val <- calculate_mean(test_data)
    if (mean_val == 3.0) {
        cat("✓ Mean calculation: Pass\n")
    } else {
        cat("✗ Mean calculation: Fail\n")
    }
    
    # Test 2: Variance
    variance <- calculate_variance(test_data)
    if (variance >= 2.4 && variance <= 2.6) {  # Expected: 2.5
        cat("✓ Variance calculation: Pass\n")
    } else {
        cat("✗ Variance calculation: Fail\n")
    }
    
    # Test 3: Linear regression
    x_vals <- c(1.0, 2.0, 3.0, 4.0, 5.0)
    y_vals <- c(2.0, 4.0, 6.0, 8.0, 10.0)  # Perfect linear relationship
    slope <- linear_regression_slope(x_vals, y_vals)
    intercept <- linear_regression_intercept(x_vals, y_vals)
    
    if (slope == 2.0 && abs(intercept) < 0.01) {  # intercept should be 0
        cat("✓ Linear regression: Pass\n")
    } else {
        cat("✗ Linear regression: Fail\n")
    }
    
    # Test 4: Correlation
    correlation <- pearson_correlation(x_vals, y_vals)
    if (correlation >= 0.99 && correlation <= 1.01) {  # Should be 1.0
        cat("✓ Pearson correlation: Pass\n")
    } else {
        cat("✗ Pearson correlation: Fail\n")
    }
    
    cat("\n")
    cat("Statistical functions validation complete\n")
}

verify_statistical_properties <- function() {
    # Verify mathematical properties
    cat("Statistical Properties Verification\n")
    cat("=================================\n")
    
    test_data <- c(1.0, 2.0, 3.0, 4.0, 5.0)
    
    # Property 1: Variance is non-negative
    variance <- calculate_variance(test_data)
    if (variance >= 0.0) {
        cat("✓ Variance non-negativity: Verified\n")
    } else {
        cat("✗ Variance non-negativity: Failed\n")
    }
    
    # Property 2: Standard deviation is square root of variance
    std_dev <- calculate_std_dev(test_data)
    if (abs(std_dev^2 - variance) < 0.01) {
        cat("✓ Standard deviation identity: Verified\n")
    } else {
        cat("✗ Standard deviation identity: Failed\n")
    }
    
    # Property 3: Correlation bounds
    x_vals <- c(1.0, 3.0, 5.0, 7.0, 9.0)
    y_vals <- c(2.0, 6.0, 10.0, 14.0, 18.0)
    correlation <- pearson_correlation(x_vals, y_vals)
    
    if (correlation >= -1.0 && correlation <= 1.0) {
        cat("✓ Correlation bounds: Verified\n")
    } else {
        cat("✗ Correlation bounds: Failed\n")
    }
    
    cat("\n")
    cat("All mathematical properties verified\n")
}

benchmark_operations <- function(iterations = 1000) {
    # Benchmark statistical operations
    cat(sprintf("Benchmarking with %d iterations...\n", iterations))
    
    # Large test data
    large_data <- 1:1000
    
    # Benchmark mean/variance
    stats_time <- system.time({
        for (i in 1:iterations) {
            calculate_mean(large_data)
            calculate_variance(large_data)
        }
    })["elapsed"]
    
    # Benchmark correlation
    x_data <- large_data
    y_data <- 2.0 * x_data + 1.0
    
    corr_time <- system.time({
        for (i in 1:iterations) {
            pearson_correlation(x_data, y_data)
        }
    })["elapsed"]
    
    # Benchmark regression
    regression_time <- system.time({
        for (i in 1:iterations) {
            linear_regression_slope(x_data, y_data)
            linear_regression_intercept(x_data, y_data)
        }
    })["elapsed"]
    
    cat(sprintf("Mean/Variance: %.4fs\n", stats_time))
    cat(sprintf("Correlation: %.4fs\n", corr_time))
    cat(sprintf("Linear Regression: %.4fs\n", regression_time))
    
    return(list(
        stats_time = stats_time,
        correlation_time = corr_time,
        regression_time = regression_time
    ))
}

analyze_complexity <- function() {
    # Analyze algorithm complexity
    cat("Statistical Operations Complexity Analysis - R\n")
    cat("==============================================\n")
    
    cat("Operation Complexities:\n")
    cat("  Mean calculation: O(n)\n")
    cat("  Variance calculation: O(n)\n")
    cat("  Standard deviation: O(n)\n")
    cat("  Linear regression: O(n)\n")
    cat("  Correlation: O(n)\n")
    cat("\n")
    
    cat("Memory Complexity:\n")
    cat("  Data storage: O(n)\n")
    cat("  Intermediate calculations: O(1)\n")
    cat("\n")
    
    cat("R Advantages:\n")
    cat("  ✓ Purpose-built for statistical computing\n")
    cat("  ✓ Comprehensive statistical function library\n")
    cat("  ✓ Vectorized operations for performance\n")
    cat("  ✓ Rich ecosystem for data analysis\n")
    cat("  ✓ Interactive statistical exploration\n")
    cat("\n")
    
    cat("Ruchy Advantages:\n")
    cat("  ✓ Compile-time formal verification\n")
    cat("  ✓ Mathematical correctness proofs\n")
    cat("  ✓ Type safety with predictable performance\n")
    cat("  ✓ Memory safety without GC overhead\n")
    cat("  ✓ Systems programming + statistical computing\n")
}

main <- function() {
    # Main demonstration function
    cat("Statistical Computing Foundation - R\n")
    cat("===================================\n")
    cat("Statistical computing gold standard comparison with Ruchy\n")
    cat("\n")
    
    # Run tests
    test_statistical_functions()
    cat("\n")
    
    # Verify properties
    verify_statistical_properties()
    cat("\n")
    
    # Run benchmarks
    benchmark_results <- benchmark_operations()
    cat("\n")
    
    # Analyze complexity
    analyze_complexity()
    cat("\n")
    
    cat("✅ R statistical analysis complete\n")
    cat("📊 Ready for cross-language performance comparison\n")
    cat("🎯 SPRINT 24: Statistical computing validation in progress\n")
}

# Run main function when script is executed
if (!interactive()) {
    main()
}