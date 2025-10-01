# Breaking Changes & Parser Issues: Ruchy v3.62.12

**Date**: October 1, 2025
**Last Updated**: October 1, 2025 (Sprint 36 - Added Breaking Change #3)
**Severity**: 🔴 CRITICAL
**Status**: RESOLVED with workarounds
**Issues Found**: 3 critical breaking changes

---

## ⚠️ CRITICAL BREAKING CHANGES IDENTIFIED

### Issue #1: `from` is Now a Reserved Keyword
**Severity**: 🔴 CRITICAL
**Impact**: ANY code using `from` as identifier will FAIL
**Solution**: ✅ Rename to `from_vertex`, `source`, etc.
**Discovered**: Sprint 35 (Day 3)
**GitHub Issue**: https://github.com/paiml/ruchy/issues/23

### Issue #2: Parser Bug with `&[T; N]` References
**Severity**: 🔴 CRITICAL
**Impact**: Cannot use array references with 3+ parameters
**Solution**: ✅ Use wrapper structs instead
**Discovered**: Sprint 35 (Day 3)
**GitHub Issue**: https://github.com/paiml/ruchy/issues/24

### Issue #3: No `mut` in Tuple Destructuring 🆕
**Severity**: 🔴 CRITICAL
**Impact**: Cannot use `mut` keywords in tuple destructuring patterns
**Solution**: ✅ Use separate `let mut` declarations after destructuring
**Discovered**: Sprint 36 (stream_processing migration)
**GitHub Issue**: https://github.com/paiml/ruchy/issues/25

---

## Summary

Ruchy v3.62.12 has **THREE** critical breaking changes:

1. **`from` keyword reservation**: All identifiers using `from` now fail with parser error
2. **Array reference parser bug**: Fixed-size array references `&[T; N]` fail as parameters when there are 3 or more total parameters
3. **No `mut` in tuple destructuring**: Cannot use `mut` keywords in destructuring patterns - must declare mutability separately

All three issues BLOCK straightforward migration and require code changes.

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


---

## Issue #3: No `mut` in Tuple Destructuring (NEWLY DISCOVERED - Sprint 36)

### Reproduction

#### ❌ FAILS: Using `mut` in tuple destructuring
```ruchy
fun create_tuple() -> ([i32; 10], i32, i32) {
    ([0; 10], 0, 0)
}

fun main() {
    let (mut arr, mut x, y) = create_tuple();
    println!("x: {}", x);
}
```
**Result**:
```
✗ Syntax error: Expected RightBrace, found Let
```

#### ✅ Works: Separate `let mut` declarations
```ruchy
fun create_tuple() -> ([i32; 10], i32, i32) {
    ([0; 10], 0, 0)
}

fun main() {
    let (arr_temp, x_temp, y) = create_tuple();
    let mut arr = arr_temp;
    let mut x = x_temp;
    println!("x: {}", x);
}
```
**Result**: `✓ Syntax is valid`

### Root Cause

The parser in v3.62.12 does **not** support `mut` keywords inside tuple destructuring patterns. This was valid syntax in v1.89.0.

**v1.89.0 - WORKED ✅**:
```ruchy
let (mut buffer, mut head, tail) = create_stream_buffer();
buffer = update_buffer(buffer);  // Can mutate
head = head + 1;                  // Can mutate
```

**v3.62.12 - FAILS ❌**:
```
✗ Syntax error: Expected RightBrace, found Let
Error: Syntax error: Expected RightBrace, found Let
```

### Solution

**Workaround: Destructure first, then declare mutability**:
```ruchy
// Step 1: Destructure without mut
let (buffer_temp, head_temp, tail) = create_stream_buffer();

// Step 2: Declare mutability separately
let mut buffer = buffer_temp;
let mut head = head_temp;

// Step 3: Use mutable variables as normal
buffer = update_buffer(buffer);
head = head + 1;
```

### Impact

**Affected Code Patterns**:
- ❌ `let (mut x, mut y) = tuple` - FAILS
- ❌ `let (mut arr, head, tail) = create_stream()` - FAILS
- ❌ `let (x, mut y, mut z) = func()` - FAILS
- ✅ `let (x, y) = tuple; let mut x = x;` - WORKS

**Files Affected**:
- `stream_processing_v189.ruchy` - ✅ FIXED (line 168)
- `topological_sort_v189.ruchy` - ⏳ Would need fixing (multiple instances)
- Any code using mutable tuple destructuring

### Reproducible Test Case

```bash
# Test 1: Confirm simple tuple destructuring works
cat > /tmp/test_tuple_no_mut.ruchy << 'EOF'
fun create_tuple() -> (i32, i32, i32) {
    (0, 1, 2)
}

fun main() {
    let (x, y, z) = create_tuple();
    println!("x: {}", x);
}
EOF
ruchy check /tmp/test_tuple_no_mut.ruchy  # Should pass

# Test 2: Confirm mut in destructuring FAILS
cat > /tmp/test_tuple_with_mut.ruchy << 'EOF'
fun create_tuple() -> (i32, i32, i32) {
    (0, 1, 2)
}

fun main() {
    let (mut x, mut y, z) = create_tuple();
    println!("x: {}", x);
}
EOF
ruchy check /tmp/test_tuple_with_mut.ruchy  # Should FAIL

# Test 3: Confirm separate mut declarations work
cat > /tmp/test_tuple_separate_mut.ruchy << 'EOF'
fun create_tuple() -> (i32, i32, i32) {
    (0, 1, 2)
}

fun main() {
    let (x_temp, y_temp, z) = create_tuple();
    let mut x = x_temp;
    let mut y = y_temp;
    x = x + 1;
    y = y + 1;
    println!("x: {}, y: {}", x, y);
}
EOF
ruchy check /tmp/test_tuple_separate_mut.ruchy  # Should pass
```

### Migration Example

**Before (v1.89.0)**:
```ruchy
fun test_stream_processing() {
    let (mut buffer, mut head, tail) = create_stream_buffer();

    while head < 500 {
        let (new_buffer, new_head, new_tail) = stream_push(buffer, head, tail, value);
        buffer = new_buffer;
        head = new_head;
    }
}
```

**After (v3.62.12)**:
```ruchy
fun test_stream_processing() {
    let (buffer_temp, head_temp, tail) = create_stream_buffer();
    let mut buffer = buffer_temp;
    let mut head = head_temp;

    while head < 500 {
        let (new_buffer_temp, new_head, new_tail) = stream_push(buffer, head, tail, value);
        buffer = new_buffer_temp;
        head = new_head;
    }
}
```

### Automated Transformation

For automated migration scripts, use `sed` to transform the pattern:

```bash
# Pattern: Replace tuple destructuring with mut
sed 's/let (\(mut [^)]*\)) = /let (\1_temp) = /' | \
  # Then add separate let mut declarations
  # (requires more complex multiline sed or manual fixing)
```

**Note**: This is difficult to automate fully with `sed` - manual review recommended.

---

## All Three Breaking Changes: Combined Impact

### Summary of Required Changes for v1.89.0 → v3.62.12 Migration

1. **Rename all `from` identifiers**:
   - Parameters: `from: i32` → `from_vertex: i32`
   - Variables: `let from = ...` → `let source = ...`
   - Struct fields: `struct Edge { from: i32 }` → `struct Edge { from_vertex: i32 }`

2. **Replace array parameters with wrapper structs**:
   ```ruchy
   // OLD:
   fun process(arr: [i32; 100], x: i32, y: i32) -> [i32; 100]

   // NEW:
   struct Array100 { data: [i32; 100] }
   fun process(arr: Array100, x: i32, y: i32) -> Array100
   ```

3. **Remove `mut` from tuple destructuring**:
   ```ruchy
   // OLD:
   let (mut x, mut y) = tuple;

   // NEW:
   let (x_temp, y_temp) = tuple;
   let mut x = x_temp;
   let mut y = y_temp;
   ```

### Migration Effort Estimate

Based on Sprint 35+36 experience:

| File Complexity | Estimated Time | Automation |
|----------------|----------------|------------|
| Simple (no tuples, few arrays) | 30-60 min | 90% automated |
| Medium (arrays, simple tuples) | 1-2 hours | 70% automated |
| Complex (nested tuples, many arrays) | 3-6 hours | 40% automated |

**Success Rate**: 80% for simple/medium files (4/5 in Sprint 36)
