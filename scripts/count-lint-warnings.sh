#!/usr/bin/env bash
# scripts/count-lint-warnings.sh - Count total Ruchy lint warnings
# Part of Sprint 43 Ticket 1: Lint No-Increase Enforcement

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if ruchy is installed
if ! command -v ruchy &> /dev/null; then
    echo -e "${RED}Error: ruchy command not found${NC}" >&2
    exit 1
fi

# Directories to scan for .ruchy files (validated examples only)
SCAN_DIRS=(
    "examples/algorithms"
    "examples/data-science"
    "examples/advanced-ai"
)

# Counter for total warnings
TOTAL_WARNINGS=0

# Temporary file for lint output
TEMP_OUTPUT=$(mktemp)
trap "rm -f $TEMP_OUTPUT" EXIT

# Function to count warnings in a single file
count_file_warnings() {
    local file="$1"

    # Run ruchy lint and capture output (always returns exit 0)
    ruchy lint "$file" > "$TEMP_OUTPUT" 2>&1 || true

    # Count warning lines (format: "Warning -" in output)
    # Also look for summary line like "Summary: 2 Errors, 4 Warnings"
    local warnings=0

    # Method 1: Count individual "Warning -" lines
    warnings=$(grep -c "Warning -" "$TEMP_OUTPUT" || true)

    # Method 2: If summary line exists, extract warning count from it
    if grep -q "Summary:" "$TEMP_OUTPUT"; then
        local summary_warnings=$(grep "Summary:" "$TEMP_OUTPUT" | sed -n 's/.*\([0-9]\+\) Warnings/\1/p' || echo "0")
        if [ -n "$summary_warnings" ] && [ "$summary_warnings" -gt 0 ]; then
            warnings="$summary_warnings"
        fi
    fi

    echo "$warnings"
}

# Main scanning loop
for dir in "${SCAN_DIRS[@]}"; do
    if [ ! -d "$dir" ]; then
        continue
    fi

    # Find all .ruchy files in directory
    while IFS= read -r -d '' file; do
        warnings=$(count_file_warnings "$file")
        TOTAL_WARNINGS=$((TOTAL_WARNINGS + warnings))
    done < <(find "$dir" -type f -name "*.ruchy" -print0)
done

# Output total count (no formatting, just number for script parsing)
echo "$TOTAL_WARNINGS"
