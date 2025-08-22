# Rust language-specific Docker image for benchmarking
FROM rosetta-ruchy/base:latest

LABEL language="rust"
LABEL description="Rust benchmark environment with cargo and rustc"

# Switch to root for installation
USER root

# Install Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    --default-toolchain stable \
    --profile minimal \
    --component rustfmt clippy

# Set Rust environment variables
ENV PATH="/root/.cargo/bin:${PATH}"
ENV RUSTFLAGS="-C target-cpu=native"
ENV CARGO_HOME="/opt/cargo"
ENV RUSTUP_HOME="/opt/rustup"

# Install additional Rust tools
RUN /root/.cargo/bin/cargo install --locked \
    cargo-bloat \
    cargo-tree \
    cargo-audit \
    hyperfine \
    && rm -rf /opt/cargo/registry

# Create Rust-specific benchmark script
RUN echo '#!/bin/bash\n\
set -e\n\
echo "🦀 Rust Benchmark Environment"\n\
echo "Rust version: $(rustc --version)"\n\
echo "Cargo version: $(cargo --version)"\n\
\n\
# Build in release mode with optimizations\n\
if [ -f Cargo.toml ]; then\n\
    echo "Building Rust project..."\n\
    cargo build --release\n\
    \n\
    # Run benchmarks if available\n\
    if cargo test --benches --no-run 2>/dev/null; then\n\
        cargo bench --no-fail-fast\n\
    fi\n\
fi\n\
\n\
exec "$@"' > /usr/local/bin/rust-benchmark && \
    chmod +x /usr/local/bin/rust-benchmark

# Make Rust tools available to benchmark user
RUN cp -r /root/.cargo /opt/cargo && \
    cp -r /root/.rustup /opt/rustup && \
    chown -R ${BENCHMARK_USER}:${BENCHMARK_USER} /opt/cargo /opt/rustup

# Switch back to benchmark user
USER ${BENCHMARK_USER}

# Update PATH for benchmark user
ENV PATH="/opt/cargo/bin:${PATH}"

CMD ["/usr/local/bin/rust-benchmark"]