# Traveling Salesman Problem (TSP)

## Problem Statement

The Traveling Salesman Problem (TSP) is a classic NP-hard optimization problem where a salesman must visit all cities exactly once and return to the starting city, minimizing the total distance traveled.

## Complexity Analysis

- **Brute Force**: O(n!) time, O(n) space
- **Dynamic Programming (Held-Karp)**: O(n²·2ⁿ) time, O(n·2ⁿ) space
- **Branch and Bound**: O(n!) worst case, often much better in practice
- **Approximation (Nearest Neighbor)**: O(n²) time, O(n) space
- **Approximation (2-opt)**: O(n²) per iteration

## Algorithm Variants

### 1. Brute Force
- Generate all permutations
- Calculate total distance for each
- Return minimum

### 2. Dynamic Programming (Held-Karp)
- Use bitmask to represent visited cities
- Build up solutions for subsets
- Optimal solution guaranteed

### 3. Branch and Bound
- Explore search tree with pruning
- Use lower bounds to eliminate branches
- Can find optimal solution efficiently for small instances

### 4. Heuristic Approaches
- **Nearest Neighbor**: Greedy approach, fast but suboptimal
- **2-opt**: Local search improvement
- **Simulated Annealing**: Probabilistic optimization
- **Genetic Algorithm**: Evolutionary approach

## Test Cases

1. **Small Complete Graph (4 cities)**
   - Optimal solution verifiable by hand
   - Tests correctness

2. **Medium Graph (10 cities)**
   - Tests performance of exact algorithms
   - Compares heuristic quality

3. **Large Graph (20+ cities)**
   - Only heuristics feasible
   - Tests scalability

4. **Special Cases**
   - Triangle inequality satisfaction
   - Symmetric vs asymmetric distances
   - Metric TSP properties

## Performance Expectations

- Exact algorithms practical up to ~20 cities
- Heuristics scale to thousands of cities
- Approximation ratios:
  - Nearest Neighbor: No guarantee
  - Christofides: 1.5x optimal (metric TSP)
  - 2-opt: Improves any initial solution

## Implementation Requirements

- Support for complete weighted graphs
- Multiple algorithm implementations
- Performance metrics and comparison
- Visualization of tours
- Quality metrics (approximation ratio)