# Action Plan - Gemini Audit Response

**Created**: 2025-10-14
**Sprint**: Post-41 Planning
**Priority**: Address quality gate gaps identified in audit

---

## Sprint 42: Quality Infrastructure Hardening

**Goal**: Address all audit findings from `gemini-audit-report-oct14.md`
**Duration**: 2 weeks
**Success Criteria**: All Toyota Way quality gates passing

---

## Week 1: Critical Fixes (Sprint 42A)

### Ticket 1: Update README.md (30 minutes)
**Priority**: LOW (cosmetic)
**Status**: Ready

**Tasks**:
- [ ] Update version: v1.7.0 → v3.78.0
- [ ] Add link to `docs/COMPREHENSIVE-TOOLCHAIN-VALIDATION.md`
- [ ] Update status badges
- [ ] Note fmt tool P0 bug with workaround
- [ ] Reference performance baseline infrastructure (80% complete)

**Deliverable**: Updated `README.md`

---

### Ticket 2: Security Audit Fix (4 hours)
**Priority**: P0 - CRITICAL
**Status**: Ready

**Tasks**:
- [ ] Run `cargo audit` to identify specific vulnerabilities
- [ ] Update vulnerable dependencies via `cargo update`
- [ ] Test all functionality after updates
- [ ] Add `cargo audit` to `.github/workflows/dogfood-quality-gates.yml`
- [ ] Add security gate to `scripts/pre-commit-hook.sh`
- [ ] Update `docs/DEVELOPMENT.md` with security policy

**Acceptance**:
- `cargo audit` reports zero vulnerabilities
- Zero warnings (or documented as acceptable)
- CI/CD blocks commits with vulnerabilities

**Deliverable**: Clean security scan + CI/CD enforcement

---

### Ticket 3: SATD Cleanup (6 hours)
**Priority**: HIGH
**Status**: Ready

**Phase 1: Detection (1 hour)**
- [ ] Run `make analyze-satd` to find all TODO/FIXME/HACK comments
- [ ] Categorize by location (Rust vs Ruchy, examples vs infrastructure)
- [ ] Export list to `reports/satd-cleanup.md`

**Phase 2: Resolution (4 hours)**
- [ ] Convert TODO → GitHub issues (tag with `tech-debt`)
- [ ] Remove HACK comments (refactor or document rationale)
- [ ] Remove FIXME comments (fix or create issue)
- [ ] Clean up example files first (high visibility)

**Phase 3: Enforcement (1 hour)**
- [ ] Add SATD detection to `scripts/pre-commit-hook.sh` (7th quality gate)
- [ ] Update `roadmap.yaml` quality gates with zero-SATD threshold
- [ ] Test pre-commit hook blocks SATD comments
- [ ] Document in `docs/DEVELOPMENT.md`

**Acceptance**:
- `make analyze-satd` reports zero SATD comments
- Pre-commit hook blocks new SATD
- All tech debt tracked in GitHub issues

**Deliverable**: Zero SATD + enforcement

---

### Ticket 4: Lint Cleanup (1 day)
**Priority**: MEDIUM
**Status**: Ready

**Tasks**:
- [ ] Run `make lint` and export warnings to `reports/lint-cleanup.md`
- [ ] Fix Rust clippy warnings: `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] Fix Ruchy lint warnings systematically (124/126 passing → 126/126)
- [ ] Document any unfixable warnings with rationale
- [ ] Add zero-warning policy to pre-commit hook
- [ ] Update CI/CD to enforce zero warnings

**Current Status**:
- Rust: Mostly clean (minor warnings)
- Ruchy: 98.4% pass rate (124/126) - fix remaining 2 examples

**Acceptance**:
- `make lint` reports zero warnings
- CI/CD enforces zero-warning policy

**Deliverable**: Clean lint results + enforcement

---

## Week 2: Structural Improvements (Sprint 42B)

### Ticket 5: Test Coverage (3 days)
**Priority**: HIGH
**Status**: Ready

**Current**: 39.51%
**Target**: 80%
**Gap**: 40.49%

**Phase 1: Infrastructure Tests (1 day)**
- [ ] Add tests to `mcp-server/` (target: 85% coverage)
  - API endpoint tests
  - Code translation tests
  - Error handling tests
  - Health check tests
- [ ] Add tests to `harness/` (target: 85% coverage)
  - Statistical runner tests
  - Benchmark framework tests
  - Result aggregation tests

**Phase 2: Script Tests (1 day)**
- [ ] Add tests to `scripts/` where applicable
  - Test benchmark-runner.sh with fixtures
  - Test pre-commit-hook.sh behavior
  - Test utility scripts

**Phase 3: Integration Tests (1 day)**
- [ ] Add end-to-end MCP server tests
- [ ] Add full benchmark pipeline tests
- [ ] Add CI/CD workflow tests

**Phase 4: Enforcement**
- [ ] Add `cargo tarpaulin` to CI/CD
- [ ] Set 80% threshold in GitHub Actions
- [ ] Document coverage requirements in `docs/DEVELOPMENT.md`

**Acceptance**:
- `cargo tarpaulin` reports ≥80% coverage
- CI/CD enforces coverage threshold
- All critical paths tested

**Deliverable**: 80%+ test coverage + enforcement

---

### Ticket 6: Complexity Refactoring (1 week)
**Priority**: MEDIUM
**Status**: Ready

**Current**:
- Max Cognitive Complexity: 63
- Max Cyclomatic Complexity: 23
- Target: ≤20

**Phase 1: Analysis (2 hours)**
- [ ] Run `make analyze-complexity` with detailed output
- [ ] Identify all functions with complexity >20
- [ ] Export to `reports/complexity-refactoring-targets.md`
- [ ] Prioritize by impact (high-traffic code first)

**Phase 2: Kaizen Refactoring (5 days)**
Apply Toyota Way Kaizen loop to each function:

1. **Identify Target**: Function with complexity >20
2. **Break Down**: Extract smaller, focused functions
3. **Test**: Ensure behavior unchanged
4. **Verify**: Confirm complexity ≤20
5. **Repeat**: Next function

**Refactoring Techniques**:
- Extract method
- Split conditionals
- Introduce parameter objects
- Replace nested loops with iterators
- Apply single responsibility principle

**Phase 3: Enforcement (1 hour)**
- [ ] Add complexity check to pre-commit hook
- [ ] Set threshold at 20 in `roadmap.yaml`
- [ ] Document complexity policy in `docs/DEVELOPMENT.md`

**Acceptance**:
- All functions have complexity ≤20
- Pre-commit hook enforces threshold
- CI/CD validates complexity

**Deliverable**: Complexity ≤20 + enforcement

---

## Blocked Items (Waiting for Ruchy Team)

### Performance Benchmarking (ROSETTA-414)
**Status**: BLOCKED - Missing `.parse::<T>()` in Ruchy 3.78.0

**Infrastructure Ready**:
- ✅ `scripts/benchmark-runner.sh` (190 lines)
- ✅ `reports/performance/BENCHMARKING_PLAN.md` (200+ lines)
- ✅ 5 algorithms selected and verified
- ✅ Rust compilation optimized

**Blocker**: Cannot parse CLI arguments for parameterized benchmarks

**When Unblocked**:
- Can execute full benchmark suite immediately
- Estimated: 1 day to run and generate report
- Will provide empirical performance data

**Action**: Monitor Ruchy releases for `.parse::<T>()` support

---

### fmt Tool Fix
**Status**: BLOCKED - P0 bug in Ruchy 3.78.0

**Issue**: fmt outputs AST debug representation, destroys files

**Workaround**: Only use `--check` flag

**When Fixed**:
- Can format all 126 examples
- Can add fmt to quality gates
- Will achieve 100% fmt pass rate

**Action**: Ruchy team needs to implement pretty-printer

---

## Sprint 42 Success Criteria

### Must-Have (Week 1)
- ✅ README.md updated to v3.78.0
- ✅ Security audit clean (zero vulnerabilities)
- ✅ Zero SATD comments
- ✅ Zero lint warnings

### Should-Have (Week 2)
- ✅ Test coverage ≥80%
- ✅ Complexity ≤20 for all functions

### Nice-to-Have (Future)
- Performance benchmarking (blocked by Ruchy)
- fmt tool usage (blocked by Ruchy)

---

## Timeline

```
Week 1 (Sprint 42A): Critical Fixes
├── Day 1: Security audit + README update
├── Day 2-3: SATD cleanup + enforcement
└── Day 4-5: Lint cleanup + enforcement

Week 2 (Sprint 42B): Structural Improvements
├── Day 1-3: Test coverage improvements
└── Day 4-5: Complexity refactoring (start)

Week 3+ (Sprint 43): Continue refactoring if needed
```

---

## Quality Gate Enforcement Matrix

After Sprint 42 completion, all quality gates will be enforced:

| Gate | Command | Threshold | Enforced In | Status |
|------|---------|-----------|-------------|--------|
| Syntax | `make test-all-examples` | 90% | CI/CD | ✅ Enforced |
| Regression | `make test-regression` | <5% drop | CI/CD Daily | ✅ Enforced |
| Security | `cargo audit` | Zero vuln | Pre-commit, CI/CD | 🔄 Sprint 42 |
| SATD | `make analyze-satd` | Zero | Pre-commit, CI/CD | 🔄 Sprint 42 |
| Lint | `make lint` | Zero warnings | Pre-commit, CI/CD | 🔄 Sprint 42 |
| Coverage | `cargo tarpaulin` | ≥80% | CI/CD | 🔄 Sprint 42 |
| Complexity | `make analyze-complexity` | ≤20 | Pre-commit, CI/CD | 🔄 Sprint 42 |
| Dogfooding | `make dogfood-quick` | 90% | Pre-commit | ✅ Enforced |

---

## Resources

**Audit Documents**:
- `docs/qa/gemini-audit-report-oct14.md` - Original audit
- `docs/qa/RESPONSE_TO_GEMINI_AUDIT_OCT14.md` - Detailed response

**Quality Infrastructure**:
- `scripts/pre-commit-hook.sh` - Local enforcement
- `.github/workflows/dogfood-quality-gates.yml` - CI/CD enforcement
- `docs/DEVELOPMENT.md` - Developer guide

**Toyota Way References**:
- Kaizen: Continuous improvement through small changes
- Genchi Genbutsu: Go and see the actual problem
- Jidoka: Build quality into automation

---

**Next Step**: Begin Sprint 42A with security audit fix (P0 priority)

🔬 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>
