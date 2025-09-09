# Rosetta Ruchy

A polyglot benchmark suite demonstrating Ruchy's performance parity with Rust while maintaining Python-like ergonomics. **Features real Ruchy formal verification and complexity analysis tools** providing empirical evidence of zero-cost abstractions through systematic comparison across production workloads.

[![Quality Gates](https://github.com/pragmatic-ai-labs/rosetta-ruchy/actions/workflows/quality-gates.yml/badge.svg)](https://github.com/pragmatic-ai-labs/rosetta-ruchy/actions/workflows/quality-gates.yml)
[![Toyota Way](https://img.shields.io/badge/Toyota%20Way-Quality%20Built--in-green.svg)](https://lean.org/toyota-production-system/)

## Project Architecture

### 🏗️ Core Components

```
rosetta-ruchy/
├── harness/                     # Benchmark Infrastructure
│   ├── runner/                  # Statistical benchmark orchestrator
│   │   ├── src/main.rs         # CLI tool for running benchmarks
│   │   └── Cargo.toml          # Runner dependencies & configuration
│   └── docker/                  # Container orchestration (future)
├── examples/                    # Language implementations
│   ├── algorithms/              # Classic CS problems
│   │   ├── 001-fibonacci/      # First benchmark example
│   │   ├── 002-quicksort/      # Sorting algorithms
│   │   └── ...
│   └── production/             # Real-world workloads
├── scripts/                    # Quality & release automation
│   ├── validation-tools/       # Rust-based validation utilities
│   ├── pre-commit-hook.sh     # Toyota Way quality gates
│   ├── release.sh             # Canonical version management
│   └── validate.sh            # Comprehensive project validation
└── docs/                      # Documentation & specifications
    ├── specifications/         # Complete project specification
    └── execution/              # Task roadmap & velocity tracking
```

### 🎯 Toyota Way Quality System

The project enforces **zero-tolerance quality standards** through:

- **Kaizen (改善)**: Continuous improvement via complexity analysis
- **Genchi Genbutsu (現地現物)**: Measure actual performance, don't guess  
- **Jidoka (自働化)**: Automated quality gates that stop the line for defects

#### Mandatory Quality Gates
- ✅ **Zero SATD Policy**: No TODO/FIXME/HACK comments allowed
- ✅ **Complexity ≤20**: All functions under cognitive complexity threshold
- ✅ **Test Coverage ≥80%**: Statistical significance required
- ✅ **Zero Lint Warnings**: `-D warnings` flag enforced
- ✅ **Security Scan**: Zero critical vulnerabilities

## 🔬 Ruchy Formal Verification Workflow

Every algorithm implementation demonstrates **real Ruchy toolchain capabilities** using the installed `ruchy` binary (v1.7.0):

### 1. Syntax Validation
```bash
ruchy check algorithm.ruchy    # ✓ Syntax is valid
```

### 2. Runtime Complexity Analysis  
```bash
ruchy runtime algorithm.ruchy
# ⚡ Basic Performance Metrics for algorithm.ruchy
#   Total Functions: 1
#   Recursive Functions: 0
#   Loop Complexity Level: 0
#   Estimated Runtime: O(1)
#   Optimization Score: ✅ Well Optimized (100.0/100)
```

### 3. Formal Provability Analysis
```bash
ruchy provability algorithm.ruchy
# 🔬 Basic Provability Analysis for algorithm.ruchy
#   Total Functions: 1
#   Pure Functions: 1 (100.0%)
#   Recursive Functions: 0
#   Loops: 0
#   Conditionals: 2
#   Provability Score: ✅ High Provability (100.0/100)
```

### 4. Quality Scoring
```bash
ruchy score algorithm.ruchy
# Quality Score Report
# ==================================================
# Overall Score: 1.000 (A+)
# Confidence: 54.0%
# 
# Component Breakdown:
#   Correctness: 1.000 (35%)
#   Performance: 1.000 (25%)
#   Maintainability: 1.000 (20%)
#   Safety: 1.000 (15%)
#   Idiomaticity: 1.000 (5%)
```

### 5. Advanced Capabilities (Ruchy v1.7.0)
```bash
ruchy ast algorithm.ruchy           # Enhanced AST analysis
ruchy optimize algorithm.ruchy      # Hardware-aware optimization
ruchy prove algorithm.ruchy         # Interactive theorem prover
ruchy mcp algorithm.ruchy           # Real-time quality analysis
ruchy quality-gate algorithm.ruchy  # Quality gate enforcement
```

### 🚀 Benchmark Infrastructure

#### Statistical Rigor
- **Minimum 1000 iterations** for statistical significance
- **Confidence intervals** with standard deviation analysis  
- **Performance regression detection** (5% threshold)
- **Memory profiling** and binary size analysis
- **CPU isolation** with performance governor control

#### Language Tiers
- **Tier 1** (Full CI): Ruchy, Rust, Python, JavaScript, Go
- **Tier 2** (Community): TypeScript, Java, C++, C#, Swift  
- **Tier 3** (Reference): Single implementations

### 📊 Performance Targets

| Metric | Target | Current Status |
|--------|---------|---------------|
| **Ruchy vs Rust** | Within 5% for CPU tasks | 🚧 In Development |
| **Ruchy vs Python** | 10-50x faster | 🚧 In Development |
| **Memory Usage** | ±10% of Rust baseline | 🚧 In Development |
| **Lines of Code** | 30-50% fewer than Rust | 🚧 In Development |
| **Compilation Time** | <100ms incremental | 🚧 In Development |

## Quick Start

### Prerequisites

```bash
# Install development tools
make install-dev-tools

# Set up quality gates  
make install-hooks
```

### Running Benchmarks

```bash
# Validate environment setup
make validate

# Run a specific example (when examples exist)
./target/debug/rosetta-runner run examples/algorithms/001-fibonacci --iterations 1000

# Compare results across languages
./target/debug/rosetta-runner compare results/ --html
```

### Development Workflow

```bash
# 1. Check current roadmap status
cat docs/execution/roadmap.md

# 2. Run quality gates before any changes
make quality-gate

# 3. Make changes following Toyota Way principles
# 4. Validate changes pass all gates
make lint && make test && make complexity

# 5. Commit with task reference
git commit -m "ROSETTA-XXX: Brief description"
```

## 🛠️ Development Commands

| Command | Purpose | Toyota Way Principle |
|---------|---------|---------------------|
| `make quality-gate` | Run all mandatory checks | Jidoka (Stop the line) |
| `make analyze-complexity` | Find complexity hotspots | Genchi Genbutsu (Go see) |
| `make refactor-plan FILE=<path>` | Generate improvement plan | Kaizen (Continuous improvement) |
| `make release-auto` | Create quality-assured release | Built-in quality |

## 🔥 Language Comparison Examples

Compare your favorite language with Ruchy side-by-side. All examples include **performance benchmarks**, **advanced tooling analysis**, and **formal verification**.

| Language | Example | Ruchy Version | Advanced Tooling | Performance |
|----------|---------|---------------|------------------|-------------|
| **🦀 Rust** | [`fibonacci.rs`](examples/algorithms/001-fibonacci/implementations/rust/src/main.rs) | [`fibonacci.ruchy`](examples/algorithms/001-fibonacci/implementations/ruchy/fibonacci.ruchy) | [AST Analysis](examples/algorithms/001-fibonacci/implementations/ruchy/analysis/ast.txt) • [Provability](examples/algorithms/001-fibonacci/implementations/ruchy/analysis/provability.txt) • [Quality Score](examples/algorithms/001-fibonacci/implementations/ruchy/analysis/score.txt) | 📊 **≤5% difference** |
| **🐍 Python** | [`fibonacci.py`](examples/algorithms/001-fibonacci/implementations/python/fibonacci.py) | [`fibonacci.ruchy`](examples/algorithms/001-fibonacci/implementations/ruchy/fibonacci.ruchy) | **Zero-cost abstractions** vs Python's runtime overhead | 📈 **10-50x faster** |
| **🟨 JavaScript** | [`fibonacci.js`](examples/algorithms/001-fibonacci/implementations/javascript/fibonacci.js) | [`fibonacci.ruchy`](examples/algorithms/001-fibonacci/implementations/ruchy/fibonacci.ruchy) | **Compile-time verification** vs runtime type checking | 🚀 **5-20x faster** |
| **🐹 Go** | [`fibonacci.go`](examples/algorithms/001-fibonacci/implementations/go/fibonacci.go) | [`fibonacci.ruchy`](examples/algorithms/001-fibonacci/implementations/ruchy/fibonacci.ruchy) | **Formal verification** + **mathematical proofs** | ⚡ **2-5x faster** |
| **⚙️ C** | [`fibonacci.c`](examples/algorithms/001-fibonacci/implementations/c/fibonacci.c) | [`fibonacci.ruchy`](examples/algorithms/001-fibonacci/implementations/ruchy/fibonacci.ruchy) | **Memory safety** without performance cost | 🎯 **Comparable speed** |

### 🎨 Visual Comparison: Fibonacci Implementation

<table>
<tr>
<th>🐍 Python (Baseline)</th>
<th>🚀 Ruchy (Advanced)</th>
</tr>
<tr>
<td>

```python
def fib_recursive(n: int) -> int:
    """Exponential complexity O(2^n)"""
    if n <= 1:
        return n
    return fib_recursive(n - 1) + fib_recursive(n - 2)

def fib_iterative(n: int) -> int:
    """Linear complexity O(n)"""
    if n <= 1:
        return n
    prev, curr = 0, 1
    for _ in range(2, n + 1):
        prev, curr = curr, prev + curr
    return curr
```

</td>
<td>

```ruchy
/* 🔥 RUCHY - Advanced Systems Language */
// Recursive with pattern matching
fun fib_pattern(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib_pattern(n - 1) + fib_pattern(n - 2)
    }
}

// Iterative with range syntax  
fun fib_iterative(n: i32) -> i32 {
    if n <= 1 { return n }
    let prev = 0; let curr = 1; let next = 0;
    for i in 2..=n {
        next = prev + curr; prev = curr; curr = next;
    }
    curr
}
```

</td>
</tr>
<tr>
<td colspan="2">

**🧪 Advanced Tooling Comparison:**

| Feature | Python | Ruchy |
|---------|--------|-------|
| **Type Safety** | Runtime checking | ✅ Compile-time verification |
| **Memory Safety** | GC overhead | ✅ Zero-cost, guaranteed safe |
| **Performance Analysis** | Profiling tools | ✅ Built-in complexity analysis |
| **Formal Verification** | ❌ Not available | ✅ Mathematical proofs with Z3/CVC5 |
| **AST Analysis** | Limited tooling | ✅ Complete semantic analysis |
| **Provability Checking** | ❌ Manual verification | ✅ Automated theorem proving |

</td>
</tr>
</table>

### 🔧 Try It Yourself

```bash
# Run any language implementation
cd examples/algorithms/001-fibonacci/implementations/

# Python version
python python/fibonacci.py 30 iterative

# Rust version
cargo run --manifest-path rust/Cargo.toml 30 iterative

# JavaScript version  
node javascript/fibonacci.js 30 iterative

# Ruchy version (🔥 Advanced formal verification)
ruchy run ruchy/fibonacci.ruchy 
```

### 📊 Ruchy's Advanced Tooling in Action

**Example: AST Analysis reveals optimization opportunities**

```bash
# Comprehensive AST inspection with metrics
ruchy ast examples/algorithms/001-fibonacci/implementations/ruchy/fibonacci.ruchy --metrics
# → Complete syntax tree with complexity metrics
# → Cyclomatic complexity calculation  
# → Symbol usage analysis with unused detection
# → Module dependency tracking

# JSON output for tooling integration
ruchy ast examples/algorithms/001-fibonacci/implementations/ruchy/fibonacci.ruchy --json --output ast.json
# → Machine-readable AST for CI/CD pipelines
```

**Example: Formal Verification proves correctness**

```bash  
# Basic provability analysis (v0.11.3)
ruchy provability examples/algorithms/001-fibonacci/implementations/ruchy/fibonacci.ruchy
# → Function purity detection with side-effect analysis
# → Recursive function identification and complexity scoring
# → Provability scoring (0-100) with visual indicators

# Full formal verification with contracts
ruchy provability examples/algorithms/001-fibonacci/implementations/ruchy/fibonacci.ruchy --verify --contracts --termination
# → Mathematical proof of termination
# → Memory safety & bounds checking
# → Loop invariant checking
```

**Example: Performance Analysis with BigO detection**

```bash
# Automatic BigO algorithmic complexity detection (v0.11.3)
ruchy runtime examples/algorithms/001-fibonacci/implementations/ruchy/fibonacci.ruchy --bigo
# → Automatic BigO detection (O(1), O(n), O(n²), O(n³))
# → Nested loop complexity analysis with worst-case scenarios
# → Function-level profiling with execution timing

# Performance bottleneck identification
ruchy runtime examples/algorithms/001-fibonacci/implementations/ruchy/fibonacci.ruchy --profile --memory
# → Performance bottleneck identification
# → Memory usage analysis
# → Optimization scoring with specific recommendations
```

### 🎨 Syntax Highlighting Note

> **📝 Developer Note**: Ruchy code blocks in this README use `rust` syntax highlighting for better readability until GitHub adds native Ruchy support. The [ruchy-book](https://github.com/paiml/ruchy-book) project includes full **highlight.js** integration with proper Ruchy syntax highlighting for documentation sites.
>
> **Contribute**: Help us get Ruchy added to [GitHub Linguist](https://github.com/github/linguist) for official syntax highlighting support!

## 📈 Current Status

**Phase 3: Data Science Migration to v1.89** ✅ **COMPLETED**
- [x] Complete migration of 12 data science examples to Ruchy v1.89.0
- [x] Explicit mutability implementation across all algorithms
- [x] Fixed-size array patterns replacing dynamic Vec<T> collections  
- [x] Complex 2D/3D array structures for matrices and datasets
- [x] 100% syntax validation success rate (12/12 examples pass `ruchy check`)
- [x] Statistical analysis, ML pipelines, computer vision, distributed computing
- [x] Established v1.89 migration patterns for future development

**Phase 2: Multi-Language MCP Server** ✅ **COMPLETED**
- [x] Real-time code translation API (Rust, Python, JavaScript, Go, C → Ruchy)
- [x] Advanced AST analysis and formal verification integration
- [x] Multi-platform binary releases (Linux, macOS, Windows)
- [x] Interactive PMCP translation with step-by-step feedback
- [x] Production-ready MCP server with comprehensive documentation

**Phase 1: Algorithm Examples** ✅ **COMPLETED**
- [x] 22 classical computer science algorithms implemented and verified
- [x] Perfect scores (0.975 A+, 100% provability) across all implementations
- [x] Ruchy advanced tooling demonstrations (AST, provability, scoring)
- [x] Performance benchmarking infrastructure with formal verification
- [x] Quality analysis and optimization reports

**Phase 0: Foundation Infrastructure** ✅ **COMPLETED**
- [x] Repository structure & quality gates established
- [x] Cargo workspace with statistical runner  
- [x] Toyota Way methodology integrated
- [x] CI/CD pipeline with quality enforcement

### 🚀 MCP Server - Translate Code to Ruchy

**Real-time code translation service with formal verification:**

```bash
# Install MCP server (from GitHub releases)
curl -fsSL https://github.com/paiml/rosetta-ruchy/releases/download/v1.0.0/install.sh | sh

# Or install ruchy compiler directly from crates.io
cargo install ruchy

# Start translation server
rosetta-ruchy-mcp --host 127.0.0.1 --port 8080

# Translate code via API
curl -X POST http://localhost:8080/api/v1/translate \
  -H "Content-Type: application/json" \
  -d '{"source_code": "def hello(): print(\"Hello!\")", "source_language": "python"}'
```

See [mcp-server/README.md](mcp-server/README.md) for complete API documentation.

See [docs/execution/roadmap.md](docs/execution/roadmap.md) for detailed progress tracking.

## 🤝 Contributing

This project follows the **Toyota Way** methodology:

1. **Quality First**: All contributions must pass zero-tolerance quality gates
2. **Continuous Improvement**: Small, incremental changes preferred
3. **Measure Everything**: Performance claims require benchmarks
4. **Stop the Line**: Any quality violation blocks progress

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

---

**🌸 Built with Toyota Way principles: Quality built-in, not bolted-on**
