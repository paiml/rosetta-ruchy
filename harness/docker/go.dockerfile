# Go language-specific Docker image for benchmarking
FROM rosetta-ruchy/base:latest

LABEL language="go"
LABEL description="Go benchmark environment with latest Go compiler"

# Switch to root for installation
USER root

# Install Go
ENV GO_VERSION=1.21.5
RUN wget -q https://go.dev/dl/go${GO_VERSION}.linux-amd64.tar.gz && \
    tar -C /usr/local -xzf go${GO_VERSION}.linux-amd64.tar.gz && \
    rm go${GO_VERSION}.linux-amd64.tar.gz

# Set Go environment variables
ENV PATH="/usr/local/go/bin:${PATH}"
ENV GOPATH="/opt/go"
ENV GOCACHE="/opt/go-cache"
ENV GO111MODULE=on
ENV GOOS=linux
ENV GOARCH=amd64
ENV CGO_ENABLED=1

# Create Go directories
RUN mkdir -p ${GOPATH}/bin ${GOPATH}/src ${GOPATH}/pkg ${GOCACHE} && \
    chown -R ${BENCHMARK_USER}:${BENCHMARK_USER} ${GOPATH} ${GOCACHE}

# Install Go benchmarking tools
RUN go install golang.org/x/perf/cmd/benchstat@latest && \
    go install github.com/google/pprof@latest && \
    mv /root/go/bin/* /usr/local/go/bin/ && \
    rm -rf /root/go

# Create Go-specific benchmark script
RUN echo '#!/bin/bash\n\
set -e\n\
echo "🐹 Go Benchmark Environment"\n\
echo "Go version: $(go version)"\n\
\n\
# Run benchmarks if available\n\
if [ -f go.mod ]; then\n\
    echo "Running Go benchmarks..."\n\
    go test -bench=. -benchmem -benchtime=10s -count=5 ./...\n\
    \n\
    # Generate CPU profile if requested\n\
    if [ "$PROFILE" = "true" ]; then\n\
        go test -bench=. -cpuprofile=cpu.prof -memprofile=mem.prof ./...\n\
    fi\n\
fi\n\
\n\
exec "$@"' > /usr/local/bin/go-benchmark && \
    chmod +x /usr/local/bin/go-benchmark

# Switch back to benchmark user
USER ${BENCHMARK_USER}

CMD ["/usr/local/bin/go-benchmark"]