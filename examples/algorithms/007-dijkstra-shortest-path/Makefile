.PHONY: all build test bench clean format validate compare-performance

# Default target
all: build test

# Build all implementations
build:
	@echo "🔨 Building Dijkstra's shortest path implementations..."
	@$(MAKE) -C implementations/rust build
	@$(MAKE) -C implementations/c all
	@go build -C implementations/go -o dijkstra *.go
	@chmod +x implementations/python/dijkstra.py
	@chmod +x implementations/javascript/dijkstra.js
	@ruchy build implementations/ruchy/dijkstra.ruchy
	@echo "🎯 Building v1.5.0 concurrent graph operations..."
	@ruchy build implementations/ruchy/concurrent_dijkstra.ruchy
	@echo "✅ All implementations built successfully"

# Run tests for all implementations
test:
	@echo "🧪 Running tests for all implementations..."
	@echo "Testing Ruchy implementation..."
	@ruchy test implementations/ruchy/dijkstra.ruchy
	@echo "Testing Ruchy v1.5.0 concurrent operations..."
	@ruchy test implementations/ruchy/concurrent_dijkstra.ruchy
	@echo "Testing Rust implementation..."
	@cd implementations/rust && cargo test
	@echo "Testing Python implementation..."
	@cd implementations/python && python -m pytest test_dijkstra.py -v
	@echo "Testing JavaScript implementation..."
	@cd implementations/javascript && node test.js
	@echo "Testing Go implementation..."
	@cd implementations/go && go test -v
	@echo "Testing C implementation..."
	@cd implementations/c && make test
	@echo "✅ All tests passed"

# Run benchmarks with concurrent operations
bench:
	@echo "⚡ Running shortest path benchmarks..."
	@echo "🔍 Testing single-source performance..."
	@ruchy bench.ruchy --single-source
	@echo "🚀 Testing parallel multi-source with v1.5.0..."
	@ruchy bench.ruchy --parallel-sources

# Format all code
format:
	@echo "🎨 Formatting code..."
	@ruchy format implementations/ruchy/dijkstra.ruchy
	@ruchy format implementations/ruchy/concurrent_dijkstra.ruchy
	@cd implementations/rust && cargo fmt
	@cd implementations/python && black . && isort .
	@cd implementations/javascript && prettier --write *.js
	@cd implementations/go && go fmt ./...
	@cd implementations/c && clang-format -i *.c

# Validate implementations against spec
validate:
	@echo "✅ Validating Dijkstra implementations..."
	@ruchy validate --spec spec.toml implementations/ruchy/dijkstra.ruchy
	@echo "🔍 Validating v1.5.0 concurrent features..."
	@ruchy ast implementations/ruchy/concurrent_dijkstra.ruchy --verify-concurrency
	@python3 ../../scripts/validate_spec.py spec.toml

# Generate performance comparison with path analysis
compare-performance:
	@echo "📊 Generating performance comparison..."
	@ruchy compare.ruchy --output results/ --include-path-quality
	@echo "📈 Analyzing algorithm efficiency across graph types..."
	@ruchy graph_analysis.ruchy --output results/graph_metrics.json

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	@cd implementations/rust && cargo clean
	@cd implementations/go && rm -f dijkstra
	@cd implementations/c && make clean
	@rm -rf results/cache/
	@rm -rf /tmp/dijkstra_graph_*