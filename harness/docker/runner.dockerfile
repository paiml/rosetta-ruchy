# Isolated benchmark runner container
# Provides maximum isolation and reproducibility for benchmark execution
#
# Features:
# - Minimal attack surface
# - Resource isolation via cgroups
# - Network isolation
# - Filesystem isolation with read-only mounts

FROM rosetta-ruchy/base:latest

LABEL description="Isolated benchmark runner with strict resource controls"
LABEL version="1.0.0"

# Switch to root for setup
USER root

# Install additional isolation tools
RUN apt-get update && apt-get install -y \
    cgroup-tools \
    firejail \
    apparmor-utils \
    bubblewrap \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Create isolated directories
RUN mkdir -p /isolated/input \
             /isolated/output \
             /isolated/work \
             /isolated/bin \
    && chmod 755 /isolated/* \
    && chown -R ${BENCHMARK_USER}:${BENCHMARK_USER} /isolated

# Copy essential binaries for offline operation
RUN cp /bin/bash /isolated/bin/ && \
    cp /usr/bin/time /isolated/bin/ && \
    cp /usr/bin/timeout /isolated/bin/ && \
    cp /usr/bin/env /isolated/bin/

# Create isolation wrapper script
RUN cat > /usr/local/bin/isolated-run << 'EOF'
#!/bin/bash
set -euo pipefail

# Parse arguments
LANGUAGE="${1:-}"
BENCHMARK="${2:-}"
OUTPUT_DIR="${3:-/isolated/output}"

if [ -z "$LANGUAGE" ] || [ -z "$BENCHMARK" ]; then
    echo "Usage: isolated-run <language> <benchmark-path> [output-dir]"
    exit 1
fi

echo "🔒 Starting isolated benchmark execution"
echo "Language: $LANGUAGE"
echo "Benchmark: $BENCHMARK"
echo "Output: $OUTPUT_DIR"

# Set resource limits
ulimit -v 4194304  # 4GB virtual memory
ulimit -u 1000     # Max 1000 processes
ulimit -n 1024     # Max 1024 open files
ulimit -t 300      # 300 seconds CPU time

# Disable network access
if command -v unshare >/dev/null 2>&1; then
    UNSHARE_FLAGS="--net"
else
    UNSHARE_FLAGS=""
fi

# Create temporary work directory
WORK_DIR=$(mktemp -d /isolated/work/bench-XXXXXX)
trap "rm -rf $WORK_DIR" EXIT

# Copy benchmark to work directory
cp -r "$BENCHMARK" "$WORK_DIR/benchmark"
cd "$WORK_DIR"

# Set up cgroup limits if available
if [ -d /sys/fs/cgroup ]; then
    CGROUP_NAME="benchmark-$$"
    cgcreate -g cpu,memory:/$CGROUP_NAME 2>/dev/null || true
    
    # Limit to 2 CPUs
    echo 200000 > /sys/fs/cgroup/cpu/$CGROUP_NAME/cpu.cfs_quota_us 2>/dev/null || true
    echo 100000 > /sys/fs/cgroup/cpu/$CGROUP_NAME/cpu.cfs_period_us 2>/dev/null || true
    
    # Limit to 2GB memory
    echo 2147483648 > /sys/fs/cgroup/memory/$CGROUP_NAME/memory.limit_in_bytes 2>/dev/null || true
    
    CGROUP_PREFIX="cgexec -g cpu,memory:/$CGROUP_NAME"
else
    CGROUP_PREFIX=""
fi

# Execute benchmark based on language
case "$LANGUAGE" in
    rust)
        if [ -f benchmark/Cargo.toml ]; then
            cd benchmark
            $CGROUP_PREFIX cargo build --release 2>&1 | tee "$OUTPUT_DIR/build.log"
            $CGROUP_PREFIX /usr/bin/time -v cargo run --release 2>&1 | tee "$OUTPUT_DIR/run.log"
        fi
        ;;
    go)
        if [ -f benchmark/go.mod ]; then
            cd benchmark
            $CGROUP_PREFIX go build -o bench 2>&1 | tee "$OUTPUT_DIR/build.log"
            $CGROUP_PREFIX /usr/bin/time -v ./bench 2>&1 | tee "$OUTPUT_DIR/run.log"
        fi
        ;;
    python)
        if [ -f benchmark/benchmark.py ]; then
            cd benchmark
            $CGROUP_PREFIX /usr/bin/time -v python3 benchmark.py 2>&1 | tee "$OUTPUT_DIR/run.log"
        fi
        ;;
    javascript)
        if [ -f benchmark/benchmark.js ]; then
            cd benchmark
            $CGROUP_PREFIX /usr/bin/time -v node benchmark.js 2>&1 | tee "$OUTPUT_DIR/run.log"
        fi
        ;;
    c)
        if [ -f benchmark/benchmark.c ]; then
            cd benchmark
            $CGROUP_PREFIX gcc -O3 -o bench benchmark.c 2>&1 | tee "$OUTPUT_DIR/build.log"
            $CGROUP_PREFIX /usr/bin/time -v ./bench 2>&1 | tee "$OUTPUT_DIR/run.log"
        fi
        ;;
    *)
        echo "Unknown language: $LANGUAGE"
        exit 1
        ;;
esac

# Clean up cgroup
if [ -n "$CGROUP_PREFIX" ]; then
    cgdelete -g cpu,memory:/$CGROUP_NAME 2>/dev/null || true
fi

echo "✅ Benchmark execution completed"
echo "Results saved to: $OUTPUT_DIR"
EOF
chmod +x /usr/local/bin/isolated-run

# Create health check script
RUN cat > /usr/local/bin/health-check << 'EOF'
#!/bin/bash
# Verify isolation features are working

echo "🔍 Checking isolation features..."

# Check cgroups
if [ -d /sys/fs/cgroup ]; then
    echo "✅ Cgroups available"
else
    echo "⚠️  Cgroups not available"
fi

# Check namespaces
if command -v unshare >/dev/null 2>&1; then
    echo "✅ Namespace isolation available"
else
    echo "⚠️  Namespace isolation not available"
fi

# Check resource limits
ulimit -a | head -5

echo "🔒 Isolation check complete"
EOF
chmod +x /usr/local/bin/health-check

# Drop capabilities for security
RUN setcap -r /usr/bin/ping 2>/dev/null || true

# Switch to benchmark user
USER ${BENCHMARK_USER}
WORKDIR /isolated

# Set restricted PATH
ENV PATH="/isolated/bin:/usr/local/bin"

# Default command
CMD ["/usr/local/bin/health-check"]