# 002: QuickSort Algorithm

## Problem Statement
Sort an array of integers using the QuickSort divide-and-conquer algorithm.

## Mathematical Definition
```
QuickSort(A, low, high):
  if low < high:
    pivot = Partition(A, low, high)
    QuickSort(A, low, pivot - 1)
    QuickSort(A, pivot + 1, high)

Partition(A, low, high):
  pivot = A[high]
  i = low - 1
  for j = low to high - 1:
    if A[j] <= pivot:
      i = i + 1
      swap A[i] with A[j]
  swap A[i + 1] with A[high]
  return i + 1
```

## Complexity Analysis
- **Average Time Complexity**: O(n log n) - Balanced partitions
- **Worst Time Complexity**: O(n²) - Unbalanced partitions (sorted input)
- **Best Time Complexity**: O(n log n) - Perfect partitions
- **Space Complexity**: O(log n) - Recursion stack depth
- **Recurrence Relation**: T(n) = 2T(n/2) + O(n)

## Hypothesis
**Ruchy can prove O(n log n) average case complexity at compile time** and detect the O(n²) worst case, while other languages can only measure empirically.

## Test Cases
```rust
Input:  [64, 34, 25, 12, 22, 11, 90]
Output: [11, 12, 22, 25, 34, 64, 90]

Input:  [3, 1, 4, 1, 5, 9, 2, 6]
Output: [1, 1, 2, 3, 4, 5, 6, 9]

Input:  [1]
Output: [1]

Input:  []
Output: []

Input:  [5, 5, 5, 5]
Output: [5, 5, 5, 5]

Worst case: [1, 2, 3, 4, 5] (already sorted)
```

## Implementation Requirements
1. In-place sorting algorithm
2. Last element as pivot (simple partitioning)
3. Handle edge cases (empty, single element, duplicates)
4. No external dependencies
5. Demonstrate both average and worst-case scenarios

## Verification Checklist
- [ ] Formal complexity proof (O(n log n) average, O(n²) worst)
- [ ] Correctness verification (maintains sort property)
- [ ] Performance benchmarks across input patterns
- [ ] Statistical analysis of complexity classes
- [ ] Reproducibility confirmed

## Scientific Report
See [SCIENTIFIC_REPORT.md](./results/SCIENTIFIC_REPORT.md) for complete analysis.