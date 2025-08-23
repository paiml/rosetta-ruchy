.PHONY: all build test bench clean format validate compare-performance

# Default target
all: build test

# Build all implementations
build:
	@echo "🔨 Building quicksort implementations..."
	@$(MAKE) -C implementations/rust build
	@$(MAKE) -C implementations/c all
	@go build -C implementations/go -o quicksort quicksort.go quicksort_test.go
	@chmod +x implementations/python/quicksort.py
	@chmod +x implementations/javascript/quicksort.js
	@ruchy build implementations/ruchy/quicksort.ruchy
	@echo "✅ All implementations built successfully"

# Run tests for all implementations
test:
	@echo "🧪 Running tests for all implementations..."
	@echo "Testing Ruchy implementation..."
	@ruchy test implementations/ruchy/quicksort.ruchy
	@echo "Testing Rust implementation..."
	@cd implementations/rust && cargo test
	@echo "Testing Python implementation..."
	@cd implementations/python && python -m pytest test_quicksort.py -v
	@echo "Testing JavaScript implementation..."
	@cd implementations/javascript && node test.js
	@echo "Testing Go implementation..."
	@cd implementations/go && go test -v
	@echo "Testing C implementation..."
	@cd implementations/c && make test
	@echo "✅ All tests passed"

# Run benchmarks
bench:
	@echo "⚡ Running benchmarks..."
	@ruchy bench.ruchy

# Format all code
format:
	@echo "🎨 Formatting code..."
	@ruchy format implementations/ruchy/quicksort.ruchy
	@cd implementations/rust && cargo fmt
	@cd implementations/python && black . && isort .
	@cd implementations/javascript && prettier --write *.js
	@cd implementations/go && go fmt ./...
	@cd implementations/c && clang-format -i *.c

# Validate implementations against spec
validate:
	@echo "✅ Validating implementations..."
	@ruchy validate --spec spec.toml implementations/ruchy/quicksort.ruchy
	@python3 ../../scripts/validate_spec.py spec.toml

# Generate performance comparison
compare-performance:
	@echo "📊 Generating performance comparison..."
	@ruchy compare.ruchy --output results/

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	@cd implementations/rust && cargo clean
	@cd implementations/go && rm -f quicksort
	@cd implementations/c && make clean
	@rm -rf results/cache/