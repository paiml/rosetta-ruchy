# Paragraph-by-Paragraph Documentation Update Specification

**Version**: 1.0.0
**Created**: 2025-11-04
**Status**: Active
**Owner**: rosetta-ruchy Documentation Team

---

## 1. Overview

### 1.1 Purpose

This specification defines a **systematic, paragraph-by-paragraph approach** to maintaining documentation consistency across the rosetta-ruchy project. It ensures that all documentation accurately reflects the current state of the codebase, Ruchy version, and project metrics.

### 1.2 Scope

This specification covers:
- **Core documentation files**: README.md, CLAUDE.md, CONTRIBUTING.md
- **Specification documents**: All files in docs/specifications/
- **Roadmap documents**: roadmap.yaml, docs/execution/roadmap.md
- **Integration tracking**: INTEGRATION.md
- **Build configuration**: Makefile, package.json, Cargo.toml

### 1.3 Principles

1. **Single Source of Truth**: INTEGRATION.md contains authoritative metrics
2. **Version Consistency**: All references to Ruchy version must match
3. **Accuracy**: Documentation reflects actual test results, not aspirational goals
4. **Reproducibility**: Every claim must be verifiable with make commands
5. **Toyota Way Alignment**: Documentation must pass same quality gates as code

---

## 2. Documentation Categories

### 2.1 Tier 1: Critical Documentation (P0)

These files are **customer-facing** and must be updated with every Ruchy version change:

| File | Purpose | Update Trigger |
|------|---------|----------------|
| `README.md` | Project overview, quick start | Version change, metric change |
| `INTEGRATION.md` | Single source of truth for metrics | Every test run |
| `CLAUDE.md` | AI assistant project context | Version change, phase completion |
| `CONTRIBUTING.md` | Contributor onboarding | Quality gate changes |
| `Makefile` | Build system, version requirements | Version change, new commands |

**Quality Gates**:
- Must pass `make lint` (zero warnings)
- Must pass `pmat analyze complexity --max 20`
- Must pass `pmat analyze satd --threshold 0`
- Version references must be consistent
- Metrics must match `test-results.json`

### 2.2 Tier 2: Specification Documents (P1)

Technical specifications that define implementation:

| File | Purpose | Update Trigger |
|------|---------|----------------|
| `docs/specifications/rosetta-spec.md` | Original specification | Scope changes |
| `docs/specifications/data-science.md` | Data science focus | New categories |
| `docs/specifications/c-bash-examples.md` | Tier 0 languages | Language additions |
| `docs/specifications/paragraph-by-paragraph-spec.md` | This document | Process improvements |

**Quality Gates**:
- Specifications must be implementable
- Examples must be runnable
- All code blocks must pass syntax validation

### 2.3 Tier 3: Reports and Summaries (P2)

Historical documentation and sprint reports:

| Category | Files | Update Trigger |
|----------|-------|----------------|
| Sprint Reports | `SPRINT_*_SUMMARY.md` | Sprint completion |
| Performance Reports | `reports/performance/*.md` | Benchmark runs |
| Analysis Reports | `docs/*_REPORT.md` | Analysis completion |

**Quality Gates**:
- Dates must be accurate
- Links must be valid
- Data must be reproducible

---

## 3. Paragraph-by-Paragraph Update Process

### 3.1 Pre-Update Phase

**Step 1: Gather Current State**

```bash
# Collect authoritative metrics
make test-all-examples
cat test-results.json
cat INTEGRATION.md

# Get current Ruchy version
ruchy --version

# Get quality metrics
make dogfood-quick
```

**Step 2: Identify Discrepancies**

```bash
# Find all version references
grep -r "3\.[0-9][0-9]\.[0-9]" README.md CLAUDE.md CONTRIBUTING.md Makefile

# Compare with current version
ruchy --version

# Find metric inconsistencies
grep -r "126/126\|100%" README.md | wc -l
```

**Step 3: Create Update Plan**

Document in ticket format:
- Which paragraphs need updates
- What the old values were
- What the new values should be
- Why the update is needed

### 3.2 Update Execution Phase

**Step 1: Update by Document Tier**

Start with Tier 1 (P0) documents:

```bash
# For each document:
1. Read entire file to understand context
2. Identify paragraphs with version/metric references
3. Update each paragraph individually
4. Verify paragraph consistency within document
5. Commit with atomic changes per file
```

**Example Update Pattern**:

```markdown
# BEFORE (outdated):
[![Ruchy Version](https://img.shields.io/badge/Ruchy-v3.79.0-blue.svg)]

**Current Version**: Ruchy 3.79.0 (verified working)

### Quality Gates
1. ✅ **Ruchy Version Check**: Ensures v3.79.0 installed

# AFTER (updated):
[![Ruchy Version](https://img.shields.io/badge/Ruchy-v3.88.0-blue.svg)]

**Current Version**: Ruchy 3.88.0 (verified working - latest stable)

### Quality Gates
1. ✅ **Ruchy Version Check**: Ensures v3.88.0 installed
```

**Step 2: Cross-Reference Validation**

After updating each document:

```bash
# Verify version consistency
grep -h "3\.[0-9][0-9]\.[0-9]" README.md CLAUDE.md CONTRIBUTING.md Makefile | sort -u

# Should return single version only
```

**Step 3: Metric Accuracy Validation**

```bash
# Compare documentation claims with test results
jq '.summary.success_rate' test-results.json
grep "success rate" README.md

# Verify counts match
jq '.summary.total' test-results.json
grep -o "[0-9]\+/[0-9]\+ examples" README.md
```

### 3.3 Post-Update Validation Phase

**Step 1: Run Quality Gates**

```bash
# Lint all updated documentation
make lint

# Check complexity
pmat analyze complexity --max 20

# Check for SATD
pmat analyze satd --threshold 0

# Verify links
make validate-links
```

**Step 2: Build Verification**

```bash
# Ensure documentation builds correctly
make docs

# Test all example commands in README
./scripts/test-readme-commands.sh
```

**Step 3: Commit Atomically**

```bash
# Single commit with all related updates
git add README.md CLAUDE.md CONTRIBUTING.md Makefile docs/

git commit -m "docs: Update entire book with Ruchy v3.88.0 - comprehensive version update

Updated all core documentation paragraph by paragraph to reflect latest Ruchy version:

Core Documentation Updates:
- README.md: Updated version badges, quality gates, toolchain examples
- CLAUDE.md: Updated integration tracking, repository status, compatibility
- CONTRIBUTING.md: Updated quality gate requirements, installation instructions
- Makefile: Updated REQUIRED_VERSION to 3.88.0

Key Changes:
- Ruchy version: 3.79.0 → 3.88.0 (latest stable)
- Algorithm count: 22 → 86 (expanded coverage)
- Data science examples: 12 → 36 (3x increase)
- Success rate: 100% (126/126 examples passing)

All documentation now consistently references Ruchy v3.88.0 with accurate
metrics from INTEGRATION.md (last updated $(date -u +%Y-%m-%d))."
```

---

## 4. PMAT Integration

### 4.1 PMAT Quality Analysis

Use **PMAT (PAIML MCP Agent Toolkit)** for automated quality validation:

#### Installation

```bash
cargo install --git https://github.com/paiml/paiml-mcp-agent-toolkit.git pmat
```

#### Quality Gate Commands

```bash
# 1. Complexity Analysis
pmat analyze complexity \
  --max-cyclomatic 20 \
  --max-cognitive 20 \
  --files "README.md,CLAUDE.md,CONTRIBUTING.md"

# 2. SATD Detection (Zero Tolerance)
pmat analyze satd \
  --threshold 0 \
  --patterns "TODO,FIXME,HACK,XXX" \
  --files "docs/**/*.md"

# 3. Link Validation
pmat validate links \
  --files "README.md,CLAUDE.md" \
  --check-external true

# 4. Consistency Validation
pmat validate consistency \
  --pattern "3\\.\\d+\\.\\d+" \
  --files "README.md,CLAUDE.md,CONTRIBUTING.md,Makefile" \
  --expect-unique true

# 5. Metrics Validation
pmat validate metrics \
  --source test-results.json \
  --docs "README.md,INTEGRATION.md" \
  --fields "success_rate,total,passing"
```

### 4.2 Pre-Commit Hook Integration

```bash
#!/bin/bash
# .git/hooks/pre-commit

# Run PMAT quality gates on documentation
pmat analyze complexity --max 20 --exit-on-fail
pmat analyze satd --threshold 0 --exit-on-fail
pmat validate links --exit-on-fail
pmat validate consistency --pattern "3\\.\\d+\\.\\d+" --expect-unique --exit-on-fail

# Verify metrics match test results
if [ -f test-results.json ]; then
    pmat validate metrics --source test-results.json --docs README.md --exit-on-fail
fi
```

### 4.3 CI/CD Integration

```yaml
# .github/workflows/documentation-quality.yml
name: Documentation Quality Gates

on: [push, pull_request]

jobs:
  pmat-validation:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install PMAT
        run: cargo install --git https://github.com/paiml/paiml-mcp-agent-toolkit.git pmat

      - name: Run Complexity Analysis
        run: pmat analyze complexity --max 20 --exit-on-fail

      - name: Run SATD Detection
        run: pmat analyze satd --threshold 0 --exit-on-fail

      - name: Validate Links
        run: pmat validate links --check-external --exit-on-fail

      - name: Validate Version Consistency
        run: |
          pmat validate consistency \
            --pattern "3\\.\\d+\\.\\d+" \
            --files "README.md,CLAUDE.md,CONTRIBUTING.md,Makefile" \
            --expect-unique \
            --exit-on-fail

      - name: Validate Metrics Accuracy
        run: |
          pmat validate metrics \
            --source test-results.json \
            --docs "README.md,INTEGRATION.md" \
            --exit-on-fail
```

---

## 5. Update Triggers and Schedules

### 5.1 Version Change Updates (MANDATORY)

**Trigger**: New Ruchy version qualified

**Process**:
1. Run `cargo install ruchy --version X.Y.Z`
2. Run `make test-all-examples`
3. If success rate ≥90%, proceed with documentation update
4. Update all Tier 1 documents paragraph-by-paragraph
5. Update roadmap.yaml with new version
6. Commit and push

**Estimated Time**: 1-2 hours

### 5.2 Metric Changes (AUTOMATED)

**Trigger**: `make test-all-examples` completes

**Process**:
1. `test-results.json` generated automatically
2. `make update-integration` updates INTEGRATION.md
3. No manual updates required for metric changes
4. README.md updated if major milestones reached (e.g., 100% success)

**Estimated Time**: Automated (5 minutes)

### 5.3 Phase Completion Updates (MILESTONE)

**Trigger**: Sprint completes major phase (e.g., Phase 3 completion)

**Process**:
1. Update CLAUDE.md repository status
2. Update README.md current status section
3. Update roadmap.yaml with completion markers
4. Create completion report in reports/
5. Update project metrics and achievements

**Estimated Time**: 2-3 hours

### 5.4 Quality Gate Changes (INFREQUENT)

**Trigger**: New quality gate added to pre-commit hook

**Process**:
1. Update CONTRIBUTING.md quality gates section
2. Update README.md quality gates list
3. Update Makefile help text
4. Document in CHANGELOG.md
5. Update CI/CD workflows if needed

**Estimated Time**: 1 hour

---

## 6. Validation and Testing

### 6.1 Documentation Test Suite

Create automated tests for documentation accuracy:

```bash
#!/bin/bash
# scripts/test-documentation.sh

set -e

echo "=== Testing Documentation Accuracy ==="

# Test 1: Version Consistency
echo "Test 1: Version consistency across documents"
VERSIONS=$(grep -ho "3\.[0-9][0-9]\.[0-9]" README.md CLAUDE.md CONTRIBUTING.md Makefile | sort -u | wc -l)
if [ "$VERSIONS" -ne 1 ]; then
    echo "❌ FAIL: Found $VERSIONS different Ruchy versions in documentation"
    exit 1
fi
echo "✅ PASS: Single Ruchy version across all docs"

# Test 2: Metrics Match test-results.json
echo "Test 2: Metrics accuracy"
DOCUMENTED_RATE=$(grep -o "success rate.*[0-9.]*%" README.md | grep -o "[0-9.]*" | head -1)
ACTUAL_RATE=$(jq -r '.summary.success_rate * 100' test-results.json)
if [ "$DOCUMENTED_RATE" != "$ACTUAL_RATE" ]; then
    echo "❌ FAIL: Documented rate ($DOCUMENTED_RATE%) != Actual rate ($ACTUAL_RATE%)"
    exit 1
fi
echo "✅ PASS: Success rates match"

# Test 3: No TODO/FIXME/HACK
echo "Test 3: SATD detection"
SATD_COUNT=$(grep -r "TODO\|FIXME\|HACK" README.md CLAUDE.md CONTRIBUTING.md | grep -v "Zero tolerance" | wc -l)
if [ "$SATD_COUNT" -gt 0 ]; then
    echo "❌ FAIL: Found $SATD_COUNT SATD comments in documentation"
    exit 1
fi
echo "✅ PASS: Zero SATD comments"

# Test 4: All links valid
echo "Test 4: Link validation"
make validate-links || exit 1
echo "✅ PASS: All links valid"

echo ""
echo "=== All Documentation Tests Passed ✅ ==="
```

### 6.2 Integration with make test

```makefile
# Makefile addition
.PHONY: test-docs
test-docs:
	@echo "Running documentation test suite..."
	./scripts/test-documentation.sh
	pmat analyze complexity --max 20 --files "README.md,CLAUDE.md,CONTRIBUTING.md"
	pmat analyze satd --threshold 0 --files "docs/**/*.md"
	pmat validate links --files "README.md,CLAUDE.md"
	@echo "✅ All documentation tests passed"

# Add to main test target
test: test-examples test-docs
```

---

## 7. Maintenance and Evolution

### 7.1 Specification Updates

This specification itself must be maintained:

**Update Triggers**:
- New documentation tier added
- New quality gate introduced
- PMAT capabilities expanded
- Process improvements identified

**Update Process**:
1. Propose changes via GitHub issue
2. Implement in feature branch
3. Validate with real documentation update
4. Merge after team review
5. Update version number in this file

### 7.2 Continuous Improvement

Following Toyota Way **Kaizen** principle:

**Monthly Review**:
- Analyze documentation update time
- Identify manual steps that can be automated
- Review PMAT effectiveness
- Gather team feedback

**Quarterly Goals**:
- Reduce manual update time by 20%
- Increase automation coverage
- Improve PMAT integration
- Enhance CI/CD pipelines

---

## 8. Appendix

### 8.1 Glossary

| Term | Definition |
|------|------------|
| **PMAT** | PAIML MCP Agent Toolkit - quality analysis tool |
| **SATD** | Self-Admitted Technical Debt (TODO/FIXME comments) |
| **Tier 1/2/3** | Documentation priority levels |
| **P0/P1/P2** | Priority classification |
| **Quality Gate** | Automated check that must pass before commit |

### 8.2 Reference Documents

- **INTEGRATION.md**: Single source of truth for metrics
- **CLAUDE.md**: Project context and development guide
- **roadmap.yaml**: Sprint planning and execution tracking
- **PMAT Documentation**: https://github.com/paiml/paiml-mcp-agent-toolkit

### 8.3 Common Update Patterns

#### Pattern 1: Version Badge Update

```markdown
# Find and replace:
[![Ruchy Version](https://img.shields.io/badge/Ruchy-v{OLD}-blue.svg)]
# With:
[![Ruchy Version](https://img.shields.io/badge/Ruchy-v{NEW}-blue.svg)]
```

#### Pattern 2: Metric Update

```markdown
# Find:
- [x] 22/22 algorithms
# Replace with:
- [x] 86/86 algorithms
```

#### Pattern 3: Phase Status Update

```markdown
# Change status emoji and description:
🚀 **ACTIVE** → ✅ **COMPLETE**
```

---

## 9. Implementation Checklist

For each documentation update session:

- [ ] **Pre-Update**
  - [ ] Run `make test-all-examples`
  - [ ] Verify `test-results.json` generated
  - [ ] Check current Ruchy version: `ruchy --version`
  - [ ] Review INTEGRATION.md for authoritative metrics
  - [ ] Create update ticket with old/new values

- [ ] **Update Execution**
  - [ ] Update README.md paragraph-by-paragraph
  - [ ] Update CLAUDE.md paragraph-by-paragraph
  - [ ] Update CONTRIBUTING.md paragraph-by-paragraph
  - [ ] Update Makefile REQUIRED_VERSION
  - [ ] Update roadmap.yaml current_status

- [ ] **Validation**
  - [ ] Run `make lint`
  - [ ] Run `pmat analyze complexity --max 20`
  - [ ] Run `pmat analyze satd --threshold 0`
  - [ ] Run `./scripts/test-documentation.sh`
  - [ ] Verify version consistency: single version across all docs

- [ ] **Commit**
  - [ ] Stage all changed files
  - [ ] Write comprehensive commit message
  - [ ] Include before/after values
  - [ ] Push to feature branch
  - [ ] Create PR with validation results

- [ ] **Post-Commit**
  - [ ] Verify CI/CD passes
  - [ ] Check documentation builds correctly
  - [ ] Update project dashboard
  - [ ] Close related issues

---

**Document Version**: 1.0.0
**Last Updated**: 2025-11-04
**Next Review**: 2025-12-04
**Owner**: Documentation Team
**Status**: ✅ Active
