#!/usr/bin/env bash
# scripts/fix-unused-variables.sh - Fix unused variable warnings automatically
# Part of Sprint 43 Ticket 3: Lint Warning Reduction
#
# Usage: ./scripts/fix-unused-variables.sh <file.ruchy>

set -euo pipefail

if [ $# -ne 1 ]; then
    echo "Usage: $0 <file.ruchy>"
    exit 1
fi

FILE="$1"

if [ ! -f "$FILE" ]; then
    echo "Error: File '$FILE' not found"
    exit 1
fi

# Get list of unused variables from lint output
unused_vars=$(ruchy lint "$FILE" 2>&1 | grep "Warning - unused variable:" | sed -n 's/.*unused variable: \(.*\)/\1/p' | sort -u)

if [ -z "$unused_vars" ]; then
    echo "No unused variables found in $FILE"
    exit 0
fi

echo "Found unused variables in $FILE:"
echo "$unused_vars"
echo ""
echo "Fixing unused variables by prefixing with underscore..."

# Create backup
cp "$FILE" "$FILE.backup"

# For each unused variable, add underscore prefix
# Be careful to only replace let declarations, not usages
while IFS= read -r var; do
    if [ -n "$var" ]; then
        echo "  Fixing: $var -> _$var"
        # Replace "let var" with "let _var"
        # Replace "let mut var" with "let mut _var"
        sed -i "s/let $var =/let _$var =/g" "$FILE"
        sed -i "s/let mut $var =/let mut _$var =/g" "$FILE"

        # Also handle function parameters: "var:" -> "_var:"
        sed -i "s/ $var:/ _$var:/g" "$FILE"
        sed -i "s/($var:/(\_$var:/g" "$FILE"
        sed -i "s/, $var:/, _$var:/g" "$FILE"
    fi
done <<< "$unused_vars"

# Verify file still passes syntax check
if ruchy check "$FILE" > /dev/null 2>&1; then
    echo "✅ File passes syntax check"
    rm "$FILE.backup"

    # Count remaining warnings
    remaining=$(ruchy lint "$FILE" 2>&1 | grep -c "Warning -" || echo "0")
    echo "✅ Remaining warnings: $remaining"
else
    echo "❌ Syntax check failed, restoring backup"
    mv "$FILE.backup" "$FILE"
    exit 1
fi
