# Base Docker image for Rosetta Ruchy benchmark environment
# Provides common dependencies and system configuration for reliable benchmarks
#
# Following Toyota Way principles:
# - Standardization (Heijunka): Consistent environment across all benchmarks
# - Quality at Source (Jidoka): Built-in isolation and monitoring
# - Continuous Improvement (Kaizen): Version-controlled and documented

FROM ubuntu:22.04

# Metadata
LABEL maintainer="Rosetta Ruchy Project"
LABEL description="Base benchmark environment with common dependencies"
LABEL version="1.0.0"

# Prevent interactive prompts during package installation
ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=UTC

# System configuration for benchmarking
ENV BENCHMARK_USER=benchmark
ENV BENCHMARK_HOME=/home/benchmark
ENV BENCHMARK_WORK=/benchmark

# Update and install essential system packages
RUN apt-get update && apt-get install -y \
    # Build essentials
    build-essential \
    cmake \
    ninja-build \
    pkg-config \
    # Version control
    git \
    # Network tools for downloading dependencies
    curl \
    wget \
    ca-certificates \
    gnupg \
    lsb-release \
    # System monitoring and profiling tools
    htop \
    iotop \
    sysstat \
    procps \
    time \
    # Performance analysis tools
    linux-tools-generic \
    linux-tools-common \
    valgrind \
    strace \
    ltrace \
    # Binary analysis tools
    binutils \
    file \
    # Python for scripting
    python3 \
    python3-pip \
    python3-venv \
    # Library dependencies
    libssl-dev \
    libffi-dev \
    libbz2-dev \
    libreadline-dev \
    libsqlite3-dev \
    libncurses5-dev \
    libncursesw5-dev \
    xz-utils \
    tk-dev \
    libxml2-dev \
    libxmlsec1-dev \
    liblzma-dev \
    # Cleanup
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Install performance monitoring Python packages
RUN pip3 install --no-cache-dir \
    psutil \
    py-cpuinfo \
    memory_profiler \
    matplotlib \
    pandas \
    numpy

# Create non-root user for running benchmarks
RUN groupadd -r ${BENCHMARK_USER} && \
    useradd -r -g ${BENCHMARK_USER} -d ${BENCHMARK_HOME} -s /bin/bash -m ${BENCHMARK_USER}

# Set up benchmark directories
RUN mkdir -p ${BENCHMARK_WORK}/results \
    ${BENCHMARK_WORK}/logs \
    ${BENCHMARK_WORK}/cache \
    ${BENCHMARK_WORK}/implementations \
    && chown -R ${BENCHMARK_USER}:${BENCHMARK_USER} ${BENCHMARK_WORK}

# Configure system limits for benchmarking
RUN echo "${BENCHMARK_USER} soft nofile 65536" >> /etc/security/limits.conf && \
    echo "${BENCHMARK_USER} hard nofile 65536" >> /etc/security/limits.conf && \
    echo "${BENCHMARK_USER} soft nproc 32768" >> /etc/security/limits.conf && \
    echo "${BENCHMARK_USER} hard nproc 32768" >> /etc/security/limits.conf

# Disable CPU frequency scaling for consistent benchmarks
# Note: This requires privileged mode at runtime
RUN echo '#!/bin/bash\n\
if [ -f /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor ]; then\n\
    for cpu in /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor; do\n\
        echo performance > $cpu 2>/dev/null || true\n\
    done\n\
fi' > /usr/local/bin/set-performance-mode && \
    chmod +x /usr/local/bin/set-performance-mode

# Create benchmark entrypoint script
RUN echo '#!/bin/bash\n\
set -e\n\
\n\
echo "🚀 Rosetta Ruchy Benchmark Environment"\n\
echo "========================================"\n\
echo "Host: $(hostname)"\n\
echo "User: $(whoami)"\n\
echo "Date: $(date)"\n\
echo "CPUs: $(nproc)"\n\
echo "Memory: $(free -h | grep Mem | awk '\''{print $2}'\'')"\\n\
echo "========================================"\n\
\n\
# Try to set performance mode (requires privileged)\n\
/usr/local/bin/set-performance-mode 2>/dev/null || echo "Note: CPU governor not set (requires --privileged)"\n\
\n\
# Disable ASLR for consistent memory measurements\n\
echo 0 | sudo tee /proc/sys/kernel/randomize_va_space 2>/dev/null || echo "Note: ASLR not disabled (requires --privileged)"\n\
\n\
# Execute provided command or shell\n\
exec "$@"' > /entrypoint.sh && \
    chmod +x /entrypoint.sh

# Switch to non-root user
USER ${BENCHMARK_USER}
WORKDIR ${BENCHMARK_WORK}

# Set entrypoint
ENTRYPOINT ["/entrypoint.sh"]
CMD ["/bin/bash"]

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD ps aux | grep -v grep | grep -q benchmark || exit 1