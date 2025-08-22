# C language-specific Docker image for benchmarking
FROM rosetta-ruchy/base:latest

LABEL language="c"
LABEL description="C benchmark environment with GCC and Clang"

# Switch to root for installation
USER root

# Install C compilers and tools
RUN apt-get update && apt-get install -y \
    gcc-12 \
    g++-12 \
    clang-15 \
    llvm-15 \
    libc6-dev \
    gdb \
    cgdb \
    autoconf \
    automake \
    libtool \
    make \
    cmake \
    ninja-build \
    ccache \
    cppcheck \
    clang-tidy-15 \
    clang-format-15 \
    libgoogle-perftools-dev \
    google-perftools \
    kcachegrind \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Set default compiler versions
RUN update-alternatives --install /usr/bin/gcc gcc /usr/bin/gcc-12 100 && \
    update-alternatives --install /usr/bin/g++ g++ /usr/bin/g++-12 100 && \
    update-alternatives --install /usr/bin/clang clang /usr/bin/clang-15 100 && \
    update-alternatives --install /usr/bin/clang++ clang++ /usr/bin/clang++-15 100

# Set compiler environment variables
ENV CC=gcc
ENV CXX=g++
ENV CFLAGS="-O3 -march=native -mtune=native -flto"
ENV CXXFLAGS="-O3 -march=native -mtune=native -flto"
ENV LDFLAGS="-flto"

# Create C-specific benchmark script
RUN echo '#!/bin/bash\n\
set -e\n\
echo "⚙️ C Benchmark Environment"\n\
echo "GCC version: $(gcc --version | head -n1)"\n\
echo "Clang version: $(clang --version | head -n1)"\n\
\n\
# Function to build and benchmark\n\
benchmark_compiler() {\n\
    local compiler=$1\n\
    local name=$2\n\
    \n\
    echo "Building with $name..."\n\
    $compiler $CFLAGS -o benchmark_$name benchmark.c -lm\n\
    \n\
    echo "Running $name benchmark..."\n\
    time -p ./benchmark_$name\n\
    \n\
    # Profile if requested\n\
    if [ "$PROFILE" = "true" ]; then\n\
        $compiler $CFLAGS -pg -o benchmark_${name}_prof benchmark.c -lm\n\
        ./benchmark_${name}_prof\n\
        gprof benchmark_${name}_prof gmon.out > ${name}_profile.txt\n\
    fi\n\
}\n\
\n\
# Build and run with different compilers\n\
if [ -f benchmark.c ]; then\n\
    benchmark_compiler gcc "GCC"\n\
    benchmark_compiler clang "Clang"\n\
    \n\
    # Compare binary sizes\n\
    echo "Binary sizes:"\n\
    ls -lh benchmark_*\n\
fi\n\
\n\
# Run make benchmark if Makefile exists\n\
if [ -f Makefile ] && grep -q "^benchmark:" Makefile; then\n\
    make clean\n\
    make benchmark\n\
fi\n\
\n\
exec "$@"' > /usr/local/bin/c-benchmark && \
    chmod +x /usr/local/bin/c-benchmark

# Switch back to benchmark user
USER ${BENCHMARK_USER}

CMD ["/usr/local/bin/c-benchmark"]