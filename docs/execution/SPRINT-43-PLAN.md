# Sprint 43: Quality Gate Enforcement + Coverage Completion

**Duration**: 2 weeks (October 28 - November 11, 2025)
**Goal**: Complete deferred Sprint 42B items and enforce quality standards
**Success Criteria**: 80% test coverage, lint enforcement active, complexity ≤20

---

## Sprint Overview

Sprint 43 focuses on **completing the quality infrastructure** established in Sprint 42 by:
1. Enforcing lint warning prevention (no growth)
2. Achieving 80% test coverage through proper unit testing
3. Reducing complexity to ≤20 across all functions
4. Beginning gradual lint warning reduction

---

## Week 1: Enforcement + Refactoring

### Ticket 1: Lint No-Increase Enforcement (2 days)
**Priority**: HIGH
**Estimated**: 2 days

**Goal**: Prevent new lint warnings from being added

**Tasks**:
- [ ] Create warning counter script for pre-commit hook
- [ ] Update `scripts/pre-commit-hook.sh` with warning count check
- [ ] Baseline: Store current warning count (2,400) in `.lint-baseline`
- [ ] Block commits that increase warning count
- [ ] Add CI/CD enforcement in `.github/workflows/dogfood-quality-gates.yml`
- [ ] Document policy in `CONTRIBUTING.md`
- [ ] Test enforcement mechanism

**Implementation**:
```bash
# scripts/pre-commit-hook.sh addition
echo -n "  Lint warning count check... "
CURRENT_WARNINGS=$(scripts/count-lint-warnings.sh)
BASELINE_WARNINGS=$(cat .lint-baseline 2>/dev/null || echo "2400")

if [ "$CURRENT_WARNINGS" -gt "$BASELINE_WARNINGS" ]; then
    echo -e "${RED}❌${NC}"
    echo -e "${RED}   Warning count increased: $BASELINE_WARNINGS → $CURRENT_WARNINGS${NC}"
    echo -e "${RED}   ❌ COMMIT BLOCKED: Lint warnings increased by $((CURRENT_WARNINGS - BASELINE_WARNINGS))${NC}"
    exit 1
else
    echo -e "${GREEN}✅ ($CURRENT_WARNINGS warnings, no increase)${NC}"
fi
```

**Deliverables**:
- `scripts/count-lint-warnings.sh` (warning counter script)
- `.lint-baseline` (2,400 warnings baseline)
- Updated `scripts/pre-commit-hook.sh`
- Updated `.github/workflows/dogfood-quality-gates.yml`
- Updated `CONTRIBUTING.md`

**Acceptance Criteria**:
- Pre-commit hook blocks commits with increased warnings
- CI/CD fails builds with increased warnings
- Documentation explains policy

---

### Ticket 2: Test Coverage Refactoring (3 days)
**Priority**: HIGH
**Estimated**: 3 days

**Goal**: Achieve 80% test coverage through unit testing

**Current State**: 39.34% coverage (952/2,412 lines)
**Target**: 80% coverage (≥1,930/2,412 lines)
**Gap**: +978 lines to cover

**Phase 1: Refactor main.rs for Testability (Day 1)**

Extract logic from main entry points into testable functions:

```rust
// harness/runner/src/main.rs - BEFORE
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    // 462 lines of logic here...
}

// harness/runner/src/main.rs - AFTER
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    run_app(cli).await
}

pub async fn run_app(cli: Cli) -> Result<()> {
    // All logic moved here - now unit testable
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_run_app_validate_command() {
        let cli = Cli { /* ... */ };
        assert!(run_app(cli).await.is_ok());
    }
}
```

**Tasks**:
- [ ] Refactor `harness/runner/src/main.rs` - extract `run_app()`
- [ ] Refactor `mcp-server/src/main.rs` - extract `start_server()`
- [ ] Ensure integration tests still pass
- [ ] Verify no behavioral changes

**Phase 2: Unit Test Core Modules (Day 2)**

Add comprehensive unit tests for poorly-covered modules:

**Target Modules**:
1. `harness/runner/src/regression.rs` (13/201 = 6.5% → 80%)
2. `harness/runner/src/reporting.rs` (12/196 = 6.1% → 80%)

**Tests to Add**:
```rust
// harness/runner/src/regression.rs
#[cfg(test)]
mod tests {
    #[test]
    fn test_regression_detection_performance_drop() { /* ... */ }

    #[test]
    fn test_regression_severity_minor() { /* ... */ }

    #[test]
    fn test_regression_severity_critical() { /* ... */ }

    #[test]
    fn test_no_regression_improvement() { /* ... */ }

    #[test]
    fn test_baseline_comparison() { /* ... */ }
    // ~12 total tests needed
}

// harness/runner/src/reporting.rs
#[cfg(test)]
mod tests {
    #[test]
    fn test_json_report_generation() { /* ... */ }

    #[test]
    fn test_html_report_with_charts() { /* ... */ }

    #[test]
    fn test_markdown_report_format() { /* ... */ }

    #[test]
    fn test_yaml_report_structure() { /* ... */ }

    #[test]
    fn test_comparison_report_multiple_runs() { /* ... */ }
    // ~10 total tests needed
}
```

**Phase 3: Verification (Day 3)**

- [ ] Run `cargo tarpaulin` to measure coverage
- [ ] Verify ≥80% coverage achieved
- [ ] Document remaining gaps
- [ ] Update INTEGRATION.md with new coverage numbers
- [ ] Generate coverage report

**Deliverables**:
- Refactored `main.rs` files with extracted logic
- ~22 new unit tests (12 regression + 10 reporting)
- Coverage report showing ≥80%
- Updated documentation

**Acceptance Criteria**:
- `cargo tarpaulin` shows ≥80% coverage
- All tests passing (117+ tests total)
- No behavioral changes in CLI/server

---

## Week 2: Gradual Improvements

### Ticket 3: Lint Warning Reduction (3 days)
**Priority**: MEDIUM
**Estimated**: 3 days

**Goal**: First 20% reduction in lint warnings

**Current**: 2,400 warnings
**Target**: <1,920 warnings (-480 warnings, -20%)

**Strategy**: Focus on highest-impact files

**Top 10 Files by Warning Count** (from TEST-COVERAGE-ANALYSIS.md):
1. `topological_sort_v189.ruchy` - 43 warnings
2. `tsp_v362.ruchy` - 42 warnings
3. `knapsack_v188.ruchy` - 37 warnings
4. `computer_vision_pipeline_v189.ruchy` - 34 warnings
5. `dataframe_ops_v189.ruchy` - 34 warnings
6. `distributed_computing.ruchy` - 34 warnings
7. `test_machine_learning_pipeline.ruchy` - 35 warnings
8. `test_time_series_forecasting.ruchy` - 34 warnings
9. `lcs_v187.ruchy` - 34 warnings
10. `edit_distance_v190.ruchy` - 31 warnings

**Total from top 10**: 358 warnings (15% of total)

**Common Warning Types**:
- 90%: `unused variable` - prefix with `_` or remove
- 10%: `unused function` - remove or call from tests

**Automated Approach**:
```bash
# For unused variables, prefix with underscore
# BEFORE: let medium = ...
# AFTER: let _medium = ...

# Script to automate:
find examples -name "*.ruchy" | while read file; do
    # Extract unused variable names from lint output
    # Apply underscore prefix
    # Verify still compiles
done
```

**Manual Review Required**:
- Verify variables are truly unused
- Ensure tests don't need them
- Check for side effects

**Tasks**:
- [ ] Create automated variable prefixing script
- [ ] Fix top 10 files manually (358 warnings)
- [ ] Run automated fix on remaining high-warning files
- [ ] Verify all 126 examples still pass `ruchy check`
- [ ] Update `.lint-baseline` to new count
- [ ] Update LINT-WARNINGS-TRACKING.md

**Deliverables**:
- ~480 warnings fixed
- Updated `.lint-baseline` (<1,920)
- Updated LINT-WARNINGS-TRACKING.md
- All 126 examples passing

**Acceptance Criteria**:
- Warning count: <1,920 (-20% from baseline)
- All examples pass `ruchy check`
- LINT-WARNINGS-TRACKING.md updated

---

### Ticket 4: Complexity Refactoring (2 days)
**Priority**: MEDIUM
**Estimated**: 2 days

**Goal**: Reduce all functions to ≤20 complexity

**Current State**: Max complexity unknown (needs measurement)
**Target**: All functions ≤20 cyclomatic/cognitive complexity

**Phase 1: Identify High-Complexity Functions (Day 1 Morning)**

```bash
# Find functions with complexity >20
cargo clippy --all-targets --all-features -- -W clippy::cognitive_complexity

# Or use custom complexity analysis
# (if pmat available)
pmat analyze complexity --max-cyclomatic 20 --max-cognitive 20
```

**Phase 2: Apply Kaizen Refactoring (Day 1 Afternoon + Day 2)**

For each high-complexity function:
1. **Extract Method**: Break into smaller functions
2. **Simplify Conditionals**: Use match/guard clauses
3. **Remove Duplication**: DRY principle
4. **Reduce Nesting**: Early returns, flat structure

**Example Refactoring**:
```rust
// BEFORE (complexity: 25)
fn process_benchmark(config: Config) -> Result<Report> {
    if config.is_valid() {
        if config.has_baseline() {
            let baseline = load_baseline()?;
            if baseline.is_compatible() {
                let results = run_benchmark()?;
                if results.is_significant() {
                    // Deep nesting...
                }
            }
        }
    }
    // More complexity...
}

// AFTER (complexity: 8)
fn process_benchmark(config: Config) -> Result<Report> {
    validate_config(&config)?;
    let baseline = load_compatible_baseline(&config)?;
    let results = run_significant_benchmark(&config)?;
    generate_report(baseline, results)
}

fn validate_config(config: &Config) -> Result<()> {
    ensure!(config.is_valid(), "Invalid config");
    ensure!(config.has_baseline(), "Missing baseline");
    Ok(())
}

fn load_compatible_baseline(config: &Config) -> Result<Baseline> {
    let baseline = load_baseline()?;
    ensure!(baseline.is_compatible(), "Incompatible baseline");
    Ok(baseline)
}
```

**Tasks**:
- [ ] Run complexity analysis to find >20 functions
- [ ] Prioritize by impact (main.rs, core logic first)
- [ ] Refactor top 5-10 high-complexity functions
- [ ] Add unit tests for extracted functions
- [ ] Verify no behavioral changes
- [ ] Document refactoring patterns used

**Deliverables**:
- Complexity analysis report
- Refactored functions (all ≤20)
- Additional unit tests for extracted functions
- Refactoring documentation

**Acceptance Criteria**:
- All functions ≤20 complexity
- No behavioral changes
- All tests still passing
- Code coverage maintained or increased

---

## Success Criteria

### Sprint 43 Overall
- [x] Sprint 43 plan created and approved
- [ ] Ticket 1: Lint enforcement active
- [ ] Ticket 2: ≥80% test coverage achieved
- [ ] Ticket 3: <1,920 lint warnings (-20%)
- [ ] Ticket 4: All functions ≤20 complexity

### Quality Gates
- [ ] All 8 quality gates passing
- [ ] Pre-commit hook enforces lint no-increase
- [ ] CI/CD enforces lint no-increase
- [ ] Test coverage ≥80%
- [ ] Complexity ≤20 for all functions

### Documentation
- [ ] CONTRIBUTING.md updated with lint policy
- [ ] LINT-WARNINGS-TRACKING.md updated with Sprint 43 progress
- [ ] TEST-COVERAGE-ANALYSIS.md updated with new numbers
- [ ] Complexity refactoring patterns documented

---

## Risk Assessment

**Risk 1: Coverage target ambitious**
- **Mitigation**: Phased approach, focus on high-impact modules first
- **Backup**: Accept 70% if 80% proves too difficult

**Risk 2: Lint fixes break examples**
- **Mitigation**: Automated testing after each change
- **Backup**: Manual review of top 10 files only

**Risk 3: Complexity refactoring introduces bugs**
- **Mitigation**: Comprehensive unit tests before and after
- **Backup**: Revert if tests fail

**Risk 4: Time estimates too optimistic**
- **Mitigation**: Daily progress tracking, adjust scope if needed
- **Backup**: Defer Ticket 4 to Sprint 44 if necessary

---

## Sprint Velocity Tracking

### Daily Progress (Week 1)
- **Day 1**: Ticket 1 - Lint enforcement implementation
- **Day 2**: Ticket 1 complete, Ticket 2 Phase 1 start
- **Day 3**: Ticket 2 Phase 1 complete (refactoring done)
- **Day 4**: Ticket 2 Phase 2 (unit tests)
- **Day 5**: Ticket 2 Phase 3 (verification) - **80% MILESTONE**

### Daily Progress (Week 2)
- **Day 6**: Ticket 3 start (top 10 files)
- **Day 7**: Ticket 3 continue (automated fixes)
- **Day 8**: Ticket 3 complete (verification)
- **Day 9**: Ticket 4 Phase 1 (complexity analysis)
- **Day 10**: Ticket 4 Phase 2 (refactoring) - **SPRINT COMPLETE**

---

## Toyota Way Alignment

### Kaizen (Continuous Improvement)
- ✅ Gradual lint reduction (20% per sprint)
- ✅ Incremental coverage increases
- ✅ Systematic complexity reduction

### Jidoka (Automation with Human Touch)
- ✅ Automated lint enforcement
- ✅ Automated complexity analysis
- ✅ Manual review for quality

### Genchi Genbutsu (Go and See)
- ✅ Measure actual coverage with tarpaulin
- ✅ Measure actual complexity with analysis tools
- ✅ Measure actual lint warnings with counter

---

## Definition of Done

A ticket is complete when:
1. ✅ All tasks checked off
2. ✅ All tests passing (100%)
3. ✅ Coverage metrics meet target (if applicable)
4. ✅ Documentation updated
5. ✅ Code reviewed (self-review for solo work)
6. ✅ Committed and pushed to main
7. ✅ Quality gates passing

---

## Sprint Retrospective Template

At end of Sprint 43, evaluate:
- What went well?
- What could be improved?
- What did we learn?
- What should we do differently in Sprint 44?

---

**Status**: ✅ SPRINT 43 PLAN APPROVED
**Start Date**: 2025-10-14 (continuing from Sprint 42)
**Next Step**: Begin Ticket 1 - Lint No-Increase Enforcement
