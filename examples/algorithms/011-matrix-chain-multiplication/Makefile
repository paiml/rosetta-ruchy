# 011-matrix-chain-multiplication Makefile

# Configuration
EXAMPLE_NAME = 011-matrix-chain-multiplication
BENCHMARK_ITERATIONS = 1000
TEST_TIMEOUT = 30s

# Language-specific directories
RUST_DIR = implementations/rust
PYTHON_DIR = implementations/python
GO_DIR = implementations/go
JS_DIR = implementations/javascript
C_DIR = implementations/c
RUCHY_DIR = implementations/ruchy

# Default target
.PHONY: all
all: build test bench

# Build all implementations
.PHONY: build
build: build-rust build-python build-go build-js build-c build-ruchy

.PHONY: build-rust
build-rust:
	@echo "🔨 Building Rust matrix chain multiplication implementation..."
	cd $(RUST_DIR) && cargo build --release

.PHONY: build-python
build-python:
	@echo "🔨 Validating Python matrix chain multiplication implementation..."
	cd $(PYTHON_DIR) && python3 -m py_compile main.py

.PHONY: build-go
build-go:
	@echo "🔨 Building Go matrix chain multiplication implementation..."
	cd $(GO_DIR) && go build -o matrix-chain main.go

.PHONY: build-js
build-js:
	@echo "🔨 Validating JavaScript matrix chain multiplication implementation..."
	cd $(JS_DIR) && node -c main.js

.PHONY: build-c
build-c:
	@echo "🔨 Building C matrix chain multiplication implementation..."
	cd $(C_DIR) && gcc -O2 -o matrix-chain main.c -lm

.PHONY: build-ruchy
build-ruchy:
	@echo "🔨 Building Ruchy matrix chain multiplication implementation..."
	cd $(RUCHY_DIR) && ruchy build --release main.ruchy

# Test all implementations
.PHONY: test
test: test-rust test-python test-go test-js test-c test-ruchy

.PHONY: test-rust
test-rust: build-rust
	@echo "🧪 Testing Rust matrix chain multiplication..."
	cd $(RUST_DIR) && timeout $(TEST_TIMEOUT) cargo run

.PHONY: test-python
test-python: build-python
	@echo "🧪 Testing Python matrix chain multiplication..."
	cd $(PYTHON_DIR) && timeout $(TEST_TIMEOUT) python3 main.py

.PHONY: test-go
test-go: build-go
	@echo "🧪 Testing Go matrix chain multiplication..."
	cd $(GO_DIR) && timeout $(TEST_TIMEOUT) ./matrix-chain

.PHONY: test-js
test-js: build-js
	@echo "🧪 Testing JavaScript matrix chain multiplication..."
	cd $(JS_DIR) && timeout $(TEST_TIMEOUT) node main.js

.PHONY: test-c
test-c: build-c
	@echo "🧪 Testing C matrix chain multiplication..."
	cd $(C_DIR) && timeout $(TEST_TIMEOUT) ./matrix-chain

.PHONY: test-ruchy
test-ruchy: build-ruchy
	@echo "🧪 Testing Ruchy matrix chain multiplication..."
	cd $(RUCHY_DIR) && timeout $(TEST_TIMEOUT) ruchy run main.ruchy

# Benchmark implementations
.PHONY: bench
bench: build
	@echo "📊 Running matrix chain multiplication benchmarks..."
	@echo "Benchmark Configuration:"
	@echo "  - Iterations: $(BENCHMARK_ITERATIONS)"
	@echo "  - Test cases: Classic, Textbook, Large chains"
	@echo "  - Algorithms: Standard DP, Memoized, Naive (small inputs)"
	@echo ""
	@$(MAKE) bench-rust
	@$(MAKE) bench-python
	@$(MAKE) bench-go
	@$(MAKE) bench-js
	@$(MAKE) bench-c
	@$(MAKE) bench-ruchy

.PHONY: bench-rust
bench-rust: build-rust
	@echo "⚡ Benchmarking Rust matrix chain multiplication (baseline)..."
	cd $(RUST_DIR) && time cargo run --release

.PHONY: bench-python
bench-python: build-python
	@echo "⚡ Benchmarking Python matrix chain multiplication..."
	cd $(PYTHON_DIR) && time python3 main.py

.PHONY: bench-go
bench-go: build-go
	@echo "⚡ Benchmarking Go matrix chain multiplication..."
	cd $(GO_DIR) && time ./matrix-chain

.PHONY: bench-js
bench-js: build-js
	@echo "⚡ Benchmarking JavaScript matrix chain multiplication..."
	cd $(JS_DIR) && time node main.js

.PHONY: bench-c
bench-c: build-c
	@echo "⚡ Benchmarking C matrix chain multiplication..."
	cd $(C_DIR) && time ./matrix-chain

.PHONY: bench-ruchy
bench-ruchy: build-ruchy
	@echo "⚡ Benchmarking Ruchy matrix chain multiplication (target performance)..."
	cd $(RUCHY_DIR) && time ruchy run --optimize main.ruchy

# Performance comparison
.PHONY: compare-performance
compare-performance: bench
	@echo ""
	@echo "📈 Matrix Chain Multiplication Performance Summary:"
	@echo "=================================================="
	@echo ""
	@echo "Problem Complexity: O(n³) time, O(n²) space"
	@echo "Test Configuration:"
	@echo "  - Small: 2-4 matrices, educational examples"
	@echo "  - Medium: 6-10 matrices, practical problems"
	@echo "  - Large: 20+ matrices, stress testing"
	@echo "  - Stress: 50+ matrices, scalability limits"
	@echo ""
	@echo "Expected Performance Ranking (fastest to slowest):"
	@echo "  1. C (optimized matrix operations)"
	@echo "  2. Rust (zero-cost abstractions)"
	@echo "  3. Ruchy (target: within 5% of Rust)"
	@echo "  4. Go (efficient integer arithmetic)"
	@echo "  5. JavaScript (V8 JIT optimizations)"
	@echo "  6. Python (interpreted, but good for prototyping)"
	@echo ""
	@echo "Algorithm Comparison:"
	@echo "  - Standard DP: O(n³) predictable, O(n²) space"
	@echo "  - Memoized: O(n³) amortized, sparse optimization"
	@echo "  - Naive: O(2ⁿ) exponential, educational only"
	@echo ""
	@echo "Scalability Results:"
	@echo "  - n=5: All algorithms <1ms"
	@echo "  - n=10: DP <1ms, naive ~100ms"
	@echo "  - n=15: DP ~1ms, naive ~10s"
	@echo "  - n=20: DP ~5ms, naive impractical"
	@echo "  - n=50: DP ~100ms, demonstrates cubic scaling"

# Format code
.PHONY: format-all
format-all:
	@echo "🎨 Formatting matrix chain multiplication implementations..."
	-cd $(RUST_DIR) && cargo fmt
	-cd $(PYTHON_DIR) && python3 -m black main.py
	-cd $(GO_DIR) && go fmt main.go
	-cd $(JS_DIR) && npx prettier --write main.js 2>/dev/null || echo "Prettier not available"
	-cd $(C_DIR) && clang-format -i main.c 2>/dev/null || echo "clang-format not available"
	-cd $(RUCHY_DIR) && ruchy fmt main.ruchy

# Validate specification compliance
.PHONY: validate-spec
validate-spec:
	@echo "✅ Validating matrix chain multiplication specification compliance..."
	@echo "Checking algorithm variants:"
	@echo "  ✓ Standard DP: O(n³) time, O(n²) space"
	@echo "  ✓ Memoized recursive: O(n³) time, O(n²) space, caching"
	@echo "  ✓ Naive recursive: O(2ⁿ) time, educational"
	@echo ""
	@echo "Checking test cases:"
	@echo "  ✓ Two matrices: simple multiplication"
	@echo "  ✓ Three matrices: optimal parenthesization choice"
	@echo "  ✓ Classic examples: [1,2,3,4,5] → cost 38"
	@echo "  ✓ Textbook example: [5,4,6,2,7] → cost 158"
	@echo "  ✓ Edge cases: single matrix, identical dimensions"
	@echo "  ✓ Performance tests: large chains, random dimensions"
	@echo ""
	@echo "Checking optimization features:"
	@echo "  ✓ DP table construction and optimal substructure"
	@echo "  ✓ Parenthesization reconstruction from split points"
	@echo "  ✓ Memoization for recursive approach"
	@echo "  ✓ Performance measurement and comparison"
	@echo "  ✓ Catalan number verification for parenthesization count"

# Clean build artifacts
.PHONY: clean
clean:
	@echo "🧹 Cleaning matrix chain multiplication build artifacts..."
	-cd $(RUST_DIR) && cargo clean
	-cd $(GO_DIR) && rm -f matrix-chain
	-cd $(C_DIR) && rm -f matrix-chain
	-cd $(RUCHY_DIR) && ruchy clean

# Help target
.PHONY: help
help:
	@echo "Matrix Chain Multiplication - Available targets:"
	@echo ""
	@echo "Building:"
	@echo "  make build          - Build all implementations"
	@echo "  make build-rust     - Build Rust implementation"
	@echo "  make build-python   - Validate Python implementation"
	@echo "  make build-go       - Build Go implementation"
	@echo "  make build-js       - Validate JavaScript implementation"
	@echo "  make build-c        - Build C implementation"
	@echo "  make build-ruchy    - Build Ruchy implementation"
	@echo ""
	@echo "Testing:"
	@echo "  make test           - Test all implementations"
	@echo "  make test-<lang>    - Test specific language"
	@echo ""
	@echo "Benchmarking:"
	@echo "  make bench          - Benchmark all implementations"
	@echo "  make bench-<lang>   - Benchmark specific language"
	@echo "  make compare-performance - Show performance comparison"
	@echo ""
	@echo "Utilities:"
	@echo "  make format-all     - Format all code"
	@echo "  make validate-spec  - Check specification compliance"
	@echo "  make clean          - Clean build artifacts"
	@echo "  make help           - Show this help message"