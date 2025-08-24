# 001: Fibonacci Sequence

## Problem Statement
Compute the nth Fibonacci number using recursive algorithm.

## Mathematical Definition
```
F(0) = 0
F(1) = 1
F(n) = F(n-1) + F(n-2) for n > 1
```

## Complexity Analysis
- **Time Complexity**: O(2^n) - Exponential due to recursive tree
- **Space Complexity**: O(n) - Call stack depth
- **Recurrence Relation**: T(n) = T(n-1) + T(n-2) + O(1)

## Hypothesis
**Ruchy can prove O(2^n) complexity at compile time**, while other languages can only measure it at runtime.

## Test Cases
```
fibonacci(0) = 0
fibonacci(1) = 1
fibonacci(2) = 1
fibonacci(3) = 2
fibonacci(5) = 5
fibonacci(10) = 55
fibonacci(20) = 6765
fibonacci(30) = 832040
```

## Implementation Requirements
1. Pure recursive implementation (no memoization)
2. Must handle n >= 0
3. Return type must be integer
4. No external dependencies

## Verification Checklist
- [ ] Formal complexity proof (Ruchy only)
- [ ] Correctness verification
- [ ] Performance benchmarks
- [ ] Statistical analysis (p < 0.05)
- [ ] Reproducibility confirmed

## Scientific Report
See [SCIENTIFIC_REPORT.md](./results/SCIENTIFIC_REPORT.md) for complete analysis.