# JavaScript/Node.js language-specific Docker image for benchmarking
FROM rosetta-ruchy/base:latest

LABEL language="javascript"
LABEL description="JavaScript benchmark environment with Node.js and Deno"

# Switch to root for installation
USER root

# Install Node.js via NodeSource repository
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Install Deno
RUN curl -fsSL https://deno.land/x/install/install.sh | sh && \
    mv /root/.deno/bin/deno /usr/local/bin/

# Install Bun (alternative JS runtime)
RUN curl -fsSL https://bun.sh/install | bash && \
    mv /root/.bun/bin/bun /usr/local/bin/

# Install global npm packages for benchmarking
RUN npm install -g \
    benchmark \
    autocannon \
    clinic \
    0x \
    node-inspect \
    why-is-node-running \
    v8-profiler-next

# Set Node.js environment variables
ENV NODE_ENV=production
ENV NODE_OPTIONS="--max-old-space-size=4096"

# Create JavaScript-specific benchmark script
RUN echo '#!/bin/bash\n\
set -e\n\
echo "📜 JavaScript Benchmark Environment"\n\
echo "Node.js version: $(node --version)"\n\
echo "Deno version: $(deno --version | head -n1)"\n\
echo "Bun version: $(bun --version)"\n\
\n\
# Run benchmarks with different runtimes\n\
if [ -f benchmark.js ]; then\n\
    echo "Running Node.js benchmarks..."\n\
    node --expose-gc benchmark.js\n\
    \n\
    echo "Running Deno benchmarks..."\n\
    deno run --allow-all benchmark.js\n\
    \n\
    echo "Running Bun benchmarks..."\n\
    bun run benchmark.js\n\
    \n\
    # Profile if requested\n\
    if [ "$PROFILE" = "true" ]; then\n\
        node --prof benchmark.js\n\
        node --prof-process isolate-*.log > profile.txt\n\
    fi\n\
fi\n\
\n\
# Run npm benchmark script if available\n\
if [ -f package.json ] && npm run benchmark 2>/dev/null; then\n\
    npm run benchmark\n\
fi\n\
\n\
exec "$@"' > /usr/local/bin/javascript-benchmark && \
    chmod +x /usr/local/bin/javascript-benchmark

# Switch back to benchmark user
USER ${BENCHMARK_USER}

CMD ["/usr/local/bin/javascript-benchmark"]