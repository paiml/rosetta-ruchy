## About This Project

This is the **Rosetta Ruchy** project, a polyglot benchmark suite for a custom language called **Ruchy**.

The project's main goal is to demonstrate Ruchy's performance parity with Rust while maintaining Python-like ergonomics. It features a rich toolchain for Ruchy that includes formal verification, complexity analysis, and quality scoring.

## Key Technologies

*   **Ruchy:** The custom programming language at the center of this project.
*   **Rust:** Used to build the benchmarking harness, an MCP server for code translation, and various validation tools. The project is structured as a Cargo workspace.
*   **Make:** Used for orchestrating builds, tests, and other development tasks.

## Development Process

The project follows the **Toyota Way** methodology, emphasizing continuous improvement and high-quality standards.

*   **Project Management:** Work is organized into sprints and phases, tracked in `ROADMAP.md`.
*   **Quality Gates:** Strict, automated quality gates are enforced before commits, including linting, testing, and complexity analysis.
*   **Tickets:** All work is tracked in a roadmap and should have a corresponding ticket.

## Project Structure

*   `examples/`: Contains algorithm implementations for benchmarking.
*   `harness/`: The Rust-based benchmark runner.
*   `mcp-server/`: A server for translating code from other languages to Ruchy.
*   `scripts/`: Contains automation and quality-related scripts.
*   `docs/`: Project documentation, specifications, and roadmaps.
