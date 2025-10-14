# Performance Benchmarking Plan - Ruchy vs Rust Baseline

**Sprint**: 41
**Ticket**: ROSETTA-414
**Date**: 2025-10-14
**Goal**: Establish empirical performance baseline comparing Ruchy 3.78.0 to Rust

## Selected Algorithms

We selected 5 algorithms representing different computational patterns and complexity classes:

### 1. Fibonacci (001-fibonacci)
- **Complexity**: O(2^n) recursive, O(n) iterative
- **Pattern**: Mathematical computation
- **Test**: Function call overhead, recursion performance
- **Input sizes**: n = 10, 20, 30, 40
- **Expected**: Should be CPU-bound, minimal memory

### 2. QuickSort (002-quicksort)
- **Complexity**: O(n log n) average, O(n²) worst
- **Pattern**: Sorting, array manipulation
- **Test**: Comparison-based sorting, array access
- **Input sizes**: n = 1K, 10K, 100K, 1M elements
- **Expected**: Memory allocation patterns matter

### 3. Binary Search (004-binary-search)
- **Complexity**: O(log n)
- **Pattern**: Search algorithm
- **Test**: Array access patterns, branch prediction
- **Input sizes**: n = 1K, 10K, 100K, 1M, 10M elements
- **Expected**: Very fast, cache-friendly

### 4. Dijkstra Shortest Path (007-dijkstra-shortest-path)
- **Complexity**: O((V+E) log V) with binary heap
- **Pattern**: Graph algorithm
- **Test**: Complex data structures, priority queue
- **Input sizes**: 10, 50, 100, 500 vertices (dense graphs)
- **Expected**: Data structure overhead significant

### 5. Edit Distance (010-edit-distance)
- **Complexity**: O(m*n)
- **Pattern**: Dynamic programming
- **Test**: String manipulation, DP table operations
- **Input sizes**: strings of length 10, 50, 100, 500
- **Expected**: Memory-intensive (2D tables)

## Benchmarking Methodology

### Environment
- **Machine**: Standardized (same machine for all tests)
- **OS**: Linux
- **CPU**: Fixed frequency (disable turbo boost)
- **Memory**: Measure both peak and average
- **Isolation**: Single-threaded, no background processes

### Metrics Collected
1. **Execution Time**:
   - Mean (µ)
   - Standard deviation (σ)
   - Min/Max
   - 95% confidence interval

2. **Memory Usage**:
   - Peak RSS (Resident Set Size)
   - Heap allocations
   - Stack usage

3. **Statistical Rigor**:
   - Minimum 100 iterations per benchmark
   - Warmup phase (10 iterations, discarded)
   - Outlier detection and removal
   - Statistical significance testing (t-test)

### Success Criteria

**Performance Parity**: Ruchy within 5% of Rust baseline
- If Ruchy time / Rust time ≤ 1.05, consider equivalent
- Document any >10% differences

**Memory Parity**: Ruchy within 10% of Rust baseline
- Memory overhead expected due to different allocators
- Document patterns, not absolute values

## Implementation Plan

### Phase 1: Benchmark Harness
Create unified benchmarking infrastructure:
```bash
scripts/benchmark-runner.sh <algorithm> <input-size>
```

Outputs:
- JSON with timing data
- CSV with per-iteration results
- Statistical summary

### Phase 2: Data Collection
For each of 5 algorithms:
1. Compile both Rust and Ruchy versions (release mode)
2. Run with all input sizes
3. Collect 100+ samples per configuration
4. Save raw data to `reports/performance/raw/`

### Phase 3: Analysis
- Calculate mean, stddev, confidence intervals
- Perform t-tests for significance
- Generate comparison charts
- Identify performance patterns

### Phase 4: Reporting
Generate `BASELINE-COMPARISON.md` with:
- Executive summary
- Per-algorithm detailed results
- Performance charts
- Statistical analysis
- Recommendations

## Expected Outcomes

### Hypothesis
Ruchy should achieve near-Rust performance because:
1. Ruchy transpiles to idiomatic Rust
2. Same LLVM backend optimization
3. Similar memory layout

### Potential Differences
Areas where performance may differ:
1. **Allocator**: Different default allocators
2. **Inlining**: Transpilation layer may affect inline decisions
3. **Monomorphization**: Generic specialization patterns
4. **Error handling**: Ruchy's error handling overhead

### Baseline Target
- **Acceptable**: 95% of Rust performance (1.05x slower)
- **Good**: 98% of Rust performance (1.02x slower)
- **Excellent**: 100% parity (within measurement error)

## Tools and Scripts

### Benchmarking Tools
- `hyperfine` - Command-line benchmarking
- `/usr/bin/time -v` - Memory usage
- Custom timing harness - Fine-grained measurements

### Analysis Tools
- Python/NumPy - Statistical analysis
- gnuplot - Chart generation
- jq - JSON processing

## Timeline

- **Day 1**: Create benchmark harness, verify compilation
- **Day 2**: Run benchmarks, collect data
- **Day 3**: Analysis, report generation

## Risks and Mitigation

### Risk: Inconsistent Results
**Mitigation**: 
- Disable CPU frequency scaling
- Run on idle system
- Large sample sizes (100+ iterations)

### Risk: Unfair Comparison
**Mitigation**:
- Same compiler flags (opt-level = 3)
- Same input data
- Measure both implementations identically

### Risk: Outliers Skew Results
**Mitigation**:
- Outlier detection (IQR method)
- Report median alongside mean
- Document and investigate outliers

## Deliverables

1. `scripts/benchmark-runner.sh` - Automated benchmark execution
2. `reports/performance/raw/*.json` - Raw benchmark data
3. `reports/performance/BASELINE-COMPARISON.md` - Final report
4. Performance charts (PNG/SVG)
5. Statistical analysis tables

---

Generated with Claude Code
Co-Authored-By: Claude <noreply@anthropic.com>
