# Heap Sort Algorithm

## Problem Statement

Heap Sort is a comparison-based sorting algorithm that uses a binary heap data structure. It's an in-place sorting algorithm with optimal O(n log n) time complexity in all cases (best, average, and worst).

The algorithm works by:
1. Building a max heap from the input array
2. Repeatedly extracting the maximum element and placing it at the end
3. Maintaining the heap property after each extraction

This implementation provides comprehensive analysis of heap operations and sorting performance.

## Complexity Analysis

### Time Complexity
- **Build Heap**: O(n) - building initial heap from unsorted array
- **Extract Max**: O(log n) - per extraction operation
- **Overall**: O(n log n) - n extractions of O(log n) each
- **Best Case**: O(n log n) - no best case optimization
- **Average Case**: O(n log n) - consistent performance
- **Worst Case**: O(n log n) - optimal worst-case guarantee

### Space Complexity
- **In-place**: O(1) auxiliary space (excluding input)
- **Call Stack**: O(log n) due to recursive heapify operations
- **Total**: O(log n) space complexity

## Algorithm Features

### 1. Max Heap Construction
- Bottom-up heapify approach
- Efficient O(n) heap building
- Maintains heap property: parent ≥ children

### 2. Heap Operations
- **Heapify**: Restore heap property after modification
- **Extract Max**: Remove and return maximum element
- **Increase Key**: Update element value maintaining heap
- **Build Heap**: Convert array to heap structure

### 3. Sorting Process
- Extract maximum elements in descending order
- Place extracted elements at array end
- Reduce heap size after each extraction
- Maintain heap invariant throughout

### 4. Performance Characteristics
- **Stable**: No (relative order not preserved)
- **In-place**: Yes (constant extra space)
- **Adaptive**: No (same complexity regardless of input)
- **Online**: No (requires full input)

## Test Cases

1. **Empty Array**
   - Edge case handling
   - Boundary condition testing

2. **Single Element**
   - Trivial sorting case
   - Heap operations on size 1

3. **Already Sorted**
   - Best case input analysis
   - Performance measurement

4. **Reverse Sorted**
   - Worst case input analysis
   - Maximum comparisons needed

5. **Random Arrays**
   - Average case performance
   - Statistical analysis

6. **Duplicates**
   - Equal element handling
   - Stability considerations

7. **Large Arrays**
   - Scalability testing
   - Memory usage analysis

## Heap Sort Variants

### 1. Bottom-up Heap Construction
- Standard approach used
- O(n) heap building
- Optimal for initial construction

### 2. Iterative vs Recursive
- Both heapify implementations
- Stack space considerations
- Performance comparison

### 3. Min-heap vs Max-heap
- Max-heap for ascending sort
- Min-heap for descending sort
- Dual-purpose implementation

## Applications

- **System-level sorting**: Guaranteed O(n log n) performance
- **Priority queues**: Underlying heap operations
- **Selection algorithms**: Finding k largest/smallest elements
- **Graph algorithms**: Dijkstra's algorithm, MST algorithms
- **Memory management**: Heap allocation strategies
- **Real-time systems**: Predictable worst-case performance

## Comparison with Other Sorts

- **vs Quick Sort**: Guaranteed O(n log n), but typically slower constants
- **vs Merge Sort**: In-place advantage, but not stable
- **vs Insertion Sort**: Better for large arrays, worse for small/sorted
- **vs Selection Sort**: Same selection principle, but more efficient

## Advanced Analysis

### Cache Performance
- Poor cache locality due to heap structure
- Non-sequential memory access patterns
- Performance impact on modern architectures

### Practical Considerations
- Often used in hybrid algorithms (Introsort)
- Good fallback when recursion depth limits hit
- Suitable for external sorting with modifications