# Radix Sort Algorithm

## Problem Statement

Radix Sort is a non-comparison based sorting algorithm that sorts integers by processing individual digits. It uses counting sort as a subroutine to sort the digits at each position, working from least significant digit (LSD) to most significant digit (MSD), or vice versa.

Unlike comparison-based sorts that have O(n log n) lower bound, Radix Sort can achieve O(d * (n + k)) time complexity, where:
- d = number of digits in the maximum number
- n = number of elements  
- k = radix (base, typically 10 for decimal)

This implementation demonstrates both LSD and MSD variants with comprehensive analysis.

## Complexity Analysis

### Time Complexity
- **Best Case**: O(d * (n + k)) - where d is digits, k is radix
- **Average Case**: O(d * (n + k)) - consistent performance
- **Worst Case**: O(d * (n + k)) - still linear
- **Counting Sort per digit**: O(n + k) - linear subroutine

### Space Complexity  
- **Auxiliary Space**: O(n + k) - for output array and counting array
- **Total Space**: O(n + k) - not in-place due to stability requirement
- **Call Stack**: O(d) for MSD recursive implementation

### Practical Complexity
- For 32-bit integers: d ≤ 10 digits (base 10), so effectively O(n)
- For comparison: better than O(n log n) when d < log n
- Cache-friendly due to sequential access patterns

## Algorithm Variants

### 1. LSD Radix Sort (Least Significant Digit)
- Processes digits from right to left (1s, 10s, 100s, ...)
- Uses stable sorting (counting sort) for each digit position
- Simpler implementation, good for fixed-width integers
- Naturally iterative approach

### 2. MSD Radix Sort (Most Significant Digit)  
- Processes digits from left to right (1000s, 100s, 10s, 1s)
- Recursively sorts subarrays with same prefix
- Can terminate early for shorter numbers
- More complex but can handle variable-length strings

### 3. Different Radix Bases
- **Base 10**: Standard decimal radix (human-readable)
- **Base 2**: Binary radix (bit-by-bit sorting)
- **Base 256**: Byte-by-byte sorting (efficient for computers)
- **Base 16**: Hexadecimal radix (balanced approach)

## Implementation Features

### Core Operations
- **Digit Extraction**: Get d-th digit of number in given base
- **Counting Sort**: Stable sort by single digit/character
- **Range Calculation**: Find min/max values for normalization
- **Base Conversion**: Support different radix values

### Optimizations
- **Negative Number Handling**: Two-pass sorting for signed integers
- **Variable Radix**: Adaptive base selection for optimal performance
- **Memory Pooling**: Reuse auxiliary arrays between passes
- **Early Termination**: Stop when no more significant digits

### Stability and Properties
- **Stable**: Maintains relative order of equal elements
- **Not In-place**: Requires O(n) auxiliary space
- **Not Adaptive**: Same complexity regardless of input order
- **Integer-specific**: Optimized for fixed-precision integers

## Test Cases

1. **Basic Sorting**
   - Small arrays with various digit counts
   - Verification of correct ordering

2. **Edge Cases**
   - Empty arrays, single elements
   - All identical values
   - Maximum and minimum integer values

3. **Negative Numbers**
   - Mixed positive and negative integers
   - All negative arrays
   - Zero handling

4. **Different Distributions**
   - Random distributions
   - Already sorted arrays
   - Reverse sorted arrays
   - Arrays with few unique values

5. **Performance Analysis**
   - Large arrays (1M+ elements)
   - Different digit ranges
   - Radix base comparison

6. **Stability Testing**
   - Duplicate values with associated data
   - Order preservation verification

## Applications

### Practical Use Cases
- **Integer Sorting**: When integers have bounded range
- **String Sorting**: Lexicographic ordering (MSD variant)
- **Database Systems**: Sorting fixed-width keys
- **Graphics Programming**: Color component sorting
- **Network Routing**: IP address sorting
- **Bucket Distribution**: Hash table bucket organization

### Performance Scenarios
- **Large datasets**: Linear time advantage over comparison sorts
- **Fixed-width keys**: Natural fit for radix sorting
- **Parallel processing**: Easy to parallelize counting sort passes
- **Memory hierarchy**: Good cache locality for sequential access

## Comparison with Other Algorithms

### vs Comparison-Based Sorts
- **Advantage**: Linear time O(d * n) vs O(n log n)
- **Disadvantage**: Only works for integers/fixed-width keys
- **Memory**: Uses more auxiliary space

### vs Counting Sort
- **Advantage**: Works for large ranges (k can be large)
- **Disadvantage**: More complex implementation
- **Relationship**: Uses counting sort as subroutine

### vs Bucket Sort
- **Advantage**: More general (doesn't assume uniform distribution)
- **Disadvantage**: Less optimal for uniformly distributed data
- **Similarity**: Both use distribution-based approach

## Advanced Considerations

### Cache Performance
- Sequential access patterns improve cache utilization
- Multiple passes can cause cache misses
- Base selection affects memory access patterns

### Parallel Implementation
- Each digit position can be sorted independently
- Counting sort phases are easily parallelizable
- Load balancing considerations for MSD variant

### Practical Optimizations
- Hybrid approaches: switch to comparison sort for small subarrays
- Radix selection: choose base to minimize total passes
- Memory management: reuse auxiliary arrays across passes