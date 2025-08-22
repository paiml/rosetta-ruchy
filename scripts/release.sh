#!/bin/bash
# Canonical Release Script - Toyota Way Quality Gates
# Prevents version regression and ensures quality

set -e

BUMP_TYPE="${1:-auto}"
PROJECT_NAME="rosetta-ruchy"

echo "🚀 Canonical Release System - $PROJECT_NAME"
echo "Bump type: $BUMP_TYPE"
echo "=============================================="

# Function to detect version bump type from git commits
detect_bump_type() {
    echo "🔍 Analyzing commits since last tag..."
    
    # Get last tag or first commit if no tags
    LAST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || git rev-list --max-parents=0 HEAD)
    
    # Check for breaking changes
    if git log "$LAST_TAG"..HEAD --oneline | grep -E "(BREAKING|breaking change|!:)" >/dev/null; then
        echo "major"
        return
    fi
    
    # Check for new features
    if git log "$LAST_TAG"..HEAD --oneline | grep -E "(feat:|feature:|add:|implement:)" >/dev/null; then
        echo "minor"
        return
    fi
    
    # Default to patch for bug fixes and other changes
    echo "patch"
}

# Function to update version in Cargo.toml files
update_cargo_version() {
    local new_version="$1"
    local workspace_root="$2"
    
    echo "📝 Updating Cargo.toml versions to $new_version..."
    
    # Update workspace root Cargo.toml
    if [ -f "$workspace_root/Cargo.toml" ]; then
        sed -i "s/^version = \".*\"/version = \"$new_version\"/" "$workspace_root/Cargo.toml"
        echo "✅ Updated $workspace_root/Cargo.toml"
    fi
    
    # Update member crates
    find "$workspace_root" -name "Cargo.toml" -not -path "*/target/*" | while read -r cargo_file; do
        if [ "$cargo_file" != "$workspace_root/Cargo.toml" ]; then
            sed -i "s/^version = \".*\"/version = \"$new_version\"/" "$cargo_file"
            echo "✅ Updated $cargo_file"
        fi
    done
}

# Function to update CHANGELOG.md
update_changelog() {
    local new_version="$1"
    local date=$(date +%Y-%m-%d)
    
    echo "📰 Updating CHANGELOG.md..."
    
    if [ ! -f "CHANGELOG.md" ]; then
        cat > CHANGELOG.md << EOF
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [$new_version] - $date

### Added
- Initial release with benchmark framework
- Toyota Way quality management system
- Automated release pipeline

EOF
    else
        # Replace [Unreleased] with version and date
        sed -i "s/## \[Unreleased\]/## [Unreleased]\n\n## [$new_version] - $date/" CHANGELOG.md
    fi
    
    echo "✅ Updated CHANGELOG.md"
}

# Main release function
main() {
    # Ensure we're in the project root
    if [ ! -f "README.md" ] || [ ! -f "LICENSE" ]; then
        echo "❌ Error: Must run from project root directory"
        exit 1
    fi
    
    # Ensure working directory is clean
    if ! git diff-index --quiet HEAD --; then
        echo "❌ Error: Working directory not clean. Commit or stash changes first."
        git status --porcelain
        exit 1
    fi
    
    # Detect bump type if auto
    if [ "$BUMP_TYPE" = "auto" ]; then
        BUMP_TYPE=$(detect_bump_type)
        echo "🤖 Auto-detected bump type: $BUMP_TYPE"
    fi
    
    # Validate bump type
    if [[ ! "$BUMP_TYPE" =~ ^(patch|minor|major)$ ]]; then
        echo "❌ Error: Invalid bump type '$BUMP_TYPE'. Must be: patch, minor, major, or auto"
        exit 1
    fi
    
    # Get current version
    if [ -f "Cargo.toml" ]; then
        CURRENT_VERSION=$(grep "^version = " Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
    else
        echo "❌ Error: No Cargo.toml found for version detection"
        exit 1
    fi
    
    echo "📋 Current version: $CURRENT_VERSION"
    
    # Calculate new version
    IFS='.' read -r -a VERSION_PARTS <<< "$CURRENT_VERSION"
    MAJOR=${VERSION_PARTS[0]}
    MINOR=${VERSION_PARTS[1]}
    PATCH=${VERSION_PARTS[2]}
    
    case "$BUMP_TYPE" in
        major)
            MAJOR=$((MAJOR + 1))
            MINOR=0
            PATCH=0
            ;;
        minor)
            MINOR=$((MINOR + 1))
            PATCH=0
            ;;
        patch)
            PATCH=$((PATCH + 1))
            ;;
    esac
    
    NEW_VERSION="$MAJOR.$MINOR.$PATCH"
    echo "🎯 New version: $NEW_VERSION"
    
    # Confirm release
    echo ""
    echo "🚨 RELEASE CONFIRMATION"
    echo "Project: $PROJECT_NAME"
    echo "Current: $CURRENT_VERSION"
    echo "New: $NEW_VERSION ($BUMP_TYPE)"
    echo ""
    read -p "Proceed with release? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "❌ Release cancelled"
        exit 1
    fi
    
    # Run pre-release quality gates
    echo "🎯 Running mandatory quality gates..."
    make pre-release-checks || {
        echo "❌ Quality gates failed - release blocked"
        exit 1
    }
    
    # Update versions
    update_cargo_version "$NEW_VERSION" "."
    update_changelog "$NEW_VERSION"
    
    # Update lock file if exists
    if [ -f "Cargo.lock" ]; then
        cargo update --workspace --quiet
        echo "✅ Updated Cargo.lock"
    fi
    
    # Create release commit
    git add .
    git commit -m "chore(release): $NEW_VERSION

Release $NEW_VERSION with Toyota Way quality gates:
- All quality gates passed
- Zero SATD comments maintained  
- Security vulnerabilities resolved
- Test coverage ≥80% maintained
- Complexity ≤20 for all functions

🤖 Generated with [Rosetta Ruchy](https://github.com/pragmatic-ai-labs/rosetta-ruchy)"
    
    # Create git tag
    git tag -a "v$NEW_VERSION" -m "Release v$NEW_VERSION

Toyota Way Quality Metrics:
- Quality Gates: ✅ All passed
- Test Coverage: ≥80%
- Complexity: ≤20 per function
- Security: No vulnerabilities
- Technical Debt: Zero SATD

Release Type: $BUMP_TYPE
Previous: v$CURRENT_VERSION
"
    
    echo "✅ Created git tag v$NEW_VERSION"
    
    # Push to remote
    echo "📤 Pushing to remote..."
    git push origin main
    git push origin "v$NEW_VERSION"
    
    # Create GitHub release (if gh CLI available)
    if command -v gh >/dev/null 2>&1; then
        echo "🐙 Creating GitHub release..."
        gh release create "v$NEW_VERSION" \
            --title "Release v$NEW_VERSION" \
            --notes "Release v$NEW_VERSION

## Toyota Way Quality Metrics ✅

- **Quality Gates**: All mandatory gates passed
- **Test Coverage**: ≥80% maintained  
- **Complexity**: ≤20 for all functions
- **Security**: Zero vulnerabilities
- **Technical Debt**: Zero SATD comments
- **Release Type**: $BUMP_TYPE

## Changes

See [CHANGELOG.md](./CHANGELOG.md) for detailed changes.

---
🤖 Generated with [Rosetta Ruchy](https://github.com/pragmatic-ai-labs/rosetta-ruchy) Canonical Release System"
        echo "✅ GitHub release created"
    else
        echo "⚠️  gh CLI not available - GitHub release skipped"
    fi
    
    echo ""
    echo "🎉 RELEASE COMPLETE!"
    echo "Version: v$NEW_VERSION"
    echo "Quality: ✅ Toyota Way standards maintained"
    echo "GitHub: https://github.com/pragmatic-ai-labs/rosetta-ruchy/releases/tag/v$NEW_VERSION"
    echo ""
}

# Run main function
main "$@"