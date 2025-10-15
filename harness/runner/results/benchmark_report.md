# Benchmark Report

Generated: 2025-10-15 08:34:14 UTC
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
| rust | 500801 | 24897 | (499256, 502346) | 0.5% |
| python | 5061311 | 754861 | (5014469, 5108154) | 0.7% |

## Performance Comparisons

### python vs rust

- **Change**: 910.6%
- **Significance**: Significantly slower by 910.6%

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

- **Iterations**: 1000
- **Warmup Iterations**: 100
- **Confidence Level**: 95%
- **Min Sample Size**: 1000
