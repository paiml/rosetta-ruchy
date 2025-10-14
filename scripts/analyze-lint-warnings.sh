#!/usr/bin/env bash
# scripts/analyze-lint-warnings.sh - Analyze lint warning distribution
# Part of Sprint 43 Ticket 3: Lint Warning Reduction

set -euo pipefail

# Find all .ruchy files and count warnings in each
find examples -name "*.ruchy" -type f | while read -r file; do
    warnings=$(ruchy lint "$file" 2>&1 | grep -c "Warning -" || echo "0")
    if [ "$warnings" -gt 0 ]; then
        echo "$warnings $file"
    fi
done | sort -rn
