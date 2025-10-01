# Breaking Changes: Ruchy v1.89.0 → v3.62.12

**Date**: October 1, 2025
**Sprint**: Sprint 35 - Phase 2
**Analysis Status**: ✅ Complete for v189 failures

---

## Summary

**Total Breaking Changes Identified**: 2
**Total v189 Files Affected**: 5/24 (20.8%)
**Impact**: Critical - blocks graph algorithms and stream processing

### Breaking Changes by Frequency

| Breaking Change | Files Affected | Severity | Status |
|----------------|----------------|----------|--------|
| #1: Function Parameter Syntax | 4 files | 🔴 Critical | Under investigation |
| #2: RightBrace/Println Issue | 1 file | 🟡 Medium | Needs analysis |

---

## Breaking Change #1: Function Parameter Restrictions

### Error Message
```
✗ Syntax error: Function parameters must be simple identifiers or destructuring patterns
Error: Syntax error: Function parameters must be simple identifiers or destructuring patterns
```

### Affected Files (4 total)

1. **examples/algorithms/007-dijkstra/implementations/ruchy/dijkstra_v189.ruchy**
   - Algorithm: Dijkstra's shortest path
   - Category: Graph algorithms
   - Impact: Blocks shortest path validation

2. **examples/algorithms/015-traveling-salesman/implementations/ruchy/tsp_v189.ruchy**
   - Algorithm: Traveling Salesman Problem (NP-hard)
   - Category: Optimization algorithms
   - Impact: Blocks NP-hard problem validation

3. **examples/algorithms/016-topological-sort/implementations/ruchy/topological_sort_v189.ruchy**
   - Algorithm: Topological sort (DAG ordering)
   - Category: Graph algorithms
   - Impact: Blocks graph ordering validation

4. **examples/data-science/009-graph-analytics/implementations/ruchy/graph_analytics_v189.ruchy**
   - Algorithm: PageRank, centrality measures
   - Category: Data science - graph analytics
   - Impact: Blocks graph analytics validation

### Pattern Analysis

**Common Pattern**: All failing files use array parameters in function signatures

**Example from dijkstra_v189.ruchy**:
```ruchy
// v1.89.0 - WORKED ✅
fun add_edge(matrix: [i32; 25], from: i32, to: i32, weight: i32) -> [i32; 25] {
    let mut new_matrix = matrix
    // ... implementation
    new_matrix
}

// v3.62.12 - ERROR ❌
// Syntax error: Function parameters must be simple identifiers or destructuring patterns
```

**Additional Examples**:
```ruchy
// These signatures now fail:
fun dijkstra_shortest_paths(matrix: [i32; 25], source: i32) -> [i32; 5]
fun find_min_distance_vertex(distances: [i32; 5], visited: [bool; 5]) -> i32
fun verify_shortest_paths(matrix: [i32; 25], distances: [i32; 5], source: i32) -> bool
```

### Root Cause Hypothesis

v3.62.12 appears to have restricted function parameter syntax to:
- Simple identifiers: `x`, `y`, `count`
- Basic types: `x: i32`, `y: bool`
- Destructuring patterns: `(a, b)`, `[x, y]`

But **NOT**:
- Fixed-size array types: `matrix: [i32; 25]` ❌
- Array types with size literals: `arr: [T; N]` ❌

**Impact**: Cannot pass arrays by value to functions

### Scientific Impact

**Blocked Capabilities**:
- Graph algorithms using adjacency matrices
- Optimization problems with state spaces
- Data science algorithms using fixed-size buffers

**Regression Severity**: 🔴 **CRITICAL**
- Blocks 16.7% of proven v189 algorithm implementations (4/24)
- Prevents validation of important algorithm classes

### Potential Workarounds

#### Option 1: Use References (if supported)
```ruchy
// Instead of:
fun process(matrix: [i32; 25]) -> [i32; 25]

// Try:
fun process(matrix: &[i32; 25]) -> [i32; 25]
// OR
fun process(matrix: &mut [i32; 25])
```

**Status**: ⏳ Needs testing

#### Option 2: Wrapper Struct
```ruchy
struct Matrix25 {
    data: [i32; 25]
}

fun process(matrix: Matrix25) -> Matrix25 {
    // Access via matrix.data
}
```

**Status**: ⏳ Needs testing

#### Option 3: Return-Value Pattern (Current)
```ruchy
// Already using this - doesn't help with parameters
fun process(/* how to pass array in? */) -> [i32; 25] {
    // ...
}
```

**Status**: ❌ Doesn't solve parameter issue

#### Option 4: Pass Individual Elements
```ruchy
// Instead of:
fun add_edge(matrix: [i32; 25], from: i32, to: i32, weight: i32) -> [i32; 25]

// Use global or struct:
struct GraphContext {
    matrix: [i32; 25]
}

fun add_edge(ctx: GraphContext, from: i32, to: i32, weight: i32) -> GraphContext
```

**Status**: ⏳ Needs testing, requires refactoring

### Migration Priority

**Priority**: 🔴 **P0 - Critical**

**Reason**: Blocks 4 proven implementations representing important algorithm classes

**Target**: Resolve by Sprint 35 Day 3

### Next Steps

1. Test each workaround option (Options 1-4)
2. Identify which workaround is cleanest/most Ruchy-idiomatic
3. Create migration pattern document
4. Migrate 4 affected files
5. Validate formal verification still works
6. Document solution in INTEGRATION.md

---

## Breaking Change #2: RightBrace/Println Syntax Issue

### Error Message
```
✗ Syntax error: Expected RightBrace, found Identifier("println")
Error: Syntax error: Expected RightBrace, found Identifier("println")
```

### Affected Files (1 total)

1. **examples/data-science/007-stream-processing/implementations/ruchy/stream_processing_v189.ruchy**
   - Algorithm: Real-time stream processing
   - Category: Data science - stream processing
   - Impact: Blocks stream processing validation

### Pattern Analysis

**Hypothesis**: Likely a different syntax issue than Breaking Change #1

**Possible Causes**:
1. Block/brace matching issue
2. Statement termination issue
3. Control flow syntax change
4. Function body syntax change

**Status**: ⏳ Needs detailed analysis

### Investigation Steps

1. Binary search through file to find exact line causing error
2. Compare with working v189 files
3. Identify specific syntax pattern that broke
4. Test minimal reproducible example

### Migration Priority

**Priority**: 🟡 **P1 - High**

**Reason**: Blocks 1 proven implementation, but isolated to stream processing

**Target**: Resolve by Sprint 35 Day 4

---

## Overall Migration Strategy

### Phase 1: Breaking Change #1 (Days 2-3)
**Target**: 4 files with function parameter issues
**Approach**:
1. Test workarounds systematically
2. Choose best option
3. Migrate all 4 files with same pattern
4. Validate formal verification

### Phase 2: Breaking Change #2 (Day 4)
**Target**: 1 file with RightBrace issue
**Approach**:
1. Locate exact syntax error
2. Compare with working examples
3. Develop fix
4. Validate

### Success Criteria

**Minimum**: Restore all 5 v189 files to passing
**Target Success Rate**: 71/101 (70.3%) → closer to 85% threshold
**Stretch Goal**: Also fix some v193 failures

---

## Testing Methodology

### Validation Commands
```bash
# Test individual file
ruchy check <file>

# Test with formal verification
ruchy provability <file>
ruchy score <file>

# Ensure A+ quality maintained
ruchy score <file> | grep "Score:"
# Expected: Score: 0.9+ (A or A+)
```

### Regression Testing
```bash
# After migration, re-run full suite
make test-all-examples

# Check success rate improvement
cat test-results.json | jq '.summary.success_rate'
# Target: >70% (from current 65.3%)
```

---

## Summary Statistics

### Impact by Category

| Category | v189 Files | Affected | Percentage |
|----------|------------|----------|------------|
| **Algorithms** | 19 | 3 | 15.8% |
| **Data Science** | 5 | 2 | 40.0% |
| **Total** | 24 | 5 | 20.8% |

### Impact by Algorithm Class

| Algorithm Class | Files Affected |
|----------------|----------------|
| Graph Algorithms | 3 (dijkstra, tsp, topological_sort, graph_analytics) |
| Stream Processing | 1 (stream_processing) |

### Breaking Changes Distribution

```
Breaking Change #1: ████████ 80% (4/5 files)
Breaking Change #2: ██ 20% (1/5 files)
```

---

## GitHub Issues to Create

### Issue #1: Function Parameter Syntax Restriction
**Title**: [v3.62.12] Breaking Change: Function parameters cannot use array types `[T; N]`
**Labels**: `breaking-change`, `v3.62.12`, `migration`, `p0-critical`
**Assignee**: TBD
**Files Affected**: 4

### Issue #2: RightBrace/Println Syntax Error
**Title**: [v3.62.12] Breaking Change: "Expected RightBrace, found Identifier(println)" syntax error
**Labels**: `breaking-change`, `v3.62.12`, `migration`, `p1-high`
**Assignee**: TBD
**Files Affected**: 1

---

*Analysis completed: October 1, 2025*
*Sprint 35 - Phase 2 Complete*
