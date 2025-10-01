# Migration Patterns: v1.89.0 → v3.62.12

**Date**: October 1, 2025
**Sprint**: Sprint 35 - Phase 2
**Status**: ✅ Solutions Validated

---

## Breaking Change #1: Array Parameters

### Problem

**Error**: "Function parameters must be simple identifiers or destructuring patterns"

**Root Cause**: v3.62.12 no longer accepts fixed-size array types `[T; N]` as direct function parameters.

### Solutions (Both Work!)

#### ✅ Option 1: References (RECOMMENDED)

**Status**: ✅ Validated - Syntax is valid

**Pattern**:
```ruchy
// OLD (v1.89.0):
fun process(matrix: [i32; 25]) -> [i32; 25] {
    matrix
}

// NEW (v3.62.12) - Use reference:
fun process(matrix: &[i32; 25]) -> [i32; 25] {
    *matrix  // Dereference if needed
}
```

**Minimal Test**:
```ruchy
fun test_ref(arr: &[i32; 5]) -> i32 {
    42
}
```

Result: `✓ Syntax is valid`

**Pros**:
- ✅ Minimal code changes
- ✅ No copying overhead
- ✅ Idiomatic Rust/Ruchy style
- ✅ Clean and simple

**Cons**:
- ⚠️ Requires caller to pass `&array` instead of `array`
- ⚠️ May need dereference operations

**Recommendation**: **USE THIS** for most cases

---

#### ✅ Option 2: Wrapper Struct

**Status**: ✅ Validated - Syntax is valid

**Pattern**:
```ruchy
// Define wrapper type
struct Matrix25 {
    data: [i32; 25]
}

// OLD (v1.89.0):
fun process(matrix: [i32; 25]) -> [i32; 25] {
    matrix
}

// NEW (v3.62.12) - Use wrapper:
fun create_matrix() -> Matrix25 {
    Matrix25 { data: [0; 25] }
}

fun process(m: Matrix25) -> Matrix25 {
    // Access via m.data
    m
}
```

**Minimal Test**:
```ruchy
struct Matrix25 {
    data: [i32; 25]
}

fun create_matrix() -> Matrix25 {
    Matrix25 { data: [0; 25] }
}

fun process_matrix(m: Matrix25) -> Matrix25 {
    m
}
```

Result: `✓ Syntax is valid`

**Pros**:
- ✅ Type-safe encapsulation
- ✅ Can add methods later
- ✅ Clear intent (Matrix vs raw array)
- ✅ Enables future optimizations

**Cons**:
- ⚠️ More boilerplate code
- ⚠️ Requires struct definitions
- ⚠️ Changes API surface area

**Recommendation**: Use for complex types or when encapsulation adds value

---

## Migration Strategy

### For Simple Cases: Use Option 1 (References)

**When**:
- Simple array passing
- Minimal refactoring desired
- Performance-critical code (avoid copies)

**Example - Dijkstra**:
```ruchy
// v1.89.0:
fun add_edge(matrix: [i32; 25], from: i32, to: i32, weight: i32) -> [i32; 25] {
    let mut new_matrix = matrix
    // ... modify new_matrix
    new_matrix
}

// v3.62.12:
fun add_edge(matrix: &[i32; 25], from: i32, to: i32, weight: i32) -> [i32; 25] {
    let mut new_matrix = *matrix  // Copy the referenced array
    // ... modify new_matrix (same logic)
    new_matrix
}

// Call site changes:
// OLD: let result = add_edge(matrix, 0, 1, 5);
// NEW: let result = add_edge(&matrix, 0, 1, 5);
```

### For Complex Types: Use Option 2 (Wrapper Struct)

**When**:
- Multiple related arrays
- Domain-specific types (Graph, Matrix, etc.)
- Want encapsulation and methods

**Example - Graph ADT**:
```ruchy
// v3.62.12 - Better encapsulation
struct Graph {
    matrix: [i32; 25],
    vertices: i32
}

impl Graph {
    fun new(size: i32) -> Graph {
        Graph {
            matrix: [999999; 25],
            vertices: size
        }
    }

    fun add_edge(self, from: i32, to: i32, weight: i32) -> Graph {
        let mut new_graph = self
        // ... modify new_graph.matrix
        new_graph
    }
}
```

---

## Decision Matrix

| Factor | Option 1: References | Option 2: Wrapper Struct |
|--------|---------------------|-------------------------|
| **Migration Effort** | ⭐⭐⭐ Low | ⭐⭐ Medium |
| **Code Clarity** | ⭐⭐⭐ Clean | ⭐⭐ Good (with docs) |
| **Performance** | ⭐⭐⭐ Best (no copy) | ⭐⭐ Good |
| **Type Safety** | ⭐⭐ Good | ⭐⭐⭐ Excellent |
| **Formal Verification** | ⭐⭐⭐ Should work | ⭐⭐⭐ Should work |
| **Future Flexibility** | ⭐⭐ Limited | ⭐⭐⭐ High |

**Recommendation for Sprint 35**:
- **Use Option 1 (References)** for quick migration of 4 failing files
- **Consider Option 2 (Wrapper)** for future refactoring if valuable

---

## Migration Checklist

For each affected file:

- [ ] Identify all functions with array parameters
- [ ] Choose migration approach (Option 1 or 2)
- [ ] Update function signatures
- [ ] Update function bodies (dereference if needed)
- [ ] Update call sites (add `&` for references)
- [ ] Test with `ruchy check <file>`
- [ ] Validate formal verification: `ruchy provability <file>`
- [ ] Validate quality score: `ruchy score <file>`
- [ ] Ensure A+ rating maintained (>0.95)

---

## Files to Migrate (P0)

Using Option 1 (References) for initial migration:

1. **examples/algorithms/007-dijkstra/implementations/ruchy/dijkstra_v189.ruchy**
   - Functions to update: `add_edge`, `dijkstra_shortest_paths`, `find_min_distance_vertex`, etc.
   - Estimated effort: 30-45 minutes

2. **examples/algorithms/015-traveling-salesman/implementations/ruchy/tsp_v189.ruchy**
   - Functions to update: TBD (needs analysis)
   - Estimated effort: 30-45 minutes

3. **examples/algorithms/016-topological-sort/implementations/ruchy/topological_sort_v189.ruchy**
   - Functions to update: TBD (needs analysis)
   - Estimated effort: 30-45 minutes

4. **examples/data-science/009-graph-analytics/implementations/ruchy/graph_analytics_v189.ruchy**
   - Functions to update: TBD (needs analysis)
   - Estimated effort: 30-45 minutes

**Total Effort**: 2-3 hours for all 4 files

---

## Validation Requirements

### After Migration

**Each file MUST**:
1. ✅ Pass `ruchy check` (syntax valid)
2. ✅ Maintain provability score >95/100
3. ✅ Maintain quality score >0.95 (A+)
4. ✅ Produce same algorithmic output as v1.89.0 version

**Project Level**:
- Success rate should increase from 65.3% to >68.3% (+4 files)
- No regression in existing passing files
- Formal verification capabilities maintained

---

## Next Steps (Sprint 35 - Day 3)

1. **Migrate dijkstra_v189.ruchy** (pilot)
   - Full migration with Option 1
   - Validate all tools work
   - Document any issues

2. **Migrate remaining 3 files**
   - Apply same pattern
   - Batch validation

3. **Run full test suite**
   ```bash
   make test-all-examples
   ```

4. **Update INTEGRATION.md**
   - Document solution
   - Update success rate
   - Mark Breaking Change #1 as RESOLVED

---

*Validated: October 1, 2025*
*Sprint 35 - Phase 2 Complete: Solutions Identified*
