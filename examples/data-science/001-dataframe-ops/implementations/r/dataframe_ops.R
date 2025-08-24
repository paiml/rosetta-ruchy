#!/usr/bin/env Rscript
# DataFrame Operations - R Implementation
# Statistical computing equivalent to Ruchy implementation
# Focus: Statistical operations with R's built-in data analysis capabilities

library(dplyr)
library(microbenchmark)

create_test_data <- function() {
    # Create test data matching Ruchy implementation
    return(c(1, 2, 3, 4, 5, 6, 7, 8, 9, 10))
}

calculate_sum <- function(data) {
    # Calculate sum of values
    return(sum(data))
}

calculate_mean <- function(data) {
    # Calculate mean of values (integer division to match Ruchy)
    if (length(data) == 0) {
        return(0)
    }
    return(as.integer(sum(data) / length(data)))
}

filter_above_threshold <- function(data, threshold) {
    # Filter values above threshold
    return(data[data > threshold])
}

sort_values <- function(data) {
    # Sort values in ascending order
    return(sort(data))
}

find_maximum <- function(data) {
    # Find maximum value
    if (length(data) == 0) {
        return(0)
    }
    return(max(data))
}

find_minimum <- function(data) {
    # Find minimum value
    if (length(data) == 0) {
        return(0)
    }
    return(min(data))
}

count_consecutive_groups <- function(data) {
    # Group consecutive values and count occurrences
    if (length(data) == 0) {
        return(c())
    }
    
    result <- c()
    current_count <- 1
    current_value <- data[1]
    
    if (length(data) > 1) {
        for (i in 2:length(data)) {
            if (data[i] == current_value) {
                current_count <- current_count + 1
            } else {
                result <- c(result, current_count)
                current_value <- data[i]
                current_count <- 1
            }
        }
    }
    
    result <- c(result, current_count)
    return(result)
}

test_data_operations <- function() {
    # Test all basic data operations
    cat("Basic Data Operations Tests - R\n")
    cat("===============================\n")
    
    test_data <- create_test_data()
    
    # Test 1: Sum calculation
    sum_result <- calculate_sum(test_data)
    if (sum_result == 55) {
        cat("✓ Sum calculation: Pass\n")
    } else {
        cat("✗ Sum calculation: Fail\n")
    }
    
    # Test 2: Mean calculation
    mean_result <- calculate_mean(test_data)
    if (mean_result == 5) {
        cat("✓ Mean calculation: Pass\n")
    } else {
        cat("✗ Mean calculation: Fail\n")
    }
    
    # Test 3: Filtering
    filtered <- filter_above_threshold(test_data, 5)
    if (length(filtered) == 5) {
        cat("✓ Filtering operations: Pass\n")
    } else {
        cat("✗ Filtering operations: Fail\n")
    }
    
    # Test 4: Sorting
    test_unsorted <- c(5, 2, 8, 1, 9)
    sorted_data <- sort_values(test_unsorted)
    if (sorted_data[1] == 1 && sorted_data[5] == 9) {
        cat("✓ Sorting operations: Pass\n")
    } else {
        cat("✗ Sorting operations: Fail\n")
    }
    
    # Test 5: Maximum/Minimum
    max_val <- find_maximum(test_data)
    min_val <- find_minimum(test_data)
    if (max_val == 10 && min_val == 1) {
        cat("✓ Min/Max operations: Pass\n")
    } else {
        cat("✗ Min/Max operations: Fail\n")
    }
    
    # Test 6: Group counting
    grouped_data <- c(1, 1, 2, 2, 2, 3)
    counts <- count_consecutive_groups(grouped_data)
    if (length(counts) == 3 && counts[1] == 2 && counts[2] == 3 && counts[3] == 1) {
        cat("✓ Group counting: Pass\n")
    } else {
        cat("✗ Group counting: Fail\n")
    }
    
    cat("\n")
    cat("Basic data operations validation complete\n")
}

benchmark_operations <- function(iterations = 10000) {
    # Benchmark operations for performance comparison
    cat(sprintf("Benchmarking with %d iterations...\n", iterations))
    
    # Generate larger test data
    large_data <- 1:1000
    
    # Benchmark sum calculation
    sum_time <- system.time({
        for (i in 1:iterations) {
            calculate_sum(large_data)
        }
    })["elapsed"]
    
    # Benchmark sorting
    sort_time <- system.time({
        for (i in 1:iterations) {
            sort_values(large_data)
        }
    })["elapsed"]
    
    # Benchmark filtering
    filter_time <- system.time({
        for (i in 1:iterations) {
            filter_above_threshold(large_data, 500)
        }
    })["elapsed"]
    
    cat(sprintf("Sum calculation: %.4fs\n", sum_time))
    cat(sprintf("Sorting: %.4fs\n", sort_time))
    cat(sprintf("Filtering: %.4fs\n", filter_time))
    
    return(list(
        sum_time = sum_time,
        sort_time = sort_time,
        filter_time = filter_time
    ))
}

analyze_complexity <- function() {
    # Analyze complexity of operations
    cat("Data Operations Complexity Analysis - R\n")
    cat("======================================\n")
    
    cat("Operation Complexities:\n")
    cat("  Sum/Mean calculation: O(n)\n")
    cat("  Filtering: O(n)\n")
    cat("  Sorting (introsort): O(n log n)\n")
    cat("  Min/Max finding: O(n)\n")
    cat("  Group counting: O(n)\n")
    cat("\n")
    
    cat("Memory Complexity:\n")
    cat("  Data storage: O(n)\n")
    cat("  Operation results: O(k) where k ≤ n\n")
    cat("\n")
    
    cat("R Advantages:\n")
    cat("  ✓ Built for statistical computing\n")
    cat("  ✓ Extensive statistical libraries\n")
    cat("  ✓ Vectorized operations\n")
    cat("  ✓ Rich data visualization\n")
    cat("  ✓ Interactive data exploration\n")
    cat("\n")
    
    cat("Ruchy Advantages:\n")
    cat("  ✓ Compile-time type safety\n")
    cat("  ✓ Formal mathematical verification\n")
    cat("  ✓ Systems programming capabilities\n")
    cat("  ✓ Predictable performance\n")
    cat("  ✓ Zero-cost abstractions\n")
}

main <- function() {
    # Main demonstration function
    cat("Data Operations Foundation - R\n")
    cat("=============================\n")
    cat("Statistical computing comparison with Ruchy implementation\n")
    cat("\n")
    
    # Run comprehensive test suite
    test_data_operations()
    cat("\n")
    
    # Run benchmarks
    benchmark_results <- benchmark_operations()
    cat("\n")
    
    # Analyze complexity
    analyze_complexity()
    cat("\n")
    
    cat("✅ R implementation complete\n")
    cat("📊 Ready for performance comparison with Ruchy\n")
    cat("🎯 SPRINT 23: Cross-language validation in progress\n")
}

# Run main function when script is executed
if (!interactive()) {
    main()
}