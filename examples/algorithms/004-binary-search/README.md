# 004-binary-search

## Problem Statement

Implement binary search algorithms with comprehensive variants:
- **Classic**: Standard binary search on sorted arrays
- **Leftmost/Rightmost**: Find first/last occurrence of duplicate elements
- **Range Query**: Find all elements within a value range
- **Interpolation Search**: Optimized for uniformly distributed data
- **Exponential Search**: Unbounded array search
- **Parallel Search**: Multi-threaded search across partitions

## Algorithm Complexity

- **Time**: O(log n) for classic binary search
- **Space**: O(1) iterative, O(log n) recursive
- **Prerequisites**: Array must be sorted
- **Stability**: Not applicable (search operation)

## Test Cases

### Basic Functionality
- Empty array: `[]` target `5` → `None`
- Single element: `[42]` target `42` → `Some(0)`
- Not found: `[1, 3, 5, 7, 9]` target `4` → `None`
- Found middle: `[1, 3, 5, 7, 9]` target `5` → `Some(2)`
- Found first: `[1, 3, 5, 7, 9]` target `1` → `Some(0)`
- Found last: `[1, 3, 5, 7, 9]` target `9` → `Some(4)`

### Advanced Cases
- Duplicates: `[1, 2, 2, 2, 3]` target `2` → leftmost `Some(1)`, rightmost `Some(3)`
- Range query: `[1, 2, 3, 4, 5, 6, 7, 8, 9]` range `[3, 6]` → `[2, 3, 4, 5]`
- Large arrays: 1M, 10M, 100M elements
- Interpolation data: uniformly distributed integers

## Performance Targets

| Metric | Target |
|--------|---------|
| 1M elements | < 20 comparisons |
| 10M elements | < 24 comparisons |
| 100M elements | < 27 comparisons |
| Cache efficiency | > 90% cache hits |
| Parallel speedup | > 75% on 4 cores |

## Ruchy v1.4.0 Advanced Features

1. **Generic Search Traits**: Type-safe search operations for any comparable type
2. **HashMap Result Caching**: Memoization for repeated searches
3. **F-string Debug Output**: Rich interpolation for search progress visualization
4. **Method Chaining Queries**: Fluent API for complex search operations
5. **Result Error Handling**: Robust bounds checking and validation
6. **Async Parallel Search**: Non-blocking multi-threaded search coordination
7. **Iterator Integration**: Seamless integration with Ruchy's iterator ecosystem

## Algorithm Variants

### Classic Binary Search
- Standard divide-and-conquer approach
- Iterative and recursive implementations
- Optimal for general-purpose searching

### Boundary Searches
- **Lower Bound**: Find insertion point for maintaining sort order
- **Upper Bound**: Find position after last occurrence
- **Equal Range**: Find range containing all equal elements

### Specialized Searches
- **Interpolation Search**: O(log log n) for uniform distributions
- **Exponential Search**: Efficient for unbounded or very large arrays
- **Ternary Search**: Alternative divide-and-conquer with 3-way splits

### Parallel Search
- **Partition Search**: Divide array across multiple threads
- **Pipeline Search**: Streaming search with work distribution
- **SIMD Search**: Vectorized comparisons for primitive types