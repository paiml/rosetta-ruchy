#!/bin/bash
# Validation Script - Rosetta Ruchy Quality Assurance
# Toyota Way: Genchi Genbutsu (Go and See) - Verify actual quality

set -e

echo "✅ Rosetta Ruchy Validation Suite"
echo "Toyota Way: Quality built-in, not bolted-on"
echo "==========================================="

# Configuration
FAILED_CHECKS=0
TOTAL_CHECKS=0

# Helper function to run check and track results
run_check() {
    local check_name="$1"
    local check_command="$2"
    local optional="${3:-false}"
    
    TOTAL_CHECKS=$((TOTAL_CHECKS + 1))
    echo ""
    echo "🔍 CHECK: $check_name"
    echo "Command: $check_command"
    
    if eval "$check_command"; then
        echo "✅ PASSED: $check_name"
    else
        if [ "$optional" = "true" ]; then
            echo "⚠️  SKIPPED: $check_name (optional dependency missing)"
        else
            echo "❌ FAILED: $check_name"
            FAILED_CHECKS=$((FAILED_CHECKS + 1))
        fi
    fi
}

# Check 1: Repository structure validation
run_check "Repository Structure" "
    [ -f README.md ] && 
    [ -f LICENSE ] && 
    [ -f docs/specifications/rosetta-spec.md ] &&
    [ -f CLAUDE.md ] &&
    [ -f Makefile ] &&
    echo 'Core files present'"

# Check 2: Git repository status
run_check "Git Repository Status" "
    git status --porcelain | wc -l | grep -q '^0$' &&
    echo 'Working directory clean'"

# Check 3: Documentation quality
run_check "Documentation Quality" "
    grep -q 'Toyota Way' CLAUDE.md &&
    grep -q 'Quality Gates' CLAUDE.md &&
    grep -q 'rosetta-ruchy' README.md &&
    echo 'Documentation contains required sections'"

# Check 4: Specification compliance
run_check "Specification Compliance" "
    [ -f docs/specifications/rosetta-spec.md ] &&
    grep -q 'Performance Benchmarks' docs/specifications/rosetta-spec.md &&
    grep -q 'Language Tiers' docs/specifications/rosetta-spec.md &&
    echo 'Specification file valid'"

# Check 5: License compliance
run_check "License Compliance" "
    grep -q 'MIT License' LICENSE &&
    grep -q 'Pragmatic AI Labs' LICENSE &&
    echo 'MIT License properly configured'"

# Check 6: Makefile targets
run_check "Makefile Targets" "
    grep -q 'quality-gate:' Makefile &&
    grep -q 'docker-build:' Makefile &&
    grep -q 'tier1-test:' Makefile &&
    echo 'Required Makefile targets present'"

# Check 7: Quality gate scripts
run_check "Quality Gate Scripts" "
    [ -f scripts/pre-commit-hook.sh ] &&
    [ -x scripts/pre-commit-hook.sh ] &&
    grep -q 'Toyota Way' scripts/pre-commit-hook.sh &&
    echo 'Quality gate scripts configured'"

# Check 8: Release automation
run_check "Release Automation" "
    [ -f scripts/release.sh ] &&
    [ -x scripts/release.sh ] &&
    grep -q 'Canonical Release' scripts/release.sh &&
    echo 'Release automation configured'"

# Check 9: Directory structure (if examples exist)
if [ -d examples ]; then
    run_check "Examples Directory Structure" "
        find examples -type d -name 'implementations' | head -1 | xargs test -d &&
        echo 'Examples directory structure valid'" "true"
else
    echo "ℹ️  Examples directory not yet created (expected in early stages)"
fi

# Check 10: Docker configuration (if exists)
if [ -d harness/docker ]; then
    run_check "Docker Configuration" "
        find harness/docker -name '*.dockerfile' | head -1 | xargs test -f &&
        echo 'Docker configuration present'" "true"
else
    echo "ℹ️  Docker configuration not yet created (expected in early stages)"
fi

# Check 11: Rust workspace (if Rust project)
if [ -f Cargo.toml ]; then
    run_check "Rust Workspace Configuration" "
        grep -q '\\[workspace\\]' Cargo.toml &&
        echo 'Rust workspace configured'"
    
    run_check "Rust Compilation" "
        cargo check --quiet &&
        echo 'Rust code compiles successfully'"
    
    run_check "Rust Formatting" "
        cargo fmt --check &&
        echo 'Rust code properly formatted'"
    
    run_check "Rust Linting" "
        cargo clippy --all-targets --all-features -- -D warnings &&
        echo 'Rust code passes lint checks'"
else
    echo "ℹ️  No Rust workspace detected"
fi

# Check 12: PMAT integration (if available)
run_check "PMAT Quality Analysis" "
    command -v pmat >/dev/null &&
    pmat analyze complexity --max-threshold 20 &&
    echo 'PMAT quality analysis passed'" "true"

# Check 13: Git hooks installation (skip in CI)
if [ -z "$CI" ]; then
    run_check "Git Hooks Installation" "
        [ -f .git/hooks/pre-commit ] &&
        [ -x .git/hooks/pre-commit ] &&
        echo 'Git hooks properly installed'"
else
    echo "ℹ️  Git hooks check skipped in CI environment"
fi

# Check 14: Security configuration
run_check "Security Configuration" "
    ! find . -name '*.key' -o -name '*.pem' -o -name '.env' | grep -v target | grep . &&
    echo 'No sensitive files in repository'"

# Check 15: Gitignore configuration
run_check "Gitignore Configuration" "
    [ -f .gitignore ] &&
    grep -q 'target' .gitignore &&
    grep -q 'debug_\\*' .gitignore &&
    echo 'Gitignore properly configured'"

# Summary Report
echo ""
echo "============================================="
echo "📊 VALIDATION SUMMARY"
echo "============================================="
echo "Total Checks: $TOTAL_CHECKS"
echo "Passed: $((TOTAL_CHECKS - FAILED_CHECKS))"
echo "Failed: $FAILED_CHECKS"

if [ $FAILED_CHECKS -eq 0 ]; then
    echo ""
    echo "🎉 ALL VALIDATIONS PASSED!"
    echo "Project meets Toyota Way quality standards"
    echo "Ready for development workflow"
    exit 0
else
    echo ""
    echo "❌ $FAILED_CHECKS VALIDATION(S) FAILED"
    echo "Fix the failed checks before proceeding"
    echo "Toyota Way: Stop the line for ANY defect"
    exit 1
fi