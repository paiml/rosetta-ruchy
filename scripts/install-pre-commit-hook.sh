#!/usr/bin/env bash
# Install pre-commit hook for Ruchy quality gates
# Usage: ./scripts/install-pre-commit-hook.sh

set -e

HOOK_DIR=".git/hooks"
HOOK_FILE="$HOOK_DIR/pre-commit"

# Check if we're in a git repository
if [ ! -d ".git" ]; then
    echo "❌ Error: Not in a git repository root"
    exit 1
fi

# Create hooks directory if it doesn't exist
mkdir -p "$HOOK_DIR"

# Create the pre-commit hook
cat > "$HOOK_FILE" <<'HOOK_END'
#!/usr/bin/env bash
# Pre-commit hook for Ruchy quality gates
# Runs dogfood-quick (3 core tools: check, lint, score)
# Fast validation (~2 min) before allowing commit

echo "🔍 Running pre-commit quality gates..."
echo ""

# Run dogfood-quick (fast validation)
if make dogfood-quick; then
    echo ""
    echo "✅ Quality gates passed - commit allowed"
    exit 0
else
    echo ""
    echo "❌ Quality gates failed - commit blocked"
    echo ""
    echo "To bypass this check (use sparingly):"
    echo "  git commit --no-verify"
    echo ""
    echo "To fix issues:"
    echo "  make test-all-examples  # See which files are failing"
    echo "  ruchy check <file>      # Check specific file syntax"
    echo "  ruchy lint <file>       # Check specific file style"
    echo ""
    exit 1
fi
HOOK_END

# Make hook executable
chmod +x "$HOOK_FILE"

echo "✅ Pre-commit hook installed successfully"
echo ""
echo "The hook will run 'make dogfood-quick' before each commit."
echo "This validates all examples with 3 core tools (check, lint, score)."
echo ""
echo "To bypass the hook (use sparingly):"
echo "  git commit --no-verify"
echo ""
echo "To test the hook:"
echo "  .git/hooks/pre-commit"
