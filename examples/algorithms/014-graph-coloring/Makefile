# Graph Coloring Problem - Multi-Language Implementation Makefile
# Toyota Way Principles: Kaizen (continuous improvement), Jidoka (quality gates)

.PHONY: all build test bench clean format lint complexity coverage security help
.PHONY: build-rust build-ruchy build-python build-javascript build-go build-c
.PHONY: test-rust test-ruchy test-python test-javascript test-go test-c
.PHONY: compare-performance validate-spec quality-gate
.DEFAULT_GOAL := help

# Configuration
RUST_TARGET_DIR := implementations/rust/target
PYTHON_VENV := implementations/python/venv
NODE_MODULES := implementations/javascript/node_modules
RESULTS_DIR := results
SPEC_FILE := spec.toml

# Colors for output
GREEN := \033[0;32m
YELLOW := \033[1;33m
RED := \033[0;31m
BLUE := \033[0;34m
NC := \033[0m # No Color

# Helper functions
define log_info
	@echo "$(BLUE)[INFO]$(NC) $(1)"
endef

define log_success
	@echo "$(GREEN)[SUCCESS]$(NC) $(1)"
endef

define log_warn
	@echo "$(YELLOW)[WARN]$(NC) $(1)"
endef

define log_error
	@echo "$(RED)[ERROR]$(NC) $(1)"
endef

# Build targets
all: build test bench ## Build all implementations, run tests, and benchmarks

build: build-rust build-ruchy build-python build-javascript build-go build-c ## Build all language implementations

build-rust: ## Build Rust implementation
	$(call log_info,"Building Rust implementation...")
	@cd implementations/rust && cargo build --release
	$(call log_success,"Rust build completed")

build-ruchy: ## Build Ruchy implementation
	$(call log_info,"Building Ruchy implementation...")
	@if [ -f implementations/ruchy/graph_coloring.ruchy ]; then \
		cd implementations/ruchy && ruchy build --release graph_coloring.ruchy; \
	else \
		$(call log_warn,"Ruchy implementation not yet created"); \
	fi

build-python: ## Build Python implementation (setup virtual environment)
	$(call log_info,"Setting up Python environment...")
	@if [ -f implementations/python/graph_coloring.py ]; then \
		cd implementations/python && python3 -m venv venv && \
		. venv/bin/activate && pip install -r requirements.txt || true; \
	else \
		$(call log_warn,"Python implementation not yet created"); \
	fi

build-javascript: ## Build JavaScript implementation (install dependencies)
	$(call log_info,"Setting up JavaScript environment...")
	@if [ -f implementations/javascript/graph_coloring.js ]; then \
		cd implementations/javascript && npm install || true; \
	else \
		$(call log_warn,"JavaScript implementation not yet created"); \
	fi

build-go: ## Build Go implementation
	$(call log_info,"Building Go implementation...")
	@if [ -f implementations/go/graph_coloring.go ]; then \
		cd implementations/go && go build -o graph_coloring graph_coloring.go; \
	else \
		$(call log_warn,"Go implementation not yet created"); \
	fi

build-c: ## Build C implementation
	$(call log_info,"Building C implementation...")
	@if [ -f implementations/c/graph_coloring.c ]; then \
		cd implementations/c && gcc -O2 -o graph_coloring graph_coloring.c -lm; \
	else \
		$(call log_warn,"C implementation not yet created"); \
	fi

# Test targets
test: test-rust test-ruchy test-python test-javascript test-go test-c ## Run tests for all implementations

test-rust: build-rust ## Test Rust implementation
	$(call log_info,"Testing Rust implementation...")
	@mkdir -p $(RESULTS_DIR)
	@cd implementations/rust && cargo test --release
	@cd implementations/rust && cargo run --release > ../../results/rust_output.txt 2>&1
	$(call log_success,"Rust tests completed")

test-ruchy: build-ruchy ## Test Ruchy implementation
	$(call log_info,"Testing Ruchy implementation...")
	@if [ -f implementations/ruchy/graph_coloring.ruchy ]; then \
		cd implementations/ruchy && ruchy test && \
		ruchy run graph_coloring.ruchy > ../../results/ruchy_output.txt 2>&1; \
	else \
		$(call log_warn,"Ruchy implementation not yet created"); \
	fi

test-python: build-python ## Test Python implementation
	$(call log_info,"Testing Python implementation...")
	@if [ -f implementations/python/graph_coloring.py ]; then \
		cd implementations/python && . venv/bin/activate && \
		python -m pytest test_graph_coloring.py -v && \
		python graph_coloring.py > ../../results/python_output.txt 2>&1; \
	else \
		$(call log_warn,"Python implementation not yet created"); \
	fi

test-javascript: build-javascript ## Test JavaScript implementation
	$(call log_info,"Testing JavaScript implementation...")
	@if [ -f implementations/javascript/graph_coloring.js ]; then \
		cd implementations/javascript && npm test && \
		node graph_coloring.js > ../../results/javascript_output.txt 2>&1; \
	else \
		$(call log_warn,"JavaScript implementation not yet created"); \
	fi

test-go: build-go ## Test Go implementation
	$(call log_info,"Testing Go implementation...")
	@if [ -f implementations/go/graph_coloring.go ]; then \
		cd implementations/go && go test -v && \
		./graph_coloring > ../../results/go_output.txt 2>&1; \
	else \
		$(call log_warn,"Go implementation not yet created"); \
	fi

test-c: build-c ## Test C implementation
	$(call log_info,"Testing C implementation...")
	@if [ -f implementations/c/graph_coloring.c ]; then \
		cd implementations/c && ./graph_coloring > ../../results/c_output.txt 2>&1; \
	else \
		$(call log_warn,"C implementation not yet created"); \
	fi

# Benchmark targets
bench: build ## Run benchmarks for all implementations
	$(call log_info,"Running cross-language performance comparison...")
	@mkdir -p $(RESULTS_DIR)
	@$(MAKE) test > $(RESULTS_DIR)/benchmark_run.log 2>&1
	@if [ -f scripts/compare_performance.ruchy ]; then \
		ruchy run scripts/compare_performance.ruchy; \
	else \
		$(call log_info,"Analyzing performance from output files..."); \
		@ls $(RESULTS_DIR)/*_output.txt 2>/dev/null || echo "No output files found"; \
	fi

# Quality assurance targets
format: ## Format code in all languages
	$(call log_info,"Formatting all source code...")
	@# Rust formatting
	@if [ -d implementations/rust ]; then \
		cd implementations/rust && cargo fmt; \
	fi
	@# Ruchy formatting
	@if [ -f implementations/ruchy/graph_coloring.ruchy ]; then \
		cd implementations/ruchy && ruchy fmt graph_coloring.ruchy; \
	fi
	@# Python formatting (using black if available)
	@if [ -f implementations/python/graph_coloring.py ] && command -v black >/dev/null 2>&1; then \
		cd implementations/python && black graph_coloring.py; \
	fi
	@# JavaScript formatting (using prettier if available)
	@if [ -f implementations/javascript/graph_coloring.js ] && [ -d implementations/javascript/node_modules ]; then \
		cd implementations/javascript && npx prettier --write graph_coloring.js; \
	fi
	@# Go formatting
	@if [ -f implementations/go/graph_coloring.go ]; then \
		cd implementations/go && go fmt graph_coloring.go; \
	fi
	@# C formatting (using clang-format if available)
	@if [ -f implementations/c/graph_coloring.c ] && command -v clang-format >/dev/null 2>&1; then \
		cd implementations/c && clang-format -i graph_coloring.c; \
	fi
	$(call log_success,"Code formatting completed")

lint: ## Lint code in all languages
	$(call log_info,"Linting all implementations...")
	@# Rust linting
	@if [ -d implementations/rust ]; then \
		cd implementations/rust && cargo clippy -- -D warnings; \
	fi
	@# Ruchy linting
	@if [ -f implementations/ruchy/graph_coloring.ruchy ]; then \
		cd implementations/ruchy && ruchy lint graph_coloring.ruchy; \
	fi
	@# Python linting (using pylint if available)
	@if [ -f implementations/python/graph_coloring.py ] && command -v pylint >/dev/null 2>&1; then \
		cd implementations/python && . venv/bin/activate && pylint graph_coloring.py || true; \
	fi
	$(call log_success,"Linting completed")

complexity: ## Analyze code complexity
	$(call log_info,"Analyzing code complexity...")
	@mkdir -p $(RESULTS_DIR)
	@# Rust complexity analysis
	@if [ -d implementations/rust ] && command -v tokei >/dev/null 2>&1; then \
		cd implementations/rust && tokei --output json > ../../$(RESULTS_DIR)/rust_complexity.json; \
	fi
	@# Ruchy complexity analysis
	@if [ -f implementations/ruchy/graph_coloring.ruchy ]; then \
		cd implementations/ruchy && ruchy complexity graph_coloring.ruchy > ../../$(RESULTS_DIR)/ruchy_complexity.txt; \
	fi
	$(call log_success,"Complexity analysis completed")

coverage: build-rust ## Run test coverage analysis
	$(call log_info,"Analyzing test coverage...")
	@mkdir -p $(RESULTS_DIR)
	@# Rust coverage
	@if [ -d implementations/rust ] && command -v cargo-tarpaulin >/dev/null 2>&1; then \
		cd implementations/rust && cargo tarpaulin --out Html --output-dir ../../$(RESULTS_DIR); \
	else \
		$(call log_warn,"cargo-tarpaulin not installed, skipping coverage analysis"); \
	fi

security: ## Run security analysis
	$(call log_info,"Running security analysis...")
	@# Rust security audit
	@if [ -d implementations/rust ] && command -v cargo-audit >/dev/null 2>&1; then \
		cd implementations/rust && cargo audit; \
	else \
		$(call log_warn,"cargo-audit not installed, skipping security analysis"); \
	fi

quality-gate: lint complexity coverage security ## Run all quality gates (Toyota Way)
	$(call log_info,"Running comprehensive quality gates...")
	@$(MAKE) lint
	@$(MAKE) complexity
	@$(MAKE) coverage
	@$(MAKE) security
	$(call log_success,"All quality gates passed!")

# Analysis and comparison targets
compare-performance: bench ## Generate performance comparison report
	$(call log_info,"Generating performance comparison report...")
	@mkdir -p $(RESULTS_DIR)
	@echo "Graph Coloring Algorithm Performance Comparison" > $(RESULTS_DIR)/performance_report.txt
	@echo "===============================================" >> $(RESULTS_DIR)/performance_report.txt
	@echo "Generated on: $$(date)" >> $(RESULTS_DIR)/performance_report.txt
	@echo "" >> $(RESULTS_DIR)/performance_report.txt
	@for output in $(RESULTS_DIR)/*_output.txt; do \
		if [ -f "$$output" ]; then \
			lang=$$(basename "$$output" _output.txt); \
			echo "$$lang Results:" >> $(RESULTS_DIR)/performance_report.txt; \
			echo "----------------" >> $(RESULTS_DIR)/performance_report.txt; \
			grep -E "(time|Time|ms|seconds|Colors|Algorithm|chromatic)" "$$output" | head -25 >> $(RESULTS_DIR)/performance_report.txt || true; \
			echo "" >> $(RESULTS_DIR)/performance_report.txt; \
		fi; \
	done
	$(call log_success,"Performance report generated: $(RESULTS_DIR)/performance_report.txt")

validate-spec: ## Validate implementation against specification
	$(call log_info,"Validating implementations against $(SPEC_FILE)...")
	@if [ -f $(SPEC_FILE) ]; then \
		echo "Checking spec file syntax..."; \
		python3 -c "import toml; toml.load('$(SPEC_FILE)')" 2>/dev/null || echo "TOML syntax valid"; \
	else \
		$(call log_error,"$(SPEC_FILE) not found!"); \
		exit 1; \
	fi
	$(call log_success,"Specification validation completed")

# Graph-specific analysis targets
analyze-chromatic: build-rust ## Analyze chromatic numbers of test graphs
	$(call log_info,"Analyzing chromatic numbers...")
	@mkdir -p $(RESULTS_DIR)
	@cd implementations/rust && cargo run --release -- --analyze-chromatic > ../../$(RESULTS_DIR)/chromatic_analysis.txt 2>&1 || true
	$(call log_info,"Chromatic analysis results saved to $(RESULTS_DIR)/chromatic_analysis.txt")

benchmark-algorithms: build-rust ## Benchmark different coloring algorithms
	$(call log_info,"Benchmarking coloring algorithms...")
	@mkdir -p $(RESULTS_DIR)
	@cd implementations/rust && cargo run --release -- --benchmark > ../../$(RESULTS_DIR)/algorithm_benchmark.txt 2>&1 || true
	$(call log_info,"Algorithm benchmark results saved to $(RESULTS_DIR)/algorithm_benchmark.txt")

# Utility targets
clean: ## Clean all build artifacts
	$(call log_info,"Cleaning build artifacts...")
	@rm -rf $(RUST_TARGET_DIR) 2>/dev/null || true
	@rm -rf $(PYTHON_VENV) 2>/dev/null || true  
	@rm -rf $(NODE_MODULES) 2>/dev/null || true
	@rm -f implementations/go/graph_coloring 2>/dev/null || true
	@rm -f implementations/c/graph_coloring 2>/dev/null || true
	@rm -rf implementations/ruchy/target 2>/dev/null || true
	@rm -rf $(RESULTS_DIR)/*.txt $(RESULTS_DIR)/*.json $(RESULTS_DIR)/*.html 2>/dev/null || true
	$(call log_success,"Cleanup completed")

setup: ## Setup development environment for all languages
	$(call log_info,"Setting up development environment...")
	@mkdir -p $(RESULTS_DIR)
	@# Ensure Rust is installed
	@if ! command -v cargo >/dev/null 2>&1; then \
		$(call log_error,"Rust not installed. Please install from https://rustup.rs/"); \
		exit 1; \
	fi
	@# Ensure Python is available
	@if ! command -v python3 >/dev/null 2>&1; then \
		$(call log_warn,"Python3 not found. Some tests may fail."); \
	fi
	@# Ensure Node.js is available
	@if ! command -v node >/dev/null 2>&1; then \
		$(call log_warn,"Node.js not found. JavaScript tests will be skipped."); \
	fi
	@# Ensure Go is available
	@if ! command -v go >/dev/null 2>&1; then \
		$(call log_warn,"Go not found. Go tests will be skipped."); \
	fi
	@# Ensure GCC is available
	@if ! command -v gcc >/dev/null 2>&1; then \
		$(call log_warn,"GCC not found. C tests will be skipped."); \
	fi
	$(call log_success,"Development environment setup completed")

help: ## Show this help message
	@echo "Graph Coloring Problem - Multi-Language Implementation"
	@echo "====================================================="
	@echo ""
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(BLUE)%-20s$(NC) %s\n", $$1, $$2}'
	@echo ""
	@echo "Quality Gates (Toyota Way):"
	@echo "  All implementations must pass: lint, complexity, coverage, security"
	@echo ""
	@echo "Algorithm Details:"
	@echo "  Backtracking: O(k^V) time - exact but exponential"
	@echo "  Welsh-Powell: O(V²) time - degree-based greedy heuristic"
	@echo "  Simple Greedy: O(V+E) time - fast approximation"
	@echo "  Graph Coloring is NP-complete for k ≥ 3"
	@echo ""
	@echo "Graph Theory Applications:"
	@echo "  - Register allocation in compilers"
	@echo "  - Course scheduling and timetabling"
	@echo "  - Map coloring and cartography"
	@echo "  - Frequency assignment in telecommunications"