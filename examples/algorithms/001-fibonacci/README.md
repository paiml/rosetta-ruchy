# Fibonacci Sequence Benchmark

## Overview
Classic recursive and iterative Fibonacci implementations for benchmarking language performance characteristics.

## Algorithm Description
The Fibonacci sequence is defined as:
- F(0) = 0
- F(1) = 1
- F(n) = F(n-1) + F(n-2) for n > 1

## Implementations
- **Recursive**: Direct recursive implementation (exponential complexity)
- **Iterative**: Loop-based implementation (linear complexity)
- **Memoized**: Recursive with caching (linear complexity)
- **Matrix**: Matrix multiplication approach (logarithmic complexity)

## Test Cases
- Small: n = 10
- Medium: n = 30
- Large: n = 40 (recursive), n = 1000 (iterative)

## Metrics
- Execution time (nanoseconds)
- Memory usage (bytes)
- Function calls (for recursive)
- Cache efficiency (for memoized)