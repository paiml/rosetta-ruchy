# Breaking Changes & Parser Issues: Ruchy v3.62.12

**Date**: October 1, 2025
**Severity**: 🔴 CRITICAL
**Status**: RESOLVED with workarounds
**Issues Found**: 2 critical breaking changes

---

## ⚠️ CRITICAL BREAKING CHANGES IDENTIFIED

### Issue #1: `from` is Now a Reserved Keyword
**Severity**: 🔴 CRITICAL
**Impact**: ANY function using `from` as parameter name will FAIL
**Solution**: ✅ Rename to `from_vertex`, `source`, etc.

### Issue #2: Parser Bug with `&[T; N]` References
**Severity**: 🔴 CRITICAL
**Impact**: Cannot use array references with 3+ parameters
**Solution**: ✅ Use wrapper structs instead

---

## Summary

Ruchy v3.62.12 has TWO critical breaking changes:

1. **`from` keyword reservation**: Parameter names using `from` now fail with parser error
2. **Array reference parser bug**: Fixed-size array references `&[T; N]` fail as parameters when there are 3 or more total parameters

Both issues BLOCK straightforward migration and require code changes.

---

## Issue #1: `from` Reserved Keyword (NEWLY DISCOVERED)

### Reproduction

#### ❌ FAILS: Using `from` as parameter name
```ruchy
fun test(from: i32) -> i32 {
    from * 5
}
```
**Result**:
```
✗ Syntax error: Function parameters must be simple identifiers or destructuring patterns
```

#### ✅ Works: Using different parameter name
```ruchy
fun test(source: i32) -> i32 {
    source * 5
}
```
**Result**: `✓ Syntax is valid`

### Root Cause

`from` became a **reserved keyword** in v3.62.12 (likely for import statements):
```ruchy
// Hypothetical future syntax:
from std.collections import HashMap
```

In v1.89.0, `from` was a valid identifier and commonly used in graph algorithms:
```ruchy
// v1.89.0 - WORKED ✅
fun add_edge(matrix: [i32; 25], from: i32, to: i32, weight: i32) -> [i32; 25]

// v3.62.12 - FAILS ❌
// Parser error: "Function parameters must be simple identifiers..."
```

### Solution

**Rename all `from` parameters**:
- `from` → `from_vertex`, `source`, `src`, `origin`, etc.
- Check for `to` as well (currently NOT reserved, but may be in future)

### Impact

**Affects**: ANY code using `from` as:
- Function parameter name
- Variable name
- Struct field name

**Common in**:
- Graph algorithms (Dijkstra, TSP, A*, etc.)
- Network code (send from/to addresses)
- Time range code (from/to dates)

---

## Issue #2: Array Reference Parser Bug

### Reproduction

#### ✅ Works: Single Parameter
```ruchy
fun test_solo(arr: &[i32; 25]) -> i32 {
    42
}
```
**Result**: `✓ Syntax is valid`

### ✅ Works: Two Parameters
```ruchy
fun test_two(arr: &[i32; 25], x: i32) -> i32 {
    42
}
```
**Result**: `✓ Syntax is valid`

### ❌ FAILS: Three Parameters
```ruchy
fun test_three(arr: &[i32; 25], x: i32, y: i32) -> i32 {
    42
}
```
**Result**:
```
✗ Syntax error: Function parameters must be simple identifiers or destructuring patterns
Error: Syntax error: Function parameters must be simple identifiers or destructuring patterns
```

### ❌ FAILS: Four Parameters (Real-World Case)
```ruchy
fun add_edge(matrix: &[i32; 25], from: i32, to: i32, weight: i32) -> [i32; 25] {
    let mut result = [0; 25]
    result
}
```
**Result**: Same error as above

---

## Pattern Analysis

### What Works ✅

| Pattern | Parameters | Status |
|---------|-----------|--------|
| `&[T; N]` | 1 | ✅ Valid |
| `&[T; N]` | 2 | ✅ Valid |
| `&T` | 3+ | ✅ Valid |
| `&[T]` (slice) | 3+ | ✅ Valid |

### What Fails ❌

| Pattern | Parameters | Status |
|---------|-----------|--------|
| `&[T; N]` | 3 | ❌ Parser error |
| `&[T; N]` | 4+ | ❌ Parser error |

---

## Root Cause Hypothesis

The Ruchy parser appears to misinterpret the **semicolon** in `&[T; N]` when parsing parameter lists with 3+ parameters.

**Possible Parser State Issue**:
1. Parser sees `&[i32`
2. Parser sees `;`
3. With 3+ parameters, parser may confuse `;` with:
   - Parameter separator (expecting `,`)
   - Statement terminator
   - Array size delimiter (correct interpretation)

**Evidence**:
- Works with 1-2 parameters (parser state different?)
- Works with `&[T]` (no semicolon)
- Works with `&T` (no brackets)
- Fails ONLY with `&[T; N]` + 3+ parameters

---

## Impact on Migration

### Blocked Migration Pattern

**Original Plan (FAILED)**:
```ruchy
// v1.89.0 → v3.62.12 using references
fun add_edge(matrix: [i32; 25], from: i32, to: i32, weight: i32) -> [i32; 25]
// ↓
fun add_edge(matrix: &[i32; 25], from: i32, to: i32, weight: i32) -> [i32; 25]  // ❌ FAILS
```

### Required Workaround

Must use **Option 2: Wrapper Struct**:
```ruchy
struct Matrix25 {
    data: [i32; 25]
}

fun add_edge(matrix: Matrix25, from: i32, to: i32, weight: i32) -> Matrix25 {
    // Access via matrix.data
    let mut new_matrix = matrix
    new_matrix.data[from * 5 + to] = weight
    new_matrix
}
```

---

## Affected Files

All 4 files from Breaking Change #1 require wrapper struct approach:

1. **examples/algorithms/007-dijkstra/implementations/ruchy/dijkstra_v189.ruchy**
   - Functions affected: `add_edge` (4 params), `find_min_distance_vertex` (2 params ✅), `dijkstra_shortest_paths` (2 params ✅)
   - **Note**: Some functions can use references (2 params), others need wrapper (3+ params)

2. **examples/algorithms/015-traveling-salesman/implementations/ruchy/tsp_v189.ruchy**
   - Analysis needed

3. **examples/algorithms/016-topological-sort/implementations/ruchy/topological_sort_v189.ruchy**
   - Analysis needed

4. **examples/data-science/009-graph-analytics/implementations/ruchy/graph_analytics_v189.ruchy**
   - Analysis needed

---

## Testing Methodology

### Automated Test Suite
```bash
# Test 1: Confirm single param works
cat > /tmp/test1.ruchy << 'EOF'
fun test(arr: &[i32; 25]) -> i32 { 42 }
EOF
ruchy check /tmp/test1.ruchy  # Should pass

# Test 2: Confirm two params work
cat > /tmp/test2.ruchy << 'EOF'
fun test(arr: &[i32; 25], x: i32) -> i32 { 42 }
EOF
ruchy check /tmp/test2.ruchy  # Should pass

# Test 3: Confirm three params FAIL
cat > /tmp/test3.ruchy << 'EOF'
fun test(arr: &[i32; 25], x: i32, y: i32) -> i32 { 42 }
EOF
ruchy check /tmp/test3.ruchy  # Should FAIL with parser error

# Test 4: Confirm slices work (no size)
cat > /tmp/test4.ruchy << 'EOF'
fun test(arr: &[i32], x: i32, y: i32) -> i32 { 42 }
EOF
ruchy check /tmp/test4.ruchy  # Should pass
```

---

## Workaround Validation

### Wrapper Struct Pattern (RECOMMENDED)
```ruchy
struct Matrix25 {
    data: [i32; 25]
}

fun create_matrix() -> Matrix25 {
    Matrix25 { data: [0; 25] }
}

fun process(m: Matrix25, x: i32, y: i32, z: i32) -> Matrix25 {
    let mut result = m
    result.data[0] = x + y + z
    result
}
```

**Validation**: ✅ Confirmed working with `ruchy check`

---

## Recommended Actions

### Immediate (Sprint 35 Day 3)
1. ✅ Document both breaking changes (this file)
2. ✅ Update MIGRATION_PATTERNS_V3.md to mark Option 1 as BLOCKED
3. ✅ Re-implement dijkstra_v362.ruchy using wrapper struct (Option 2)
4. ✅ Rename all `from` parameters to `from_vertex`
5. ✅ Validate wrapper struct migration passes syntax check
6. ⏳ Apply same pattern to remaining 3 files

### Short-Term (Post-Sprint 35)
6. ⏳ Create GitHub issue in Ruchy project repository
7. ⏳ Provide reproducible test case to Ruchy maintainers
8. ⏳ Monitor for parser fix in future Ruchy releases

### Long-Term (Future Sprints)
9. ⏳ If parser is fixed, consider migrating wrapper structs back to references
10. ⏳ Update INTEGRATION.md to track parser bug resolution

---

## GitHub Issue Template

**Title**: [v3.62.12] Parser bug: `&[T; N]` fails with 3+ function parameters

**Body**:
```markdown
## Description
Ruchy v3.62.12 parser fails when a function has:
- 3 or more parameters
- First parameter is a fixed-size array reference: `&[T; N]`

## Reproduction
<!-- paste minimal test case -->

## Expected Behavior
Should parse successfully (works with 1-2 parameters)

## Actual Behavior
```
✗ Syntax error: Function parameters must be simple identifiers or destructuring patterns
```

## Environment
- Ruchy version: 3.62.12
- OS: Linux 6.8.0-79-lowlatency
- Installed via: cargo install ruchy

## Workaround
Use wrapper struct instead of direct array reference.

## Impact
Blocks migration from v1.89.0 (which allowed `[T; N]` parameters)
```

---

## Scientific Impact

**Regression Severity**: 🔴 CRITICAL

**Blocked Capabilities**:
- Cannot migrate graph algorithms using adjacency matrices
- Cannot use ergonomic reference passing for multi-parameter functions
- Forces additional boilerplate (wrapper structs)

**Migration Cost**:
- **Original estimate**: 2-3 hours for 4 files (Option 1: References)
- **Revised estimate**: 4-6 hours for 4 files (Option 2: Wrapper Structs)
  - Additional complexity: struct definitions, method implementations
  - More code changes: call sites need struct construction/deconstruction

**Success Criteria Impact**:
- Can still achieve 100% migration success
- Quality scores should remain A+ (structs are idiomatic)
- Provability should be maintained (structs don't affect logic)
- **BUT**: More verbose code, reduces Ruchy's "ergonomic" advantage

---

*Discovered: October 1, 2025 during Sprint 35 - Phase 3 migration*
*Status: Blocking - requires wrapper struct workaround*
