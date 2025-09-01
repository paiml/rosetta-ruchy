# Cross-Language Performance Benchmarking Harness
# Phase 11: Empirical Performance Validation

.PHONY: all benchmark-all analyze report clean

# Benchmark configuration
ITERATIONS ?= 1000
WARMUP ?= 100
ALGORITHMS := fibonacci quicksort binary-search hash-table graph-coloring knapsack

# Language implementations
RUCHY := ruchy
RUSTC := rustc
PYTHON := python3
NODE := node

all: benchmark-all analyze report

# Run all language benchmarks
benchmark-all: benchmark-ruchy benchmark-rust benchmark-python benchmark-js
	@echo "✅ All benchmarks complete"

# Ruchy benchmarks
benchmark-ruchy:
	@echo "🚀 Running Ruchy benchmarks..."
	@mkdir -p results/ruchy
	@for algo in $(ALGORITHMS); do \
		echo "  - Benchmarking $$algo..."; \
		$(RUCHY) run benchmark_$$algo.ruchy --iterations $(ITERATIONS) > results/ruchy/$$algo.json 2>&1 || true; \
	done

# Rust benchmarks
benchmark-rust:
	@echo "🦀 Running Rust benchmarks..."
	@mkdir -p results/rust
	@for algo in $(ALGORITHMS); do \
		if [ -f "rust/$$algo.rs" ]; then \
			echo "  - Benchmarking $$algo..."; \
			$(RUSTC) -O rust/$$algo.rs -o rust/$$algo && \
			./rust/$$algo --iterations $(ITERATIONS) > results/rust/$$algo.json 2>&1 || true; \
		fi; \
	done

# Python benchmarks
benchmark-python:
	@echo "🐍 Running Python benchmarks..."
	@mkdir -p results/python
	@for algo in $(ALGORITHMS); do \
		if [ -f "python/$$algo.py" ]; then \
			echo "  - Benchmarking $$algo..."; \
			$(PYTHON) python/$$algo.py --iterations $(ITERATIONS) > results/python/$$algo.json 2>&1 || true; \
		fi; \
	done

# JavaScript benchmarks
benchmark-js:
	@echo "📜 Running JavaScript benchmarks..."
	@mkdir -p results/javascript
	@for algo in $(ALGORITHMS); do \
		if [ -f "javascript/$$algo.js" ]; then \
			echo "  - Benchmarking $$algo..."; \
			$(NODE) javascript/$$algo.js --iterations $(ITERATIONS) > results/javascript/$$algo.json 2>&1 || true; \
		fi; \
	done

# Analyze benchmark results
analyze:
	@echo "📊 Analyzing benchmark results..."
	@$(RUCHY) run analyze_benchmarks.ruchy

# Generate performance report
report:
	@echo "📈 Generating performance report..."
	@$(RUCHY) run generate_report.ruchy > PERFORMANCE_REPORT.md

# Statistical validation
validate:
	@echo "🔬 Running statistical validation..."
	@$(RUCHY) run statistical_validation.ruchy

# Clean benchmark artifacts
clean:
	@echo "🧹 Cleaning benchmark artifacts..."
	@rm -rf results/
	@rm -f rust/fibonacci rust/quicksort rust/binary-search rust/hash-table rust/graph-coloring rust/knapsack
	@rm -f PERFORMANCE_REPORT.md

# Individual algorithm benchmarks
benchmark-%:
	@echo "Benchmarking $* across all languages..."
	@$(MAKE) benchmark-ruchy-$*
	@$(MAKE) benchmark-rust-$*
	@$(MAKE) benchmark-python-$*
	@$(MAKE) benchmark-js-$*

benchmark-ruchy-%:
	@mkdir -p results/ruchy
	@$(RUCHY) run benchmark_$*.ruchy --iterations $(ITERATIONS) > results/ruchy/$*.json

benchmark-rust-%:
	@mkdir -p results/rust
	@if [ -f "rust/$*.rs" ]; then \
		$(RUSTC) -O rust/$*.rs -o rust/$* && \
		./rust/$* --iterations $(ITERATIONS) > results/rust/$*.json; \
	fi

benchmark-python-%:
	@mkdir -p results/python
	@if [ -f "python/$*.py" ]; then \
		$(PYTHON) python/$*.py --iterations $(ITERATIONS) > results/python/$*.json; \
	fi

benchmark-js-%:
	@mkdir -p results/javascript
	@if [ -f "javascript/$*.js" ]; then \
		$(NODE) javascript/$*.js --iterations $(ITERATIONS) > results/javascript/$*.json; \
	fi

# Help target
help:
	@echo "Cross-Language Performance Benchmarking"
	@echo "======================================="
	@echo "Targets:"
	@echo "  all              - Run all benchmarks and generate report"
	@echo "  benchmark-all    - Run benchmarks for all languages"
	@echo "  benchmark-ruchy  - Run only Ruchy benchmarks"
	@echo "  benchmark-rust   - Run only Rust benchmarks"
	@echo "  benchmark-python - Run only Python benchmarks"
	@echo "  benchmark-js     - Run only JavaScript benchmarks"
	@echo "  analyze          - Analyze benchmark results"
	@echo "  report           - Generate performance report"
	@echo "  validate         - Run statistical validation"
	@echo "  clean            - Remove all benchmark artifacts"
	@echo ""
	@echo "Configuration:"
	@echo "  ITERATIONS=1000  - Number of benchmark iterations"
	@echo "  WARMUP=100       - Number of warmup iterations"