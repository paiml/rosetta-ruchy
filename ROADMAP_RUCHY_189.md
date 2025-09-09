# Ruchy 1.89 Migration Roadmap

**Priority**: P0 - Critical migration required
**Current Status**: Ruchy 1.89.0 installed
**Target**: 100% compatibility with explicit mutability model

## Phase 1: Migration Foundation (Sprint 1)

### 1.1 Update Project Configuration
- [x] Upgrade to Ruchy 1.89.0
- [ ] Update INTEGRATION.md with v1.89.0 status
- [ ] Update Makefile REQUIRED_VERSION to 1.89.0
- [ ] Test all Ruchy toolchain commands (check, runtime, provability, score)

### 1.2 Establish Migration Standards
- [ ] **MANDATORY**: Use `fun` keyword only (not `fn`) in all examples
- [ ] **MANDATORY**: Add `mut` keyword for all variable reassignments
- [ ] **MANDATORY**: Remove all shebang lines from `.ruchy` files
- [ ] **MANDATORY**: Update array/vector syntax to v1.89 standard

## Phase 2: Systematic Algorithm Migration (Sprint 2-4)

### 2.1 Critical Path - Core Algorithms (Sprint 2)
**Target**: 5 algorithms working perfectly

1. **001-fibonacci** ✅ (Already working)
2. **002-quicksort** - Add `mut` to swap variables and loop counters
3. **003-mergesort** - Add `mut` to merge operation variables
4. **004-binary-search** - Add `mut` to search bounds variables
5. **021-counting-sort** ✅ (Already working)
6. **022-selection-sort** ✅ (Already working)

### 2.2 Data Structure Algorithms (Sprint 3)
**Target**: Graph and tree algorithms working

- **005-hash-table** - Add `mut` to hash table operations
- **006-red-black-tree** - Add `mut` to tree rotation variables
- **007-dijkstra** - Add `mut` to distance arrays and queue operations
- **017-binary-search-tree** - Add `mut` to tree modification operations

### 2.3 Dynamic Programming Algorithms (Sprint 4)
**Target**: Complex DP patterns working

- **008-longest-common-subsequence** - Add `mut` to DP table updates
- **009-knapsack-problem** - Add `mut` to value table operations
- **010-edit-distance** - Add `mut` to distance matrix operations
- **011-matrix-chain-multiplication** - Add `mut` to cost matrix operations

## Phase 3: Quality Enforcement Integration (Sprint 5)

### 3.1 Update Quality Gates for v1.89
- [ ] Update pre-commit hook to check for v1.89.0
- [ ] Add explicit mutability validation to quality gates
- [ ] Ensure `fun` keyword enforcement
- [ ] Update complexity verification for new syntax

### 3.2 Toyota Way Quality Standards
- [ ] **ZERO** stub implementations allowed
- [ ] **ZERO** SATD comments (TODO/FIXME) 
- [ ] **100%** explicit mutability compliance
- [ ] **100%** algorithm complexity proofs generated

## Phase 4: Advanced Features Validation (Sprint 6)

### 4.1 New v1.89 Features Testing
- [ ] Test explicit return statements
- [ ] Validate array syntax `[T; N]` improvements
- [ ] Test array initialization `[value; size]`
- [ ] Verify both `fun` and `fn` support (prefer `fun`)

### 4.2 Formal Verification Upgrade
- [ ] Ensure provability analysis works with explicit mutability
- [ ] Validate complexity analysis with new syntax
- [ ] Test quality scoring with v1.89 patterns
- [ ] Verify SMT solver integration still functional

## Migration Pattern Template

For each algorithm, follow this exact pattern:

### Before (v1.88.0 and earlier):
```rust
fun quicksort(arr: &mut [i32]) {
    let len = arr.len();
    let pivot_index = partition(arr);
    // Implicit mutability - BROKEN in v1.89
}
```

### After (v1.89.0 explicit mutability):
```rust
fun quicksort(arr: &mut [i32]) {
    let len = arr.len();
    let mut pivot_index = partition(arr);  // Explicit mut required
    pivot_index = adjust_partition(pivot_index);  // Reassignment now works
}
```

## Success Criteria

### Sprint Completion Gates:
1. **Sprint 2**: 5/22 algorithms working (22.7% → 45.4%)
2. **Sprint 3**: 13/22 algorithms working (59.0%)
3. **Sprint 4**: 22/22 algorithms working (100%)
4. **Sprint 5**: All quality gates passing
5. **Sprint 6**: Full formal verification operational

### Quality Metrics:
- **Syntax Validation**: 100% `ruchy check` pass rate
- **Complexity Proofs**: 100% `ruchy provability` success
- **Quality Scores**: Maintain A+ (1.00/1.0) ratings
- **Performance**: Maintain ≤5% Rust baseline variance

## Risk Mitigation

### High-Risk Areas:
1. **Complex loop structures** - Multiple reassignments
2. **Tree/graph algorithms** - Pointer manipulation patterns
3. **DP algorithms** - Matrix/table update patterns
4. **Sorting algorithms** - In-place modification patterns

### Mitigation Strategy:
1. Test each algorithm individually after migration
2. Maintain comprehensive test coverage
3. Use quality gates to catch regressions
4. Document all syntax changes in INTEGRATION.md

## Implementation Priority

**Week 1**: Foundation + Core algorithms (Sprints 1-2)  
**Week 2**: Data structures + DP algorithms (Sprints 3-4)  
**Week 3**: Quality enforcement + Advanced features (Sprints 5-6)

**DELIVERABLE**: 100% compatibility with Ruchy 1.89.0 explicit mutability model while maintaining scientific rigor and Toyota Way quality standards.