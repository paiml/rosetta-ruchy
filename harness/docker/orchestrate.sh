#!/bin/bash
# Container orchestration script for Rosetta Ruchy benchmarks
# Manages the lifecycle of benchmark containers with proper isolation

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
RESULTS_DIR="${PROJECT_ROOT}/results"
DOCKER_COMPOSE="docker-compose -f ${SCRIPT_DIR}/docker-compose.yml"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Help message
show_help() {
    cat << EOF
Rosetta Ruchy Container Orchestration

Usage: $(basename "$0") [COMMAND] [OPTIONS]

Commands:
    build       Build all Docker images
    run         Run benchmarks for specified languages
    clean       Clean up containers and volumes
    status      Show container status
    logs        Show container logs
    shell       Open interactive shell in container
    monitor     Start resource monitoring
    results     Aggregate and display results

Options:
    -l, --language  Specify language(s) to benchmark (comma-separated)
    -e, --example   Path to example to benchmark
    -i, --iterations Number of iterations (default: 100)
    -t, --timeout   Timeout in seconds (default: 300)
    -p, --parallel  Run benchmarks in parallel
    -h, --help      Show this help message

Examples:
    $(basename "$0") build
    $(basename "$0") run -l rust,go,python
    $(basename "$0") run -l all -e examples/fibonacci
    $(basename "$0") monitor &
    $(basename "$0") results

EOF
    exit 0
}

# Build Docker images
build_images() {
    log_info "Building Docker images..."
    
    # Build base image first
    docker build -t rosetta-ruchy/base:latest -f "${SCRIPT_DIR}/base.dockerfile" "${SCRIPT_DIR}"
    
    # Build language-specific images
    for lang in rust go python javascript c; do
        log_info "Building $lang image..."
        docker build -t "rosetta-ruchy/$lang:latest" -f "${SCRIPT_DIR}/$lang.dockerfile" "${SCRIPT_DIR}"
    done
    
    # Build runner image
    docker build -t rosetta-ruchy/runner:latest -f "${SCRIPT_DIR}/runner.dockerfile" "${SCRIPT_DIR}"
    
    log_success "All images built successfully"
}

# Run benchmarks
run_benchmarks() {
    local languages="${1:-all}"
    local example="${2:-}"
    local iterations="${3:-100}"
    local parallel="${4:-false}"
    
    log_info "Running benchmarks..."
    log_info "Languages: $languages"
    log_info "Iterations: $iterations"
    log_info "Parallel: $parallel"
    
    # Prepare results directory
    mkdir -p "${RESULTS_DIR}"
    local timestamp=$(date +%Y%m%d_%H%M%S)
    local run_dir="${RESULTS_DIR}/run_${timestamp}"
    mkdir -p "${run_dir}"
    
    # Determine which languages to run
    if [ "$languages" = "all" ]; then
        languages="rust,go,python,javascript,c"
    fi
    
    # Convert comma-separated list to array
    IFS=',' read -ra LANG_ARRAY <<< "$languages"
    
    # Run benchmarks
    if [ "$parallel" = "true" ]; then
        log_info "Starting parallel execution..."
        for lang in "${LANG_ARRAY[@]}"; do
            run_single_benchmark "$lang" "$example" "$iterations" "$run_dir" &
        done
        wait
    else
        for lang in "${LANG_ARRAY[@]}"; do
            run_single_benchmark "$lang" "$example" "$iterations" "$run_dir"
        done
    fi
    
    log_success "Benchmarks completed. Results in: $run_dir"
}

# Run single language benchmark
run_single_benchmark() {
    local lang="$1"
    local example="$2"
    local iterations="$3"
    local output_dir="$4"
    
    log_info "Running $lang benchmark..."
    
    # Create language output directory
    local lang_dir="${output_dir}/${lang}"
    mkdir -p "${lang_dir}"
    
    # Run container with isolated runner
    docker run --rm \
        --name "benchmark-${lang}-$$" \
        --cpuset-cpus="0-3" \
        --memory="4g" \
        --memory-swap="4g" \
        --privileged \
        --security-opt seccomp=unconfined \
        -v "${PROJECT_ROOT}/examples:/benchmark/examples:ro" \
        -v "${lang_dir}:/isolated/output" \
        -e "BENCHMARK_ITERATIONS=${iterations}" \
        rosetta-ruchy/runner:latest \
        isolated-run "$lang" "/benchmark/examples/${example:-fibonacci}" "/isolated/output"
    
    log_success "$lang benchmark completed"
}

# Clean up containers and volumes
clean_up() {
    log_info "Cleaning up containers and volumes..."
    
    # Stop all containers
    $DOCKER_COMPOSE down -v
    
    # Remove dangling images
    docker image prune -f
    
    # Clean benchmark work directories
    docker run --rm -v rosetta-work:/work alpine rm -rf /work/*
    
    log_success "Cleanup completed"
}

# Show container status
show_status() {
    log_info "Container status:"
    docker ps -a --filter "name=rosetta-" --format "table {{.Names}}\t{{.Status}}\t{{.Size}}"
    
    log_info "Image status:"
    docker images --filter "reference=rosetta-ruchy/*" --format "table {{.Repository}}\t{{.Tag}}\t{{.Size}}"
    
    log_info "Volume status:"
    docker volume ls --filter "name=rosetta-" --format "table {{.Name}}\t{{.Driver}}\t{{.Mountpoint}}"
}

# Show container logs
show_logs() {
    local container="${1:-}"
    
    if [ -z "$container" ]; then
        log_info "Available containers:"
        docker ps -a --filter "name=rosetta-" --format "{{.Names}}"
    else
        docker logs -f "rosetta-${container}"
    fi
}

# Open interactive shell
open_shell() {
    local lang="${1:-base}"
    
    log_info "Opening shell in $lang container..."
    docker run --rm -it \
        --name "rosetta-shell-$$" \
        -v "${PROJECT_ROOT}/examples:/benchmark/examples" \
        "rosetta-ruchy/${lang}:latest" \
        /bin/bash
}

# Start resource monitoring
start_monitoring() {
    log_info "Starting resource monitoring..."
    
    # Create monitoring output directory
    local monitor_dir="${RESULTS_DIR}/monitoring"
    mkdir -p "${monitor_dir}"
    
    # Start monitoring container
    docker run -d \
        --name "rosetta-monitor" \
        --restart unless-stopped \
        -v /var/run/docker.sock:/var/run/docker.sock:ro \
        -v "${monitor_dir}:/data" \
        -e "MONITOR_INTERVAL=1" \
        rosetta-ruchy/base:latest \
        bash -c 'while true; do docker stats --no-stream --format "json" >> /data/stats.jsonl; sleep ${MONITOR_INTERVAL}; done'
    
    log_success "Monitoring started. Data saved to: ${monitor_dir}"
}

# Aggregate and display results
aggregate_results() {
    log_info "Aggregating benchmark results..."
    
    # Find most recent run
    local latest_run=$(ls -td "${RESULTS_DIR}/run_"* 2>/dev/null | head -1)
    
    if [ -z "$latest_run" ]; then
        log_error "No benchmark results found"
        exit 1
    fi
    
    log_info "Processing results from: $latest_run"
    
    # Create summary
    echo "Benchmark Results Summary" > "${latest_run}/summary.txt"
    echo "=========================" >> "${latest_run}/summary.txt"
    echo "" >> "${latest_run}/summary.txt"
    
    for lang_dir in "${latest_run}"/*; do
        if [ -d "$lang_dir" ]; then
            lang=$(basename "$lang_dir")
            echo "Language: $lang" >> "${latest_run}/summary.txt"
            
            if [ -f "${lang_dir}/run.log" ]; then
                # Extract timing information
                grep "Elapsed (wall clock) time" "${lang_dir}/run.log" >> "${latest_run}/summary.txt" 2>/dev/null || true
                grep "Maximum resident set size" "${lang_dir}/run.log" >> "${latest_run}/summary.txt" 2>/dev/null || true
            fi
            echo "" >> "${latest_run}/summary.txt"
        fi
    done
    
    cat "${latest_run}/summary.txt"
    log_success "Results aggregated to: ${latest_run}/summary.txt"
}

# Main script logic
main() {
    # Parse command
    COMMAND="${1:-help}"
    shift || true
    
    # Parse options
    LANGUAGES=""
    EXAMPLE=""
    ITERATIONS="100"
    TIMEOUT="300"
    PARALLEL="false"
    
    while [[ $# -gt 0 ]]; do
        case $1 in
            -l|--language)
                LANGUAGES="$2"
                shift 2
                ;;
            -e|--example)
                EXAMPLE="$2"
                shift 2
                ;;
            -i|--iterations)
                ITERATIONS="$2"
                shift 2
                ;;
            -t|--timeout)
                TIMEOUT="$2"
                shift 2
                ;;
            -p|--parallel)
                PARALLEL="true"
                shift
                ;;
            -h|--help)
                show_help
                ;;
            *)
                log_error "Unknown option: $1"
                show_help
                ;;
        esac
    done
    
    # Execute command
    case "$COMMAND" in
        build)
            build_images
            ;;
        run)
            run_benchmarks "$LANGUAGES" "$EXAMPLE" "$ITERATIONS" "$PARALLEL"
            ;;
        clean)
            clean_up
            ;;
        status)
            show_status
            ;;
        logs)
            show_logs "$LANGUAGES"
            ;;
        shell)
            open_shell "$LANGUAGES"
            ;;
        monitor)
            start_monitoring
            ;;
        results)
            aggregate_results
            ;;
        help)
            show_help
            ;;
        *)
            log_error "Unknown command: $COMMAND"
            show_help
            ;;
    esac
}

# Run main function
main "$@"