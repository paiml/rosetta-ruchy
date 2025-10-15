# C and Bash Examples Specification

**Version**: 1.0.0
**Date**: 2025-10-15
**Status**: Planning
**Sprint**: 45

## Executive Summary

This specification defines the systematic addition of **C** and **Bash** implementations to the rosetta-ruchy benchmark suite to support two sister projects:

1. **decy** (https://github.com/paiml/decy) - C-to-Rust transpiler
2. **bashrs** (https://github.com/paiml/bashrs) - Rust-to-Shell/Shell-to-Rust transpiler

By adding C and Bash implementations, rosetta-ruchy becomes a comprehensive validation corpus for these transpilation projects, providing real-world algorithm implementations for testing transpilation accuracy, performance, and correctness.

## Strategic Goals

### Primary Objectives

1. **Transpilation Validation Corpus**: Provide gold-standard algorithm implementations in C and Bash for validating transpiler output
2. **Performance Baselines**: Establish C and Bash performance baselines for comparison with transpiled Rust
3. **Cross-Language Verification**: Enable verification that transpiled code produces identical results
4. **Real-World Test Cases**: Provide diverse algorithm complexity for testing transpiler edge cases

### Sister Project Integration

#### decy Integration
- **Input**: C implementations from rosetta-ruchy
- **Output**: Transpiled Rust code
- **Validation**: Compare transpiled Rust against rosetta-ruchy's reference Rust implementations
- **Metrics**: Performance parity, safety (unsafe count), correctness (identical results)

#### bashrs Integration
- **Input**: Bash implementations from rosetta-ruchy
- **Output**: Safe Rust code OR purified Bash scripts
- **Validation**: Compare transpiled code against reference implementations
- **Metrics**: POSIX compliance, safety improvements, correctness

## Language Tier Updates

### Current Tier Structure (Phase 3: Data Science)

**Tier 1**: Primary Data Science Languages
- Ruchy, Julia, Python/pandas, R

**Tier 2**: JVM Data Science Languages
- Kotlin, Scala

**Tier 3**: Reference Implementations
- Rust, JavaScript

### Proposed Addition: Tier 0 - Transpilation Source Languages

**Tier 0**: Transpilation Source Languages (New)
- **C**: Source language for decy transpilation
- **Bash**: Source language for bashrs transpilation
- **Purpose**: Provide canonical implementations for transpiler validation

**Rationale**:
- C and Bash are not competing with Ruchy for performance/ergonomics
- They serve as **input sources** for transpilers, not benchmark competitors
- Separate tier clarifies their role as transpilation validation corpus

## Implementation Strategy

### Phase 1: Algorithm Examples (22 algorithms)

#### Priority Classification

**P0 - Core Algorithms** (Implement First - 8 algorithms):
1. `001-fibonacci` - Simple recursion, memory management
2. `002-quicksort` - Arrays, in-place algorithms, pointers
3. `003-mergesort` - Recursion, merge operations
4. `004-binary-search` - Arrays, search algorithms
5. `005-hash-table` - Data structures, dynamic allocation
6. `017-binary-search-tree` - Tree structures, pointers
7. `018-heap-sort` - Heap operations, array manipulation
8. `019-radix-sort` - Bucket sorting, string/integer handling

**P1 - Graph & Dynamic Programming** (10 algorithms):
9. `006-red-black-tree` - Complex data structures
10. `007-dijkstra` - Graph algorithms, priority queues
11. `008-longest-common-subsequence` - DP, 2D arrays
12. `009-knapsack-problem` - DP, optimization
13. `010-edit-distance` - DP, string algorithms
14. `011-matrix-chain-multiplication` - DP, matrices
15. `012-coin-change` - DP, greedy algorithms
16. `013-rod-cutting` - DP, optimization
17. `014-graph-coloring` - Graph algorithms, backtracking
18. `015-traveling-salesman` - Graph algorithms, NP-hard

**P2 - Advanced Sorting** (4 algorithms):
19. `016-topological-sort` - Graph algorithms, ordering
20. `020-bucket-sort` - Distribution sorting
21. `021-counting-sort` - Integer sorting
22. `022-selection-sort` - Simple sorting

### Phase 2: Data Science Examples (12 examples)

**Approach**: Limited implementation for proof-of-concept
- Data science in pure C/Bash is impractical for most examples
- Focus on demonstrating transpiler capabilities, not feature completeness
- Implement only examples that make sense in C/Bash context

**Recommended Data Science Examples**:
1. `001-dataframe-ops` - C: Basic array operations, Bash: CSV processing
2. `002-statistical-analysis` - C: Mathematical computations, Bash: Basic stats
3. `005-io-memory` - C: File I/O, memory operations, Bash: File processing
4. `006-concurrent-processing` - C: pthreads, Bash: background jobs

**Excluded**: Machine learning, computer vision, advanced analytics (impractical in C/Bash)

## C Implementation Guidelines

### Code Standards

**Style**: Follow K&R or GNU C standards with rosetta-ruchy consistency
```c
// Standard C implementation template
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Algorithm implementation
int fibonacci(int n) {
    if (n <= 1) return n;
    return fibonacci(n - 1) + fibonacci(n - 2);
}

// Test harness
int main(void) {
    // Run tests
    assert(fibonacci(0) == 0);
    assert(fibonacci(1) == 1);
    assert(fibonacci(10) == 55);

    printf("All tests passed\n");
    return 0;
}
```

### Memory Safety Requirements

1. **No Memory Leaks**: All `malloc` must have matching `free`
2. **Bounds Checking**: Validate all array accesses
3. **Null Checks**: Check all pointer dereferences
4. **Error Handling**: Proper return codes and error messages

### Testing Requirements

1. **Valgrind Clean**: Zero memory leaks, zero errors
2. **Static Analysis**: Clean scan from `cppcheck` or `clang-tidy`
3. **Compiler Warnings**: Zero warnings with `-Wall -Wextra -Werror`
4. **Test Coverage**: Minimum 80% line coverage via `gcov`

### Build Integration

```makefile
# Standard C Makefile pattern for each algorithm
CC = gcc
CFLAGS = -std=c11 -Wall -Wextra -Werror -O3 -g

all: algorithm test

algorithm: algorithm.c
	$(CC) $(CFLAGS) -o algorithm algorithm.c

test: algorithm
	./algorithm
	valgrind --leak-check=full --error-exitcode=1 ./algorithm

clean:
	rm -f algorithm *.o
```

## Bash Implementation Guidelines

### Code Standards

**Style**: Follow Google Shell Style Guide + rosetta-ruchy additions
```bash
#!/usr/bin/env bash
set -euo pipefail  # Exit on error, undefined vars, pipe failures

# Algorithm implementation
fibonacci() {
    local n=$1

    if (( n <= 1 )); then
        echo "$n"
        return 0
    fi

    local a=0 b=1
    for (( i=2; i<=n; i++ )); do
        local temp=$((a + b))
        a=$b
        b=$temp
    done

    echo "$b"
}

# Test harness
main() {
    # Run tests
    [[ $(fibonacci 0) -eq 0 ]] || { echo "Test failed: fib(0)"; exit 1; }
    [[ $(fibonacci 1) -eq 1 ]] || { echo "Test failed: fib(1)"; exit 1; }
    [[ $(fibonacci 10) -eq 55 ]] || { echo "Test failed: fib(10)"; exit 1; }

    echo "All tests passed"
}

main "$@"
```

### Safety Requirements

1. **Strict Mode**: Always use `set -euo pipefail`
2. **Shellcheck Clean**: Zero warnings from `shellcheck`
3. **POSIX Compliance**: Prefer POSIX constructs when possible
4. **Input Validation**: Validate all arguments and inputs
5. **Error Handling**: Proper exit codes and error messages

### Testing Requirements

1. **Shellcheck**: Zero warnings/errors
2. **BATS Testing**: Use BATS framework for test automation
3. **Exit Code Validation**: Verify proper exit codes (0 success, non-zero failure)
4. **Integration Tests**: Test with various shells (bash, dash, zsh)

### Build Integration

```makefile
# Standard Bash Makefile pattern for each algorithm
SHELLCHECK = shellcheck
BATS = bats

all: test

lint:
	$(SHELLCHECK) algorithm.sh

test: lint
	$(BATS) test_algorithm.bats

clean:
	# Bash scripts don't produce artifacts
```

## Directory Structure

### Per-Algorithm Structure

```
examples/algorithms/001-fibonacci/
├── README.md                          # Problem statement
├── Makefile                          # Orchestration
├── spec.toml                         # Test cases
├── implementations/
│   ├── ruchy/                        # ✅ Existing
│   │   └── fibonacci.ruchy
│   ├── rust/                         # ✅ Existing (reference for decy)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   ├── python/                       # ✅ Existing
│   │   └── fibonacci.py
│   ├── c/                            # 🆕 NEW (decy source)
│   │   ├── Makefile
│   │   ├── fibonacci.c
│   │   ├── fibonacci.h
│   │   └── test_fibonacci.c
│   └── bash/                         # 🆕 NEW (bashrs source)
│       ├── Makefile
│       ├── fibonacci.sh
│       └── test_fibonacci.bats
└── transpilation/                    # 🆕 NEW (optional)
    ├── decy/                         # decy C→Rust output
    │   └── fibonacci_transpiled.rs
    └── bashrs/                       # bashrs Bash→Rust output
        └── fibonacci_transpiled.rs
```

## Benchmark Integration

### C Benchmarking

**Approach**: Use existing harness with C library wrapping
```rust
// harness/runner/src/c_benchmark.rs
use std::ffi::CString;
use std::os::raw::c_char;

#[link(name = "fibonacci_c")]
extern "C" {
    fn fibonacci(n: i32) -> i32;
}

pub fn benchmark_c_fibonacci(n: i32) -> i32 {
    unsafe { fibonacci(n) }
}
```

**Build**: Compile C to static library, link with Rust harness
```bash
gcc -c -fPIC fibonacci.c -o fibonacci.o
ar rcs libfibonacci_c.a fibonacci.o
```

### Bash Benchmarking

**Approach**: Shell out to Bash script, measure execution time
```rust
// harness/runner/src/bash_benchmark.rs
use std::process::Command;
use std::time::Instant;

pub fn benchmark_bash_fibonacci(n: i32) -> (i32, Duration) {
    let start = Instant::now();

    let output = Command::new("bash")
        .arg("implementations/bash/fibonacci.sh")
        .arg(n.to_string())
        .output()
        .expect("Failed to execute bash script");

    let duration = start.elapsed();
    let result: i32 = String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse()
        .expect("Failed to parse result");

    (result, duration)
}
```

## Transpilation Validation Workflow

### decy Validation (C → Rust)

**Step 1**: Use rosetta-ruchy C implementation as input
```bash
cd examples/algorithms/001-fibonacci/implementations/c/
decy fibonacci.c -o fibonacci_transpiled.rs
```

**Step 2**: Compare transpiled Rust with reference Rust
```bash
# Run both implementations with same test cases
diff <(cargo run --manifest-path=rust/Cargo.toml --example test) \
     <(cargo run --manifest-path=transpiled/Cargo.toml --example test)
```

**Step 3**: Benchmark transpiled vs reference
```bash
cd harness/runner
cargo run -- run examples/algorithms/001-fibonacci \
    --languages rust,c,decy_transpiled
```

### bashrs Validation (Bash → Rust → Bash)

**Step 1**: Use rosetta-ruchy Bash implementation as input
```bash
cd examples/algorithms/001-fibonacci/implementations/bash/
bashrs transpile fibonacci.sh -o fibonacci_transpiled.rs
```

**Step 2**: Compare transpiled Bash with original
```bash
bashrs generate-shell fibonacci_transpiled.rs -o fibonacci_purified.sh
diff fibonacci.sh fibonacci_purified.sh
```

**Step 3**: Verify correctness
```bash
# Both should produce identical results
bash fibonacci.sh 10
bash fibonacci_purified.sh 10
```

## Quality Gates for C/Bash

### C Quality Gates

1. ✅ **Compilation**: `gcc -std=c11 -Wall -Wextra -Werror` (zero warnings)
2. ✅ **Memory Safety**: `valgrind --leak-check=full` (zero leaks/errors)
3. ✅ **Static Analysis**: `cppcheck --enable=all` (zero warnings)
4. ✅ **Test Coverage**: `gcov` ≥80% line coverage
5. ✅ **Correctness**: All test cases pass, results match spec.toml
6. ✅ **Performance**: Within expected complexity bounds

### Bash Quality Gates

1. ✅ **Shellcheck**: `shellcheck` (zero warnings/errors)
2. ✅ **POSIX Compliance**: `checkbashisms` (POSIX-compliant)
3. ✅ **Safety**: `set -euo pipefail` enforced
4. ✅ **Testing**: BATS tests pass with 100% success rate
5. ✅ **Correctness**: All test cases pass, results match spec.toml
6. ✅ **Integration**: Works with bash, dash, zsh

## Sprint 45 Implementation Plan

### Sprint Goals

**Duration**: 1-2 weeks
**Objective**: Implement P0 core algorithms in C and Bash (8 algorithms)

### Ticket Breakdown

**Ticket 1: Infrastructure Setup** (1 day)
- [ ] Create C/Bash directory structure template
- [ ] Add C compilation to root Makefile
- [ ] Add Bash testing infrastructure (BATS)
- [ ] Update harness to support C/Bash benchmarking
- [ ] Create quality gate scripts for C/Bash

**Ticket 2: Simple Algorithms** (2 days - 4 algorithms)
- [ ] `001-fibonacci` - C + Bash implementations
- [ ] `004-binary-search` - C + Bash implementations
- [ ] `022-selection-sort` - C + Bash implementations
- [ ] `021-counting-sort` - C + Bash implementations

**Ticket 3: Array/Sorting Algorithms** (2 days - 4 algorithms)
- [ ] `002-quicksort` - C + Bash implementations
- [ ] `003-mergesort` - C + Bash implementations
- [ ] `018-heap-sort` - C + Bash implementations
- [ ] `019-radix-sort` - C + Bash implementations

**Ticket 4: Documentation & Integration** (1 day)
- [ ] Update README.md with C/Bash tier
- [ ] Document transpilation validation workflow
- [ ] Create decy integration guide
- [ ] Create bashrs integration guide
- [ ] Add C/Bash to CI/CD pipeline

### Success Criteria

✅ **P0 Complete**: 8 core algorithms implemented in both C and Bash
✅ **Quality Gates**: All C/Bash implementations pass quality gates
✅ **Benchmark Integration**: C/Bash results appear in benchmark reports
✅ **Transpiler Ready**: Implementations validated as suitable for transpiler input
✅ **Documentation**: Complete specifications and integration guides

## Future Phases

### Sprint 46: P1 Algorithms (10 algorithms)
- Graph algorithms (Dijkstra, graph coloring, etc.)
- Dynamic programming algorithms
- Complex data structures (red-black tree, etc.)

### Sprint 47: P2 Algorithms (4 algorithms)
- Advanced sorting algorithms
- Topological sort

### Sprint 48: Data Science Examples (4 examples)
- Selected data science examples where C/Bash make sense
- Focus on I/O, basic statistics, concurrency

### Sprint 49: Transpiler Integration
- Automated decy validation pipeline
- Automated bashrs validation pipeline
- Continuous transpilation testing in CI/CD

## Risks and Mitigations

### Risk 1: C Memory Safety Complexity
**Mitigation**:
- Start with simple algorithms (fibonacci, binary search)
- Mandatory valgrind checks in CI/CD
- Code review focused on memory safety

### Risk 2: Bash Performance Limitations
**Mitigation**:
- Accept that Bash will be slower (educational value for transpiler)
- Focus on correctness over performance
- Document performance expectations

### Risk 3: Maintenance Burden
**Mitigation**:
- Start with P0 only (8 algorithms)
- Evaluate before expanding to P1/P2
- Leverage sister projects for validation

## Sister Project Collaboration

### decy Integration Points

1. **Test Corpus**: Use rosetta-ruchy C implementations for testing
2. **Validation**: Compare transpiled Rust against reference Rust
3. **Benchmarking**: Performance comparison in rosetta-ruchy harness
4. **Documentation**: Share transpilation patterns and idioms

### bashrs Integration Points

1. **Test Corpus**: Use rosetta-ruchy Bash implementations for testing
2. **Safety Validation**: Demonstrate safety improvements in purified scripts
3. **POSIX Compliance**: Validate POSIX-compliant output
4. **Documentation**: Share Rust-to-Shell patterns

## Conclusion

Adding C and Bash implementations to rosetta-ruchy serves the dual purpose of:

1. **Comprehensive Benchmarking**: Comparing Ruchy against low-level (C) and scripting (Bash) languages
2. **Transpiler Validation**: Providing gold-standard corpus for decy and bashrs projects

This systematic approach ensures that rosetta-ruchy becomes the definitive validation corpus for the entire paiml ecosystem of transpilation tools.

---

**Next Steps**: Begin Sprint 45 with Ticket 1 (Infrastructure Setup)
