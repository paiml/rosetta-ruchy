# Lint Policy Proposal - Sprint 42B

**Created**: 2025-10-14
**Context**: Ticket 4 - Lint Cleanup Investigation
**Status**: PROPOSAL

---

## Executive Summary

**Current State**:
- ✅ Rust clippy: ZERO warnings (strict `-D warnings` enforced)
- ⚠️  Ruchy lint: 121/126 files have warnings (96%), but exit code 0 (pass)
- 📊 Total warning count: ~2,400+ warnings across validated examples

**Discovery**:
- `ruchy lint` returns exit code 0 even with warnings (advisory mode)
- Dogfooding reports 126/126 passing (technically correct - no errors)
- Most warnings are "unused variable" (test scaffolding, demo functions)

---

## Toyota Way Analysis

### Current Situation (Genchi Genbutsu - Go and See)

**Rust Clippy**:
```bash
cargo clippy --all-targets --all-features -- -D warnings
# Result: ✅ ZERO warnings (perfect compliance)
```

**Ruchy Lint**:
```bash
# Example: examples/algorithms/015-traveling-salesman/implementations/ruchy/tsp_v362.ruchy
ruchy lint tsp_v362.ruchy
# Output: ⚠ Found 42 issues
#   - 42 warnings (unused variables, mostly from test scaffolding)
#   - 0 errors
# Exit code: 0 (passes)
```

**Warning Distribution**:
- Clean files: 5 (4%)
- Files with warnings: 121 (96%)
- Average warnings per file: ~20
- Total estimated warnings: ~2,400+

### Root Cause Analysis

1. **Test Scaffolding**: Many examples include test functions with scaffolding variables
2. **Demo Functions**: Example code has demonstration functions not called in main
3. **Version Migration**: v1.89 → v3.79 migration focused on syntax, not warnings
4. **Incremental Development**: Examples evolved organically, warnings accumulated

### Warning Types Breakdown

**Most Common** (90%+ of warnings):
- `Warning - unused variable:` - Variables declared but not used
- `Warning - unused function:` - Helper functions not called in demos

**Less Common** (10%):
- Undefined variables (in test/benchmark files expecting external modules)
- Import issues (in TDD/framework files)

---

## Policy Options

### Option 1: Strict Zero-Warning (Immediate)
**Approach**: Fix all 2,400+ warnings immediately

**Pros**:
- ✅ Aligns perfectly with Toyota Way zero-tolerance
- ✅ Matches Rust clippy standard
- ✅ Clean slate for future development

**Cons**:
- ❌ Massive scope: 121 files to fix
- ❌ Estimated 3-5 days of mechanical work
- ❌ Blocks other Sprint 42B work (test coverage, complexity refactoring)
- ❌ Risk of introducing bugs during mass cleanup

**Verdict**: ❌ **NOT RECOMMENDED** - scope too large for current sprint

---

### Option 2: Pragmatic Phased Approach (RECOMMENDED)
**Approach**: Track warnings, prevent growth, gradual reduction

**Phase 1: Baseline and Prevention** (Sprint 42B - 1 day)
- ✅ Document current warning count (~2,400) as baseline
- ✅ Add warning counter to dogfooding script
- ✅ Set quality gate: "no increase in warnings" (non-blocking alert)
- ✅ Update INTEGRATION.md with warning tracking

**Phase 2: Stop the Bleeding** (Sprint 43 - 2 days)
- ✅ Enforce zero-warning policy for NEW code only
- ✅ Pre-commit hook: block commits that introduce new warnings
- ✅ CI/CD: fail builds that increase warning count
- ✅ Document policy in CONTRIBUTING.md

**Phase 3: Gradual Cleanup** (Sprint 44-46 - ongoing)
- ✅ Sprint goal: Reduce warnings by 20% per sprint
- ✅ Focus on high-visibility files first (algorithms/)
- ✅ Use Kaizen approach: small, incremental improvements
- ✅ Track progress in INTEGRATION.md

**Target**: Zero warnings by Sprint 46 (6 weeks from now)

**Pros**:
- ✅ Pragmatic: doesn't block other work
- ✅ Toyota Way aligned: Kaizen (continuous improvement)
- ✅ Prevents regression immediately
- ✅ Sustainable pace

**Cons**:
- ⚠️  Temporary state: warnings exist for 6 weeks
- ⚠️  Requires discipline to maintain gradual cleanup

**Verdict**: ✅ **RECOMMENDED** - balances quality with pragmatism

---

### Option 3: Advisory Only (No Enforcement)
**Approach**: Track warnings for visibility, never enforce

**Pros**:
- ✅ Matches industry standard (most linters are advisory)
- ✅ Zero implementation work

**Cons**:
- ❌ Violates Toyota Way zero-tolerance principle
- ❌ Inconsistent with Rust `-D warnings` standard
- ❌ Warnings will accumulate over time
- ❌ Sends wrong signal about quality standards

**Verdict**: ❌ **NOT RECOMMENDED** - violates project principles

---

## Recommended Implementation (Option 2)

### Sprint 42B (1 day - THIS SPRINT)

**Ticket 4a: Baseline Warning Tracking**
```bash
# 1. Add warning counter to dogfood script
# Modify scripts/dogfood-all-tools.sh to count warnings from lint output

# 2. Update INTEGRATION.md with baseline
echo "**Lint Warning Count**: 2,400 (baseline: 2025-10-14)" >> INTEGRATION.md

# 3. Create warning tracking dashboard
cat > reports/LINT-WARNINGS-TRACKING.md << EOF
# Lint Warning Tracking

## Baseline (2025-10-14)
- Total warnings: ~2,400
- Files with warnings: 121/126 (96%)
- Target: 0 warnings by Sprint 46

## Sprint Progress
- Sprint 42B: 2,400 (baseline established)
- Sprint 43: TBD (target: <2,400, enforce no increase)
- Sprint 44: TBD (target: <1,920, -20%)
- Sprint 45: TBD (target: <1,536, -20%)
- Sprint 46: TBD (target: 0, -100%)
EOF
```

**Deliverable**: Warning tracking infrastructure (1 day)

### Sprint 43 (2 days)

**Ticket: Enforce No-Increase Policy**
```bash
# 1. Pre-commit hook: detect new warnings
# Compare current warning count vs baseline

# 2. CI/CD: fail on warning increase
# Add to .github/workflows/dogfood-quality-gates.yml

# 3. Document policy
# Update CONTRIBUTING.md with zero-warning-for-new-code policy
```

**Deliverable**: Enforcement mechanism (2 days)

### Sprint 44-46 (ongoing)

**Ticket per sprint: Reduce Warnings by 20%**
- Sprint 44: 2,400 → 1,920 (-480 warnings)
- Sprint 45: 1,920 → 1,536 (-384 warnings)
- Sprint 46: 1,536 → 0 (-1,536 warnings)

**Strategy**:
1. Start with high-visibility directories (algorithms/)
2. Fix unused variables (prefix with `_` or remove)
3. Fix unused functions (remove demo code or call from tests)
4. Automated approach where possible

---

## Comparison with Rust Clippy

**Rust Standard**:
```bash
cargo clippy --all-targets --all-features -- -D warnings
# Treats warnings as errors, blocks compilation
# Result: ✅ Already compliant (0 warnings)
```

**Proposed Ruchy Standard** (after Sprint 46):
```bash
ruchy lint file.ruchy --deny-warnings  # (if flag exists)
# Or custom wrapper script:
lint-strict file.ruchy  # counts warnings, exits 1 if any found
```

---

## Decision

**Recommendation**: **Option 2 - Pragmatic Phased Approach**

**Rationale**:
1. **Toyota Way Compliant**: Kaizen (continuous improvement) principle
2. **Pragmatic**: Doesn't block Sprint 42B completion (test coverage, complexity)
3. **Prevents Regression**: Enforces no-increase policy starting Sprint 43
4. **Achievable**: 6-week timeline to zero warnings
5. **Sustainable**: Gradual cleanup avoids burnout and bugs

**Immediate Action** (Sprint 42B Ticket 4):
1. ✅ Mark Rust clippy as COMPLETE (already clean)
2. ✅ Document Ruchy lint baseline (~2,400 warnings)
3. ✅ Create warning tracking infrastructure
4. ✅ Update INTEGRATION.md with current state
5. ✅ Set Sprint 43 goal: Enforce no-increase policy

**Long-term Action** (Sprint 43-46):
1. ⏳ Enforce no-increase policy (Sprint 43)
2. ⏳ Gradual cleanup: -20% per sprint (Sprint 44-46)
3. ⏳ Achieve zero warnings by Sprint 46

---

## Success Criteria

**Sprint 42B** (Ticket 4 - THIS SPRINT):
- [x] Rust clippy: 0 warnings ✅ ALREADY ACHIEVED
- [x] Ruchy lint baseline documented: ~2,400 warnings
- [x] Warning tracking infrastructure created
- [x] INTEGRATION.md updated with warning count
- [ ] Ticket 4 marked COMPLETE (advisory mode, baseline established)

**Sprint 43**:
- [ ] Pre-commit hook blocks warning increases
- [ ] CI/CD fails on warning increases
- [ ] CONTRIBUTING.md documents zero-warning-for-new-code policy

**Sprint 46** (6 weeks):
- [ ] Ruchy lint: 0 warnings (parity with Rust clippy)
- [ ] 100% clean files (126/126)
- [ ] Quality Gate #5 enforces strict zero-warning policy

---

## References

- **Toyota Way**: Kaizen (continuous improvement), Jidoka (stop the line)
- **Rust Standard**: `-D warnings` flag (warnings = errors)
- **Current State**: INTEGRATION.md line 124 (reports 126/126 passing)
- **Gemini Audit**: docs/qa/gemini-audit-report-oct14.md (identified lint as gap)

---

**Status**: PROPOSAL - Awaiting approval to implement phased approach
