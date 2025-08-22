# Rosetta Ruchy

A polyglot benchmark suite demonstrating Ruchy's performance parity with Rust while maintaining Python-like ergonomics. The repository provides empirical evidence of zero-cost abstractions through systematic comparison across production workloads.

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

## 📈 Current Status

**Phase 0: Foundation Infrastructure** ✅ 
- [x] Repository structure & quality gates established
- [x] Cargo workspace with statistical runner
- [x] Toyota Way methodology integrated
- [x] CI/CD pipeline with quality enforcement
- [ ] 🔄 **Next**: Initialize first benchmark example (ROSETTA-004)

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
