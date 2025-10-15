# Transpiler CI/CD Integration Guide

**Version**: 1.0.0
**Date**: 2025-10-15
**Status**: Production Ready

## Overview

This guide explains how to use and maintain the automated transpiler validation workflows for **decy** (C→Rust) and **bashrs** (Bash↔Rust) transpilers.

rosetta-ruchy provides comprehensive CI/CD infrastructure that continuously validates transpiler correctness, performance, and safety using the Tier 0 algorithm implementations (8 algorithms in C and Bash).

---

## Table of Contents

1. [Workflows Overview](#workflows-overview)
2. [Quick Start](#quick-start)
3. [Workflow Details](#workflow-details)
4. [Monitoring and Debugging](#monitoring-and-debugging)
5. [Maintenance](#maintenance)
6. [Troubleshooting](#troubleshooting)
7. [Integration with Sister Projects](#integration-with-sister-projects)

---

## Workflows Overview

### 1. decy Transpiler Validation

**File**: `.github/workflows/decy-validation.yml`

**Purpose**: Validate C→Rust transpilation correctness, performance, and safety

**Triggers**:
- 📅 Daily at 6am UTC (scheduled)
- 🔄 On push to `examples/algorithms/*/implementations/c/**`
- 🔀 On pull requests affecting C implementations
- 🎯 Manual via `workflow_dispatch`

**What It Does**:
1. Installs decy transpiler from source
2. Transpiles all 8 C algorithms to Rust
3. Compiles transpiled Rust code
4. Runs comprehensive test suites
5. Compares output with original C
6. Benchmarks performance
7. Analyzes safety (unsafe block count)
8. Uploads artifacts for debugging

**Matrix Strategy**: Runs 8 algorithms in parallel for faster feedback

---

### 2. bashrs Transpiler Validation

**File**: `.github/workflows/bashrs-validation.yml`

**Purpose**: Validate Bash↔Rust bidirectional transpilation

**Triggers**:
- 📅 Daily at 6am UTC (scheduled)
- 🔄 On push to `examples/algorithms/*/implementations/bash/**`
- 🔀 On pull requests affecting Bash implementations
- 🎯 Manual via `workflow_dispatch`

**What It Does**:
1. Installs bashrs v1.0.0-rc1 from crates.io
2. Transpiles all 8 Bash algorithms to Rust
3. Compiles transpiled Rust code
4. Generates purified Bash (Rust→Bash)
5. Validates POSIX compliance (shellcheck)
6. Tests across shells (bash, dash, zsh)
7. Benchmarks performance
8. Uploads artifacts for debugging

**Unique Features**:
- Bidirectional transpilation testing
- Cross-shell compatibility verification
- POSIX compliance analysis

---

### 3. Automated Issue Creation

**File**: `.github/workflows/transpiler-issue-report.yml`

**Purpose**: Automatically create GitHub issues for validation failures

**Triggers**:
- 🔔 On completion of decy/bashrs validation workflows

**What It Does**:
1. **On Failure**: Creates or updates GitHub issue with:
   - List of failed algorithms
   - Failure details and reproduction steps
   - Links to workflow runs and artifacts
   - Automated labeling for triage
2. **On Success**: Auto-closes any open validation issues
3. **Duplicate Prevention**: Updates existing issues instead of creating new ones

**Issue Labels**:
- `transpiler-validation` - Category
- `decy` or `bashrs` - Specific transpiler
- `automated` - Auto-generated
- `bug` - Failure indication

---

### 4. Performance Regression Detection

**File**: `.github/workflows/transpiler-performance.yml`

**Purpose**: Track transpiler performance and detect regressions

**Triggers**:
- 📅 Weekly on Sunday at 6am UTC (scheduled)
- 🎯 Manual via `workflow_dispatch`

**What It Does**:
1. **Establish Baseline**:
   - Benchmarks all 8 algorithms for both transpilers
   - Measures transpilation time and execution time
   - Uses hyperfine for statistical accuracy
   - Stores baseline artifacts (90-day retention)

2. **Detect Regressions**:
   - Compares current vs previous baseline
   - Detects performance regressions (>10% slower)
   - Detects improvements (>10% faster)
   - Creates GitHub issues for regressions
   - Generates performance reports

**Metrics Tracked**:
- Original execution time (C/Bash)
- Transpilation time
- Transpiled execution time (Rust)
- Purification time (bashrs only)
- Purified execution time (bashrs only)

---

## Quick Start

### Running Workflows Manually

```bash
# Trigger decy validation
gh workflow run decy-validation.yml

# Trigger bashrs validation
gh workflow run bashrs-validation.yml

# Trigger performance benchmarking
gh workflow run transpiler-performance.yml

# Establish baseline only (no regression detection)
gh workflow run transpiler-performance.yml -f baseline_only=true
```

### Viewing Workflow Status

```bash
# List recent runs
gh run list --workflow=decy-validation.yml
gh run list --workflow=bashrs-validation.yml
gh run list --workflow=transpiler-performance.yml

# View specific run details
gh run view <run-id>

# Watch a running workflow
gh run watch <run-id>
```

### Downloading Artifacts

```bash
# Download artifacts from specific run
gh run download <run-id>

# Download latest artifacts
gh run download --repo paiml/rosetta-ruchy

# List available artifacts
gh run view <run-id> --log
```

---

## Workflow Details

### decy Validation Workflow

#### Installation Steps

1. **System Dependencies**:
   ```bash
   apt-get install llvm-14-dev libclang-14-dev clang-14 valgrind gcc make
   ```

2. **decy Transpiler**:
   ```bash
   git clone https://github.com/paiml/decy.git
   cd decy
   cargo install --path .
   ```

3. **Validation Tools**:
   ```bash
   cargo install hyperfine
   ```

#### Validation Stages

For each algorithm:

1. **Build Original C**:
   ```bash
   cd examples/algorithms/{algorithm}/implementations/c/
   make clean && make all
   ```

2. **Transpile**:
   ```bash
   decy *.c -o transpiled.rs
   ```

3. **Compile Transpiled Rust**:
   ```bash
   rustc -C opt-level=3 transpiled.rs -o transpiled_bin
   ```

4. **Test**:
   ```bash
   ./transpiled_bin test
   ```

5. **Compare Correctness**:
   ```bash
   diff c-test-output.txt rust-test-output.txt
   ```

6. **Benchmark**:
   ```bash
   hyperfine --warmup 3 --runs 10 \
     './c_binary' \
     './transpiled_bin'
   ```

7. **Safety Analysis**:
   ```bash
   grep -c "unsafe" transpiled.rs
   ```

#### Artifacts Generated

- `transpiled.rs` - Generated Rust code
- `transpile-output.txt` - Transpilation logs
- `compile-output.txt` - Compilation logs
- `c-test-output.txt` - Original C test results
- `rust-test-output.txt` - Transpiled Rust test results
- `benchmark-results.json` - Performance comparison
- `correctness-diff.txt` - Output differences (if any)
- `validation-report.md` - Summary report

---

### bashrs Validation Workflow

#### Installation Steps

1. **bashrs Transpiler**:
   ```bash
   cargo install bashrs --version 1.0.0-rc1
   ```

2. **Shells and Tools**:
   ```bash
   apt-get install bash dash zsh shellcheck
   cargo install hyperfine
   ```

#### Validation Stages

For each algorithm:

1. **Test Original Bash**:
   ```bash
   cd examples/algorithms/{algorithm}/implementations/bash/
   bash *.sh test
   ```

2. **Shellcheck Original**:
   ```bash
   shellcheck *.sh
   ```

3. **Transpile Bash→Rust**:
   ```bash
   bashrs *.sh -o transpiled.rs
   ```

4. **Compile Transpiled Rust**:
   ```bash
   rustc -C opt-level=3 transpiled.rs -o transpiled_bin
   ```

5. **Test Transpiled Rust**:
   ```bash
   ./transpiled_bin test
   ```

6. **Generate Purified Bash**:
   ```bash
   bashrs --to-bash transpiled.rs -o purified.sh
   ```

7. **Validate POSIX Compliance**:
   ```bash
   shellcheck purified.sh
   ```

8. **Cross-Shell Testing**:
   ```bash
   bash purified.sh test
   dash purified.sh test
   zsh purified.sh test
   ```

9. **Benchmark**:
   ```bash
   hyperfine --warmup 3 --runs 10 \
     'bash original.sh' \
     './transpiled_bin' \
     'bash purified.sh'
   ```

#### Artifacts Generated

- `transpiled.rs` - Bash→Rust generated code
- `purified.sh` - Rust→Bash generated code
- `transpile-output.txt` - Bash→Rust transpilation logs
- `purify-output.txt` - Rust→Bash purification logs
- `compile-output.txt` - Compilation logs
- `bash-test-output.txt` - Original Bash test results
- `rust-test-output.txt` - Transpiled Rust test results
- `purified-test-output.txt` - Purified Bash test results
- `bash-result.txt`, `dash-result.txt`, `zsh-result.txt` - Cross-shell results
- `shellcheck-original.txt` - Original Bash analysis
- `shellcheck-purified.txt` - Purified Bash analysis
- `benchmark-results.json` - Performance comparison
- `validation-report.md` - Summary report

---

## Monitoring and Debugging

### GitHub Actions UI

1. **Navigate to Actions Tab**:
   ```
   https://github.com/paiml/rosetta-ruchy/actions
   ```

2. **Select Workflow**:
   - decy Transpiler Validation
   - bashrs Transpiler Validation
   - Transpiler Issue Reporter
   - Transpiler Performance Tracking

3. **View Run Details**:
   - Click on specific run
   - Review job logs
   - Download artifacts
   - Re-run failed jobs

### Command Line Monitoring

```bash
# Watch workflow execution in real-time
gh run watch

# View logs for specific job
gh run view <run-id> --log

# List all workflow runs with status
gh run list --workflow=decy-validation.yml --limit 20

# Filter by status
gh run list --workflow=decy-validation.yml --status=failure
```

### Artifact Analysis

```bash
# Download and analyze artifacts
gh run download <run-id>

# Navigate to algorithm artifact
cd 001-fibonacci-decy-validation/

# Review validation report
cat validation-report.md

# Check transpiled code
cat transpiled.rs

# Review test outputs
diff c-test-output.txt rust-test-output.txt

# Analyze benchmark results
jq '.' benchmark-results.json
```

### Common Failure Patterns

#### Transpilation Failures

**Symptoms**: `transpile-output.txt` contains errors

**Debug Steps**:
1. Review error message in `transpile-output.txt`
2. Check if issue is in C source or transpiler
3. Reproduce locally:
   ```bash
   cd examples/algorithms/{algorithm}/implementations/c/
   decy *.c -o test.rs
   ```
4. Report to decy team if confirmed transpiler bug

#### Compilation Failures

**Symptoms**: `compile-output.txt` contains errors

**Debug Steps**:
1. Review generated Rust code (`transpiled.rs`)
2. Check for invalid Rust syntax
3. Verify Rust version compatibility
4. Reproduce locally:
   ```bash
   rustc transpiled.rs
   ```

#### Test Failures

**Symptoms**: Test exit code non-zero or output mismatch

**Debug Steps**:
1. Compare `c-test-output.txt` vs `rust-test-output.txt`
2. Check `correctness-diff.txt` for differences
3. Run tests locally for detailed output
4. Verify test inputs are identical

#### Performance Regressions

**Symptoms**: Automated issue created with "Performance Regression"

**Debug Steps**:
1. Download baseline artifacts
2. Compare `benchmark-results.json` files
3. Identify which stage regressed (transpilation vs execution)
4. Profile with additional tools:
   ```bash
   cargo install cargo-flamegraph
   cargo flamegraph --bin transpiled_bin
   ```

---

## Maintenance

### Updating Transpiler Versions

#### decy Updates

decy is installed from source, so updates require modifying the workflow:

```yaml
# .github/workflows/decy-validation.yml

- name: Install decy transpiler
  run: |
    cd /tmp
    git clone https://github.com/paiml/decy.git
    cd decy
    git checkout v0.2.0  # Pin to specific version
    cargo install --path .
```

#### bashrs Updates

bashrs is installed from crates.io, update the version in workflow:

```yaml
# .github/workflows/bashrs-validation.yml

env:
  BASHRS_VERSION: '1.0.0'  # Update this
```

### Adding New Algorithms

When new Tier 0 algorithms are added:

1. **Update Matrix Strategy**:

```yaml
# Both decy-validation.yml and bashrs-validation.yml

strategy:
  matrix:
    algorithm:
      - 001-fibonacci
      - 004-binary-search
      # ... existing algorithms ...
      - 023-new-algorithm  # Add here
```

2. **Verify Workflow Runs**:
```bash
# Trigger validation for new algorithm
gh workflow run decy-validation.yml
gh workflow run bashrs-validation.yml
```

3. **Update Documentation**:
- Update this guide with new algorithm count
- Update integration guides if needed

### Performance Baseline Management

#### Manual Baseline Establishment

```bash
# Establish new baseline without regression detection
gh workflow run transpiler-performance.yml -f baseline_only=true
```

#### Adjusting Regression Threshold

Default threshold is 10%. To adjust:

```yaml
# .github/workflows/transpiler-performance.yml

# In compare-performance step:
if (( $(echo "$change > 15" | bc -l) )); then  # Changed from 10 to 15
  REGRESSIONS="$REGRESSIONS\n- $algo: ${change}% slower"
fi
```

#### Purging Old Baselines

Artifacts are retained for 90 days. To change:

```yaml
# .github/workflows/transpiler-performance.yml

- name: Upload baseline artifacts
  uses: actions/upload-artifact@v3
  with:
    name: performance-baseline-${{ github.sha }}
    path: performance-results/
    retention-days: 180  # Changed from 90
```

---

## Troubleshooting

### Workflow Not Running

**Problem**: Workflow doesn't trigger on schedule or push

**Solutions**:
1. **Check workflow syntax**:
   ```bash
   # Install yamllint
   pip install yamllint

   # Validate workflow files
   yamllint .github/workflows/decy-validation.yml
   yamllint .github/workflows/bashrs-validation.yml
   ```

2. **Verify GitHub Actions enabled**:
   - Go to repository Settings → Actions
   - Ensure "Allow all actions" is selected

3. **Check workflow permissions**:
   ```yaml
   # Add to workflow file if needed
   permissions:
     contents: read
     issues: write
   ```

4. **Manual trigger test**:
   ```bash
   gh workflow run decy-validation.yml
   ```

### Transpiler Installation Failing

**Problem**: decy or bashrs installation fails

**decy Solutions**:
1. **Check LLVM dependencies**:
   ```bash
   apt-get install llvm-14-dev libclang-14-dev clang-14
   ```

2. **Verify Rust version**:
   ```bash
   rustc --version  # Should be 1.70.0+
   ```

3. **Clone depth**:
   ```bash
   git clone --depth 1 https://github.com/paiml/decy.git
   ```

**bashrs Solutions**:
1. **Check crates.io availability**:
   ```bash
   cargo search bashrs
   ```

2. **Try specific version**:
   ```bash
   cargo install bashrs --version 1.0.0-rc1
   ```

3. **Build from source**:
   ```bash
   cargo install --git https://github.com/paiml/bashrs
   ```

### Artifacts Not Uploading

**Problem**: Artifacts missing from workflow runs

**Solutions**:
1. **Check artifact paths exist**:
   ```yaml
   - name: Upload artifacts
     if: always()  # Ensure runs even on failure
     uses: actions/upload-artifact@v3
     with:
       path: |
         path/to/files/**
       if-no-files-found: warn  # Change to 'ignore' or 'error'
   ```

2. **Verify artifact size limits**:
   - GitHub has 10GB limit per artifact
   - Compress large files if needed

3. **Check retention settings**:
   - Free tier: 90 days default
   - Paid tier: customizable

### Performance Benchmark Failures

**Problem**: hyperfine benchmarks fail or timeout

**Solutions**:
1. **Increase timeout**:
   ```yaml
   - name: Performance benchmark
     timeout-minutes: 30  # Increase if needed
   ```

2. **Reduce benchmark runs**:
   ```bash
   hyperfine --runs 5  # Instead of 20
   ```

3. **Skip slow algorithms**:
   ```bash
   if [[ "$algo" != "002-quicksort" ]]; then
     hyperfine ...
   fi
   ```

---

## Integration with Sister Projects

### Reporting Issues to decy

When validation failures are confirmed as decy bugs:

1. **Review Automated Issue**:
   - Check automatically created issue in rosetta-ruchy
   - Download artifacts
   - Reproduce locally

2. **Create decy Issue**:
   ```markdown
   Title: [Bug] Transpilation failure for [algorithm]

   **Source**: rosetta-ruchy validation corpus
   **Algorithm**: [e.g., 001-fibonacci]
   **rosetta-ruchy Issue**: #[issue-number]

   ## Reproduction

   ```bash
   git clone https://github.com/paiml/rosetta-ruchy.git
   cd rosetta-ruchy/examples/algorithms/[algorithm]/implementations/c/
   decy *.c -o transpiled.rs
   ```

   ## Expected
   [What should happen]

   ## Actual
   [What actually happened]

   ## Environment
   - decy version: [version]
   - LLVM version: [version]
   - OS: [OS]

   ## Artifacts
   [Link to rosetta-ruchy workflow artifacts]
   ```

3. **Link Issues**:
   - Add comment to rosetta-ruchy issue with decy issue link
   - Track resolution in both repositories

### Reporting Issues to bashrs

Similar process as decy, adapted for bashrs:

1. Review automated issue
2. Create bashrs issue with:
   - Link to rosetta-ruchy validation
   - Reproduction steps
   - Expected vs actual behavior
   - Environment details
3. Link issues bidirectionally

### Performance Feedback Loop

Weekly performance reports can be sent to transpiler teams:

1. **Download Performance Report**:
   ```bash
   gh run download --name performance-report
   ```

2. **Review Trends**:
   - Identify regressions or improvements
   - Analyze per-algorithm performance
   - Compare transpilation vs execution time

3. **Share with Teams**:
   - Post summary in decy/bashrs discussions
   - Reference specific workflow runs
   - Provide benchmark artifacts

---

## Best Practices

### For rosetta-ruchy Maintainers

1. **Monitor Daily**:
   - Check Actions tab daily
   - Review automated issues
   - Investigate persistent failures

2. **Update Promptly**:
   - Update transpiler versions when released
   - Add new algorithms to matrix
   - Adjust thresholds as needed

3. **Document Changes**:
   - Update this guide when workflows change
   - Document workarounds for known issues
   - Maintain changelog of workflow versions

### For Transpiler Developers

1. **Use Validation Corpus**:
   - Run local validation before releases
   - Reference rosetta-ruchy in release notes
   - Track success rate over versions

2. **Respond to Issues**:
   - Triage automated issues promptly
   - Provide feedback on false positives
   - Suggest workflow improvements

3. **Performance Awareness**:
   - Review weekly performance reports
   - Investigate regressions
   - Celebrate improvements

---

## Workflow Versions and Changelog

### v1.0.0 (2025-10-15) - Initial Release

**Workflows Added**:
- decy-validation.yml (C→Rust validation)
- bashrs-validation.yml (Bash↔Rust validation)
- transpiler-issue-report.yml (Automated issue creation)
- transpiler-performance.yml (Performance regression detection)

**Features**:
- Daily validation of 8 algorithms
- Matrix strategy for parallel execution
- Automated issue creation and closing
- Performance regression detection (>10% threshold)
- Comprehensive artifact uploads
- Cross-shell testing for bashrs
- Safety analysis for decy

**Validation Corpus**:
- 8 C algorithms (001-fibonacci through 019-radix-sort)
- 8 Bash algorithms (same algorithms)
- Complete test suites
- Reference implementations

---

## Support and Resources

### Documentation

- **Integration Guides**:
  - [decy Integration Guide](guides/decy-integration.md)
  - [bashrs Integration Guide](guides/bashrs-integration.md)
  - [Transpilation Validation Workflow](TRANSPILATION_VALIDATION.md)

- **Specifications**:
  - [Sprint 46 Specification](specifications/sprint-46-transpiler-ci.md)
  - [C and Bash Examples Spec](specifications/c-bash-examples.md)

### Sister Projects

- **decy**: https://github.com/paiml/decy
- **bashrs**: https://github.com/paiml/bashrs
- **rosetta-ruchy**: https://github.com/paiml/rosetta-ruchy

### Getting Help

1. **GitHub Issues**: Report problems or suggest improvements
2. **GitHub Discussions**: Ask questions about workflows
3. **Workflow Logs**: Check detailed logs in Actions tab
4. **Artifacts**: Download and analyze validation artifacts

---

**Last Updated**: 2025-10-15
**Version**: 1.0.0
**Status**: Production Ready
**Workflows**: 4 (decy, bashrs, issues, performance)
**Algorithms Validated**: 8 per transpiler
**Validation Frequency**: Daily + Weekly performance
