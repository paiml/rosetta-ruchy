# 002-quicksort

## Problem Statement

Implement the quicksort sorting algorithm with the following variants:
- **Classic**: In-place partition with random pivot
- **Three-way**: Optimize for duplicate elements  
- **Parallel**: Multi-threaded sorting for large arrays

## Algorithm Complexity

- **Time**: O(n log n) average, O(n²) worst case
- **Space**: O(log n) for recursive stack
- **Stability**: Not stable (can be made stable with modifications)

## Test Cases

### Basic Functionality
- Empty array: `[]`
- Single element: `[42]`
- Already sorted: `[1, 2, 3, 4, 5]`
- Reverse sorted: `[5, 4, 3, 2, 1]`
- Random elements: `[3, 1, 4, 1, 5, 9, 2, 6]`

### Edge Cases
- All duplicates: `[5, 5, 5, 5, 5]`
- Large arrays: 10K, 100K, 1M elements
- Nearly sorted arrays (few inversions)

## Performance Targets

| Metric | Target |
|--------|---------|
| 10K elements | < 2ms |
| 100K elements | < 25ms |
| 1M elements | < 300ms |
| Memory overhead | < 10% of array size |

## Ruchy Advantages

1. **Provable Correctness**: SMT solver verifies sorting invariants
2. **Complexity Verification**: Automated BigO analysis confirms O(n log n)
3. **Memory Safety**: Zero-cost bounds checking without performance penalty
4. **Pattern Matching**: Elegant pivot selection and partitioning
5. **Pipeline Operators**: Functional style with imperative performance