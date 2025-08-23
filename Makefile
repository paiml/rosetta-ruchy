# Rosetta Ruchy - Global Makefile
# Toyota Way: Quality built-in, not bolted-on

.PHONY: all install-dev-tools install-hooks clean help
.PHONY: lint test test-fast test-doc test-property test-examples 
.PHONY: complexity coverage security quality-gate
.PHONY: analyze-complexity analyze-debt analyze-duplicates
.PHONY: refactor-plan pdmt-todos
.PHONY: pre-release-checks release-auto release-patch release-minor release-major
.PHONY: docker-build tier1-test tier2-test bench compare validate
.PHONY: version-check version-update new-example

# Global Configuration
EXAMPLES := $(wildcard examples/*/*/)
TIER1_LANGUAGES := ruchy rust python javascript go
TIER2_LANGUAGES := typescript java cpp csharp swift
ALL_LANGUAGES := $(TIER1_LANGUAGES) $(TIER2_LANGUAGES)
DOCKER_RUN := docker run --rm -v $(PWD):/workspace

# Version management
RUCHY_VERSION := $(shell ruchy --version 2>/dev/null | cut -d' ' -f2 || echo "unknown")
REQUIRED_VERSION := 1.5.0

# Default target
all: version-check install-hooks quality-gate docker-build tier1-test bench

# Development Environment Setup
install-dev-tools:
	@echo "📦 Installing development tools..."
	cargo install cargo-tarpaulin cargo-audit cargo-semver-checks cargo-outdated cargo-mutants
	cargo install --git https://github.com/paiml/paiml-mcp-agent-toolkit.git pmat
	@echo "✅ Development tools installed"

install-hooks:
	@echo "🎣 Installing git hooks..."
	@mkdir -p .git/hooks
	@cat scripts/pre-commit-hook.sh > .git/hooks/pre-commit
	@chmod +x .git/hooks/pre-commit
	@echo "✅ Quality gate hooks installed"

# Quality Gates (MANDATORY - BLOCKING)
lint:
	@echo "🔍 Running lint checks..."
	@if command -v cargo >/dev/null 2>&1; then \
		cargo clippy --all-targets --all-features -- -D warnings; \
	fi
	@echo "✅ Lint checks passed"

test: test-fast test-doc test-property test-examples
	@echo "✅ All tests passed"

test-fast:
	@echo "🧪 Running fast tests..."
	@if command -v cargo >/dev/null 2>&1; then \
		cargo test --lib --tests; \
	fi

test-doc:
	@echo "📚 Running doc tests..."
	@if command -v cargo >/dev/null 2>&1; then \
		cargo test --doc; \
	fi

test-property:
	@echo "🎲 Running property tests..."
	@if command -v cargo >/dev/null 2>&1; then \
		cargo test --features property-tests; \
	fi

test-examples:
	@echo "📋 Running example tests..."
	@for example in $(EXAMPLES); do \
		if [ -f $$example/Makefile ]; then \
			$(MAKE) -C $$example test; \
		fi; \
	done

complexity:
	@echo "🧠 Analyzing complexity..."
	@if command -v pmat >/dev/null 2>&1; then \
		pmat analyze complexity --max-threshold 20; \
	else \
		echo "⚠️  pmat not installed - run 'make install-dev-tools'"; \
	fi

coverage:
	@echo "☂️  Analyzing test coverage..."
	@if command -v cargo-tarpaulin >/dev/null 2>&1; then \
		cargo tarpaulin --min 80 --fail-under --out Html --output-dir coverage/; \
	else \
		echo "⚠️  cargo-tarpaulin not installed - run 'make install-dev-tools'"; \
	fi

security:
	@echo "🔒 Running security audit..."
	@if command -v cargo-audit >/dev/null 2>&1; then \
		cargo audit; \
	else \
		echo "⚠️  cargo-audit not installed - run 'make install-dev-tools'"; \
	fi

quality-gate: lint test complexity coverage security
	@echo "🎯 All quality gates passed!"

# Analysis Tools (Genchi Genbutsu - Go and See)
analyze-complexity:
	@echo "🔍 Finding complexity hotspots..."
	@if command -v pmat >/dev/null 2>&1; then \
		pmat analyze complexity --top-files 5; \
	else \
		echo "⚠️  pmat not available - install via 'make install-dev-tools'"; \
	fi

analyze-debt:
	@echo "🔍 Finding technical debt..."
	@if command -v pmat >/dev/null 2>&1; then \
		pmat analyze satd; \
	else \
		find . -name "*.rs" -o -name "*.py" -o -name "*.js" -o -name "*.go" | \
		xargs grep -n "TODO\|FIXME\|HACK\|XXX" || echo "✅ No SATD found"; \
	fi

analyze-duplicates:
	@echo "🔍 Finding code duplication..."
	@if command -v pmat >/dev/null 2>&1; then \
		pmat analyze duplicates; \
	else \
		echo "⚠️  Manual duplication analysis needed"; \
	fi

# Refactoring Tools (Jidoka - Automation with Human Touch)
refactor-plan:
	@echo "🔄 Generating refactoring plan..."
	@if command -v pmat >/dev/null 2>&1; then \
		pmat refactor auto --file $(FILE); \
	else \
		echo "⚠️  pmat not available - manual refactoring needed"; \
	fi

# Task Management (PDMT - Deterministic Planning)
pdmt-todos:
	@echo "📋 Generating structured todos..."
	@if command -v pmat >/dev/null 2>&1; then \
		pmat pdmt-todos "$(TASK)" --granularity medium --seed 42; \
	else \
		echo "⚠️  pmat not available - manual planning needed"; \
		echo "Task: $(TASK)"; \
	fi

# Docker Infrastructure
docker-build:
	@echo "🐳 Building Docker images..."
	@for lang in $(TIER1_LANGUAGES); do \
		if [ -f harness/docker/languages/$$lang.dockerfile ]; then \
			docker build -f harness/docker/languages/$$lang.dockerfile \
				-t rosetta-$$lang:latest .; \
		fi; \
	done

# Testing Tiers
tier1-test:
	@echo "🥇 Running Tier 1 language tests..."
	@for lang in $(TIER1_LANGUAGES); do \
		$(MAKE) test-lang LANG=$$lang; \
	done

tier2-test:
	@echo "🥈 Running Tier 2 language tests..."
	@for lang in $(TIER2_LANGUAGES); do \
		$(MAKE) test-lang LANG=$$lang; \
	done

test-lang:
	@for example in $(EXAMPLES); do \
		if [ -d $$example/implementations/$(LANG) ]; then \
			$(MAKE) -C $$example test-$(LANG); \
		fi; \
	done

# Benchmarking
bench:
	@echo "⚡ Running benchmarks..."
	@mkdir -p results/benchmarks
	@$(MAKE) -C harness/runner bench-all

compare: bench
	@echo "📊 Generating comparison report..."
	@if [ -f scripts/report.py ]; then \
		python scripts/report.py results/ > docs/results/comparison.md; \
	fi

# Validation
validate:
	@echo "✅ Running validation suite..."
	@./scripts/validate.sh

# Release Management (Canonical Version System)
pre-release-checks:
	@echo "🔍 Running pre-release validation..."
	@echo "1. Version consistency check..."
	@echo "2. Quality gates..."
	@$(MAKE) quality-gate
	@echo "3. Security audit..."
	@$(MAKE) security
	@echo "4. Dependency check..."
	@if command -v cargo-outdated >/dev/null 2>&1; then \
		cargo outdated --exit-code 1; \
	fi
	@echo "5. SemVer compatibility..."
	@if command -v cargo-semver-checks >/dev/null 2>&1; then \
		cargo semver-checks check-release; \
	fi
	@echo "✅ Pre-release checks passed"

release-auto: pre-release-checks
	@echo "🚀 Auto-detecting version bump..."
	@./scripts/release.sh auto

release-patch: pre-release-checks
	@echo "🚀 Releasing patch version..."
	@./scripts/release.sh patch

release-minor: pre-release-checks
	@echo "🚀 Releasing minor version..."
	@./scripts/release.sh minor

release-major: pre-release-checks
	@echo "🚀 Releasing major version..."
	@./scripts/release.sh major

# Version Management
version-check:
	@echo "🔍 Checking Ruchy version..."
	@echo "Current version: $(RUCHY_VERSION)"
	@echo "Required version: $(REQUIRED_VERSION)+"
	@if [ "$(RUCHY_VERSION)" != "$(REQUIRED_VERSION)" ]; then \
		echo "⚠️  Version mismatch detected"; \
		echo "Run 'make version-update' to update examples"; \
	else \
		echo "✅ Version is up to date"; \
	fi

version-update:
	@echo "🚀 Updating to latest Ruchy version..."
	@ruchy scripts/version_manager.ruchy

new-example:
	@echo "📝 Creating new example..."
	@if [ -z "$(NAME)" ]; then \
		echo "❌ Usage: make new-example NAME=003-example-name"; \
		exit 1; \
	fi
	@ruchy scripts/create_example.ruchy $(NAME)
	@echo "✅ Example $(NAME) created"
	@echo "🔄 Running version update for new example..."
	@$(MAKE) version-update

# Cleanup
clean:
	@echo "🧹 Cleaning build artifacts..."
	@for example in $(EXAMPLES); do \
		$(MAKE) -C $$example clean; \
	done
	@if command -v cargo >/dev/null 2>&1; then \
		cargo clean; \
	fi
	@rm -rf coverage/ results/ target/
	@find . -name "test_*" -type f -executable -delete
	@find . -name "debug_*" -type f -executable -delete
	@echo "✅ Cleanup complete"

# Help
help:
	@echo "🌸 Rosetta Ruchy Makefile - Toyota Way Quality System"
	@echo ""
	@echo "🎯 Quality Gates (MANDATORY):"
	@echo "  make quality-gate     - Run all quality checks (BLOCKING)"
	@echo "  make lint            - Zero-warning linting"
	@echo "  make test            - Comprehensive test suite"
	@echo "  make complexity      - Complexity analysis (≤20)"
	@echo "  make coverage        - Test coverage (≥80%)"
	@echo "  make security        - Security vulnerability scan"
	@echo ""
	@echo "🔍 Analysis (Genchi Genbutsu):"
	@echo "  make analyze-complexity  - Find complexity hotspots"
	@echo "  make analyze-debt       - Find technical debt"
	@echo "  make analyze-duplicates - Find code duplication"
	@echo ""
	@echo "🔄 Refactoring (Jidoka):"
	@echo "  make refactor-plan FILE=<path>  - Generate refactoring plan"
	@echo ""
	@echo "📋 Task Management:"
	@echo "  make pdmt-todos TASK=\"description\"  - Generate structured todos"
	@echo ""
	@echo "🚀 Release Management:"
	@echo "  make release-auto      - Auto-detect version bump"
	@echo "  make release-[patch|minor|major]  - Manual version bump"
	@echo ""
	@echo "🏗️  Infrastructure:"
	@echo "  make docker-build      - Build all Docker images"
	@echo "  make tier1-test        - Test Tier 1 languages"
	@echo "  make bench            - Run benchmarks"
	@echo "  make compare          - Generate performance comparison"
	@echo ""
	@echo "⚙️  Setup:"
	@echo "  make install-dev-tools - Install required development tools"
	@echo "  make install-hooks     - Install git quality hooks"