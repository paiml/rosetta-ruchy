---
name: Breaking Change (v3.x Migration)
about: Report a breaking change from Ruchy v1.89.0 to v3.62.12
title: '[v3.62.12] Breaking Change: '
labels: 'breaking-change, v3.62.12, migration'
assignees: ''

---

## Breaking Change Description

**Error Message**:
```
<!-- Paste the exact error message from `ruchy check` -->
```

**Affected Version Range**: v1.89.0 → v3.62.12

**Category**: <!-- e.g., Syntax, Type System, Standard Library -->

---

## Affected Files

**Total Files Affected**: <!-- Number -->

**List of Files**:
1. `examples/.../file1.ruchy`
2. `examples/.../file2.ruchy`

**Impact by Category**:
- Algorithms: X files
- Data Science: Y files
- Advanced AI: Z files

---

## Code Example

### Working Code (v1.89.0) ✅

```ruchy
<!-- Code that worked in v1.89.0 -->
```

### Broken Code (v3.62.12) ❌

```ruchy
<!-- Same code failing in v3.62.12 -->
```

**Error Output**:
```
<!-- Full error message and stack trace if available -->
```

---

## Root Cause Analysis

**Hypothesis**:
<!-- What changed in v3.62.12 that broke this? -->

**Evidence**:
<!-- Patterns observed, other affected files, similar issues -->

---

## Proposed Workarounds

### Option 1: <!-- Name of workaround -->

```ruchy
<!-- Example code for workaround -->
```

**Pros**:
-

**Cons**:
-

**Testing Status**: ⏳ Not tested / ✅ Works / ❌ Doesn't work

### Option 2: <!-- Alternative workaround -->

```ruchy
<!-- Example code -->
```

**Pros**:
-

**Cons**:
-

**Testing Status**: ⏳ Not tested / ✅ Works / ❌ Doesn't work

---

## Migration Plan

**Priority**:
- [ ] 🔴 P0 - Critical (blocks validation)
- [ ] 🟡 P1 - High (impacts important features)
- [ ] 🟢 P2 - Medium (legacy files)
- [ ] ⚪ P3 - Low (experimental files)

**Target Resolution**: Sprint 35 Day <!-- X -->

**Migration Steps**:
1. Test workaround options
2. Choose best approach
3. Migrate affected files
4. Validate formal verification still works
5. Update INTEGRATION.md

---

## Scientific Impact

**Blocked Capabilities**:
<!-- What algorithms/features can't be validated? -->

**Regression Severity**:
- [ ] 🔴 Critical - Blocks core validation goals
- [ ] 🟡 Medium - Reduces coverage
- [ ] 🟢 Low - Experimental features only

**Formal Verification Impact**:
<!-- Does this affect `ruchy provability` or `ruchy score`? -->

---

## Testing Validation

**Test Commands**:
```bash
# Verify fix works
ruchy check examples/.../fixed_file.ruchy

# Verify formal verification maintained
ruchy provability examples/.../fixed_file.ruchy
ruchy score examples/.../fixed_file.ruchy

# Expected results:
# - Syntax valid: ✓
# - Provability score: >95/100
# - Quality score: >0.95 (A+)
```

**Success Criteria**:
- [ ] All affected files pass `ruchy check`
- [ ] Formal verification scores maintained
- [ ] Success rate increases by expected amount

---

## References

- **Baseline Analysis**: `docs/BASELINE_V3_62_12.md`
- **Breaking Changes Doc**: `docs/BREAKING_CHANGES_V3.md`
- **Sprint Plan**: `docs/SPRINT_35_V3_MIGRATION.md`
- **Test Results**: `test-results.json`

---

## Additional Context

<!-- Any other relevant information about this breaking change -->
