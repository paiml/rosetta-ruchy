# 003-mergesort

## Problem Statement

Implement the merge sort algorithm with advanced features:
- **Classic**: Divide-and-conquer with O(n log n) guaranteed performance
- **Bottom-up**: Iterative implementation for memory efficiency
- **Stable**: Maintains relative order of equal elements
- **Adaptive**: Optimizes for partially sorted data
- **Parallel**: Multi-threaded divide-and-conquer

## Algorithm Complexity

- **Time**: O(n log n) guaranteed (best, average, and worst case)
- **Space**: O(n) auxiliary space for merging
- **Stability**: Stable (preserves relative order of equal elements)
- **Adaptivity**: Can be optimized for nearly sorted data

## Test Cases

### Correctness Tests
- Empty array: `[]`
- Single element: `[42]`
- Already sorted: `[1, 2, 3, 4, 5]`
- Reverse sorted: `[5, 4, 3, 2, 1]`
- Random elements: `[3, 1, 4, 1, 5, 9, 2, 6]`
- Stability test: `[(1,'a'), (2,'b'), (1,'c')] → [(1,'a'), (1,'c'), (2,'b')]`

### Performance Tests
- Large arrays: 100K, 1M, 10M elements
- Nearly sorted arrays (few inversions)
- Arrays with many duplicates
- Worst-case patterns for other algorithms

## Performance Targets

| Metric | Target |
|--------|---------|
| 100K elements | < 15ms |
| 1M elements | < 150ms |
| 10M elements | < 1.8s |
| Memory overhead | ≤ 100% of input size |
| Stability | 100% maintained |

## Ruchy v1.4.0 Advantages

1. **HashMap Analytics**: Real-time performance tracking per data pattern
2. **F-string Reporting**: Rich interpolated debugging and metrics
3. **Method Chaining**: Elegant functional pipeline operations
4. **Result Error Handling**: Robust memory allocation handling
5. **Advanced Type System**: Generic implementations with trait bounds
6. **Async Parallel Sorting**: Non-blocking parallel merge operations
7. **Iterator Adaptors**: Zero-cost abstractions for data processing

## Algorithm Variants Implemented

### Classic Recursive
- Traditional top-down divide-and-conquer
- Recursive splitting with merge operations
- Stack-based memory usage

### Bottom-up Iterative  
- Iterative merging from smallest to largest subarrays
- Constant stack space usage
- Cache-friendly memory access patterns

### Adaptive Merge Sort
- Detects existing sorted runs
- Skips unnecessary merge operations
- Optimizes for nearly sorted data

### Parallel Merge Sort
- Multi-threaded divide-and-conquer
- Work-stealing task distribution
- NUMA-aware memory allocation