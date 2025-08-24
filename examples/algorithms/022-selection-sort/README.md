# Selection Sort Algorithm

## Problem Statement

Selection Sort is a comparison-based sorting algorithm that works by repeatedly finding the minimum element from the unsorted portion and placing it at the beginning. It divides the input list into two parts: sorted and unsorted. The algorithm repeatedly selects the smallest (or largest) element from the unsorted portion and swaps it with the first element of the unsorted portion.

This implementation demonstrates Ruchy's formal complexity verification capabilities with proven O(n²) quadratic time complexity.

## Complexity Analysis

### Time Complexity
- **Best Case**: O(n²) - Always performs the same number of comparisons
- **Average Case**: O(n²) - Number of comparisons remains constant
- **Worst Case**: O(n²) - No dependency on input distribution

### Space Complexity
- **Auxiliary Space**: O(1) - In-place sorting with constant extra memory
- **Total Space**: O(n) - Input array only

### When Optimal
- Small datasets where simplicity matters over performance
- Memory-constrained environments requiring in-place sorting
- When minimizing the number of writes is important (only n-1 swaps maximum)

## Algorithm Properties

### Advantages
- **In-Place**: O(1) auxiliary space requirement
- **Simple Implementation**: Easy to understand and implement
- **Minimal Swaps**: At most n-1 swaps (good for expensive swap operations)
- **Deterministic**: Same performance regardless of input distribution

### Limitations
- **Quadratic Time**: O(n²) makes it inefficient for large datasets
- **Non-Stable**: Does not preserve relative order of equal elements
- **Non-Adaptive**: Cannot optimize for nearly-sorted inputs
- **Poor Cache Performance**: Non-sequential memory access patterns

## Implementation Phases

1. **Outer Loop**: O(n) - Iterate through positions 0 to n-2
2. **Find Minimum**: O(n-i) - Find minimum in unsorted portion
3. **Swap Operation**: O(1) - Exchange elements if needed
4. **Invariant Maintenance**: Sorted portion grows by 1 each iteration

## Formal Verification (Ruchy)

**Ruchy's unique capabilities demonstrated:**
- Formal proof that implementation achieves O(n²) complexity
- Static verification of sorting correctness invariants
- Mathematical proof of in-place property (O(1) space)
- Compile-time bounds checking for array accesses

**Complexity Invariants:**
- Comparisons: Exactly n(n-1)/2 for any input
- Swaps: At most n-1 swaps
- Memory: Constant auxiliary space

## Applications

- **Educational Purposes**: Teaching basic sorting concepts
- **Small Arrays**: When n < 50 and simplicity matters
- **Memory-Constrained Systems**: When O(1) space is critical
- **Write-Expensive Operations**: When swaps are costly but comparisons are cheap
- **Embedded Systems**: Simple, predictable behavior

## Comparison with Other Algorithms

| Algorithm | Time | Space | Stable | Adaptive | Swaps |
|-----------|------|-------|--------|----------|-------|
| **Selection Sort** | **O(n²)** | **O(1)** | **❌** | **❌** | **≤n-1** |
| Insertion Sort | O(n²) | O(1) | ✅ | ✅ | O(n²) |
| Bubble Sort | O(n²) | O(1) | ✅ | ✅ | O(n²) |
| Quick Sort | O(n log n) | O(log n) | ❌ | ❌ | O(n log n) |
| Merge Sort | O(n log n) | O(n) | ✅ | ❌ | 0 |

**Selection Sort minimizes swaps** but has poor time complexity, making it suitable only for small datasets or when write operations are expensive.