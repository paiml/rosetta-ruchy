#!/bin/bash
# Pre-commit Quality Gates - Toyota Way Zero Defect Policy
# BLOCKS commits that violate quality standards

set -e

echo "🔒 MANDATORY Quality Gates (Toyota Way - Jidoka)"
echo "================================================"

# GATE 1: Basic functionality check (if Rust project exists)
if [ -f "Cargo.toml" ]; then
    echo "🔧 GATE 1: Basic compilation check..."
    if ! cargo check --quiet; then
        echo "❌ FATAL: Code does not compile"
        echo "Fix compilation errors before committing"
        exit 1
    fi
    echo "✅ Compilation check passed"
fi

# GATE 2: Complexity enforcement
echo "🧠 GATE 2: Complexity analysis..."
if command -v pmat >/dev/null 2>&1; then
    if ! pmat analyze complexity --max-threshold 20 --fail-fast; then
        echo "❌ BLOCKED: Complexity exceeds 20"
        echo "Refactor complex functions before committing"
        echo "Use: make refactor-plan FILE=<path>"
        exit 1
    fi
    echo "✅ Complexity check passed"
else
    echo "⚠️  pmat not available - complexity check skipped"
fi

# GATE 3: Zero SATD policy (Self-Admitted Technical Debt)
echo "💳 GATE 3: SATD (technical debt) detection..."
if grep -r "TODO\|FIXME\|HACK\|XXX" --include="*.rs" --include="*.py" --include="*.js" --include="*.go" . 2>/dev/null; then
    echo "❌ BLOCKED: SATD comments found"
    echo "Remove TODO/FIXME/HACK comments or file GitHub issues instead"
    echo "Zero tolerance for self-admitted technical debt"
    exit 1
fi
echo "✅ Zero SATD policy maintained"

# GATE 4: Lint zero tolerance
echo "🔍 GATE 4: Lint analysis..."
if [ -f "Cargo.toml" ]; then
    if ! cargo clippy --all-targets --all-features -- -D warnings --quiet; then
        echo "❌ BLOCKED: Clippy warnings found"
        echo "Fix all warnings before committing (-D warnings policy)"
        exit 1
    fi
    echo "✅ Lint check passed"
fi

# GATE 5: Test execution
echo "🧪 GATE 5: Test execution..."
if [ -f "Cargo.toml" ]; then
    if ! cargo test --quiet; then
        echo "❌ BLOCKED: Tests failing"
        echo "All tests must pass before committing"
        exit 1
    fi
    echo "✅ Test execution passed"
fi

# GATE 6: Documentation tests (if applicable)
if [ -f "Cargo.toml" ] && cargo test --doc --quiet >/dev/null 2>&1; then
    echo "📚 GATE 6: Documentation tests..."
    if ! cargo test --doc --quiet; then
        echo "❌ BLOCKED: Doc tests failing"
        echo "Fix documentation examples before committing"
        exit 1
    fi
    echo "✅ Documentation tests passed"
fi

# GATE 7: Security scan (if tools available)
echo "🔒 GATE 7: Security scan..."
if command -v cargo-audit >/dev/null 2>&1; then
    if ! cargo audit --quiet; then
        echo "❌ BLOCKED: Security vulnerabilities found"
        echo "Update dependencies to resolve security issues"
        exit 1
    fi
    echo "✅ Security scan passed"
else
    echo "⚠️  cargo-audit not available - security check skipped"
fi

# GATE 8: File hygiene check
echo "🧹 GATE 8: Repository hygiene..."
if find . -name "test_*" -type f -executable -not -path "./target/*" | grep -q .; then
    echo "❌ BLOCKED: Test executable files found in repository"
    echo "Remove debug/test binaries before committing:"
    find . -name "test_*" -type f -executable -not -path "./target/*"
    echo "Run: make clean"
    exit 1
fi

if find . -name "debug_*" -type f -executable -not -path "./target/*" | grep -q .; then
    echo "❌ BLOCKED: Debug executable files found in repository"
    echo "Remove debug binaries before committing:"
    find . -name "debug_*" -type f -executable -not -path "./target/*"
    echo "Run: make clean"
    exit 1
fi
echo "✅ Repository hygiene maintained"

# Success message
echo ""
echo "🎉 ALL QUALITY GATES PASSED"
echo "Commit approved under Toyota Way standards"
echo "========================================"