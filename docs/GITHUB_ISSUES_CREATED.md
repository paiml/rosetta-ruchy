# GitHub Issues Created for Ruchy Project

**Date**: October 1, 2025
**Repository**: https://github.com/paiml/ruchy
**Created During**: Sprint 36 migration work

---

## Summary

Created **3 comprehensive GitHub issues** documenting all critical breaking changes discovered during Sprint 35+36 migration work.

---

## Issues Created

### Issue #23: Reserved Keyword Breaking Change
**Title**: [v3.62.12] Breaking Change: 'from' is now a reserved keyword
**URL**: https://github.com/paiml/ruchy/issues/23
**Severity**: 🔴 CRITICAL
**Impact**: Affects ANY code using `from` as identifier

**Summary**:
- `from` became reserved keyword in v3.62.12+
- Breaks function parameters, variables, struct fields
- Common in graph algorithms, networking code
- No deprecation warning provided

**Reproducible Test**: ✅ Included
**Workaround**: Rename to `from_vertex`, `source`, etc.

---

### Issue #24: Array Reference Parser Bug
**Title**: [v3.62.12] Parser Bug: Array references '&[T; N]' fail with 3+ function parameters
**URL**: https://github.com/paiml/ruchy/issues/24
**Severity**: 🔴 CRITICAL
**Impact**: Blocks idiomatic reference passing in multi-parameter functions

**Summary**:
- `&[T; N]` works with 1-2 parameters, fails with 3+
- Parser appears to misinterpret semicolon in array type
- Forces verbose wrapper struct workaround
- Affects ALL array-heavy code

**Reproducible Test**: ✅ Included (4 test cases)
**Workaround**: Use wrapper structs instead of references

---

### Issue #25: Tuple Destructuring Syntax Change
**Title**: [v3.62.12] Breaking Change: No 'mut' keywords allowed in tuple destructuring patterns
**URL**: https://github.com/paiml/ruchy/issues/25
**Severity**: 🔴 CRITICAL
**Impact**: Breaks functional-style mutable state patterns

**Summary**:
- `let (mut x, mut y) = tuple` worked in v1.89.0, fails in v3.62.12+
- Common in stream processing, state machines
- Parser rejects `mut` in destructuring patterns
- No clear documentation of why

**Reproducible Test**: ✅ Included (3 test cases)
**Workaround**: Separate `let mut` declarations after destructuring

---

## Quality Standards Met

✅ **Reproducible Test Cases**: All 3 issues include complete reproduction steps
✅ **Environment Details**: Exact versions documented (3.62.12, 3.63.0)
✅ **Impact Analysis**: Real-world examples and affected files
✅ **Workarounds**: Validated solutions for each issue
✅ **Root Cause Hypothesis**: Technical analysis where possible
✅ **Suggested Fixes**: Concrete recommendations for maintainers

---

## Documentation Cross-References

All issues reference:
- **Main Documentation**: https://github.com/paiml/rosetta-ruchy/blob/main/docs/PARSER_BUG_V3_62_12.md
- **Sprint Retrospectives**: docs/SPRINT_35_RESULTS.md, docs/SPRINT_36_RESULTS.md
- **Project Context**: Scientific validation of Ruchy performance parity

---

## Next Steps

1. ✅ **Issues Created** - Complete
2. ⏳ **Monitor for Responses** - Watch for maintainer feedback
3. ⏳ **Update Documentation** - Link issues in PARSER_BUG_V3_62_12.md
4. ⏳ **Track Fixes** - Update INTEGRATION.md when issues resolved

---

## Impact Summary

**Rosetta-Ruchy Project**:
- 4/5 target files successfully migrated using documented workarounds
- All migration patterns validated and reusable
- Sprint 36 achieved 80% completion rate

**Community Benefit**:
- Comprehensive issue reports help other Ruchy users
- Workarounds enable continued development
- Reproducible test cases facilitate bug fixes

---

*Created: October 1, 2025*
*Sprint 36 - GitHub Issues Phase Complete*
