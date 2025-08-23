.PHONY: all build test bench clean format validate compare-performance

# Default target
all: build test

# Build all implementations
build:
	@echo "🔨 Building hash table implementations..."
	@$(MAKE) -C implementations/rust build
	@$(MAKE) -C implementations/c all  
	@go build -C implementations/go -o hash_table *.go
	@chmod +x implementations/python/hash_table.py
	@chmod +x implementations/javascript/hash_table.js
	@ruchy build implementations/ruchy/hash_table.ruchy
	@echo "🎯 Building v1.5.0 self-hosting hash generation..."
	@ruchy build implementations/ruchy/self_hosting_hashing.ruchy
	@echo "✅ All implementations built successfully"

# Run tests for all implementations
test:
	@echo "🧪 Running tests for all implementations..."
	@echo "Testing Ruchy implementation..."
	@ruchy test implementations/ruchy/hash_table.ruchy
	@echo "Testing Ruchy v1.5.0 self-hosting hash features..."  
	@ruchy test implementations/ruchy/self_hosting_hashing.ruchy
	@echo "Testing Rust implementation..."
	@cd implementations/rust && cargo test
	@echo "Testing Python implementation..."
	@cd implementations/python && python -m pytest test_hash_table.py -v
	@echo "Testing JavaScript implementation..."
	@cd implementations/javascript && node test.js
	@echo "Testing Go implementation..."
	@cd implementations/go && go test -v
	@echo "Testing C implementation..."
	@cd implementations/c && make test
	@echo "✅ All tests passed"

# Run benchmarks with v1.5.0 features
bench:
	@echo "⚡ Running hash table benchmarks..."
	@echo "🔍 Testing hash function quality analysis..."
	@ruchy bench.ruchy --include-hash-analysis
	@echo "🚀 Testing self-hosting hash generation performance..."
	@ruchy bench.ruchy --include-self-hosting-hashing

# Format all code  
format:
	@echo "🎨 Formatting code..."
	@ruchy format implementations/ruchy/hash_table.ruchy
	@ruchy format implementations/ruchy/self_hosting_hashing.ruchy  
	@cd implementations/rust && cargo fmt
	@cd implementations/python && black . && isort .
	@cd implementations/javascript && prettier --write *.js
	@cd implementations/go && go fmt ./...
	@cd implementations/c && clang-format -i *.c

# Validate implementations against spec
validate:
	@echo "✅ Validating hash table implementations..."
	@ruchy validate --spec spec.toml implementations/ruchy/hash_table.ruchy
	@echo "🔍 Validating v1.5.0 self-hosting hash features..."
	@ruchy ast implementations/ruchy/self_hosting_hashing.ruchy --verify-self-hosting
	@python3 ../../scripts/validate_spec.py spec.toml

# Generate performance comparison with hash analysis
compare-performance:
	@echo "📊 Generating performance comparison..."
	@ruchy compare.ruchy --output results/ --include-hash-metrics
	@echo "📈 Analyzing hash function quality across implementations..."
	@ruchy hash_analysis.ruchy --output results/hash_quality.json

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	@cd implementations/rust && cargo clean
	@cd implementations/go && rm -f hash_table
	@cd implementations/c && make clean
	@rm -rf results/cache/
	@rm -rf /tmp/ruchy_hash_generated*