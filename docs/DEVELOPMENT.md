# Development Guide - Rosetta Ruchy

This guide covers development workflows, quality gates, and contribution guidelines.

## Quick Start

```bash
# Clone repository
git clone https://github.com/paiml/rosetta-ruchy.git
cd rosetta-ruchy

# Install Ruchy 3.78.0
cargo install ruchy --version 3.78.0

# Run comprehensive tests
make test-all-examples

# Install pre-commit hook (recommended)
./scripts/install-pre-commit-hook.sh
```

## Quality Gates

### Pre-Commit Hook (Local)

**Recommended**: Install the pre-commit hook to catch issues before pushing:

```bash
./scripts/install-pre-commit-hook.sh
```

**What it does**:
- Runs `make dogfood-quick` (3 core tools: check, lint, score)
- Validates all 126 examples in ~2 minutes
- Blocks commit if quality gates fail
- Can be bypassed with `git commit --no-verify` (use sparingly)

**Testing the hook**:
```bash
.git/hooks/pre-commit
```

**Bypassing the hook** (emergency only):
```bash
git commit --no-verify -m "Emergency fix"
```

### CI/CD Quality Gates (Automated)

**Runs on**: Every push and pull request to main branch

**Workflow**: `.github/workflows/dogfood-quality-gates.yml`

**Steps**:
1. Run `make test-all-examples` - Validate all 126 files
2. Check success rate >= 90% threshold (currently 100%)
3. Run `make dogfood-quality` - 7 tools validation
4. Upload test results as artifacts

**Quality Tools Validated**:
- `ruchy check` - Syntax validation
- `ruchy lint` - Style analysis
- `ruchy score` - Quality scoring
- `ruchy provability` - Formal verification
- `ruchy runtime` - Complexity analysis
- `ruchy quality-gate` - Quality enforcement
- `ruchy test` - Test execution

### Regression Detection (Daily)

**Runs on**: Daily at 6:00 AM UTC

**Workflow**: `.github/workflows/regression-check.yml`

**Steps**:
1. Run `make test-all-examples`
2. Compare success rate vs baseline (100%)
3. Create GitHub issue if drop >5%
4. Alert on quality regressions

## Development Workflow

### Making Changes

```bash
# 1. Create a branch (optional - we work on main)
git checkout -b feature/my-change

# 2. Make your changes
vim examples/algorithms/new-algorithm/implementation.ruchy

# 3. Test locally
make test-all-examples
ruchy check examples/algorithms/new-algorithm/implementation.ruchy

# 4. Run quality validation
make dogfood-quick   # Fast (2 min)
make dogfood-quality # Comprehensive (5 min)

# 5. Commit (pre-commit hook runs automatically)
git add .
git commit -m "feat: Add new algorithm implementation"

# 6. Push to GitHub (CI/CD runs automatically)
git push origin main
```

### Quality Validation Commands

```bash
# Quick validation (3 tools, ~2 min)
make dogfood-quick

# Quality validation (7 tools, ~5 min)
make dogfood-quality

# Full validation (10 tools, ~10 min)
make dogfood-full

# Comprehensive validation (15 tools, ~20 min)
make dogfood-comprehensive

# Test all examples
make test-all-examples

# Check specific file
ruchy check <file>.ruchy
ruchy lint <file>.ruchy
ruchy score <file>.ruchy
ruchy provability <file>.ruchy
```

## Testing

### Running Tests

```bash
# Test all examples
make test-all-examples

# View test results
cat test-results.json

# Test specific category
find examples/algorithms -name "*.ruchy" -exec ruchy check {} \;
```

### Test Results Format

```json
{
  "timestamp": "2025-10-14T10:55:16Z",
  "ruchy_version": "3.78.0",
  "summary": {
    "total_examples": 126,
    "passing": 126,
    "failing": 0,
    "success_rate": 1.000
  },
  "by_category": {
    "algorithms": {"total": 86, "passing": 86, "rate": 1.000},
    "data-science": {"total": 36, "passing": 36, "rate": 1.000},
    "advanced-ai": {"total": 4, "passing": 4, "rate": 1.000}
  }
}
```

## Code Standards

### Ruchy Code Style

- Follow Ruchy formatting conventions
- Use meaningful variable names
- Add comments for complex logic
- Keep functions focused and small (complexity ≤20)

### Quality Thresholds

- **Success Rate**: ≥90% (currently 100%)
- **Provability Score**: ≥90/100
- **Quality Score**: ≥0.90 (A-)
- **Complexity**: ≤20 per function

### Commit Messages

Follow conventional commits format:

```
feat(category): Add new feature
fix(category): Fix bug
docs(category): Update documentation
test(category): Add tests
refactor(category): Refactor code
```

Examples:
```
feat(algorithms): Add topological sort implementation
fix(data-science): Fix dataframe filtering edge case
docs(sprint-41): Update development guide with pre-commit hooks
```

## Troubleshooting

### Pre-Commit Hook Issues

**Hook not running**:
```bash
# Reinstall hook
./scripts/install-pre-commit-hook.sh

# Check hook is executable
ls -la .git/hooks/pre-commit
```

**Hook failing unexpectedly**:
```bash
# Run manually to see output
.git/hooks/pre-commit

# Check which files are failing
make test-all-examples
cat test-results.json
```

**Need to bypass hook temporarily**:
```bash
git commit --no-verify -m "Emergency commit"
```

### CI/CD Issues

**Build failing on GitHub**:
1. Check workflow run logs on GitHub Actions
2. Run `make dogfood-quality` locally
3. Fix any failing examples
4. Commit and push fixes

**Regression detected**:
1. Check automated GitHub issue
2. Run `make test-all-examples` locally
3. Compare with baseline results
4. Fix regressions or update baseline

## Contributing

### Before Submitting

1. ✅ Install pre-commit hook
2. ✅ Run `make test-all-examples` (100% pass required)
3. ✅ Run `make dogfood-quality` (≥99% pass required)
4. ✅ Update documentation if needed
5. ✅ Write meaningful commit messages
6. ✅ Ensure CI/CD passes on GitHub

### Pull Request Process

1. Fork the repository
2. Create feature branch (optional - we work on main)
3. Make changes with quality validation
4. Push to GitHub
5. CI/CD validates automatically
6. Address any CI/CD failures
7. Merge when all checks pass

## Resources

- **Roadmap**: `roadmap.yaml` - Sprint planning and tickets
- **Integration Status**: `INTEGRATION.md` - Current test results
- **15-Tool Strategy**: `docs/15-TOOL-STRATEGY.md` - Complete tool documentation
- **Formal Verification**: `docs/FORMAL-VERIFICATION-SHOWCASE.md` - Verification results
- **Red Team Validation**: `docs/RED-TEAM-VALIDATION.md` - Tool validation proof

---

**Quality Philosophy**: Quality built-in, not bolted-on. Every commit maintains our 100% test success rate through automated quality gates.

Generated with Claude Code
Co-Authored-By: Claude <noreply@anthropic.com>
