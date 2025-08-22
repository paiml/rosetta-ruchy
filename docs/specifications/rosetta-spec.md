# Rosetta Ruchy Repository Specification

## Executive Summary

A polyglot benchmark suite demonstrating Ruchy's performance parity with Rust while maintaining Python-like ergonomics. The repository provides empirical evidence of zero-cost abstractions through systematic comparison across production workloads.

## Architecture Principles

1. **Measurable superiority** - Every example must demonstrate quantifiable advantage
2. **Minimal maintenance surface** - Tiered language support with clear ownership
3. **Reproducible benchmarks** - Statistical rigor with environmental controls
4. **Progressive complexity** - From toy problems to production systems

## Language Tiers

### Tier 1: Core (Full CI, All Examples)
- **Ruchy** - Reference implementation
- **Rust** - Performance baseline
- **Python** - Ergonomics baseline
- **JavaScript** - Ecosystem comparison
- **Go** - Concurrency patterns

### Tier 2: Community (Subset Coverage)
- **TypeScript**, **Java**, **C++**, **C#**, **Swift**
- Maintained by language experts
- Minimum 3 examples required

### Tier 3: Reference (Single Example)
- **Kotlin**, **Ruby**, **PHP**, **SQL**, **Elixir**, **Dart**, **R**, **Julia**, **Zig**, **Scala**
- Demonstration only, no CI requirements

## Repository Structure

```
rosetta-ruchy/
├── Makefile                     # Global orchestration
├── rust-toolchain.toml          # Pinned Rust version
├── .cargo/
│   └── config.toml              # Shared Cargo config
├── .github/
│   ├── workflows/
│   │   ├── tier1-ci.yml        # Core language CI
│   │   ├── tier2-ci.yml        # Community CI
│   │   ├── benchmarks.yml      # Performance suite
│   │   └── release.yml         # Documentation generation
│   ├── languages.yml            # Language tier definitions
│   └── CODEOWNERS              # Language maintainers
├── examples/
│   ├── algorithms/              # Classical CS problems
│   │   ├── 001-fibonacci/
│   │   ├── 002-quicksort/
│   │   ├── 003-btree/
│   │   ├── 004-raytrace/
│   │   └── 005-regex/
│   └── production/              # Real-world workloads
│       ├── 101-rest-api/
│       ├── 102-data-pipeline/
│       ├── 103-game-ecs/
│       ├── 104-compiler/
│       └── 105-actor-system/
├── harness/                     # Benchmark infrastructure
│   ├── runner/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs         # Statistical benchmark runner
│   │       ├── environment.rs  # CPU isolation, governor control
│   │       └── report.rs       # JSON/Markdown generation
│   └── docker/
│       ├── base.dockerfile     # Common dependencies
│       └── languages/
│           ├── ruchy.dockerfile
│           ├── rust.dockerfile
│           └── [...]
├── scripts/
│   ├── validate.sh              # Pre-commit validation
│   ├── benchmark.sh             # Local benchmark runner
│   └── report.py                # Comparison matrix generator
└── docs/
    ├── RULES.md                 # Implementation constraints
    ├── BENCHMARKING.md          # Methodology documentation
    └── results/
        └── latest.json          # Machine-readable results
```

## Example Structure

```
examples/algorithms/001-fibonacci/
├── README.md                    # Problem statement, complexity
├── Makefile                     # Example orchestration
├── spec.toml                    # Test cases, benchmark config
├── implementations/
│   ├── ruchy/
│   │   ├── Makefile            # Language-specific targets
│   │   ├── fibonacci.ruc
│   │   ├── fibonacci_test.ruc
│   │   ├── fibonacci_bench.ruc
│   │   └── Ruchy.toml
│   ├── rust/
│   │   ├── Makefile
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   └── main.rs
│   │   └── benches/
│   │       └── bench.rs
│   ├── python/
│   │   ├── Makefile
│   │   ├── fibonacci.py
│   │   ├── test_fibonacci.py
│   │   ├── bench_fibonacci.py
│   │   └── requirements.txt
│   └── [...]
└── results/
    └── benchmarks.json          # Cached results
```

## Makefile Architecture

### Global Makefile
```makefile
# Global orchestration
EXAMPLES := $(wildcard examples/*/*/)
LANGUAGES := ruchy rust python javascript go
DOCKER_RUN := docker run --rm -v $(PWD):/workspace

.PHONY: all test bench lint format clean docker-build

all: docker-build test bench

docker-build:
	@for lang in $(LANGUAGES); do \
		docker build -f harness/docker/languages/$$lang.dockerfile \
			-t rosetta-$$lang:latest .; \
	done

test:
	@$(MAKE) -C harness/runner test-all

bench:
	@$(MAKE) -C harness/runner bench-all

format:
	@for example in $(EXAMPLES); do \
		$(MAKE) -C $$example format-all; \
	done

clean:
	@for example in $(EXAMPLES); do \
		$(MAKE) -C $$example clean; \
	done
	@cargo clean

# Tier-specific targets
tier1-test:
	@for lang in ruchy rust python javascript go; do \
		$(MAKE) test-lang LANG=$$lang; \
	done

test-lang:
	@for example in $(EXAMPLES); do \
		if [ -d $$example/implementations/$(LANG) ]; then \
			$(MAKE) -C $$example test-$(LANG); \
		fi; \
	done

# Performance comparison
compare: bench
	@python scripts/report.py results/ > docs/results/comparison.md

# CI validation
validate:
	@./scripts/validate.sh
```

### Example-Level Makefile
```makefile
# examples/algorithms/001-fibonacci/Makefile
SPEC := spec.toml
LANGUAGES := $(wildcard implementations/*)
RESULTS_DIR := results

.PHONY: all test bench clean format-all

all: test bench

test:
	@for lang in $(LANGUAGES); do \
		$(MAKE) -C $$lang test || exit 1; \
	done

bench:
	@mkdir -p $(RESULTS_DIR)
	@for lang in $(LANGUAGES); do \
		$(MAKE) -C $$lang bench | tee $(RESULTS_DIR)/$$(basename $$lang).json; \
	done

format-all:
	@for lang in $(LANGUAGES); do \
		if [ -f $$lang/Makefile ]; then \
			$(MAKE) -C $$lang format 2>/dev/null || true; \
		fi; \
	done

clean:
	@rm -rf $(RESULTS_DIR)
	@for lang in $(LANGUAGES); do \
		$(MAKE) -C $$lang clean; \
	done

# Comparison targets
compare-performance:
	@cargo run --manifest-path ../../harness/runner/Cargo.toml -- \
		--spec $(SPEC) \
		--results $(RESULTS_DIR)

validate-spec:
	@cargo run --manifest-path ../../harness/runner/Cargo.toml -- \
		--validate $(SPEC)
```

### Language-Specific Makefile (Ruchy)
```makefile
# implementations/ruchy/Makefile
RUCHY := ruchy
RUCHY_FLAGS := --edition 2025
SPEC := ../../spec.toml

.PHONY: build test bench lint format clean verify

build:
	$(RUCHY) build $(RUCHY_FLAGS)

test: build
	$(RUCHY) test --property-tests 1000
	$(RUCHY) mutate --min-score 0.85

bench: build
	$(RUCHY) bench --export json

lint:
	$(RUCHY) clippy -- -W clippy::all
	$(RUCHY) tidy --check

format:
	$(RUCHY) fmt

verify:
	$(RUCHY) verify --smt-solver z3 --spec $(SPEC)

clean:
	$(RUCHY) clean
	rm -rf target/

# CI-specific targets
ci-test: verify test
ci-bench: bench
	@$(RUCHY) bench --compare-baseline rust
```

### Language-Specific Makefile (Rust)
```makefile
# implementations/rust/Makefile
CARGO := cargo
CARGO_FLAGS := --release
SPEC := ../../spec.toml

.PHONY: build test bench lint format clean

build:
	$(CARGO) build $(CARGO_FLAGS)

test: build
	$(CARGO) test $(CARGO_FLAGS)

bench: build
	$(CARGO) bench --bench bench -- --output-format bencher | \
		cargo run --manifest-path ../../../../harness/runner/Cargo.toml -- \
		--format rust-bench --export json

lint:
	$(CARGO) clippy -- -D warnings

format:
	$(CARGO) fmt

clean:
	$(CARGO) clean

# Optimization validation
check-opt:
	$(CARGO) rustc $(CARGO_FLAGS) -- --emit=llvm-ir
	@echo "LLVM IR generated in target/release/deps/"
```

## GitHub Actions Design

### Tier 1 CI Pipeline
```yaml
# .github/workflows/tier1-ci.yml
name: Tier 1 CI

on:
  push:
    branches: [main]
    paths:
      - 'examples/**/implementations/ruchy/**'
      - 'examples/**/implementations/rust/**'
      - 'examples/**/implementations/python/**'
      - 'examples/**/implementations/javascript/**'
      - 'examples/**/implementations/go/**'
  pull_request:
    branches: [main]

env:
  RUST_BACKTRACE: 1
  CARGO_TERM_COLOR: always

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Validate Specifications
        run: make validate

  test-matrix:
    needs: validate
    strategy:
      matrix:
        language: [ruchy, rust, python, javascript, go]
        example:
          - algorithms/001-fibonacci
          - algorithms/002-quicksort
          - algorithms/003-btree
        os: [ubuntu-latest]
    
    runs-on: ${{ matrix.os }}
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Language
        uses: ./.github/actions/setup-${{ matrix.language }}
        with:
          version: stable
      
      - name: Cache Dependencies
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo
            ~/.rustup
            ~/.cache/pip
            ~/.npm
            ~/go/pkg/mod
          key: ${{ matrix.language }}-${{ hashFiles('**/Cargo.lock', '**/requirements.txt', '**/package-lock.json', '**/go.sum') }}
      
      - name: Run Tests
        run: |
          cd examples/${{ matrix.example }}/implementations/${{ matrix.language }}
          make test
      
      - name: Check Formatting
        if: matrix.language == 'ruchy' || matrix.language == 'rust'
        run: |
          cd examples/${{ matrix.example }}/implementations/${{ matrix.language }}
          make format
          git diff --exit-code

  ruchy-verification:
    needs: validate
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Ruchy
        run: |
          curl -sSf https://ruchy-lang.org/install.sh | sh
          echo "$HOME/.ruchy/bin" >> $GITHUB_PATH
      
      - name: Install SMT Solvers
        run: |
          sudo apt-get update
          sudo apt-get install -y z3 cvc5
      
      - name: Verify All Ruchy Examples
        run: |
          for example in examples/*/*/implementations/ruchy; do
            echo "Verifying $example"
            make -C $example verify
          done
      
      - name: Mutation Testing
        run: |
          for example in examples/*/*/implementations/ruchy; do
            echo "Mutation testing $example"
            make -C $example ci-test
          done
```

### Benchmark Pipeline
```yaml
# .github/workflows/benchmarks.yml
name: Performance Benchmarks

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]
  schedule:
    - cron: '0 0 * * 0'  # Weekly full run
  workflow_dispatch:
    inputs:
      examples:
        description: 'Examples to benchmark (comma-separated)'
        required: false
        default: 'all'

jobs:
  benchmark:
    runs-on: [self-hosted, benchmark]  # Dedicated runner
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Benchmark Environment
        run: |
          sudo cpupower frequency-set --governor performance
          sudo sh -c 'echo 1 > /proc/sys/kernel/numa_balancing'
          sudo sh -c 'echo 0 > /proc/sys/kernel/randomize_va_space'
      
      - name: Build Docker Images
        run: make docker-build
      
      - name: Run Benchmarks
        run: |
          if [ "${{ github.event.inputs.examples }}" = "all" ]; then
            make bench
          else
            for example in $(echo "${{ github.event.inputs.examples }}" | tr ',' ' '); do
              make -C examples/$example bench
            done
          fi
      
      - name: Generate Report
        run: make compare
      
      - name: Upload Results
        uses: actions/upload-artifact@v3
        with:
          name: benchmark-results-${{ github.sha }}
          path: docs/results/
      
      - name: Comment PR
        if: github.event_name == 'pull_request'
        uses: actions/github-script@v7
        with:
          script: |
            const fs = require('fs');
            const report = fs.readFileSync('docs/results/comparison.md', 'utf8');
            
            // Extract key metrics
            const ruchyVsRust = report.match(/Ruchy vs Rust: ([\d.]+)x/);
            const ruchyVsPython = report.match(/Ruchy vs Python: ([\d.]+)x faster/);
            
            const comment = `## Benchmark Results
            
            ${ruchyVsRust ? `🚀 **Ruchy vs Rust**: ${ruchyVsRust[1]}x relative performance` : ''}
            ${ruchyVsPython ? `📈 **Ruchy vs Python**: ${ruchyVsPython[1]}x faster` : ''}
            
            <details>
            <summary>Full Report</summary>
            
            ${report}
            </details>`;
            
            await github.rest.issues.createComment({
              issue_number: context.issue.number,
              owner: context.repo.owner,
              repo: context.repo.repo,
              body: comment
            });
      
      - name: Check Performance Regression
        if: github.event_name == 'pull_request'
        run: |
          cargo run --manifest-path harness/runner/Cargo.toml -- \
            --check-regression \
            --baseline main \
            --threshold 5  # Allow 5% regression

  benchmark-production:
    runs-on: [self-hosted, benchmark]
    if: github.event_name == 'schedule' || github.event_name == 'workflow_dispatch'
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Production Environment
        run: |
          docker-compose -f harness/docker/compose.yml up -d
      
      - name: Run Production Benchmarks
        run: |
          for example in examples/production/*; do
            make -C $example bench-production
          done
      
      - name: Store Historical Data
        run: |
          cargo run --manifest-path harness/runner/Cargo.toml -- \
            --store-historical \
            --database postgresql://bench@localhost/rosetta
```

### Tier 2 Community CI
```yaml
# .github/workflows/tier2-ci.yml
name: Tier 2 Community CI

on:
  pull_request:
    paths:
      - 'examples/**/implementations/typescript/**'
      - 'examples/**/implementations/java/**'
      - 'examples/**/implementations/cpp/**'
      - 'examples/**/implementations/csharp/**'
      - 'examples/**/implementations/swift/**'
  schedule:
    - cron: '0 0 * * 1'  # Weekly validation

jobs:
  community-test:
    strategy:
      matrix:
        language: [typescript, java, cpp, csharp, swift]
      fail-fast: false  # Continue on failure
    
    runs-on: ubuntu-latest
    continue-on-error: true  # Non-blocking
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Check CODEOWNERS
        run: |
          if ! grep -q "${{ matrix.language }}" .github/CODEOWNERS; then
            echo "::warning::No maintainer for ${{ matrix.language }}"
          fi
      
      - name: Setup Language
        uses: ./.github/actions/setup-${{ matrix.language }}
        continue-on-error: true
      
      - name: Run Available Tests
        run: |
          for example in examples/*/*/implementations/${{ matrix.language }}; do
            if [ -f "$example/Makefile" ]; then
              make -C $example test || echo "::warning::Test failed for $example"
            fi
          done
      
      - name: Report Status
        if: always()
        run: |
          echo "## ${{ matrix.language }} Status" >> $GITHUB_STEP_SUMMARY
          echo "Tests completed with status: ${{ job.status }}" >> $GITHUB_STEP_SUMMARY
```

## Implementation Constraints

```markdown
# docs/RULES.md

## Fair Comparison Rules

1. **Standard Library Only**
   - No external dependencies except testing frameworks
   - No FFI to C libraries for performance
   - Native language features only

2. **Algorithm Implementation**
   - Same algorithmic approach across languages
   - Production-quality error handling required
   - No language-specific optimizations that change complexity class

3. **Code Constraints**
   - Single file implementation (<500 LOC)
   - No code generation or macros that obscure logic
   - Comments only for non-obvious optimizations

4. **Benchmark Environment**
   - Fixed CPU frequency (performance governor)
   - Isolated CPU cores
   - Minimum 1000 iterations
   - Statistical analysis required (std dev, percentiles)

5. **Memory Management**
   - Default allocator only
   - No manual memory pools
   - GC languages use default settings
```

## Benchmark Harness Design

```rust
// harness/runner/src/main.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize, Deserialize)]
struct BenchmarkResult {
    language: String,
    example: String,
    metrics: Metrics,
}

#[derive(Serialize, Deserialize)]
struct Metrics {
    mean_ns: u64,
    median_ns: u64,
    std_dev: u64,
    min_ns: u64,
    max_ns: u64,
    memory_kb: u64,
    binary_size_kb: Option<u64>,
    lines_of_code: u32,
}

impl BenchmarkRunner {
    pub fn run_isolated<F>(&self, f: F) -> Metrics 
    where 
        F: Fn() + Send + 'static
    {
        // Pin to isolated CPU
        self.set_cpu_affinity();
        
        // Warm up
        for _ in 0..self.config.warmup {
            black_box(f());
        }
        
        // Collect samples
        let mut samples = Vec::with_capacity(self.config.iterations);
        for _ in 0..self.config.iterations {
            let start = std::time::Instant::now();
            black_box(f());
            samples.push(start.elapsed().as_nanos() as u64);
        }
        
        // Statistical analysis
        samples.sort_unstable();
        let mean = samples.iter().sum::<u64>() / samples.len() as u64;
        let median = samples[samples.len() / 2];
        let variance = samples.iter()
            .map(|&x| ((x as f64 - mean as f64).powi(2)))
            .sum::<f64>() / samples.len() as f64;
        let std_dev = variance.sqrt() as u64;
        
        Metrics {
            mean_ns: mean,
            median_ns: median,
            std_dev,
            min_ns: samples[0],
            max_ns: samples[samples.len() - 1],
            memory_kb: self.measure_memory(),
            binary_size_kb: self.measure_binary_size(),
            lines_of_code: self.count_lines(),
        }
    }
}
```

## Success Criteria

1. **Performance**: Ruchy within 5% of Rust for CPU-bound tasks
2. **Memory**: Comparable heap usage to Rust (±10%)
3. **Ergonomics**: 30-50% fewer lines than Rust
4. **Correctness**: 100% spec compliance, verified by SMT solver
5. **Compilation**: <100ms incremental builds for 1KLOC

## Timeline

- **Week 1-2**: Core infrastructure (harness, Docker, CI)
- **Week 3-4**: First example (Fibonacci) in Tier 1 languages
- **Week 5-8**: Algorithms 1-5 in Tier 1
- **Week 9-12**: Production example (REST API) in Tier 1
- **Quarter 2**: Community expansion, Tier 2 languages
- **Quarter 3**: Full production suite
- **Quarter 4**: Performance dashboard, documentation