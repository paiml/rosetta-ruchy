# Transpilation Validation Workflow

This document describes how to use rosetta-ruchy's Tier 0 (C and Bash) implementations as a validation corpus for transpiler projects.

## Overview

Rosetta-ruchy provides **8 P0 core algorithms** implemented in both C and Bash, serving as a comprehensive validation corpus for:

- **[decy](https://github.com/paiml/decy)**: C-to-Rust transpiler
- **[bashrs](https://github.com/paiml/bashrs)**: Bash↔Rust bidirectional transpiler

Each implementation includes:
- ✅ Comprehensive test suites (7-8 test cases per algorithm)
- ✅ Quality gates (gcc -Werror, shellcheck clean)
- ✅ Memory safety verification (valgrind for C)
- ✅ Reference Rust implementations for comparison

## Available Algorithms

### Simple Algorithms (4)
1. **fibonacci** (001-fibonacci) - Recursive, iterative, memoized variants
2. **binary-search** (004-binary-search) - Iterative and recursive
3. **selection-sort** (022-selection-sort) - In-place O(n²)
4. **counting-sort** (021-counting-sort) - O(n+k) linear time

### Sorting Algorithms (4)
5. **quicksort** (002-quicksort) - O(n log n) average case
6. **mergesort** (003-mergesort) - O(n log n) divide-and-conquer
7. **heap-sort** (018-heap-sort) - O(n log n) in-place
8. **radix-sort** (019-radix-sort) - O(d×(n+k)) linear time

## Validation Workflow

### For decy (C→Rust Transpiler)

#### Step 1: Select Algorithm
```bash
cd examples/algorithms/001-fibonacci/implementations/
ls -la
# c/           - Source C implementation
# rust/        - Reference Rust implementation
```

#### Step 2: Transpile C to Rust
```bash
cd c/
decy fibonacci.c -o fibonacci_transpiled.rs
```

#### Step 3: Validate Correctness
Compare transpiled output against reference implementation:

```bash
# Build and test transpiled version
rustc fibonacci_transpiled.rs -o fibonacci_transpiled
./fibonacci_transpiled test

# Compare with reference
cd ../rust/
cargo test

# Verify identical behavior on same inputs
```

#### Step 4: Performance Comparison
```bash
# Benchmark transpiled version
hyperfine './fibonacci_transpiled 40'

# Compare with reference Rust
cd ../rust/
cargo build --release
hyperfine './target/release/fibonacci 40'

# Compare with original C
cd ../c/
gcc -O3 fibonacci.c -o fibonacci
hyperfine './fibonacci 40'
```

#### Step 5: Memory Safety Analysis
```bash
# Verify transpiled code has no unsafe blocks
grep -r "unsafe" fibonacci_transpiled.rs

# Run under sanitizers
rustc -Z sanitizer=address fibonacci_transpiled.rs
./fibonacci_transpiled test

# Compare memory usage
valgrind --tool=massif ./fibonacci_original
valgrind --tool=massif ./fibonacci_transpiled
```

### For bashrs (Bash↔Rust Transpiler)

#### Step 1: Select Algorithm
```bash
cd examples/algorithms/001-fibonacci/implementations/
ls -la
# bash/        - Source Bash implementation
# rust/        - Reference Rust implementation
```

#### Step 2: Transpile Bash to Rust
```bash
cd bash/
bashrs fibonacci.sh -o fibonacci_transpiled.rs
```

#### Step 3: Validate Correctness
```bash
# Test transpiled version
rustc fibonacci_transpiled.rs -o fibonacci_transpiled
./fibonacci_transpiled test

# Compare outputs with original Bash
./fibonacci.sh test
diff <(./fibonacci.sh sort 5 3 8 1) <(./fibonacci_transpiled sort 5 3 8 1)
```

#### Step 4: POSIX Compliance Verification (Bash→Bash purification)
```bash
# Generate purified Bash from Rust
bashrs --to-bash fibonacci_transpiled.rs -o fibonacci_purified.sh

# Verify POSIX compliance
shellcheck fibonacci_purified.sh
checkbashisms fibonacci_purified.sh

# Test across multiple shells
bash fibonacci_purified.sh test
dash fibonacci_purified.sh test
zsh fibonacci_purified.sh test
```

#### Step 5: Safety Improvements Analysis
```bash
# Compare original vs purified Bash
diff -u fibonacci.sh fibonacci_purified.sh

# Verify improvements:
# - Proper error handling (set -euo pipefail)
# - Input validation
# - Safe array operations
# - Bounds checking
```

## Validation Metrics

### Correctness Validation
| Metric | Method | Pass Criteria |
|--------|--------|---------------|
| **Test Suite** | Run all test cases | 100% pass rate |
| **Output Comparison** | Diff outputs on identical inputs | Exact match |
| **Edge Cases** | Empty, single, max inputs | Correct behavior |
| **Error Handling** | Invalid inputs | Proper error messages |

### Performance Validation
| Metric | Method | Pass Criteria |
|--------|--------|---------------|
| **Time Complexity** | Benchmark scaling | Matches O(n) class |
| **Runtime Performance** | hyperfine comparison | Within 2x of reference |
| **Memory Usage** | massif/heaptrack | Within 2x of reference |
| **Binary Size** | ls -lh comparison | Within 3x of reference |

### Safety Validation (C→Rust)
| Metric | Method | Pass Criteria |
|--------|--------|---------------|
| **Unsafe Blocks** | grep "unsafe" | Minimize count |
| **Memory Leaks** | valgrind comparison | Zero leaks |
| **Buffer Overflows** | AddressSanitizer | Zero errors |
| **Data Races** | ThreadSanitizer | Zero races (if concurrent) |

### Quality Validation (Bash→Rust)
| Metric | Method | Pass Criteria |
|--------|--------|---------------|
| **POSIX Compliance** | shellcheck, checkbashisms | Zero warnings |
| **Error Handling** | set -euo pipefail | Properly implemented |
| **Input Validation** | Malformed input tests | Graceful errors |
| **Portability** | Test on dash, zsh | Works on all |

## Automated Validation Pipeline

### CI/CD Integration Example

```yaml
# .github/workflows/transpiler-validation.yml
name: Transpiler Validation

on: [push, pull_request]

jobs:
  validate-decy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install decy
        run: cargo install decy

      - name: Transpile all C examples
        run: |
          for algo in examples/algorithms/*/implementations/c/; do
            cd $algo
            decy *.c -o transpiled.rs
            rustc transpiled.rs -o transpiled_bin
            ./transpiled_bin test
          done

      - name: Compare performance
        run: |
          cd scripts/validation-tools/
          cargo run --bin compare-transpiled

  validate-bashrs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install bashrs
        run: cargo install bashrs

      - name: Transpile all Bash examples
        run: |
          for algo in examples/algorithms/*/implementations/bash/; do
            cd $algo
            bashrs *.sh -o transpiled.rs
            rustc transpiled.rs -o transpiled_bin
            ./transpiled_bin test
          done

      - name: Validate POSIX compliance
        run: |
          cd scripts/validation-tools/
          cargo run --bin check-posix-compliance
```

## Regression Testing

### Maintaining Validation Quality

As transpilers evolve, re-validate against rosetta-ruchy corpus:

```bash
# Full validation suite
cd rosetta-ruchy/
make validate-transpilers

# Individual algorithm validation
cd examples/algorithms/001-fibonacci/
make validate-decy        # Test C→Rust transpilation
make validate-bashrs      # Test Bash→Rust transpilation
```

### Version Tracking

Document transpiler version compatibility:

```markdown
## Transpiler Compatibility Matrix

| Transpiler | Version | Success Rate | Issues |
|------------|---------|--------------|--------|
| decy       | 0.1.0   | 8/8 (100%)   | None   |
| bashrs     | 0.2.0   | 8/8 (100%)   | None   |
```

## Example: Complete Validation Run

### Fibonacci (decy validation)

```bash
# Navigate to algorithm
cd examples/algorithms/001-fibonacci/implementations/

# Original C implementation
cd c/
gcc -O3 fibonacci.c -o fib_c
time ./fib_c 40
# Output: fib(40) = 102334155
# Time: 0.52s

# Transpile with decy
decy fibonacci.c -o fib_transpiled.rs

# Compile and test transpiled Rust
rustc -O fib_transpiled.rs -o fib_transpiled
time ./fib_transpiled 40
# Output: fib(40) = 102334155
# Time: 0.54s (4% slower - acceptable!)

# Memory safety check
valgrind ./fib_c 30              # C version - 0 leaks
valgrind ./fib_transpiled 30     # Transpiled - 0 leaks

# Safety analysis
grep "unsafe" fib_transpiled.rs  # Count: 0 (perfect!)

# ✅ VALIDATION PASSED
# - Correctness: ✓ Identical output
# - Performance: ✓ Within 5%
# - Memory: ✓ Zero leaks
# - Safety: ✓ Zero unsafe blocks
```

## Contribution Guidelines

### Adding New Validation Cases

When adding algorithms to Tier 0:

1. **Implement in C and Bash** following existing patterns
2. **Add comprehensive tests** (minimum 7 test cases)
3. **Verify quality gates** (gcc -Werror, shellcheck, valgrind)
4. **Test with transpilers** before merging
5. **Document any transpiler issues** discovered

### Reporting Transpiler Issues

When validation fails:

```markdown
## Transpiler Issue Report

**Transpiler**: decy v0.1.0
**Algorithm**: fibonacci (001-fibonacci)
**Issue**: Incorrect handling of recursive calls

**Expected**: fib(40) = 102334155
**Actual**: fib(40) = 0

**Transpiled Code**:
```rust
// Generated by decy - ISSUE HERE
fn fibonacci(n: i32) -> i32 {
    // Recursive calls not properly transpiled
    return 0;
}
```

**Original C**:
```c
int fibonacci(int n) {
    if (n <= 1) return n;
    return fibonacci(n-1) + fibonacci(n-2);
}
```

**Workaround**: None
**Status**: Reported to decy project
```

## Future Enhancements

### Planned Additions

1. **Sprint 46**: P1 Algorithms (graph algorithms, dynamic programming)
2. **Sprint 47**: P2 Algorithms (advanced sorting)
3. **Sprint 48**: Data science examples (CSV processing, statistics)
4. **Sprint 49**: Automated validation infrastructure

### Advanced Validation

- **Fuzzing**: Property-based testing with arbitrary inputs
- **Concurrency**: Multi-threaded algorithm validation
- **Optimization Levels**: Compare -O0, -O1, -O2, -O3 outputs
- **Cross-compilation**: Validate on ARM, RISC-V, WASM targets

## Resources

- **C Implementation Spec**: [docs/specifications/c-bash-examples.md](../specifications/c-bash-examples.md)
- **Sprint 45 Summary**: [docs/sprints/sprint-45-summary.md](../sprints/sprint-45-summary.md)
- **decy Project**: https://github.com/paiml/decy
- **bashrs Project**: https://github.com/paiml/bashrs
- **Test Results**: [INTEGRATION.md](../../INTEGRATION.md)

---

**Last Updated**: 2025-10-15
**Validation Corpus**: 8 algorithms (C + Bash)
**Status**: Production Ready ✅
