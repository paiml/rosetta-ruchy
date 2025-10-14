#!/usr/bin/env bash
# Benchmark Runner - Compare Ruchy vs Rust performance
# Usage: ./scripts/benchmark-runner.sh <algorithm> <input-size> [iterations]

set -e

ALGORITHM=$1
INPUT_SIZE=$2
ITERATIONS=${3:-10}  # Default 10 iterations for quick test

if [ -z "$ALGORITHM" ] || [ -z "$INPUT_SIZE" ]; then
    echo "Usage: $0 <algorithm> <input-size> [iterations]"
    echo ""
    echo "Example: $0 001-fibonacci 30 100"
    echo ""
    echo "Available algorithms:"
    echo "  001-fibonacci"
    echo "  002-quicksort"
    echo "  004-binary-search"
    echo "  007-dijkstra-shortest-path"
    echo "  010-edit-distance"
    exit 1
fi

ALGO_DIR="examples/algorithms/$ALGORITHM"
RUST_DIR="$ALGO_DIR/implementations/rust"
RUCHY_DIR="$ALGO_DIR/implementations/ruchy"

if [ ! -d "$ALGO_DIR" ]; then
    echo "❌ Algorithm not found: $ALGORITHM"
    exit 1
fi

echo "🔬 Benchmarking: $ALGORITHM"
echo "📊 Input size: $INPUT_SIZE"
echo "🔁 Iterations: $ITERATIONS"
echo ""

# Create output directory
mkdir -p reports/performance/raw
TIMESTAMP=$(date +%Y%m%d-%H%M%S)
OUTPUT_FILE="reports/performance/raw/${ALGORITHM}-${INPUT_SIZE}-${TIMESTAMP}.json"

# Compile Rust version
echo "🦀 Compiling Rust implementation..."
if [ -d "$RUST_DIR" ] && [ -f "$RUST_DIR/Cargo.toml" ]; then
    (cd "$RUST_DIR" && cargo build --release --quiet)
    RUST_BIN="$RUST_DIR/target/release/$(grep '^name' $RUST_DIR/Cargo.toml | head -1 | cut -d'"' -f2)"
    echo "   ✅ Rust compiled: $RUST_BIN"
else
    echo "   ⚠️  No Rust Cargo project found"
    RUST_BIN=""
fi

# Find Ruchy version (use simplest working version)
echo "🔧 Finding Ruchy implementation..."
RUCHY_FILE=""
for candidate in "$RUCHY_DIR/fibonacci_simple.ruchy" "$RUCHY_DIR/fibonacci.ruchy" "$RUCHY_DIR"/*.ruchy; do
    if [ -f "$candidate" ] && ruchy check "$candidate" &>/dev/null; then
        RUCHY_FILE="$candidate"
        echo "   ✅ Ruchy file: $(basename $RUCHY_FILE)"
        break
    fi
done

if [ -z "$RUCHY_FILE" ]; then
    echo "   ❌ No working Ruchy implementation found"
    exit 1
fi

# Benchmark Rust
RUST_TIMES=()
if [ -n "$RUST_BIN" ] && [ -x "$RUST_BIN" ]; then
    echo ""
    echo "⏱️  Benchmarking Rust..."
    for i in $(seq 1 $ITERATIONS); do
        START=$(date +%s%N)
        "$RUST_BIN" "$INPUT_SIZE" &>/dev/null || true
        END=$(date +%s%N)
        DURATION=$(( (END - START) / 1000000 ))  # Convert to milliseconds
        RUST_TIMES+=($DURATION)
        printf "\r   Iteration $i/$ITERATIONS: ${DURATION}ms"
    done
    echo ""
fi

# Benchmark Ruchy
echo ""
echo "⏱️  Benchmarking Ruchy..."
RUCHY_TIMES=()
for i in $(seq 1 $ITERATIONS); do
    START=$(date +%s%N)
    ruchy run "$RUCHY_FILE" "$INPUT_SIZE" &>/dev/null || true
    END=$(date +%s%N)
    DURATION=$(( (END - START) / 1000000 ))  # Convert to milliseconds
    RUCHY_TIMES+=($DURATION)
    printf "\r   Iteration $i/$ITERATIONS: ${DURATION}ms"
done
echo ""

# Calculate statistics
calc_stats() {
    local times=("$@")
    local sum=0
    local count=${#times[@]}
    
    # Mean
    for t in "${times[@]}"; do
        sum=$((sum + t))
    done
    local mean=$((sum / count))
    
    # Min/Max
    local min=${times[0]}
    local max=${times[0]}
    for t in "${times[@]}"; do
        [ $t -lt $min ] && min=$t
        [ $t -gt $max ] && max=$t
    done
    
    echo "$mean $min $max"
}

if [ ${#RUST_TIMES[@]} -gt 0 ]; then
    RUST_STATS=($(calc_stats "${RUST_TIMES[@]}"))
    RUST_MEAN=${RUST_STATS[0]}
    RUST_MIN=${RUST_STATS[1]}
    RUST_MAX=${RUST_STATS[2]}
fi

RUCHY_STATS=($(calc_stats "${RUCHY_TIMES[@]}"))
RUCHY_MEAN=${RUCHY_STATS[0]}
RUCHY_MIN=${RUCHY_STATS[1]}
RUCHY_MAX=${RUCHY_STATS[2]}

# Display results
echo ""
echo "📊 Results:"
echo "==========="
if [ -n "$RUST_MEAN" ]; then
    echo "Rust:  ${RUST_MEAN}ms (min: ${RUST_MIN}ms, max: ${RUST_MAX}ms)"
fi
echo "Ruchy: ${RUCHY_MEAN}ms (min: ${RUCHY_MIN}ms, max: ${RUCHY_MAX}ms)"

if [ -n "$RUST_MEAN" ] && [ $RUST_MEAN -gt 0 ]; then
    RATIO=$((RUCHY_MEAN * 100 / RUST_MEAN))
    echo ""
    echo "Performance ratio: ${RATIO}% of Rust"
    
    if [ $RATIO -le 105 ]; then
        echo "✅ Within 5% of Rust (excellent)"
    elif [ $RATIO -le 110 ]; then
        echo "✅ Within 10% of Rust (good)"
    else
        echo "⚠️  More than 10% slower than Rust"
    fi
fi

# Save JSON results
cat > "$OUTPUT_FILE" <<JSON_END
{
  "algorithm": "$ALGORITHM",
  "input_size": $INPUT_SIZE,
  "iterations": $ITERATIONS,
  "timestamp": "$TIMESTAMP",
  "rust": {
    "mean_ms": ${RUST_MEAN:-null},
    "min_ms": ${RUST_MIN:-null},
    "max_ms": ${RUST_MAX:-null},
    "times": [$(IFS=,; echo "${RUST_TIMES[*]:-}")]
  },
  "ruchy": {
    "mean_ms": $RUCHY_MEAN,
    "min_ms": $RUCHY_MIN,
    "max_ms": $RUCHY_MAX,
    "times": [$(IFS=,; echo "${RUCHY_TIMES[*]}")]
  }
}
JSON_END

echo ""
echo "💾 Results saved: $OUTPUT_FILE"

