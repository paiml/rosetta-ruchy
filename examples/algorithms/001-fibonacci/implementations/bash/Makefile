# Bash Fibonacci Makefile
# Part of rosetta-ruchy Sprint 45 Tier 0 implementations

SHELLCHECK = shellcheck
SCRIPT = fibonacci.sh

.PHONY: all test lint benchmark clean help

all: lint test

# Run shellcheck for static analysis
lint:
	@echo "Running shellcheck..."
	$(SHELLCHECK) $(SCRIPT)
	@echo "✓ Shellcheck passed"

# Run built-in test suite
test: lint
	@echo "Running Bash test suite..."
	./$(SCRIPT) test

# Run benchmark
benchmark: lint
	@echo "Running benchmarks..."
	./$(SCRIPT) benchmark 30

# Run specific fibonacci calculation
run:
	@./$(SCRIPT) $(N)

# Clean (no artifacts for Bash)
clean:
	@echo "No artifacts to clean for Bash implementation"

# Help
help:
	@echo "Bash Fibonacci Makefile"
	@echo "======================="
	@echo ""
	@echo "Targets:"
	@echo "  all        - Run lint and test (default)"
	@echo "  lint       - Run shellcheck static analysis"
	@echo "  test       - Run test suite"
	@echo "  benchmark  - Run performance benchmarks"
	@echo "  run N=<n>  - Calculate fibonacci(n)"
	@echo "  clean      - Clean artifacts (none for Bash)"
	@echo "  help       - Show this help"
	@echo ""
	@echo "Examples:"
	@echo "  make"
	@echo "  make test"
	@echo "  make benchmark"
	@echo "  make run N=10"
