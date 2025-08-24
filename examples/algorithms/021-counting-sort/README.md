# Counting Sort Algorithm

## Problem Statement

Counting Sort is a non-comparison based sorting algorithm that operates by counting the number of objects of each distinct key value, then using arithmetic to determine the positions of each key value in the output sequence. It achieves O(n + k) linear time complexity where n is the number of elements and k is the range of the input.

This implementation demonstrates Ruchy's formal complexity verification capabilities alongside baseline comparisons.

## Complexity Analysis

### Time Complexity
- **Best Case**: O(n + k) - Linear in both input size and range
- **Average Case**: O(n + k) - No dependence on input distribution
- **Worst Case**: O(n + k) - Maintains linear time regardless of input

### Space Complexity
- **Auxiliary Space**: O(n + k) - Counting array O(k) + Output array O(n)
- **In-place Variant**: O(k) - Only counting array needed (sacrifices stability)

### When Optimal
- When k = O(n) or k = O(n log n) - Range not significantly larger than input
- Integer inputs with known, bounded range
- Stability required for equal elements

## Algorithm Properties

### Advantages
- **Linear Time**: O(n + k) beats comparison-based O(n log n) lower bound
- **Stable**: Maintains relative order of equal elements
- **Non-Comparison**: Direct indexing instead of element comparisons
- **Predictable**: Same complexity regardless of input distribution

### Limitations
- **Range Dependency**: Performance degrades when k >> n
- **Integer Only**: Requires discrete, bounded key values
- **Space Usage**: O(n + k) auxiliary space requirement
- **Non-Adaptive**: Cannot optimize for nearly-sorted inputs

## Implementation Phases

1. **Range Validation**: Verify min_val ≤ max_val and reasonable k
2. **Count Phase**: O(n) - Count occurrences of each value
3. **Cumulative Transform**: O(k) - Convert counts to positions
4. **Stable Output**: O(n) - Build output maintaining stability
5. **Copy Back**: O(n) - Transfer to original array

## Formal Verification (Ruchy)

**Ruchy's unique capabilities demonstrated:**
- Formal proof that implementation achieves O(n + k) complexity
- Static verification of stability property
- Mathematical proof of correctness invariants
- Compile-time bounds checking for array accesses

## Applications

- **Small Range Sorting**: When k ≤ n or k ≤ n log n
- **Radix Sort Subroutine**: Building block for multi-digit sorting
- **Histogram Generation**: Counting frequency distributions
- **Bucket Initialization**: First phase of bucket sort algorithms
- **Database Indexing**: Small discrete key ranges

## Comparison with Other Algorithms

| Algorithm | Time | Space | Stable | Range Limit |
|-----------|------|-------|---------|-------------|
| **Counting Sort** | **O(n + k)** | **O(n + k)** | **✅** | **Small k** |
| Quick Sort | O(n log n) | O(log n) | ❌ | Any |
| Merge Sort | O(n log n) | O(n) | ✅ | Any |
| Radix Sort | O(d(n + k)) | O(n + k) | ✅ | Multi-digit |
| Bucket Sort | O(n + k) avg | O(n + k) | ✅ | Uniform dist |

**Counting Sort is optimal when k = O(n)** and provides the theoretical best possible time complexity for bounded integer sorting.