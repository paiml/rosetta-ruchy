# Fibonacci Benchmark Makefile
# Orchestrates building and running benchmarks across all languages

.PHONY: all build test benchmark clean help

# Configuration
RUNNER := ../../../harness/runner/target/release/rosetta-runner
DOCKER_ORCHESTRATE := ../../../harness/docker/orchestrate.sh
RESULTS_DIR := results
TIMESTAMP := $(shell date +%Y%m%d_%H%M%S)
RESULTS_FILE := $(RESULTS_DIR)/benchmark_$(TIMESTAMP).json

# Languages to benchmark
LANGUAGES := rust go python javascript c

# Default target
all: build benchmark

# Help message
help:
	@echo "Fibonacci Benchmark Makefile"
	@echo "============================"
	@echo ""
	@echo "Targets:"
	@echo "  all        - Build and run all benchmarks"
	@echo "  build      - Build all language implementations"
	@echo "  test       - Run correctness tests"
	@echo "  benchmark  - Run performance benchmarks"
	@echo "  clean      - Clean build artifacts and results"
	@echo "  help       - Show this help message"
	@echo ""
	@echo "Language-specific targets:"
	@echo "  build-<lang>     - Build specific language"
	@echo "  test-<lang>      - Test specific language"
	@echo "  benchmark-<lang> - Benchmark specific language"
	@echo ""
	@echo "Available languages: $(LANGUAGES)"

# Create results directory
$(RESULTS_DIR):
	@mkdir -p $(RESULTS_DIR)

# Build all implementations
build: $(RESULTS_DIR)
	@echo "🔨 Building all implementations..."
	@for lang in $(LANGUAGES); do \
		$(MAKE) build-$$lang || exit 1; \
	done
	@echo "✅ All implementations built"

# Run correctness tests
test: build
	@echo "🧪 Running correctness tests..."
	@for lang in $(LANGUAGES); do \
		$(MAKE) test-$$lang || exit 1; \
	done
	@echo "✅ All tests passed"

# Run performance benchmarks
benchmark: build $(RESULTS_DIR)
	@echo "📊 Running performance benchmarks..."
	@echo "{\"timestamp\": \"$(TIMESTAMP)\", \"benchmarks\": [" > $(RESULTS_FILE)
	@first=true; \
	for lang in $(LANGUAGES); do \
		if [ "$$first" = true ]; then \
			first=false; \
		else \
			echo "," >> $(RESULTS_FILE); \
		fi; \
		$(MAKE) benchmark-$$lang 2>/dev/null | grep -E '^{' >> $(RESULTS_FILE); \
	done
	@echo "]}" >> $(RESULTS_FILE)
	@echo "✅ Benchmarks complete. Results: $(RESULTS_FILE)"

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	@for lang in $(LANGUAGES); do \
		$(MAKE) clean-$$lang 2>/dev/null || true; \
	done
	@rm -rf $(RESULTS_DIR)/*.tmp
	@echo "✅ Cleanup complete"

# --- Rust targets ---
build-rust:
	@echo "  Building Rust implementation..."
	@if [ -f implementations/rust/Cargo.toml ]; then \
		cd implementations/rust && cargo build --release; \
	fi

test-rust:
	@echo "  Testing Rust implementation..."
	@if [ -f implementations/rust/Cargo.toml ]; then \
		cd implementations/rust && cargo test; \
	fi

benchmark-rust:
	@echo "  Benchmarking Rust implementation..."
	@if [ -f implementations/rust/Cargo.toml ]; then \
		cd implementations/rust && cargo bench --quiet | tail -n 1; \
	fi

clean-rust:
	@if [ -d implementations/rust ]; then \
		cd implementations/rust && cargo clean; \
	fi

# --- Go targets ---
build-go:
	@echo "  Building Go implementation..."
	@if [ -f implementations/go/go.mod ]; then \
		cd implementations/go && go build -o fibonacci; \
	fi

test-go:
	@echo "  Testing Go implementation..."
	@if [ -f implementations/go/go.mod ]; then \
		cd implementations/go && go test; \
	fi

benchmark-go:
	@echo "  Benchmarking Go implementation..."
	@if [ -f implementations/go/go.mod ]; then \
		cd implementations/go && go test -bench=. -benchmem; \
	fi

clean-go:
	@if [ -d implementations/go ]; then \
		cd implementations/go && go clean; \
		rm -f implementations/go/fibonacci; \
	fi

# --- Python targets ---
build-python:
	@echo "  Building Python implementation..."
	@# Python doesn't need build step

test-python:
	@echo "  Testing Python implementation..."
	@if [ -f implementations/python/fibonacci.py ]; then \
		cd implementations/python && python3 -m pytest test_fibonacci.py 2>/dev/null || python3 test_fibonacci.py; \
	fi

benchmark-python:
	@echo "  Benchmarking Python implementation..."
	@if [ -f implementations/python/fibonacci.py ]; then \
		cd implementations/python && python3 -m timeit -s "from fibonacci import fib_iterative" "fib_iterative(100)"; \
	fi

clean-python:
	@if [ -d implementations/python ]; then \
		find implementations/python -name "*.pyc" -delete; \
		find implementations/python -name "__pycache__" -type d -delete; \
	fi

# --- JavaScript targets ---
build-javascript:
	@echo "  Building JavaScript implementation..."
	@if [ -f implementations/javascript/package.json ]; then \
		cd implementations/javascript && npm install; \
	fi

test-javascript:
	@echo "  Testing JavaScript implementation..."
	@if [ -f implementations/javascript/fibonacci.js ]; then \
		cd implementations/javascript && npm test 2>/dev/null || node test.js; \
	fi

benchmark-javascript:
	@echo "  Benchmarking JavaScript implementation..."
	@if [ -f implementations/javascript/fibonacci.js ]; then \
		cd implementations/javascript && node benchmark.js; \
	fi

clean-javascript:
	@if [ -d implementations/javascript ]; then \
		rm -rf implementations/javascript/node_modules; \
	fi

# --- C targets ---
build-c:
	@echo "  Building C implementation..."
	@if [ -f implementations/c/fibonacci.c ]; then \
		cd implementations/c && gcc -O3 -o fibonacci fibonacci.c; \
	fi

test-c:
	@echo "  Testing C implementation..."
	@if [ -f implementations/c/fibonacci ]; then \
		cd implementations/c && ./fibonacci test; \
	fi

benchmark-c:
	@echo "  Benchmarking C implementation..."
	@if [ -f implementations/c/fibonacci ]; then \
		cd implementations/c && time ./fibonacci benchmark; \
	fi

clean-c:
	@if [ -d implementations/c ]; then \
		rm -f implementations/c/fibonacci implementations/c/*.o; \
	fi

# Docker-based targets
docker-build:
	@$(DOCKER_ORCHESTRATE) build

docker-benchmark:
	@$(DOCKER_ORCHESTRATE) run -l all -e algorithms/001-fibonacci

docker-clean:
	@$(DOCKER_ORCHESTRATE) clean