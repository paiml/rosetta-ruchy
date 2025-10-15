# Benchmark Report

Generated: 2025-10-15 08:56:14 UTC
Generator: Rosetta Ruchy Benchmark Harness
Suite Version: 1.6.0

## Executive Summary

- **Fastest Implementation**: rust
- **Implementations Tested**: 2
- **Statistical Significance**: 1 comparisons performed

## Environment

- **OS**: linux
- **Architecture**: x86_64
- **CPU**: Unknown CPU
- **Memory**: 16.0 GB
- **Rust Version**: 1.70

## Performance Results

| Implementation | Mean (ns) | Std Dev (ns) | 95% CI | Outliers |
|---|---|---|---|---|
| python | 5020844 | 668074 | (4888284, 5153405) | 1.0% |
| rust | 499730 | 22059 | (495353, 504107) | 0.0% |

## Performance Comparisons

### python vs rust

- **Change**: 904.7%
- **Significance**: Significantly slower by 904.7%

## Key Insights

- Benchmarked 2 different implementations
- High performance variance detected in: python
- 1 statistically significant performance differences found

## Recommendations

- Ensure consistent environment isolation for reliable benchmarks
- Run multiple iterations to achieve statistical significance
- Investigate performance regressions for optimization opportunities
- Consider memory usage and binary size in addition to execution time
- Validate results on different hardware configurations

## Configuration

- **Iterations**: 100
- **Warmup Iterations**: 10
- **Confidence Level**: 95%
- **Min Sample Size**: 30
