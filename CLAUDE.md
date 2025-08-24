# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is rosetta-ruchy, a polyglot benchmark suite designed to demonstrate Ruchy's performance parity with Rust while maintaining Python-like ergonomics. The repository provides empirical evidence of zero-cost abstractions through systematic comparison across production workloads.

**NEW: MCP Server Integration** - This repository also functions as an MCP (Model Context Protocol) server that provides real-time code translation capabilities to Claude Code agents, allowing seamless conversion from any language to Ruchy with immediate formal verification and performance analysis.

**CRITICAL P0 REQUIREMENTS**: 
1. Every Ruchy example MUST showcase the language's advanced tooling capabilities (AST analysis, provability checking, formal verification, hardware-aware optimization). This is Ruchy's defining trait and primary competitive advantage.
2. **ALL SCRIPTING MUST BE IN RUCHY** - No bash, Python, or other scripting languages. Use `.ruchy` files for all automation, benchmarking, and tooling. This demonstrates Ruchy's capability as a complete ecosystem replacement.

## Ruchy Integration Tracking

**MANDATORY**: All Ruchy integration work must be documented in [INTEGRATION.md](./INTEGRATION.md) for scientific rigor:

- **Current Version**: Ruchy 1.8.0 (verified working)
- **Feature Status**: What works, what has limitations, what's unavailable
- **Version Migration**: When upgrading Ruchy, test and document changes
- **Feedback Loop**: Report issues to Ruchy team with reproducible examples
- **Scientific Impact**: Document how limitations affect validation goals

**Core Principle**: This is science. Every version change requires systematic testing and documentation of capabilities to maintain reproducible results and provide valuable feedback to the Ruchy development team.

## Repository Status

**Phase 2 Complete - Algorithm Validation**: ✅ **HISTORIC ACHIEVEMENT**
- ✅ **22/22 algorithms implemented and verified** with perfect scores (0.975 A+, 100% provability)
- ✅ **Systematic validation methodology** established and proven across complexity classes
- ✅ **Ruchy v1.9.3 compatibility patterns** documented and validated against v1.9.8 trunk
- ✅ **Complete documentation** with formal verification results and performance analysis
- ✅ **Scientific rigor** maintained throughout with reproducible results

**Phase 3 Active - Data Science Focus**: 🚀 **NEW STRATEGIC DIRECTION**
- 🎯 **Numerical Computing & Data Science** paradigms now primary focus
- 🔬 **DataFrame-first approach** to showcase Ruchy's type safety in data workflows  
- 📊 **Target languages expanded**: Julia, R, Python/pandas, Kotlin, Scala
- 🧪 **Advanced formal verification** for statistical algorithms and data transformations
- 📈 **High-impact domain** targeting real-world data science workloads

## Planned Architecture (from specification)

### Repository Structure (Phase 3: Data Science Focus)
```
rosetta-ruchy/
├── Makefile                     # Global orchestration
├── examples/
│   ├── algorithms/              # ✅ COMPLETE: 22 classical CS algorithms
│   └── data-science/           # 🆕 ACTIVE: Numerical computing & data analysis
│       ├── 001-dataframe-ops/  # Core DataFrame operations
│       ├── 002-statistical-analysis/  # Stats with formal verification
│       ├── 003-data-cleaning/  # Missing data, outliers, validation
│       ├── 004-time-series/    # Temporal analysis & forecasting
│       ├── 005-machine-learning/  # ML pipelines with type safety
│       └── ...                 # Up to 012-reproducible-research/
├── harness/                     # Benchmark infrastructure
│   ├── runner/                 # ✅ Statistical benchmark orchestrator
│   └── data-science-runner/    # 🆕 Specialized data science benchmarks
├── scripts/                     # Automation scripts (all .ruchy files)
└── docs/                        # Documentation and specifications
    └── specifications/
        ├── rosetta-spec.md     # Original specification
        └── data-science.md    # 🆕 Data science focus specification
```

### Language Tiers (Phase 3: Data Science)
#### Tier 1: Primary Data Science Languages (Full Implementation)
- **Ruchy**: Type-safe data science with formal verification
- **Julia**: High-performance scientific computing
- **Python/pandas**: Data science industry standard  
- **R**: Statistical computing and analysis

#### Tier 2: JVM Data Science Languages (Community Maintained)
- **Kotlin**: Enterprise data science on JVM
- **Scala**: Big data and distributed computing (Spark ecosystem)

#### Tier 3: Reference Implementations
- **Rust**: Polars, ndarray ecosystem
- **JavaScript**: D3.js, Observable notebooks

## Development Commands

### Global Commands (from root Makefile) - ✅ IMPLEMENTED
- `make all` - Build Docker images, run tests and benchmarks
- `make quality-gate` - Run Toyota Way mandatory quality checks
- `make test` - Run all tests across examples
- `make bench` - Run all benchmarks  
- `make validate` - Run comprehensive project validation
- `make compare` - Generate performance comparison reports

### Phase 3: Data Science Commands - 🆕 NEW FOCUS
```bash
# Data science specific validation and benchmarks
make data-science-validate      # Numerical accuracy testing vs Julia/R/pandas
make data-science-bench         # Performance benchmarking across languages
make data-science-verify        # Formal verification of statistical properties
make data-science-repro         # Reproducibility testing across platforms

# Sprint-based data science development
make dataframe-sprint           # Current: DataFrame operations sprint
make stats-sprint              # Next: Statistical analysis sprint  
make ml-sprint                 # Future: Machine learning pipeline sprint
```

### Example-Level Commands - ✅ ESTABLISHED PATTERN
- `make test` - Run tests for all languages in example
- `make bench` - Run benchmarks comparing all language implementations
- `make verify` - Run Ruchy formal verification and compare with reference implementations
- `make accuracy` - Validate numerical accuracy across language implementations

### Data Science Methodology - 🔬 SCIENTIFIC APPROACH

**Phase 3 Sprint Pattern**: Building on our proven 22-algorithm success
```bash
# Each data science sprint follows our established 4-stage verification:

# STAGE 1: Implementation (in Sprint 23+ pattern)
# - Implement core data science functionality in Ruchy with type safety
# - Create equivalent implementations in Julia, Python/pandas, R
# - Focus on dataframe-first approach with formal verification

# STAGE 2: Verification (proven methodology)
ruchy check dataframe_ops.ruchy           # Syntax validation
ruchy runtime dataframe_ops.ruchy         # Complexity analysis
ruchy provability dataframe_ops.ruchy     # Mathematical verification
ruchy score dataframe_ops.ruchy           # Quality assessment

# STAGE 3: Numerical Validation (new for data science)
ruchy numerical-accuracy dataframe_ops.ruchy --compare julia,pandas,r
ruchy statistical-verify dataframe_ops.ruchy --properties normality,convergence
ruchy reproducibility dataframe_ops.ruchy --platforms linux,macos,windows

# STAGE 4: Performance Benchmarking (extended for data science)
ruchy benchmark dataframe_ops.ruchy --scales small,medium,large,xlarge
ruchy memory-profile dataframe_ops.ruchy --track allocations,peak-usage
ruchy energy-profile dataframe_ops.ruchy --measure power-consumption
```

### Language-Specific Commands (Ruchy) - ✅ VERIFIED v1.9.8 COMPATIBILITY

**Validated against Ruchy Binary v1.9.8** (upgraded from v1.7.0):

```bash
# STEP 1: Syntax Validation (MANDATORY for every Ruchy file)
ruchy check fibonacci.ruchy                   # ✓ Syntax is valid
ruchy parse fibonacci.ruchy                   # Detailed AST analysis

# STEP 2: Formal Verification (CORE CAPABILITY)
ruchy provability fibonacci.ruchy             # Formal correctness verification
# 🔬 Basic Provability Analysis for fibonacci.ruchy
#   Total Functions: 5
#   Pure Functions: 5 (100.0%)
#   Recursive Functions: 1
#   Provability Score: ✅ High Provability (100.0/100)

ruchy runtime fibonacci.ruchy                 # Complexity and performance analysis
# ⚡ Basic Performance Metrics for fibonacci.ruchy  
#   Total Functions: 5
#   Recursive Functions: 1
#   Estimated Runtime: O(1)
#   Optimization Score: ✅ Well Optimized (85.0/100)

ruchy score fibonacci.ruchy                   # Unified quality score
# Quality Score Report
# ==================================================
# Overall Score: 0.975 (A+)
# Confidence: 80.0%

# STEP 3: Advanced Analysis (SHOWCASE Ruchy's advantage)
ruchy ast fibonacci.ruchy                     # Enhanced AST analysis (v0.9.12)
ruchy optimize fibonacci.ruchy                # Hardware-aware optimization (RUCHY-0816)
ruchy prove fibonacci.ruchy                   # Interactive theorem prover (RUCHY-0820)

# STEP 4: Quality Gates and MCP Integration
ruchy quality-gate fibonacci.ruchy           # Quality gate enforcement (RUCHY-0815)
ruchy mcp fibonacci.ruchy                     # Real-time quality analysis (RUCHY-0811)

# STEP 5: Additional Capabilities (26 total commands)
ruchy fmt fibonacci.ruchy                     # Format source code
ruchy lint fibonacci.ruchy                    # Lint for issues and style violations
ruchy doc fibonacci.ruchy                     # Generate documentation
ruchy transpile fibonacci.ruchy               # Transpile to Rust
ruchy run fibonacci.ruchy                     # Compile and run (may have transpilation issues)
```

**PROVEN CAPABILITIES**: All basic analysis commands (`check`, `runtime`, `provability`, `score`) work correctly and provide real formal verification results. The toolchain demonstrates:
- ✅ **Perfect Syntax Validation**: All implementations pass `ruchy check`
- ✅ **100% Function Purity**: Formal verification confirms mathematical purity
- ✅ **A+ Quality Scores**: 1.000/1.000 ratings achieved
- ✅ **High Provability**: 100.0/100 formal verification scores

### Ruchy as the Universal Scripting Language

**MANDATORY**: Use Ruchy for ALL scripting tasks:
- Build automation: `build.ruchy` instead of Makefile/build.sh
- Test runners: `test_runner.ruchy` instead of test.sh
- Benchmarking: `benchmark.ruchy` instead of benchmark.py
- CI/CD scripts: `ci.ruchy` instead of .github/workflows
- Data processing: `process.ruchy` instead of process.py
- System administration: `admin.ruchy` instead of admin.sh
- **MCP Server**: `mcp_server.ruchy` for code translation services

## MCP Server Integration

This repository provides a comprehensive MCP (Model Context Protocol) server implementation that enables real-time code translation and analysis services.

### MCP Server Capabilities

#### Core Translation Services
The MCP server provides endpoints for:
- **Code Translation**: Convert any supported language to idiomatic Ruchy
- **Performance Analysis**: Predict performance characteristics before translation
- **Formal Verification**: Immediate provability analysis during translation
- **Quality Assessment**: Real-time code quality scoring and improvement suggestions
- **Benchmark Comparison**: Live performance comparisons against existing implementations

#### PMCP Integration
Supports **PMCP (Protocol for MCP)** for enhanced capabilities:
- **Interactive Translation**: Step-by-step guided translation with user feedback
- **Real-time Verification**: Live formal verification as code is translated
- **Performance Insights**: Immediate benchmark projections and optimization suggestions
- **Quality Gates**: Real-time validation against Ruchy quality standards

### MCP Server Implementation

#### Server Architecture
```rust
#!/usr/bin/env ruchy
// mcp_server.ruchy - Main MCP server implementation

use std::net::TcpListener;
use std::json::{Json, JsonValue};
use rosetta_analysis::{translate, verify, benchmark};

struct RosettaMCPServer {
    host: String,
    port: u16,
    capabilities: MCPCapabilities,
}

impl RosettaMCPServer {
    fun new() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            capabilities: MCPCapabilities::new()
        }
    }
    
    fun start(&self) {
        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port))?;
        
        for stream in listener.incoming() {
            let stream = stream?;
            self.handle_client(stream);
        }
    }
    
    fun handle_translation_request(&self, request: TranslationRequest) -> TranslationResponse {
        // Step 1: Parse source code and detect language
        let source_lang = detect_language(&request.source_code);
        
        // Step 2: Translate to Ruchy
        let ruchy_code = translate::to_ruchy(&request.source_code, source_lang)?;
        
        // Step 3: Run Ruchy advanced tooling
        let ast_analysis = ruchy::ast(&ruchy_code)?;
        let provability = ruchy::provability(&ruchy_code)?;
        let quality_score = ruchy::score(&ruchy_code)?;
        
        // Step 4: Generate performance predictions
        let performance_prediction = benchmark::predict(&ruchy_code, source_lang)?;
        
        TranslationResponse {
            ruchy_code,
            ast_analysis,
            provability_score: provability.score,
            quality_score,
            performance_prediction,
            verification_status: provability.verified,
            optimization_suggestions: ruchy::optimize::suggest(&ruchy_code)?
        }
    }
}
```

#### Translation Endpoints
- `POST /api/v1/translate` - Translate code to Ruchy
- `POST /api/v1/analyze` - Analyze existing code without translation  
- `POST /api/v1/benchmark` - Compare performance across languages
- `POST /api/v1/verify` - Run formal verification on Ruchy code
- `GET /api/v1/capabilities` - Get server capabilities and supported languages

#### Integration with Claude Code
```json
{
  "mcp_servers": {
    "rosetta-ruchy-translator": {
      "command": "ruchy",
      "args": ["mcp_server.ruchy", "--host", "127.0.0.1", "--port", "8080"],
      "capabilities": [
        "code_translation",
        "performance_analysis",
        "formal_verification", 
        "quality_assessment"
      ]
    }
  }
}
```

Example Ruchy script structure:
```rust
#!/usr/bin/env ruchy

// Build script in Ruchy
fun main() {
    let args = std::env::args();
    
    match args.get(1) {
        Some("build") => build_all(),
        Some("test") => run_tests(),
        Some("bench") => run_benchmarks(),
        _ => show_help()
    }
}

fun build_all() {
    println("🔨 Building all implementations...");
    
    // Use Ruchy's built-in process execution
    for lang in ["rust", "go", "python", "javascript", "c"] {
        let result = std::process::run(
            format!("make build-{}", lang)
        );
        
        if !result.success {
            panic!("Build failed for {}", lang);
        }
    }
    
    println("✅ All builds complete");
}
```

This approach:
- Proves Ruchy can replace traditional scripting languages
- Leverages Ruchy's tooling even for build scripts
- Ensures type safety and verification for automation
- Demonstrates Ruchy as a complete ecosystem solution

## Implementation Constraints

### Fair Comparison Rules
1. **Standard Library Only** - No external dependencies except testing frameworks
2. **Algorithm Implementation** - Same algorithmic approach across languages
3. **Code Constraints** - Single file implementation (<500 LOC)
4. **Benchmark Environment** - Fixed CPU frequency, isolated cores, statistical analysis
5. **Memory Management** - Default allocator only, no manual pools

### Example Structure Pattern
Each example should follow:
```
examples/{category}/{number}-{name}/
├── README.md                    # Problem statement, complexity
├── Makefile                     # Example orchestration
├── spec.toml                    # Test cases, benchmark config
├── implementations/
│   ├── ruchy/                   # Ruchy implementation
│   ├── rust/                    # Rust implementation
│   ├── python/                  # Python implementation
│   └── [other languages]
└── results/                     # Cached benchmark results
```

## Success Criteria
- **Performance**: Ruchy within 5% of Rust for CPU-bound tasks
- **Memory**: Comparable heap usage to Rust (±10%)
- **Ergonomics**: 30-50% fewer lines than Rust
- **Correctness**: 100% spec compliance, SMT solver verified
- **Compilation**: <100ms incremental builds for 1KLOC

## Project Management Protocol (Toyota Way)

### The Toyota Way: Our Guiding Philosophy

- **Kaizen (改善): Continuous, Incremental Improvement** - We improve the codebase one component at a time. Every change is small, verifiable, and moves us toward quality goals. Avoid large, sweeping changes.
- **Genchi Genbutsu (現地現物): Go and See** - We don't guess where problems are. We use analysis tools to find the *actual* root cause of quality issues, such as complexity hotspots or technical debt.
- **Jidoka (自働化): Automation with a Human Touch** - We use automated tools to create plans, but intelligent verification must ensure correctness.

### Sacred Rules (Zero Tolerance)

1. **NEVER Use Shell Scripts** - ALL scripting must be in Ruchy. No .sh, .py, .js files for automation. Only .ruchy files demonstrate the language's capability.
2. **NEVER Leave Stub Implementations** - This is P0 priority. Never leave stub implementations with "not yet implemented" or "TODO". Every feature must be fully functional.
3. **NEVER Add SATD Comments** - Zero tolerance for self-admitted technical debt. Never add "TODO", "FIXME", "For now", etc. Every implementation must be complete.
4. **NEVER Use Simple Heuristics** - Zero tolerance for approximations. Always use proper analysis, full implementations, and accurate algorithms.
5. **NEVER Duplicate Core Logic** - One implementation per feature. All interfaces (CLI, MCP, HTTP) must use the same underlying logic.
6. **NEVER Bypass Quality Gates** - `git commit --no-verify` is FORBIDDEN. If quality gates fail, fix the underlying issues.

## 🔬 SCIENTIFIC REPRODUCIBILITY PROTOCOL

### Core Principle: Every Claim Must Be Proven
We follow the scientific method for all performance and correctness claims:
1. **Hypothesis**: State what Ruchy can do differently
2. **Experiment**: Write reproducible benchmarks using Ruchy tooling
3. **Measurement**: Collect empirical data with statistical rigor
4. **Analysis**: Use Ruchy's formal verification to prove properties
5. **Report**: Generate scientific reports with graphs and proofs
6. **Reproduction**: Anyone can run `make reproduce` to verify

### Sprint-Based Development Process
**MANDATORY**: Every sprint ends with a commit and push to GitHub

```bash
# Sprint Structure (2-3 day cycles)
Sprint N: Algorithm XXX Implementation
├── Day 1: Implement Makefile and Ruchy proofs
├── Day 2: Collect benchmark data across languages
├── Day 3: Generate scientific report and commit
└── END: git commit && git push (MANDATORY)

# Sprint Commit Message Format
git commit -m "sprint(N): Complete algorithm XXX with scientific validation

Reproducible Results:
- Complexity: O(n) formally proven by Ruchy
- Performance: XXXms (Ruchy) vs YYYms (Rust)
- Provability Score: 100/100
- Benchmark: make reproduce

Scientific Report: examples/algorithms/XXX/SCIENTIFIC_REPORT.md"
```

### Required Deliverables Per Algorithm
```
algorithm-XXX/
├── Makefile                    # All Ruchy commands (reproducible)
├── SCIENTIFIC_REPORT.md        # Complete analysis with graphs
├── results/
│   ├── complexity.txt          # ruchy runtime output
│   ├── provability.txt         # ruchy provability output
│   ├── quality_score.txt       # ruchy score output
│   ├── benchmarks.json         # Performance data (all languages)
│   └── statistical_analysis.md # Error bars, confidence intervals
└── implementations/
    └── ruchy/
        ├── algorithm.ruchy     # Clean, verified implementation
        └── benchmark.ruchy     # Benchmark harness in Ruchy
```

### Makefile Template (MANDATORY for each example)
```makefile
# MANDATORY: Every algorithm must have this Makefile
.PHONY: all verify benchmark report reproduce clean

# Ruchy binary location
RUCHY := ruchy

all: verify benchmark report

# Step 1: Formal Verification (Ruchy's unique capability)
verify:
	@echo "=== FORMAL VERIFICATION ==="
	$(RUCHY) check algorithm.ruchy
	$(RUCHY) runtime algorithm.ruchy > results/complexity.txt
	$(RUCHY) provability algorithm.ruchy > results/provability.txt
	$(RUCHY) score algorithm.ruchy > results/quality_score.txt
	$(RUCHY) prove algorithm.ruchy > results/formal_proof.txt

# Step 2: Performance Benchmarking
benchmark:
	@echo "=== PERFORMANCE BENCHMARKING ==="
	$(RUCHY) bench algorithm.ruchy --iterations 1000 > results/ruchy_bench.json
	cargo bench --bench algorithm > results/rust_bench.json
	python3 benchmark.py > results/python_bench.json
	node benchmark.js > results/js_bench.json

# Step 3: Generate Scientific Report
report:
	@echo "=== GENERATING SCIENTIFIC REPORT ==="
	$(RUCHY) run generate_report.ruchy > SCIENTIFIC_REPORT.md

# Step 4: Reproducibility Check
reproduce: clean all
	@echo "=== VERIFICATION COMPLETE ==="
	@echo "Results are reproducible. Check SCIENTIFIC_REPORT.md"

clean:
	rm -rf results/*.txt results/*.json
```

### MANDATORY Quality Gates (BLOCKING)

```bash
# These commands MUST pass before any commit:
make lint          # Zero clippy warnings allowed (-D warnings)
make test          # All tests must pass
make complexity    # Complexity ≤20 for all functions
make coverage      # Minimum 80% test coverage
make security      # Zero critical vulnerabilities
```

**CRITICAL**: All quality gates are BLOCKING, not advisory. Never use `--no-verify` to bypass.

### The Kaizen Refactoring Loop ("Kata")

This is the core workflow for improving the codebase:

#### Step 1: Find the Target (Genchi Genbutsu)
```bash
# Identify the most critical area for improvement
make analyze-complexity --top-files 5   # Find complexity hotspots
make analyze-debt                        # Find technical debt
make analyze-duplicates                  # Find code duplication
```

#### Step 2: Create the Refactoring Plan (Jidoka)
```bash
# Generate automated refactoring plan
make refactor-plan --file <path/to/target/file>
# Review and apply the suggested improvements
```

#### Step 3: Verify the Improvement
```bash
# Ensure quality improved and no regressions
make quality-gate --file <path/to/target/file>
make test-fast
# Commit only after both pass
```

### MANDATORY Commit Protocol

```bash
# Every commit must reference task and include metrics:
git commit -m "ALG-001: Implement fibonacci algorithm in Ruchy

Validates: docs/specifications/rosetta-spec.md Section 2.1
Performance: 98.5% of Rust baseline (target: 95%)
Complexity: 8/20 (under threshold)
Coverage: 94% (exceeds 80% requirement)"
```

### Task Management System

#### PDMT (Pragmatic Deterministic MCP Templating)

For ALL task planning, use deterministic todo generation:

```bash
# Generate structured todos with quality gates
make pdmt-todos "Implement quicksort example" --granularity medium --seed 42
```

**PDMT Requirements:**
- **Deterministic**: Fixed seed (42) for reproducible planning
- **Quality-Enforced**: Includes validation commands and success criteria
- **Structured**: Each todo has implementation specs and quality gates
- **Complete**: No vague descriptions or stub placeholders

#### Task Structure Example
```yaml
- task: "Implement Rust quicksort baseline"
  validation: "cargo test --package quicksort-rust"
  success_criteria: 
    - "Passes all property tests"
    - "Complexity ≤15"
    - "Performance within 5% of reference"
  implementation_spec:
    - "Use in-place partitioning algorithm"
    - "Handle edge cases (empty, single element)"
    - "Include comprehensive test suite"
```

### Release Management (Canonical Version System)

#### Pre-Release Quality Gates
```bash
# 12 automated quality checks
make pre-release-checks
```

Validates:
- Version consistency across workspace
- All tests passing
- Zero SATD tolerance
- Security audit (cargo-audit)
- SemVer compatibility
- Outdated dependencies

#### Version Bump Detection
```bash
# Auto-detect version bump type
make release-auto

# Manual version bumps
make release-patch   # Bug fixes (x.y.Z)
make release-minor   # New features (x.Y.z)  
make release-major   # Breaking changes (X.y.z)
```

#### Release Process
```bash
# Complete release workflow:
make release-auto
# This automatically:
# 1. Runs all pre-release checks
# 2. Updates versions in workspace
# 3. Updates CHANGELOG.md
# 4. Creates git commit and tag
# 5. Pushes to GitHub
# 6. Creates GitHub release
# 7. Publishes to appropriate registries
```

### BigO Complexity Verification Protocol

**CRITICAL P0 REQUIREMENT**: Every algorithm implementation MUST showcase Ruchy's formal verification capabilities by proving its computational complexity. This is Ruchy's core differentiator and cannot be skipped.

#### Mandatory BigO Validation Steps

**STEP 1: Theoretical Complexity Declaration**
Every algorithm must declare its expected complexity in the Ruchy implementation:

```rust
#!/usr/bin/env ruchy
// algorithm_name.ruchy - REQUIRED complexity annotations

#[complexity(time = "O(n log n)", space = "O(log n)")]
#[worst_case(time = "O(n^2)", space = "O(log n)")]  
#[best_case(time = "O(n)", space = "O(1)")]
fn quicksort(arr: &mut [i32]) -> SortResult {
    // Implementation with complexity invariants
}

// MANDATORY: Complexity proofs for each operation
#[prove_complexity]
#[verify_bounds(input_size = "1..1000000")]
fn benchmark_complexity() {
    // Automated complexity verification tests
}
```

**STEP 2: Ruchy Formal Verification (MANDATORY)**
```bash
# MUST RUN these commands for every algorithm implementation:

# 1. Verify theoretical complexity matches implementation
ruchy complexity algorithm_name.ruchy --verify-bounds --prove-correctness

# 2. Generate formal complexity proofs  
ruchy prove algorithm_name.ruchy --complexity-analysis --export-proof

# 3. Validate against known complexity classes
ruchy benchmark algorithm_name.ruchy --verify-complexity O(n*log(n)) --statistical-validation

# 4. Generate complexity visualization
ruchy plot algorithm_name.ruchy --complexity-growth --export-svg

# 5. Compare with theoretical optimums
ruchy optimal algorithm_name.ruchy --compare-theoretical --detect-improvements
```

**STEP 3: Complexity Report Generation (REQUIRED OUTPUT)**
Every algorithm must generate these artifacts:
- `complexity_proof.json` - Formal mathematical proof of complexity
- `complexity_plot.svg` - Visual growth rate verification  
- `statistical_validation.csv` - Empirical complexity measurements
- `optimality_analysis.md` - Comparison with theoretical bounds

**STEP 4: Integration with Algorithm Documentation**
```rust
// MANDATORY: Include complexity verification in algorithm documentation
#[doc(complexity_verified = true)]
#[doc(proof_file = "complexity_proof.json")]
#[doc(empirical_validation = "statistical_validation.csv")]  
#[doc(optimality_proven = true)]
pub fn algorithm_implementation() {
    // Ruchy's formal verification ensures this matches declared complexity
}
```

#### Examples of Required Complexity Validation

**For Sorting Algorithms:**
```bash
# QuickSort example - MUST verify O(n log n) average case
ruchy complexity quicksort.ruchy --verify "O(n*log(n))" --prove-average-case
ruchy complexity quicksort.ruchy --verify "O(n^2)" --prove-worst-case  

# Radix Sort example - MUST verify O(d*(n+k)) linear time
ruchy complexity radix_sort.ruchy --verify "O(d*(n+k))" --prove-linear
ruchy complexity radix_sort.ruchy --compare-theoretical --validate-superiority
```

**For Graph Algorithms:**  
```bash
# Dijkstra example - MUST verify O((V+E) log V) with binary heap
ruchy complexity dijkstra.ruchy --verify "O((V+E)*log(V))" --prove-heap-operations
ruchy complexity dijkstra.ruchy --validate-data-structure fibonacci_heap --prove-improvement
```

**For Dynamic Programming:**
```bash
# Knapsack example - MUST verify O(n*W) time and space bounds
ruchy complexity knapsack.ruchy --verify "O(n*W)" --prove-dp-table-size
ruchy complexity knapsack.ruchy --validate-space-optimization --prove-O(W)
```

#### Failure Cases - BLOCKING ISSUES

**❌ COMMIT BLOCKED** if any of these fail:
- Complexity proof generation fails
- Empirical measurements don't match theoretical bounds (±5% tolerance)
- Ruchy formal verification cannot prove correctness
- Missing complexity annotations in code
- No statistical validation data generated

**Quality Gate Integration:**
```bash
# These commands are integrated into pre-commit hooks
make complexity-verify    # Runs all algorithm complexity proofs
make prove-bounds        # Verifies empirical matches theoretical
make optimality-check    # Ensures no better complexity exists
```

#### Why This Is Critical

**Ruchy's Competitive Advantage:**
1. **Formal Verification** - No other language can prove complexity automatically
2. **Mathematical Rigor** - Static analysis proves correctness before runtime
3. **Performance Guarantees** - Compiler enforces complexity bounds
4. **Optimization Detection** - Identifies when better algorithms are possible

**Demonstration Value:**
- Shows Ruchy can formally verify what other languages only claim
- Proves performance characteristics before deployment
- Provides mathematical guarantees other languages cannot offer
- Enables automatic optimization suggestions

**CRITICAL**: This complexity verification is what separates Ruchy implementations from all others. Every algorithm MUST demonstrate this capability to justify Ruchy's existence.

### Continuous Deployment Protocol

**MANDATORY**: Every new algorithm implementation or significant update must:
1. **BigO Complexity Verified** - All formal verification steps completed
2. **Push to GitHub immediately** after complexity proofs generated
3. **Trigger automatic release** to all package registries
4. **Update version numbers** following semantic versioning

#### Sprint Release Workflow
After completing each algorithm example:
```bash
# Example: After implementing 015-topological-sort
git add examples/algorithms/015-topological-sort/
git commit -m "feat: Add topological sort algorithm implementation"
git push origin main
make release-patch  # Triggers v1.0.X release

# This automatically publishes to:
# - crates.io (Rust packages)
# - npm (if JavaScript examples included)
# - PyPI (if Python examples included)  
# - Docker Hub (benchmark images)
# - GitHub Packages
```

#### Per-Sprint Deliverables
Each development sprint must deliver:
- **New Algorithm**: Fully implemented in all Tier 1 languages
- **GitHub Push**: Code available in main branch
- **Package Release**: Published to all relevant registries
- **Documentation**: Updated with performance metrics
- **Benchmarks**: Comparison data in results/

### Documentation Standards

#### Living Documentation
- All specifications must include executable examples
- Every function must have doctests that demonstrate usage
- Performance claims must include benchmarks
- Architecture decisions must include rationale

#### Documentation Quality Gates
```bash
make doc-test      # All doctests must pass
make doc-coverage  # Minimum 90% documentation coverage
make doc-links     # All links must be valid
```

### Development Environment Setup

#### Required Tools
```bash
# Install quality enforcement tools
make install-dev-tools
```

Installs:
- `cargo-tarpaulin` - Coverage analysis
- `cargo-audit` - Security scanning
- `cargo-semver-checks` - API compatibility
- `cargo-outdated` - Dependency freshness
- `cargo-mutants` - Mutation testing

#### Pre-commit Hooks
```bash
# Install mandatory quality gates
make install-hooks
```

Blocks commits that:
- Have complexity >20
- Include SATD comments
- Fail any tests
- Have clippy warnings
- Reduce coverage below 80%
- **Missing BigO complexity verification for algorithm implementations**
- **Failed Ruchy formal complexity proofs**
- **Empirical complexity measurements don't match theoretical bounds**
- **Missing required complexity artifacts (proof.json, plot.svg, validation.csv)**

## Development Notes

### Getting Started
1. **Install Development Tools**: Run `make install-dev-tools` first
2. **Set Up Quality Gates**: Run `make install-hooks` to enable pre-commit checks
3. **Review Specification**: Study `docs/specifications/rosetta-spec.md`
4. **Generate Initial Tasks**: Use `make pdmt-todos "Bootstrap rosetta-ruchy infrastructure"`
5. **Start with Infrastructure**: Begin with harness, Docker, and CI (not examples)

### Implementation Priorities
1. **Infrastructure First**: Benchmark harness, Docker images, CI/CD pipeline
2. **Single Example Validation**: Complete algorithms/001-fibonacci in all Tier 1 languages
3. **Quality Gate Integration**: Ensure all quality gates work end-to-end
4. **Scale Horizontally**: Add more algorithms only after first example is perfect

### Algorithm Implementation Workflow (MANDATORY PROCESS)

**Every algorithm implementation MUST follow this exact sequence:**

#### Phase 1: Research and Design
```bash
# 1. Research theoretical complexity
# Document expected time/space complexity for:
# - Best case, Average case, Worst case
# - Different input distributions
# - Space-time tradeoffs

# 2. Design Ruchy implementation with complexity annotations
# Include formal complexity declarations in code
```

#### Phase 2: Ruchy Implementation with Formal Verification
```bash
# 3. Create Ruchy implementation with complexity annotations
examples/algorithms/XXX-algorithm-name/implementations/ruchy/algorithm.ruchy

# 4. MANDATORY: Run Ruchy complexity verification
cd examples/algorithms/XXX-algorithm-name/implementations/ruchy/
ruchy complexity algorithm.ruchy --verify-bounds --prove-correctness
ruchy prove algorithm.ruchy --complexity-analysis --export-proof
ruchy benchmark algorithm.ruchy --verify-complexity "O(n*log(n))" --statistical-validation
ruchy plot algorithm.ruchy --complexity-growth --export-svg
ruchy optimal algorithm.ruchy --compare-theoretical --detect-improvements

# 5. REQUIRED: Generate complexity artifacts
# - complexity_proof.json (formal mathematical proof)
# - complexity_plot.svg (visual growth rate verification)
# - statistical_validation.csv (empirical measurements)
# - optimality_analysis.md (comparison with theoretical bounds)
```

#### Phase 3: Baseline Implementations (Rust, Python, etc.)
```bash
# 6. Implement in Rust for performance baseline
cd examples/algorithms/XXX-algorithm-name/implementations/rust/
cargo build --release
cargo test

# 7. Implement in Python for ergonomics comparison
cd examples/algorithms/XXX-algorithm-name/implementations/python/
python3 -m pytest

# 8. Run comparative benchmarks
cd examples/algorithms/XXX-algorithm-name/
make bench-all  # Compare all implementations
```

#### Phase 4: Verification and Validation
```bash
# 9. MANDATORY: Verify Ruchy's complexity claims hold empirically
ruchy validate complexity_proof.json --empirical-test statistical_validation.csv
ruchy verify algorithm.ruchy --performance-bounds --against-baseline rust

# 10. MANDATORY: Ensure Ruchy matches or beats baseline performance  
ruchy compare algorithm.ruchy --baseline rust --tolerance 5% --prove-superiority

# 11. Generate final comparison report
ruchy report algorithm.ruchy --include-complexity-proofs --export-markdown
```

#### Phase 5: Integration and Deployment  
```bash
# 12. Update workspace and test compilation
# Add to Cargo.toml workspace members

# 13. Run quality gates - MUST PASS ALL
make lint          # Zero clippy warnings
make test          # All tests pass  
make complexity    # Ruchy complexity proofs validated
make prove-bounds  # Empirical matches theoretical

# 14. Commit with complexity verification evidence
git add examples/algorithms/XXX-algorithm-name/
git commit -m "feat(algorithms): Implement XXX-algorithm-name with verified O(complexity)

- Ruchy formal verification proves O(complexity) bounds
- Empirical validation confirms theoretical complexity  
- Performance matches/exceeds Rust baseline
- Generated complexity proof artifacts"

# 15. Push and trigger release
git push origin main
./scripts/release.sh auto
```

**CRITICAL SUCCESS CRITERIA:**
- ✅ Ruchy complexity verification generates proof artifacts
- ✅ Empirical measurements match theoretical bounds (±5% tolerance)  
- ✅ Performance meets or beats Rust baseline
- ✅ All complexity visualization and proof files generated
- ✅ Formal verification confirms correctness and termination

**FAILURE CONDITIONS (BLOCKING):**
- ❌ Ruchy complexity verification fails to generate proofs
- ❌ Empirical complexity doesn't match theoretical (>5% deviation)
- ❌ Missing any required complexity artifacts
- ❌ Performance significantly worse than baseline (>10% regression)
- ❌ Formal verification cannot prove algorithm correctness

This workflow ensures every algorithm demonstrates Ruchy's formal verification capabilities while maintaining performance parity with systems languages.

### Testing Strategy
- **Statistical Rigor**: Minimum 1000 iterations, standard deviation analysis
- **Formal Verification**: SMT solver verification for Ruchy implementations
- **Mutation Testing**: Minimum 85% mutation score for critical algorithms
- **Performance Regression**: Automated detection with 5% threshold
- **Property Testing**: QuickCheck-style tests for algorithm correctness

### Repository Hygiene
- **Zero Cruft**: No temporary files, debug binaries, or uncommitted artifacts
- **Clean History**: Squash commits before pushing, meaningful commit messages
- **Documentation Sync**: Keep CLAUDE.md, README.md, and specs synchronized
- **Dependency Management**: Regular updates, security scanning, license compatibility

### Success Metrics Dashboard
- **Performance**: Real-time comparison matrix (Ruchy vs baselines)
- **Quality**: Complexity trends, test coverage evolution
- **Velocity**: Features completed per sprint, technical debt reduction
- **User Experience**: Example compilation times, error message quality
- always check for new ruchy version on each sprint after you commit.
- we do not do "theortical", or "work arounds", we ONLY use the ruchy language in current form.  if a flaw exists we practice TOYOTA way, and flag the problem at the root level via INTEGRATION.md in the same way as ../ruchy-book.