## About This Project

This is the **Rosetta Ruchy** project, a polyglot benchmark suite for a custom language called **Ruchy**.

The project's main goal is to demonstrate Ruchy's performance parity with Rust while maintaining Python-like ergonomics. It features a rich toolchain for Ruchy that includes formal verification, complexity analysis, and quality scoring, providing empirical evidence of zero-cost abstractions.

## Key Technologies

*   **Ruchy (v1.7.0):** The custom programming language at the center of this project. It is designed for performance and safety, with a toolchain that supports formal verification using Z3/CVC5.
*   **Rust:** Used to build the benchmarking harness, an MCP server for code translation, and various validation tools. The project is structured as a Cargo workspace.
*   **Make:** Used for orchestrating builds, tests, and other development tasks.

## Development Process

The project follows the **Toyota Way** methodology, emphasizing continuous improvement and high-quality standards.

*   **Project Management:** Work is organized into sprints and phases, tracked in `ROADMAP.md`.
*   **Quality Gates:** Strict, automated quality gates are enforced before commits, including:
    *   Zero SATD (TODO/FIXME/HACK comments)
    *   Cognitive complexity <= 20
    *   Test coverage >= 80%
    *   Zero lint warnings
    *   Security scans
*   **Tickets:** All work is tracked in a roadmap and should have a corresponding ticket.

## Project Structure

*   `harness/`: The Rust-based benchmark infrastructure, including the `runner` for statistical benchmark orchestration.
*   `examples/`: Algorithm implementations in various languages for benchmarking.
*   `mcp-server/`: A server for translating code from other languages to Ruchy.
*   `scripts/`: Contains automation, quality-related scripts, and validation tools.
*   `docs/`: Project documentation, specifications, and roadmaps.

## Ruchy Toolchain Workflow

The project heavily utilizes the Ruchy toolchain to ensure code quality:

1.  **Syntax Validation:** `ruchy check`
2.  **Runtime Complexity Analysis:** `ruchy runtime`
3.  **Formal Provability Analysis:** `ruchy provability`
4.  **Quality Scoring:** `ruchy score`

## Quick Start

1.  **Install dependencies:** `make install-dev-tools`
2.  **Set up quality gates:** `make install-hooks`
3.  **Validate environment:** `make validate`
4.  **Run quality gates:** `make quality-gate`