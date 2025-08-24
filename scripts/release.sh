#!/bin/bash
# Canonical Release System - Toyota Way Quality Integration
# Automated version management and multi-registry publishing

set -e

RELEASE_TYPE="${1:-auto}"
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\([^"]*\)"/\1/')

echo "🚀 Canonical Release System"
echo "=========================================="
echo "Current version: $CURRENT_VERSION"
echo "Release type: $RELEASE_TYPE"
echo ""

# Version bump logic
bump_version() {
    local version=$1
    local bump_type=$2
    
    IFS='.' read -r major minor patch <<< "$version"
    
    case $bump_type in
        major)
            major=$((major + 1))
            minor=0
            patch=0
            ;;
        minor)
            minor=$((minor + 1))
            patch=0
            ;;
        patch)
            patch=$((patch + 1))
            ;;
        auto)
            # Auto-detect based on recent commits
            if git log --oneline -10 | grep -qE "(feat|BREAKING|major):"; then
                minor=$((minor + 1))
                patch=0
            else
                patch=$((patch + 1))
            fi
            ;;
    esac
    
    echo "$major.$minor.$patch"
}

NEW_VERSION=$(bump_version "$CURRENT_VERSION" "$RELEASE_TYPE")
echo "🔢 New version: $NEW_VERSION"

# Update version in Cargo.toml
echo "📝 Updating version numbers..."
sed -i "s/^version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" Cargo.toml

# Update workspace version
sed -i "/\[workspace\.package\]/,/^$/ s/^version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" Cargo.toml

# Create git tag
echo "🏷️  Creating git tag v$NEW_VERSION..."
git add Cargo.toml
git commit -m "chore: Release v$NEW_VERSION

🚀 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>"
git tag "v$NEW_VERSION"

echo "✅ Release v$NEW_VERSION ready!"
echo ""
echo "Next steps:"
echo "- git push origin main"
echo "- git push origin v$NEW_VERSION"
echo "- Publish to registries (cargo publish, etc.)"
echo "- Create GitHub release"

echo ""
echo "🌸 Rosetta Ruchy Release Complete - Toyota Way Quality Maintained"