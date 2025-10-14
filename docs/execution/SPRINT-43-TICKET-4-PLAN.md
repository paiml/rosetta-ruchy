# Sprint 43 Ticket 4: Complexity Refactoring - Detailed Plan

**Date**: 2025-10-14
**Sprint**: 43 (Week 1)
**Ticket**: Ticket 4 - Complexity Refactoring
**Status**: 🔄 IN PROGRESS

---

## Executive Summary

Sprint 43 Ticket 4 aims to reduce all functions to ≤20 complexity using Kaizen refactoring patterns.

**Current State**: 6 functions exceed complexity threshold (25)
**Target**: All functions ≤20 complexity
**Approach**: Extract Method pattern to break large functions into smaller, focused functions

---

## High-Complexity Functions Identified

Using `cargo clippy --all-targets --all-features -- -W clippy::cognitive_complexity`:

| File | Line | Function | Complexity | Threshold | Ratio |
|------|------|----------|------------|-----------|-------|
| **main.rs** | **262** | **`run_benchmark`** | **284** | **25** | **11.4x** 🔴 |
| **main.rs** | **888** | **`run_app`** | **78** | **25** | **3.1x** 🔴 |
| memory_profiler.rs | 531 | (unnamed) | 66 | 25 | 2.6x 🔴 |
| binary_analyzer.rs | 621 | (unnamed) | 60 | 25 | 2.4x 🔴 |
| isolation.rs | 130 | (unnamed) | 31 | 25 | 1.2x 🟡 |
| mcp_server.rs | 218 | (unnamed) | 30 | 25 | 1.2x 🟡 |

**Priority Order**: Refactor by ratio (highest first)

---

## Refactoring Plan

### Priority 1: `run_benchmark()` (Complexity: 284 → <20)

**File**: `harness/runner/src/main.rs:262`
**Current**: 311 lines, 284 complexity
**Target**: <20 complexity

#### Current Structure Analysis

The function has 5 major sections:

```rust
async fn run_benchmark(&self, example_path: &Path, languages: &[String]) -> Result<Vec<BenchmarkResult>> {
    // Step 1: Environment isolation setup (lines 267-298, ~30 lines)
    // - Log startup
    // - Create EnvironmentController
    // - Detect and apply isolation
    // - Log isolation status

    // Step 2: Benchmark each language (lines 299-518, ~220 lines!)
    // - Loop over languages
    // - For each language:
    //   - Start memory profiling (optional)
    //   - Run benchmark measurements
    //   - Perform statistical analysis
    //   - Stop memory profiling and generate report
    //   - Perform binary analysis and generate report
    //   - Perform Ruchy analysis (if applicable)
    //   - Create BenchmarkResult

    // Step 3: Cleanup environment (lines 519-524, ~5 lines)
    // - Restore environment settings

    // Step 4: Generate reports (lines 526-540, ~15 lines)
    // - Create report generator
    // - Convert results to report format
    // - Generate comprehensive reports

    // Step 5: Regression detection (lines 542-597, ~55 lines)
    // - Create regression detector
    // - Detect regressions
    // - Log status (Critical/Warning/Healthy/Inconclusive)
    // - Generate regression report
    // - Establish baselines if needed

    // Return
    info!("✅ Benchmark run completed");
    Ok(results)
}
```

####Extract Method Refactoring

**New Structure** (Target complexity: ~8):
```rust
async fn run_benchmark(&self, example_path: &Path, languages: &[String]) -> Result<Vec<BenchmarkResult>> {
    self.log_benchmark_start(example_path, languages); // complexity: 1

    let (mut env_controller, isolation_result) = self.setup_environment_isolation().await?; // complexity: 2

    let results = self.benchmark_all_languages(example_path, languages, &isolation_result).await?; // complexity: 2

    self.cleanup_environment(&mut env_controller).await; // complexity: 1

    self.generate_benchmark_reports(&results, &env_controller, &isolation_result).await; // complexity: 1

    self.perform_regression_analysis(&results, example_path).await; // complexity: 1

    info!("✅ Benchmark run completed for {} languages", results.len());
    Ok(results) // complexity: 1
}
// Total estimated complexity: 8 ✅
```

**Extracted Methods**:

1. **`log_benchmark_start()`** - Complexity: ~1
   ```rust
   fn log_benchmark_start(&self, example_path: &Path, languages: &[String]) {
       info!("🚀 Starting benchmark...");
       info!("Example: {}", example_path.display());
       info!("Languages: {:?}", languages);
       info!("Iterations: {}", self.config.iterations);
   }
   ```

2. **`setup_environment_isolation()`** - Complexity: ~5
   ```rust
   async fn setup_environment_isolation(&self) -> Result<(EnvironmentController, IsolationResult)> {
       let mut env_controller = EnvironmentController::new()
           .with_isolated_cores(self.config.cpu_affinity.clone())
           .with_governor("performance")
           .with_freq_scaling_control(true);

       env_controller.detect_environment().await?;
       let isolation_result = env_controller.apply_isolation().await?;

       self.log_isolation_status(&isolation_result);
       Ok((env_controller, isolation_result))
   }
   ```

3. **`log_isolation_status()`** - Complexity: ~3
   ```rust
   fn log_isolation_status(&self, isolation_result: &IsolationResult) {
       if !isolation_result.success {
           warn!("⚠️ Environment isolation partially failed");
           for error in &isolation_result.errors {
               warn!("  Error: {}", error);
           }
       }
       for warning in &isolation_result.warnings {
           warn!("  Warning: {}", warning);
       }
   }
   ```

4. **`benchmark_all_languages()`** - Complexity: ~8 (loop + error handling)
   ```rust
   async fn benchmark_all_languages(
       &self,
       example_path: &Path,
       languages: &[String],
       isolation_result: &IsolationResult,
   ) -> Result<Vec<BenchmarkResult>> {
       let mut results = Vec::new();
       let analyzer = self.create_statistical_analyzer();

       for language in languages {
           info!("📊 Benchmarking {} implementation", language);
           let result = self.benchmark_single_language(
               example_path,
               language,
               &analyzer,
               isolation_result
           ).await?;
           results.push(result);
       }

       Ok(results)
   }
   ```

5. **`create_statistical_analyzer()`** - Complexity: ~2
   ```rust
   fn create_statistical_analyzer(&self) -> StatisticalAnalyzer {
       StatisticalAnalyzer::new()
           .with_min_sample_size(if self.config.iterations >= 1000 { 1000 } else { 30 })
           .with_confidence_level(0.95)
   }
   ```

6. **`benchmark_single_language()`** - Complexity: ~15 (needs further extraction)
   ```rust
   async fn benchmark_single_language(
       &self,
       example_path: &Path,
       language: &str,
       analyzer: &StatisticalAnalyzer,
       isolation_result: &IsolationResult,
   ) -> Result<BenchmarkResult> {
       let memory_profile = self.profile_memory_if_enabled(language).await;
       let raw_measurements = self.simulate_benchmark_measurements(language)?;
       let statistical_analysis = analyzer.analyze(&raw_measurements)?;

       self.log_statistics(language, &statistical_analysis);

       let time_stats = self.convert_to_time_statistics(&statistical_analysis);
       let memory_profile = self.finalize_memory_profile(memory_profile, language).await;
       let binary_analysis = self.analyze_binary_if_available(language).await;
       let ruchy_analysis = if language == "ruchy" {
           Some(self.perform_ruchy_analysis().await?)
       } else {
           None
       };

       Ok(BenchmarkResult {
           language: language.to_string(),
           example: example_path.to_string_lossy().to_string(),
           metrics: self.create_performance_metrics(language, &time_stats),
           statistics: statistical_analysis,
           isolation: isolation_result.clone(),
           system_info: self.get_system_info()?,
           config: self.config.clone(),
           memory_profile,
           binary_analysis,
           ruchy_analysis,
       })
   }
   ```

7. **`profile_memory_if_enabled()`** - Complexity: ~6
8. **`finalize_memory_profile()`** - Complexity: ~8
9. **`analyze_binary_if_available()`** - Complexity: ~8
10. **`log_statistics()`** - Complexity: ~1
11. **`convert_to_time_statistics()`** - Complexity: ~2
12. **`create_performance_metrics()`** - Complexity: ~2

13. **`cleanup_environment()`** - Complexity: ~2
    ```rust
    async fn cleanup_environment(&self, env_controller: &mut EnvironmentController) {
        if let Err(e) = env_controller.restore_environment().await {
            warn!("Failed to restore environment: {}", e);
        }
    }
    ```

14. **`generate_benchmark_reports()`** - Complexity: ~5
    ```rust
    async fn generate_benchmark_reports(
        &self,
        results: &[BenchmarkResult],
        env_controller: &EnvironmentController,
        isolation_result: &IsolationResult,
    ) {
        if results.is_empty() {
            return;
        }

        info!("📊 Generating comprehensive benchmark reports");
        let report_generator = self.create_report_generator();
        let report_results = self.convert_to_report_format(results).ok();
        let environment_report = self.create_environment_report(env_controller, isolation_result).ok();
        let config = self.create_benchmark_config();

        if let (Some(report_results), Some(environment_report)) = (report_results, environment_report) {
            if let Err(e) = report_generator
                .generate_report(report_results, environment_report, config)
                .await
            {
                warn!("Failed to generate reports: {}", e);
            }
        }
    }
    ```

15. **`perform_regression_analysis()`** - Complexity: ~12
    ```rust
    async fn perform_regression_analysis(&self, results: &[BenchmarkResult], example_path: &Path) {
        if results.is_empty() {
            return;
        }

        info!("🔍 Performing regression analysis with 5% threshold");
        let regression_detector = self.create_regression_detector();
        let current_stats = self.extract_statistical_analysis(results);

        let analysis = match regression_detector
            .detect_regressions(&current_stats, example_path.to_str().unwrap_or("unknown"))
            .await
        {
            Ok(analysis) => analysis,
            Err(e) => {
                warn!("Failed to perform regression analysis: {}", e);
                return;
            }
        };

        self.handle_regression_status(&analysis, results, example_path, &regression_detector).await;
        self.generate_regression_report(&analysis, &regression_detector).await;
    }
    ```

16. **`handle_regression_status()`** - Complexity: ~8
17. **`generate_regression_report()`** - Complexity: ~4

**Total Extracted Methods**: 17 new methods
**Estimated Complexity After Refactoring**: Main function: 8, Largest extracted: 15

---

### Priority 2: `run_app()` (Complexity: 78 → <20)

**File**: `harness/runner/src/main.rs:888`
**Current**: ~421 lines, 78 complexity
**Target**: <20 complexity

This function is a large match statement handling different CLI commands. Extract each command handler into its own method.

**Structure**:
```rust
pub async fn run_app(cli: Cli) -> Result<()> {
    // Setup logging
    // Match on command:
    //   - Commands::Validate => handle_validate_command()
    //   - Commands::Benchmark => handle_benchmark_command()
    //   - Commands::Compare => handle_compare_command()
    //   - Commands::Analyze => handle_analyze_command()
    //   - Commands::Report => handle_report_command()
    //   - Commands::Regression => handle_regression_command()
}
```

**Refactoring**: Extract each match arm into a separate `handle_*_command()` method.

---

### Priority 3-6: Remaining Functions

**memory_profiler.rs:531** (66) - Extract profile analysis logic
**binary_analyzer.rs:621** (60) - Extract section analysis logic
**isolation.rs:130** (31) - Extract environment detection logic
**mcp_server.rs:218** (30) - Extract request handling logic

---

## Implementation Steps

### Phase 1: `run_benchmark()` Refactoring (2-3 hours)

1. **Step 1**: Extract helper methods (low-risk)
   - `log_benchmark_start()`
   - `log_isolation_status()`
   - `create_statistical_analyzer()`
   - `cleanup_environment()`

2. **Step 2**: Extract major sections (medium-risk)
   - `setup_environment_isolation()`
   - `generate_benchmark_reports()`
   - `perform_regression_analysis()`

3. **Step 3**: Extract language benchmark loop (high-risk)
   - `benchmark_all_languages()`
   - `benchmark_single_language()`

4. **Step 4**: Extract language-specific analysis (medium-risk)
   - `profile_memory_if_enabled()`
   - `finalize_memory_profile()`
   - `analyze_binary_if_available()`

5. **Step 5**: Refactor main function to call extracted methods
6. **Step 6**: Run tests to verify no behavioral changes
7. **Step 7**: Verify complexity with `cargo clippy`

### Phase 2: `run_app()` Refactoring (1-2 hours)

1. Extract each command handler
2. Update main function to simple match + delegate
3. Test each command
4. Verify complexity

### Phase 3: Remaining Functions (2-3 hours)

1. Apply same extract method pattern
2. Test after each refactoring
3. Verify complexity

---

## Testing Strategy

**After Each Extraction**:
```bash
# 1. Check compilation
cargo check -p rosetta-runner

# 2. Run unit tests
cargo test -p rosetta-runner

# 3. Run integration tests
cargo test -p rosetta-runner --test cli_tests

# 4. Verify complexity reduction
cargo clippy --all-targets -- -W clippy::cognitive_complexity 2>&1 | grep "cognitive_complexity"
```

**Final Verification**:
```bash
# All tests pass
make test

# All quality gates pass
make quality-gate

# Complexity check
cargo clippy --all-targets -- -W clippy::cognitive_complexity
```

---

## Risk Mitigation

**Risks**:
1. Breaking behavioral changes during refactoring
2. Introducing new bugs in extracted methods
3. Missing error handling paths
4. Lifetime/borrow checker issues with async code

**Mitigations**:
1. **Incremental Refactoring**: Extract one method at a time, test after each
2. **Git Safety**: Commit after each successful extraction
3. **Test Coverage**: Run all tests after every change
4. **Code Review**: Careful review of async/await boundaries
5. **Rollback Ready**: Can revert any problematic extraction

---

## Success Criteria

- [ ] All 6 high-complexity functions reduced to ≤20 complexity
- [ ] All tests passing (unit + integration)
- [ ] No behavioral changes (verified by test suite)
- [ ] Cargo clippy shows no complexity warnings
- [ ] Code more maintainable (smaller, focused functions)

---

## Deliverables

1. **Refactored Code**: All functions ≤20 complexity
2. **Test Report**: All tests passing
3. **Complexity Report**: Cargo clippy output showing improvements
4. **Documentation**: This plan document + commit messages
5. **Lessons Learned**: Document refactoring patterns used

---

## Timeline

**Estimated**: 6-8 hours total
- Priority 1 (run_benchmark): 2-3 hours
- Priority 2 (run_app): 1-2 hours
- Priority 3-6 (remaining): 2-3 hours
- Testing & verification: 1 hour

**Sprint 43 Allocation**: 2 days (Week 1)

---

**Document Status**: ✅ COMPLETE (Plan Ready for Execution)
**Author**: Sprint 43 Development Team
**Last Updated**: 2025-10-14 21:30:00 UTC
**Next Action**: Begin Phase 1 refactoring of `run_benchmark()`
