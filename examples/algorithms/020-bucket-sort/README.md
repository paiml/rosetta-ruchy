# Bucket Sort Algorithm

## Problem Statement

Bucket Sort is a distribution-based sorting algorithm that works by distributing elements into a number of buckets, sorting individual buckets, and then concatenating the sorted buckets. It achieves linear average-case time complexity O(n + k) when input is uniformly distributed across the range.

The algorithm is particularly effective for:
- Uniformly distributed floating-point numbers in [0, 1)
- Integers with known range and uniform distribution
- Data that can be easily partitioned into roughly equal-sized buckets

This implementation provides multiple variants and comprehensive analysis of bucket distribution strategies.

## Complexity Analysis

### Time Complexity
- **Best Case**: O(n + k) - when elements are uniformly distributed
- **Average Case**: O(n + k) - with uniform distribution assumption
- **Worst Case**: O(n²) - when all elements fall into one bucket
- **Bucket Sorting**: Depends on chosen subroutine (typically O(m log m) per bucket)

### Space Complexity
- **Auxiliary Space**: O(n + k) - for buckets and temporary storage
- **Total Space**: O(n + k) - not in-place due to bucket storage
- **Per Bucket**: O(n/k) average, O(n) worst case

### Performance Factors
- **Distribution uniformity**: Critical for good performance
- **Number of buckets**: Trade-off between memory and bucket size
- **Subroutine choice**: Affects individual bucket sorting performance
- **Range knowledge**: Required for optimal bucket assignment

## Algorithm Variants

### 1. Standard Bucket Sort
- Fixed number of buckets based on range
- Uniform bucket size distribution
- Suitable for known range data

### 2. Adaptive Bucket Sort
- Dynamic bucket count based on data characteristics
- Adjusts to actual data distribution
- Better for unknown distributions

### 3. Floating-Point Bucket Sort
- Specialized for [0, 1) range floats
- Direct bucket assignment via multiplication
- Optimal for uniformly distributed decimals

### 4. Integer Range Bucket Sort
- Custom range specification [min, max]
- Integer-optimized bucket assignment
- Handles negative numbers naturally

## Implementation Features

### Core Operations
- **Bucket Assignment**: Efficient mapping from value to bucket index
- **Distribution Phase**: Scatter elements into appropriate buckets
- **Individual Sorting**: Sort each non-empty bucket
- **Concatenation Phase**: Gather sorted elements from buckets

### Bucket Assignment Strategies
- **Linear Mapping**: value → floor((value - min) / range * bucket_count)
- **Hash-Based**: Custom hash function for specialized distributions
- **Quantile-Based**: Use data quantiles to balance bucket sizes
- **Logarithmic**: For exponentially distributed data

### Subroutine Algorithms
- **Insertion Sort**: O(m²), good for small buckets (m < 10)
- **Quick Sort**: O(m log m), general-purpose for medium buckets
- **Merge Sort**: O(m log m), stable and predictable
- **Radix Sort**: O(d*m), good for integer buckets

### Optimizations
- **Empty Bucket Skipping**: Avoid processing empty buckets
- **Small Bucket Insertion**: Use insertion sort for tiny buckets
- **Bucket Size Monitoring**: Adjust strategy based on actual sizes
- **Memory Pooling**: Reuse bucket storage across calls

## Test Cases

1. **Uniform Distribution**
   - Randomly distributed values in known range
   - Ideal case for bucket sort demonstration

2. **Skewed Distribution**
   - Most elements in few buckets
   - Shows worst-case behavior

3. **Already Sorted Data**
   - Elements naturally distributed across buckets
   - Performance with minimal work needed

4. **Reverse Sorted Data**
   - Elements in reverse order of buckets
   - Tests concatenation correctness

5. **Floating-Point Arrays**
   - Values in [0, 1) range
   - Classical bucket sort application

6. **Integer Arrays**
   - Known range [min, max]
   - Integer-specific optimizations

7. **Large Datasets**
   - Performance with many elements and buckets
   - Memory usage analysis

8. **Few Unique Values**
   - Many duplicates across few buckets
   - Efficiency with concentrated data

## Applications

### Practical Use Cases
- **Graphics Programming**: Color value sorting, pixel intensity
- **Database Systems**: Range partitioning, histogram analysis
- **Scientific Computing**: Sorting measurement data with known ranges
- **External Sorting**: First phase of large dataset sorting
- **Parallel Processing**: Natural parallelization across buckets
- **Data Warehousing**: ETL processes with known data ranges

### Performance Scenarios
- **Uniform Data**: Optimal performance with linear time
- **External Memory**: Reduces I/O when data doesn't fit in RAM
- **Parallel Systems**: Each bucket can be processed independently
- **Real-time Systems**: Predictable performance with good distribution

## Comparison with Other Algorithms

### vs Radix Sort
- **Advantage**: Works with floating-point numbers and arbitrary ranges
- **Disadvantage**: Sensitive to data distribution, not always linear
- **Use Case**: Better for continuous ranges and uniform distributions

### vs Quick Sort
- **Advantage**: Linear time with good distribution, stable variant possible
- **Disadvantage**: Worst-case O(n²), requires range knowledge
- **Memory**: Uses more space but can be more predictable

### vs Counting Sort
- **Advantage**: Works with larger ranges (k can be smaller)
- **Disadvantage**: Not as efficient for small discrete ranges
- **Similarity**: Both use distribution-based approach

### vs Merge Sort
- **Advantage**: Can be faster with uniform data
- **Disadvantage**: Less consistent performance, more complex
- **Stability**: Can be made stable with stable subroutine

## Advanced Considerations

### Distribution Analysis
- **Uniformity Testing**: Detect if data suits bucket sort
- **Histogram Analysis**: Understand bucket fill patterns
- **Adaptive Strategies**: Adjust bucket count dynamically
- **Range Estimation**: Handle unknown or dynamic ranges

### Parallel Implementation
- **Bucket Independence**: Natural parallelization opportunity
- **Load Balancing**: Handle uneven bucket distributions
- **Memory Locality**: Optimize for NUMA architectures
- **Synchronization**: Minimal coordination between threads

### Cache Performance
- **Sequential Access**: Good locality during distribution
- **Bucket Size**: Balance between cache fits and sorting efficiency
- **Memory Layout**: Optimize bucket storage for access patterns
- **Prefetching**: Predict bucket access patterns

### Practical Optimizations
- **Hybrid Approaches**: Combine with other algorithms based on bucket size
- **Threshold Tuning**: Switch algorithms based on bucket characteristics
- **Memory Management**: Efficient bucket allocation and deallocation
- **Range Preprocessing**: Analyze data to optimize bucket strategy

## Mathematical Properties

### Expected Bucket Size
- With n elements and k buckets: E[bucket_size] = n/k
- Standard deviation depends on distribution uniformity
- Probability of empty buckets: P(empty) = (1 - 1/k)^n

### Load Balancing
- Perfect balance: all buckets have n/k elements
- Load factor: max_bucket_size / (n/k)
- Uniformity measure: variance in bucket sizes

### Performance Prediction
- Expected time: O(n + k + Σ(b_i * log b_i)) where b_i = bucket i size
- Best case: all b_i ≈ n/k, giving O(n + k + n log(n/k))
- Worst case: one bucket has n elements, giving O(n²)