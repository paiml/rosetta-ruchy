#!/usr/bin/env bash
# Debug script to find failing dogfood files

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Find all .ruchy files
RUCHY_FILES=$(find "${PROJECT_ROOT}/examples" -name "*.ruchy" -type f | sort)

echo "Testing each file with check, lint, score..."
echo ""

FAIL_COUNT=0

for file in $RUCHY_FILES; do
    RELATIVE="${file#${PROJECT_ROOT}/}"

    # Test check
    if ! ruchy check "$file" &>/dev/null; then
        echo "❌ CHECK FAIL: $RELATIVE"
        FAIL_COUNT=$((FAIL_COUNT + 1))
    fi

    # Test lint
    if ! ruchy lint "$file" &>/dev/null; then
        echo "❌ LINT FAIL: $RELATIVE"
        FAIL_COUNT=$((FAIL_COUNT + 1))
    fi

    # Test score
    if ! ruchy score "$file" &>/dev/null; then
        echo "❌ SCORE FAIL: $RELATIVE"
        FAIL_COUNT=$((FAIL_COUNT + 1))
    fi
done

echo ""
echo "Total failures: $FAIL_COUNT"
