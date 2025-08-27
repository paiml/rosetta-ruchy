# Ruchy Debugging Guide

**Sprint 39**: Advanced Debugging Tools  
**Date**: 2025-08-27  
**Version**: v1.21.0

## Overview

This guide provides comprehensive debugging strategies for Ruchy programs, including error diagnosis, verification workflows, and troubleshooting patterns.

## Available Debugging Tools

### Core Verification Suite
| Tool | Command | Purpose | Status |
|------|---------|---------|--------|
| Syntax Check | `ruchy check` | Validate syntax | ✅ Working |
| Provability | `ruchy provability` | Function purity analysis | ✅ Working |
| Runtime | `ruchy runtime` | Complexity analysis | ✅ Working |
| Quality Score | `ruchy score` | Code quality assessment | ✅ Working |
| Parse | `ruchy parse` | Generate AST | ✅ Working |
| Prove | `ruchy prove` | Interactive theorem proving | ✅ Working |

### Advanced Tools
| Tool | Command | Purpose | Status |
|------|---------|---------|--------|
| AST | `ruchy ast` | Enhanced AST analysis | ⚠️ Limited |
| Optimize | `ruchy optimize` | Hardware optimization | ❌ Not implemented |
| Actor:observe | `ruchy actor:observe` | Actor debugging | ❌ Issues |
| Dataflow:debug | `ruchy dataflow:debug` | DataFrame debugging | ❌ Issues |

## Debugging Workflows

### 1. Basic Verification Pipeline
```bash
# Step 1: Check syntax
ruchy check file.ruchy

# Step 2: Analyze provability
ruchy provability file.ruchy

# Step 3: Check complexity
ruchy runtime file.ruchy

# Step 4: Assess quality
ruchy score file.ruchy

# Step 5: Test compilation
ruchy compile file.ruchy --output test
```

### 2. Error Diagnosis Script
Use the automated diagnosis tool:
```bash
./scripts/diagnose_errors.sh file.ruchy
```

Output includes:
- Syntax validation
- Provability score
- Complexity analysis
- Quality assessment
- Compilation test
- Parse tree analysis

### 3. Interactive Debugging
```bash
# Start interactive prove session
ruchy prove file.ruchy

# Generate counterexamples
ruchy prove file.ruchy --counterexample

# Export proofs
ruchy prove file.ruchy --export proof.txt
```

## Common Error Patterns

### 1. Variable Declaration Issues

#### Error: "var not found in scope"
**Symptom**:
```
error[E0425]: cannot find value `var` in this scope
```

**Cause**: The `var` keyword is not properly transpiled to Rust.

**Solution**:
```ruchy
// ❌ Broken
fun broken() {
    var x = 0;
    while x < 10 {
        x = x + 1;
    }
}

// ✅ Fixed - Use recursion
fun fixed(x: i32) -> i32 {
    if x >= 10 {
        x
    } else {
        fixed(x + 1)
    }
}
```

### 2. Ownership Problems

#### Error: "use of moved value"
**Symptom**:
```
error[E0382]: use of moved value: `numbers`
```

**Cause**: Value ownership transferred and then used again.

**Solution**:
```ruchy
// ❌ Broken
let nums = vec![1, 2, 3];
process(nums);    // moves ownership
use_again(nums);  // ERROR

// ✅ Fixed - Clone
let nums = vec![1, 2, 3];
process(nums.clone());
use_again(nums);
```

### 3. Type Inference Failures

#### Error: "cannot infer type"
**Symptom**:
```
error[E0282]: type annotations needed
```

**Solution**:
```ruchy
// ❌ Broken
let result = Vec::new();

// ✅ Fixed
let result: Vec<i32> = Vec::new();
```

### 4. Syntax Errors

#### Error: "Expected RightParen"
**Symptom**: Mismatched brackets or parentheses.

**Debugging Steps**:
1. Count opening/closing brackets
2. Check function definitions
3. Verify block structures
4. Use `ruchy parse` to see where parsing fails

## Parse Tree Analysis

### Understanding AST Output
```bash
ruchy parse file.ruchy
```

Key elements to examine:
- **Function definitions**: Check parameters and return types
- **Block structures**: Verify proper nesting
- **Expression types**: Ensure type consistency
- **Span information**: Locate exact error positions

### Example Parse Tree
```
Expr {
    kind: Function {
        name: "main",
        params: [],
        return_type: None,
        body: Block([...])
    }
}
```

## Quality Metrics

### Interpreting Scores

| Score | Grade | Meaning |
|-------|-------|---------|
| 0.95+ | A+ | Excellent, production ready |
| 0.90-0.94 | A | Very good, minor improvements |
| 0.85-0.89 | B+ | Good, some refactoring needed |
| 0.80-0.84 | B | Acceptable, improvements recommended |
| <0.80 | C+ | Needs significant work |

### Improving Quality Scores
1. **Increase function purity**: Avoid side effects
2. **Reduce complexity**: Simplify nested structures
3. **Add type annotations**: Help type inference
4. **Use functional patterns**: Prefer immutability

## Verification Strategies

### 1. Incremental Testing
```bash
# Start with minimal code
echo 'fun main() { println("test"); }' > test.ruchy
ruchy check test.ruchy

# Add complexity gradually
# Test after each addition
```

### 2. Binary Search Debugging
When compilation fails:
1. Comment out half the code
2. Test compilation
3. Narrow down to problematic section
4. Repeat until issue isolated

### 3. Transpilation Analysis
```bash
# View generated Rust code
ruchy transpile file.ruchy > output.rs

# Analyze for issues:
# - Extra braces
# - Ownership problems
# - Type mismatches
```

## Error Pattern Library

### Compilation Blockers
1. **var keyword**: Always fails
2. **Complex ownership**: Often fails
3. **Mutable references**: Not supported
4. **Generic constraints**: Limited support

### Runtime Issues
1. **Stack overflow**: Deep recursion
2. **Integer overflow**: No bounds checking
3. **Array access**: Check bounds manually

### Logic Errors
1. **Off-by-one**: Common in loops
2. **Type confusion**: i32 vs Vec<i32>
3. **Pattern matching**: Limited support

## Troubleshooting Checklist

### Before Reporting Issues
- [ ] Syntax validated with `ruchy check`
- [ ] Quality score checked with `ruchy score`
- [ ] Compilation tested with `ruchy compile`
- [ ] Parse tree examined with `ruchy parse`
- [ ] Error diagnosis script run
- [ ] Common patterns checked

### Information to Gather
1. **Error message**: Complete output
2. **Ruchy version**: `ruchy --version`
3. **Minimal example**: Smallest code reproducing issue
4. **Expected behavior**: What should happen
5. **Actual behavior**: What happens instead

## Debugging Tools Integration

### Makefile Integration
```makefile
debug:
    @echo "=== DEBUGGING PIPELINE ==="
    ruchy check $(FILE)
    ruchy provability $(FILE)
    ruchy runtime $(FILE)
    ruchy score $(FILE)
    ruchy parse $(FILE) > parse.txt
    ./scripts/diagnose_errors.sh $(FILE)
```

### CI/CD Pipeline
```yaml
debug:
  script:
    - ruchy check src/*.ruchy
    - ruchy score src/*.ruchy --min 0.85
    - ./scripts/diagnose_errors.sh src/main.ruchy
```

## Advanced Debugging Techniques

### 1. Proof-Based Debugging
```bash
# Generate proofs for correctness
ruchy prove file.ruchy --check

# Export proof for analysis
ruchy prove file.ruchy --export proof.json --format json

# Check counterexamples
ruchy prove file.ruchy --counterexample
```

### 2. Performance Profiling
```bash
# Analyze complexity
ruchy runtime file.ruchy

# Check optimization suggestions
ruchy optimize file.ruchy  # When implemented

# Benchmark execution
time ruchy run file.ruchy
```

### 3. Quality Gate Enforcement
```bash
# Set minimum thresholds
ruchy quality-gate file.ruchy --min-score 0.85

# Use in pre-commit hooks
ruchy score file.ruchy || exit 1
```

## Best Practices

### Development Workflow
1. **Write tests first**: TDD approach
2. **Check early and often**: Run verification frequently
3. **Start simple**: Build complexity gradually
4. **Use functional patterns**: Avoid mutable state
5. **Document issues**: Keep error log

### Error Prevention
1. **Avoid var keyword**: Use recursion
2. **Clone when needed**: Prevent ownership issues
3. **Type explicitly**: Help inference
4. **Test incrementally**: Catch issues early
5. **Review transpilation**: Understand generated code

## Conclusion

Effective debugging in Ruchy requires understanding both the language's verification tools and common error patterns. The combination of automated diagnosis scripts, interactive proving, and systematic workflows enables efficient problem resolution.

Key takeaways:
- Use the diagnosis script for quick assessment
- Understand common error patterns
- Leverage parse tree analysis
- Maintain high quality scores
- Test incrementally

---

**Resources**:
- Error diagnosis script: `scripts/diagnose_errors.sh`
- Debug helper: `scripts/debug_ruchy.ruchy`
- Example workflows: This guide
- Community support: Report issues with diagnosis