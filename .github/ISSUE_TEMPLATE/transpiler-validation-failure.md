---
name: Transpiler Validation Failure
about: Report a transpiler validation failure (usually auto-generated)
title: '[TRANSPILER] Validation Failure: [Algorithm Name]'
labels: transpiler-validation, bug
assignees: ''
---

## Transpiler Validation Failure

**Transpiler**: [decy/bashrs]
**Algorithm**: [e.g., 001-fibonacci]
**Date**: [YYYY-MM-DD]
**Workflow Run**: [Link to GitHub Actions run]

### Failure Description

<!-- Describe what failed during validation -->

**Stage Failed**: [Transpilation / Compilation / Testing / Performance]

**Error Message**:
```
[Paste error message here]
```

### Reproduction Steps

```bash
# Clone rosetta-ruchy
git clone https://github.com/paiml/rosetta-ruchy.git
cd rosetta-ruchy

# Navigate to algorithm
cd examples/algorithms/[algorithm]/implementations/[c|bash]/

# Run transpiler
[decy|bashrs] [source file] -o transpiled.rs

# Compile
rustc -C opt-level=3 transpiled.rs -o transpiled_bin

# Test
./transpiled_bin test
```

### Expected Behavior

<!-- What should happen -->

### Actual Behavior

<!-- What actually happened -->

### Environment

- **OS**: [e.g., Ubuntu 22.04]
- **Transpiler Version**: [e.g., decy 0.1.0, bashrs 1.0.0-rc1]
- **Rust Version**: [e.g., 1.75.0]
- **LLVM Version** (decy only): [e.g., 14.0.0]

### Artifacts

<!-- Attach or link to relevant files -->

- [ ] Transpiled Rust code
- [ ] Compilation errors
- [ ] Test output
- [ ] Validation report

### Additional Context

<!-- Any other information that might be helpful -->

### Checklist

- [ ] Issue reproduced locally
- [ ] Artifacts attached
- [ ] Error message included
- [ ] Environment details provided
