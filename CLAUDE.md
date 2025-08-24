# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is rosetta-ruchy, a polyglot benchmark suite designed to demonstrate Ruchy's performance parity with Rust while maintaining Python-like ergonomics. The repository provides empirical evidence of zero-cost abstractions through systematic comparison across production workloads.

**NEW: MCP Server Integration** - This repository also functions as an MCP (Model Context Protocol) server that provides real-time code translation capabilities to Claude Code agents, allowing seamless conversion from any language to Ruchy with immediate formal verification and performance analysis.

**CRITICAL P0 REQUIREMENTS**: 
1. Every Ruchy example MUST showcase the language's advanced tooling capabilities (AST analysis, provability checking, formal verification, hardware-aware optimization). This is Ruchy's defining trait and primary competitive advantage.
2. **ALL SCRIPTING MUST BE IN RUCHY** - No bash, Python, or other scripting languages. Use `.ruchy` files for all automation, benchmarking, and tooling. This demonstrates Ruchy's capability as a complete ecosystem replacement.

## Repository Status

**Current State**: Initial setup phase
- Only basic documentation exists (README, license, specifications)
- No implementation code has been created yet
- The project follows the comprehensive architecture outlined in `docs/specifications/rosetta-spec.md`

## Planned Architecture (from specification)

### Repository Structure
```
rosetta-ruchy/
├── Makefile                     # Global orchestration
├── examples/
│   ├── algorithms/              # Classical CS problems (001-fibonacci, 002-quicksort, etc.)
│   └── production/              # Real-world workloads (101-rest-api, 102-data-pipeline, etc.)
├── harness/                     # Benchmark infrastructure
├── scripts/                     # Automation scripts
└── docs/                        # Documentation and results
```

### Language Tiers
- **Tier 1**: Ruchy, Rust, Python, JavaScript, Go (full CI coverage)
- **Tier 2**: TypeScript, Java, C++, C#, Swift (community maintained)
- **Tier 3**: Reference implementations only

## Development Commands

**Note**: These commands are planned but not yet implemented. Check for existence before use.

### Global Commands (from root Makefile)
- `make all` - Build Docker images, run tests and benchmarks
- `make docker-build` - Build language-specific Docker images
- `make test` - Run all tests across examples
- `make bench` - Run all benchmarks
- `make format` - Format all code
- `make clean` - Clean all build artifacts
- `make validate` - Run pre-commit validation
- `make compare` - Generate performance comparison reports

### Example-Level Commands
- `make test` - Run tests for all languages in example
- `make bench` - Run benchmarks for all languages
- `make format-all` - Format code in all languages
- `make compare-performance` - Generate performance comparisons
- `make validate-spec` - Validate example specification

### Language-Specific Commands (Ruchy) - ALWAYS USE THESE
```bash
# STEP 1: Analysis (MANDATORY for every Ruchy file)
ruchy ast fibonacci.ruchy --format json      # AST structure analysis
ruchy provability fibonacci.ruchy             # Formal correctness verification
ruchy runtime fibonacci.ruchy                 # Complexity and performance analysis
ruchy score fibonacci.ruchy                   # Unified quality score

# STEP 2: Verification (REQUIRED before benchmarking)
ruchy quality-gate fibonacci.ruchy --threshold 0.95
ruchy provability fibonacci.ruchy --verify-correctness
ruchy provability fibonacci.ruchy --verify-termination

# STEP 3: Optimization (SHOWCASE Ruchy's advantage)
ruchy optimize fibonacci.ruchy --target-cpu native
ruchy optimize fibonacci.ruchy --vectorize
ruchy transpile fibonacci.ruchy --optimize-level 3

# STEP 4: Testing and Benchmarking
ruchy test --property-tests 10000 --mutation-testing
ruchy bench --verify-complexity O(n) --compare-languages rust,c

# STEP 5: Quality Reporting (ALWAYS generate these)
ruchy mcp --analyze fibonacci.ruchy          # Real-time quality dashboard
ruchy doc fibonacci.ruchy --include-proofs   # Documentation with formal proofs
```

**IMPORTANT**: Never run Ruchy code without first running the analysis and verification steps. This is what differentiates Ruchy from other languages.

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
```ruchy
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
```ruchy
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

### Continuous Deployment Protocol

**MANDATORY**: Every new algorithm implementation or significant update must:
1. **Push to GitHub immediately** after completion
2. **Trigger automatic release** to all package registries
3. **Update version numbers** following semantic versioning

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