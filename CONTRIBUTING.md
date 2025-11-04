# Contributing to Rosetta Ruchy

Thank you for your interest in contributing to Rosetta Ruchy! This document outlines our development practices and quality standards.

## Table of Contents

- [Development Philosophy](#development-philosophy)
- [Quality Gates](#quality-gates)
- [Lint Warning Policy](#lint-warning-policy)
- [Development Workflow](#development-workflow)
- [Testing Requirements](#testing-requirements)
- [Commit Standards](#commit-standards)

---

## Development Philosophy

Rosetta Ruchy follows the **Toyota Way** principles:

1. **Kaizen (改善)**: Continuous, incremental improvement
2. **Genchi Genbutsu (現地現物)**: Go and see - use analysis tools to find root causes
3. **Jidoka (自働化)**: Automation with human intelligence

### Sacred Rules (Zero Tolerance)

1. **NEVER Use Shell Scripts** - All scripting must be in Ruchy (`.ruchy` files only)
2. **NEVER Leave Stub Implementations** - Every feature must be fully functional
3. **NEVER Add SATD Comments** - No `TODO`, `FIXME`, `HACK` comments allowed
4. **NEVER Use Simple Heuristics** - Always use proper analysis and full implementations
5. **NEVER Duplicate Core Logic** - One implementation per feature
6. **NEVER Bypass Quality Gates** - `git commit --no-verify` is forbidden

---

## Quality Gates

All contributions must pass the following automated quality gates:

### Gate 1: Ruchy Version Check
- **Requirement**: Ruchy v3.88.0 must be installed
- **Command**: `ruchy --version`

### Gate 2: SATD Detection (Zero Tolerance)
- **Requirement**: Zero `TODO`, `FIXME`, `HACK`, or `XXX` comments
- **Policy**: Convert to GitHub issues or resolve immediately
- **Command**: `grep -r "TODO\|FIXME\|HACK" --include="*.rs" --include="*.ruchy"`

### Gate 3: Ruchy Syntax Validation
- **Requirement**: All `.ruchy` files must pass `ruchy check`
- **Command**: `ruchy check <file>`

### Gate 4: Complexity Check
- **Requirement**:
  - Max cyclomatic complexity: 20
  - Max cognitive complexity: 20
- **Command**: `pmat analyze complexity --max-cyclomatic 20 --max-cognitive 20`

### Gate 5: Rust Linting
- **Requirement**: Zero clippy warnings (warnings treated as errors)
- **Command**: `cargo clippy --all-targets --all-features -- -D warnings`

### Gate 6: Security Audit
- **Requirement**: Zero critical vulnerabilities
- **Command**: `cargo audit`
- **Installation**: `cargo install cargo-audit`

### Gate 7: Dogfood Validation
- **Requirement**: All examples must pass Ruchy validation
- **Command**: `make dogfood-quick`

### Gate 8: Stub Implementation Check
- **Requirement**: Zero `unimplemented!()`, `todo!()`, or `unreachable!()` macros
- **Policy**: All code paths must be fully implemented

### Gate 9: Lint Warning No-Increase (Sprint 43+)
- **Requirement**: New commits cannot increase lint warning count
- **Baseline**: 744 warnings (as of Sprint 43 start)
- **Policy**: **Kaizen improvement only** - warnings can only decrease or stay the same
- **Command**: `./scripts/count-lint-warnings.sh`

### Gate 10: Comprehensive Ruchy Tools Testing (Sprint 47+)
- **Requirement**: All 18+ Ruchy tools tested on new examples
- **Policy**: Following ruchy-book comprehensive testing methodology
- **Tools Tested**: check, parse, provability, runtime, score, ast, optimize, prove, quality-gate, mcp, fmt, lint, doc, transpile, build, run, test, benchmark, profile, energy, complexity, verify, validate
- **Command**: `make test-ruchy-tools-comprehensive`
- **Success Criteria**: Core tools (check, provability, runtime, score) must pass 100%

### Gate 11: Language Benchmarking (Sprint 47+)
- **Requirement**: Performance comparison against baseline languages
- **Baseline**: Rust performance (100% reference)
- **Languages**: Ruchy, Rust, Python, JavaScript, Go, Julia, R
- **Methodology**: EXACT ruchy-book benchmarking (warmup + iterations)
- **Command**: `make bench-language-comparison`
- **Success Criteria**: Ruchy within target performance range (documented per algorithm)

### Gate 12: Quality Tool Verification (Sprint 47+)
- **Requirement**: All quality tools installed and verified
- **Tools**: Ruchy v3.88.0, bashrs v1.0.0-rc1, pmat v2.192.0 (or fallback), shellcheck
- **Command**: `make verify-tools`
- **Fallback**: If pmat unavailable, use `scripts/pmat-style-validation.sh`

---

## Lint Warning Policy

### Overview

Starting with Sprint 43 (October 14, 2025), we enforce a **no-increase policy** on Ruchy lint warnings.

### Current Status

- **Baseline**: 744 warnings (stored in `.lint-baseline`)
- **Target**: Zero warnings by Sprint 46
- **Strategy**: Gradual 20% reduction per sprint (Kaizen continuous improvement)

### Policy Details

#### No-Increase Enforcement (MANDATORY)

1. **Pre-commit Hook**: Automatically checks warning count before every commit
2. **CI/CD Pipeline**: GitHub Actions enforces policy on all PRs and pushes
3. **Blocking**: Commits with increased warnings are **automatically rejected**

#### How It Works

```bash
# Get current warning count
./scripts/count-lint-warnings.sh

# Compare with baseline
cat .lint-baseline

# If current > baseline, commit is blocked
```

#### What Counts as a Warning

The warning counter scans all `.ruchy` files in:
- `examples/algorithms/`
- `examples/data-science/`
- `examples/advanced-ai/`

Common warning types:
- **Unused variable** (90% of warnings) - Prefix with `_` or remove
- **Unused function** (10% of warnings) - Remove or call from tests

#### Gradual Cleanup Schedule

| Sprint | Target Warnings | Reduction |
|--------|----------------|-----------|
| Sprint 42 (Baseline) | 744 | - |
| Sprint 43 | ≤744 (no increase) | Enforcement starts |
| Sprint 44 | ≤595 (-20%) | First reduction sprint |
| Sprint 45 | ≤476 (-20%) | Second reduction sprint |
| Sprint 46 | 0 (zero warnings) | Complete cleanup |

#### How to Fix Warnings

**For unused variables:**
```rust
// BEFORE: Warning - unused variable
let result = expensive_calculation();

// AFTER: Fixed by prefixing with underscore
let _result = expensive_calculation();

// OR: Remove if truly unnecessary
expensive_calculation();
```

**For unused functions:**
```rust
// BEFORE: Warning - unused function
fn helper_function() { ... }

// AFTER: Either use it
fn main() {
    helper_function();
}

// OR: Remove if not needed
```

#### Checking Your Warning Count

```bash
# Count warnings in your working directory
./scripts/count-lint-warnings.sh

# View baseline
cat .lint-baseline

# See detailed warnings for a specific file
ruchy lint examples/algorithms/003-mergesort/implementations/ruchy/mergesort_v189.ruchy
```

#### When Commits Are Blocked

If your commit is blocked due to increased warnings:

```
❌ Lint warning count increased
Baseline: 744 warnings
Current:  750 warnings
Increase: +6 warnings

❌ COMMIT BLOCKED: Fix new lint warnings before committing
```

**Resolution steps:**

1. Run `./scripts/count-lint-warnings.sh` to confirm current count
2. Use `ruchy lint <file>` to identify which files have new warnings
3. Fix the warnings by prefixing unused variables with `_` or removing unused code
4. Verify count: `./scripts/count-lint-warnings.sh` should show ≤744
5. Retry commit

#### Contributing to Reduction Sprints

During Sprint 44-46, we actively reduce warnings by 20% per sprint. Contributions welcome!

**To help with warning reduction:**

1. Check top warning files in `reports/LINT-WARNINGS-TRACKING.md`
2. Pick a file to clean up
3. Fix warnings (prefix unused variables, remove unused functions)
4. Verify: `ruchy check <file>` still passes
5. Commit with reduced warning count - hook will celebrate: `✅ (738 warnings, -6 from baseline!)`

---

## Development Workflow

### 1. Fork and Clone

```bash
git clone https://github.com/yourusername/rosetta-ruchy.git
cd rosetta-ruchy
```

### 2. Install Development Tools

#### Option A: Sprint 47 Comprehensive Installation (Recommended)

```bash
# Install ALL quality tools (Ruchy, bashrs, pmat, shellcheck)
make install-quality-tools

# Verify installations
make verify-tools

# Install pre-commit hooks
make install-hooks
```

**Tools Installed**:
- **Ruchy v3.88.0** - Core language toolchain (18+ commands)
- **bashrs v1.0.0-rc1** - Bash transpiler (optional)
- **pmat v2.192.0** - Quality management (fallback: pmat-style-validation.sh)
- **shellcheck** - Shell script linting (optional)
- **cargo-audit** - Security auditing
- **cargo-tarpaulin** - Code coverage

#### Option B: Manual Installation

```bash
# Install Ruchy v3.88.0
cargo install ruchy --version 3.88.0 --locked

# Install quality gate tools
cargo install cargo-audit --locked
cargo install cargo-tarpaulin --locked

# Install pre-commit hook
cp scripts/pre-commit-hook.sh .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```

**Note**: Sprint 47+ contributions benefit from comprehensive tool installation for full quality validation.

### 3. Create a Branch

**Note**: We work directly on `main` branch following trunk-based development.

```bash
git checkout main
git pull origin main
```

### 4. Make Changes

Follow the [Sacred Rules](#sacred-rules-zero-tolerance) and ensure all [Quality Gates](#quality-gates) pass.

### 5. Run Tests

```bash
# Run all tests
make test-all-examples

# Check success rate (must be ≥90%)
jq -r '.summary.success_rate' test-results.json

# Run dogfood validation
make dogfood-quick
```

### 6. Commit Changes

```bash
# Pre-commit hook automatically runs all quality gates
git add .
git commit -m "feat: Add quicksort example with verified O(n log n) complexity"

# If lint warning check blocks you:
# 1. Check current count: ./scripts/count-lint-warnings.sh
# 2. Fix new warnings in your changed files
# 3. Retry commit
```

### 7. Push to Main

```bash
# We don't use branches - trunk-based development
git push origin main
```

---

## Testing Requirements

### Unit Tests

- Minimum 80% code coverage required
- Use property-based testing where applicable
- All tests must be deterministic (no flaky tests)

### Integration Tests

- Test all CLI entry points with `assert_cmd`
- Test server startup and configuration
- Validate end-to-end workflows

### Benchmark Tests

- Statistical rigor: minimum 1000 iterations
- Include standard deviation analysis
- Use isolated CPU cores for reproducibility

---

## Commit Standards

### Commit Message Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `refactor`: Code refactoring
- `test`: Adding tests
- `chore`: Maintenance tasks

**Example:**

```
feat(algorithms): Implement topological sort with O(V+E) verified complexity

- Ruchy formal verification proves O(V+E) time complexity
- Empirical validation confirms theoretical bounds
- Performance matches Rust baseline (within 5%)
- Generated complexity proof artifacts

Validates: docs/specifications/rosetta-spec.md Section 2.3
```

---

## Questions?

- **Documentation**: See `README.md` and `docs/` directory
- **Issues**: Check existing [GitHub Issues](https://github.com/yourusername/rosetta-ruchy/issues)
- **Discussions**: Start a [GitHub Discussion](https://github.com/yourusername/rosetta-ruchy/discussions)

---

**Thank you for contributing to Rosetta Ruchy!**

By following these guidelines, you help maintain the high quality standards that make this project a reliable benchmark suite and demonstration of Ruchy's capabilities.
