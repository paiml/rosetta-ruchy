# Python language-specific Docker image for benchmarking
FROM rosetta-ruchy/base:latest

LABEL language="python"
LABEL description="Python benchmark environment with CPython and PyPy"

# Switch to root for installation
USER root

# Install multiple Python versions for comparison
RUN apt-get update && apt-get install -y \
    python3.10 \
    python3.11 \
    python3-dev \
    pypy3 \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Install Python benchmarking packages
RUN pip3 install --no-cache-dir \
    pytest-benchmark \
    pyperf \
    line_profiler \
    memory_profiler \
    py-spy \
    scalene \
    snakeviz \
    cython \
    numba \
    numpy \
    scipy

# Set Python environment variables
ENV PYTHONDONTWRITEBYTECODE=1
ENV PYTHONUNBUFFERED=1
ENV PYTHONPATH="/opt/python"

# Create Python-specific benchmark script
RUN echo '#!/bin/bash\n\
set -e\n\
echo "🐍 Python Benchmark Environment"\n\
echo "Python version: $(python3 --version)"\n\
echo "PyPy version: $(pypy3 --version 2>&1 | head -n1)"\n\
\n\
# Run benchmarks with different interpreters\n\
if [ -f benchmark.py ]; then\n\
    echo "Running CPython benchmarks..."\n\
    python3 -m pyperf timeit -s "from benchmark import main" "main()" --rigorous\n\
    \n\
    echo "Running PyPy benchmarks..."\n\
    pypy3 -m pyperf timeit -s "from benchmark import main" "main()" --rigorous\n\
    \n\
    # Profile if requested\n\
    if [ "$PROFILE" = "true" ]; then\n\
        python3 -m cProfile -o profile.stats benchmark.py\n\
        python3 -m memory_profiler benchmark.py\n\
    fi\n\
fi\n\
\n\
# Run pytest benchmarks if available\n\
if [ -f test_benchmark.py ]; then\n\
    pytest test_benchmark.py --benchmark-only --benchmark-autosave\n\
fi\n\
\n\
exec "$@"' > /usr/local/bin/python-benchmark && \
    chmod +x /usr/local/bin/python-benchmark

# Switch back to benchmark user
USER ${BENCHMARK_USER}

CMD ["/usr/local/bin/python-benchmark"]