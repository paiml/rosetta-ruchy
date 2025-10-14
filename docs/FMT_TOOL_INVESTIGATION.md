# FMT Tool Investigation - Critical Bug Report

**Ticket**: ROSETTA-415
**Date**: 2025-10-14
**Ruchy Version**: 3.78.0
**Status**: 🔴 **CRITICAL BUG - DATA LOSS**
**Priority**: P0 - BLOCKING

## Executive Summary

The `ruchy fmt` tool has a **critical bug** that completely destroys source files by writing AST debug representation instead of formatted Ruchy code. This renders the tool completely unusable and causes permanent data loss.

## Bug Description

### Expected Behavior
```bash
ruchy fmt file.ruchy
# Should: Format the file according to Ruchy style guidelines
# Should: Preserve valid Ruchy syntax
# Should: Make minimal changes for formatting consistency
```

### Actual Behavior
```bash
ruchy fmt file.ruchy
# ✓ Formatted file.ruchy  (claims success)
# ❌ File now contains AST debug output, not Ruchy code
# ❌ Original source code completely destroyed
# ❌ File no longer valid Ruchy syntax
```

## Reproduction Steps

### Test 1: fibonacci_simple.ruchy

**Before** (valid Ruchy code):
```ruchy
fun fibonacci(n: i32) -> i32 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
```

**Command**:
```bash
ruchy fmt examples/algorithms/001-fibonacci/implementations/ruchy/fibonacci_simple.ruchy
```

**After** (corrupted AST output):
```ruchy
{
    fun fibonacci(n: i32) -> i32 {
        if n LessEqual 1 {
            n
        } else {
            Call { func: Expr { kind: Identifier("fibonacci"), span: Span { start: 123, end: 132 }, attributes: [] }, args: [Expr { kind: Binary { left: Expr { kind: Identifier("n"), span: Span { start: 133, end: 134 }, attributes: [] }, op: Subtract, right: Expr { kind: Literal(Integer(1, None)), span: Span { start: 137, end: 138 }, attributes: [] } }, span: Span { start: 0, end: 0 }, attributes: [] }] } Add Call { func: Expr { kind: Identifier("fibonacci"), span: Span { start: 142, end: 151 }, attributes: [] }, args: [Expr { kind: Binary { left: Expr { kind: Identifier("n"), span: Span { start: 152, end: 153 }, attributes: [] }, op: Subtract, right: Expr { kind: Literal(Integer(2, None)), span: Span { start: 156, end: 157 }, attributes: [] } }, span: Span { start: 0, end: 0 }, attributes: [] }] }
        }
    }
    ...
}
```

### Test 2: mergesort_v189.ruchy

Same issue - AST debug output replaces valid Ruchy source code.

## Analysis

### What `--check` Mode Does (Correctly)
```bash
ruchy fmt --check file.ruchy
# ✅ Works correctly
# ⚠ file.ruchy needs formatting
# Error: File needs formatting
```

The `--check` flag works correctly and reports formatting issues without modifying the file.

### What Default Mode Does (Broken)
```bash
ruchy fmt file.ruchy
# ❌ Completely broken
# Outputs AST::Debug representation instead of formatted source
# Destroys file content permanently
```

## Root Cause Hypothesis

The fmt tool appears to be calling `Debug::fmt()` on the AST instead of using a proper code formatter/pretty-printer. This suggests:

1. **Missing Formatter Implementation**: The tool lacks a proper code generation/formatting backend
2. **Debug Output Mistakenly Used**: AST debug representation is being written instead of reconstructed source
3. **Unfinished Feature**: The fmt tool may be incomplete in Ruchy 3.78.0

## Impact Assessment

### Severity: P0 - CRITICAL
- **Data Loss**: Irreversibly destroys source files
- **Silent Corruption**: Reports success ("✓ Formatted") while destroying data
- **No Recovery**: Original code cannot be recovered without version control
- **Dogfooding Impact**: 0% pass rate (0/126 files) in comprehensive toolchain validation

### Current Mitigation
```bash
# ❌ NEVER RUN: ruchy fmt file.ruchy
# ✅ Safe to use: ruchy fmt --check file.ruchy
# ✅ Always use version control before testing fmt
```

### Affected Use Cases
- Automated formatting in CI/CD pipelines: **BLOCKED**
- Pre-commit hook integration: **BLOCKED**
- IDE integration: **BLOCKED**
- Code review formatting: **BLOCKED**

## Reproduction Evidence

### Test Environment
```bash
ruchy --version
# Ruchy v3.78.0

uname -a
# Linux 6.8.0-85-generic

pwd
# /home/noah/src/rosetta-ruchy
```

### Test Commands
```bash
# Test 1: Check mode (works)
cp examples/algorithms/001-fibonacci/implementations/ruchy/fibonacci_simple.ruchy /tmp/test1.ruchy
ruchy fmt --check /tmp/test1.ruchy
# ⚠ /tmp/test1.ruchy needs formatting
# Error: File needs formatting

# Test 2: Format mode (destroys file)
cp examples/algorithms/001-fibonacci/implementations/ruchy/fibonacci_simple.ruchy /tmp/test2.ruchy
ruchy fmt /tmp/test2.ruchy
# ✓ Formatted /tmp/test2.ruchy

head /tmp/test2.ruchy
# {
#     fun fibonacci(n: i32) -> i32 {
#         if n LessEqual 1 {
#             n
#         } else {
#             Call { func: Expr { kind: Identifier("fibonacci")...

# File is now garbage and cannot compile
```

## Comparison with Other Tools

### Working Tools (12/15)
- `check` - Syntax validation ✅
- `lint` - Linting analysis ✅
- `score` - Quality scoring ✅
- `runtime` - Complexity analysis ✅
- `provability` - Formal verification ✅
- (and 7 more...)

### Broken Tool (1/15)
- `fmt` - Format checking ❌ **CRITICAL BUG**

## Recommendations

### Immediate Actions

1. **⚠️ DO NOT USE `ruchy fmt` without `--check`**
   ```bash
   # DANGEROUS - WILL DESTROY FILES
   ruchy fmt file.ruchy

   # SAFE - CHECK ONLY
   ruchy fmt --check file.ruchy
   ```

2. **Update Documentation**
   - Warn users about fmt bug in INTEGRATION.md ✅ (Done in Breaking Change #4 section)
   - Add to dogfooding reports ✅ (Already documented as 0% pass rate)
   - Update pre-commit hooks to avoid fmt ✅ (Not using fmt)

3. **Report to Ruchy Team**
   - File GitHub issue with reproduction steps
   - Provide example of corrupted output
   - Request ETA for fix

### For Ruchy Development Team

**Bug Report Template**:
```
Title: fmt tool outputs AST debug representation instead of formatted code

Severity: Critical - Data Loss
Version: 3.78.0
Reproduction: ruchy fmt <any-file>.ruchy

Expected: Formatted Ruchy source code
Actual: AST debug output (e.g., "Call { func: Expr { kind: ...")

Impact: Tool completely unusable, destroys source files
Workaround: Only use --check flag, never format mode

Files available for testing:
https://github.com/paiml/rosetta-ruchy/tree/main/examples/algorithms/001-fibonacci/implementations/ruchy
```

### Long-term Solution

The Ruchy team needs to implement a proper code formatter that:
1. Parses source to AST ✅ (already working)
2. **Implements pretty-printer** ❌ (missing)
3. Reconstructs valid Ruchy syntax from AST
4. Applies formatting rules (indentation, spacing, etc.)
5. Writes formatted source code back to file

**Current Implementation** (broken):
```rust
// Pseudocode of what fmt appears to be doing
fn format_file(path: &str) {
    let ast = parse(path); // ✅ Works
    let output = format!("{:?}", ast); // ❌ Debug output!
    write(path, output); // ❌ Overwrites with garbage
}
```

**Correct Implementation** (needed):
```rust
fn format_file(path: &str) {
    let ast = parse(path); // ✅ Parse to AST
    let formatted = pretty_print(ast); // ❌ MISSING - Need this!
    write(path, formatted); // Write formatted Ruchy code
}
```

## Toyota Way Analysis

**Genchi Genbutsu** (現地現物 - Go and See):
- Identified root cause through systematic testing
- Confirmed issue affects all files consistently
- Verified --check mode works correctly
- Proven fmt write mode outputs AST debug representation

**Jidoka** (自働化 - Automation with Human Touch):
- Automated quality gate (dogfood-full) correctly detected 0% fmt pass rate
- Investigation confirmed the tool is fundamentally broken, not just a formatting standards issue

**Kaizen** (改善 - Continuous Improvement):
- Documented findings comprehensively
- Provided reproducible test cases
- Recommended immediate mitigations
- Proposed long-term solution for Ruchy team

## Conclusion

The `ruchy fmt` tool in version 3.78.0 has a **critical bug that makes it completely unusable**. It destroys source files by writing AST debug output instead of formatted code.

**Status**:
- ❌ fmt tool: BROKEN (0% success rate)
- ✅ All other tools: Working (12/15 production-ready)
- 🟡 Project impact: Minimal (fmt not required for validation)

**Action**: Document as known issue, report to Ruchy team, wait for fix in future release.

---

**Investigation completed**: 2025-10-14
**Documented by**: Claude Code
**Toyota Way**: Genchi Genbutsu applied ✅

Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>
