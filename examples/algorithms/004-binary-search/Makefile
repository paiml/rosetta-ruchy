.PHONY: all build test bench clean format validate compare-performance

# Default target
all: build test

# Build all implementations
build:
	@echo "🔨 Building binary search implementations..."
	@$(MAKE) -C implementations/rust build
	@$(MAKE) -C implementations/c all
	@go build -C implementations/go -o binary_search *.go
	@chmod +x implementations/python/binary_search.py
	@chmod +x implementations/javascript/binary_search.js
	@ruchy build implementations/ruchy/binary_search.ruchy
	@echo "🎯 Building v1.5.0 self-hosting integration..."
	@ruchy build implementations/ruchy/compiler_integration.ruchy
	@echo "✅ All implementations built successfully"

# Run tests for all implementations
test:
	@echo "🧪 Running tests for all implementations..."
	@echo "Testing Ruchy implementation..."
	@ruchy test implementations/ruchy/binary_search.ruchy
	@echo "Testing Ruchy v1.5.0 self-hosting features..."
	@ruchy test implementations/ruchy/compiler_integration.ruchy
	@echo "Testing Rust implementation..."
	@cd implementations/rust && cargo test
	@echo "Testing Python implementation..."
	@cd implementations/python && python -m pytest test_binary_search.py -v
	@echo "Testing JavaScript implementation..."
	@cd implementations/javascript && node test.js
	@echo "Testing Go implementation..."
	@cd implementations/go && go test -v
	@echo "Testing C implementation..."
	@cd implementations/c && make test
	@echo "✅ All tests passed"

# Run benchmarks with v1.5.0 features
bench:
	@echo "⚡ Running benchmarks..."
	@echo "🚀 Testing self-hosting compiler performance..."
	@ruchy bench.ruchy --include-self-hosting

# Format all code
format:
	@echo "🎨 Formatting code..."
	@ruchy format implementations/ruchy/binary_search.ruchy
	@ruchy format implementations/ruchy/compiler_integration.ruchy
	@cd implementations/rust && cargo fmt
	@cd implementations/python && black . && isort .
	@cd implementations/javascript && prettier --write *.js
	@cd implementations/go && go fmt ./...
	@cd implementations/c && clang-format -i *.c

# Validate implementations against spec
validate:
	@echo "✅ Validating implementations..."
	@ruchy validate --spec spec.toml implementations/ruchy/binary_search.ruchy
	@echo "🔍 Validating v1.5.0 self-hosting features..."
	@ruchy ast implementations/ruchy/compiler_integration.ruchy --verify-self-hosting
	@python3 ../../scripts/validate_spec.py spec.toml

# Generate performance comparison
compare-performance:
	@echo "📊 Generating performance comparison..."
	@ruchy compare.ruchy --output results/ --include-compiler-metrics

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	@cd implementations/rust && cargo clean
	@cd implementations/go && rm -f binary_search
	@cd implementations/c && make clean
	@rm -rf results/cache/
	@rm -rf /tmp/ruchy_generated*