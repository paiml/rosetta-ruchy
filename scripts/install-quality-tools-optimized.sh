#!/usr/bin/env bash
#
# install-quality-tools-optimized.sh - Comprehensive quality tooling installer (OPTIMIZED)
#
# Sprint 53 GREEN Phase: Optimized version with Sprint 49/51/52 patterns
# - Tool availability caching (TOOL_CACHE)
# - Installation status caching (INSTALL_CACHE)
# - Command path caching (COMMAND_CACHE)
# - Performance tracking
# - Enhanced error messages
# - Progress reporting
#
# This script installs all required quality tools for rosetta-ruchy project:
# - Ruchy (v3.88.0) - Core language toolchain
# - bashrs (v1.0.0-rc1) - Bash→Rust transpiler and linter
# - pmat (v2.192.0) - Roadmap and quality management
# - shellcheck (latest) - Shell script linting
# - Additional quality tools
#
# Uses direct downloads instead of cargo install to work around network restrictions.
#

set -euo pipefail

# Sprint 53 Optimization: Performance tracking (Sprint 49/51/52 pattern)
START_TIME=$(date +%s.%N)

# Sprint 53 Optimization: Caching mechanisms
declare -A TOOL_CACHE
declare -A INSTALL_CACHE
declare -A COMMAND_CACHE

# Cache statistics
CACHE_TOOL_HITS=0
CACHE_INSTALL_HITS=0
CACHE_COMMAND_HITS=0

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;36m'
NC='\033[0m' # No Color

# Tool versions
RUCHY_VERSION="3.88.0"
BASHRS_VERSION="1.0.0-rc1"
PMAT_VERSION="2.192.0"
SHELLCHECK_VERSION="latest"

# Installation directory
INSTALL_DIR="${HOME}/.local/bin"
TEMP_DIR="/tmp/rosetta-tools-install"

# Create directories
mkdir -p "$INSTALL_DIR"
mkdir -p "$TEMP_DIR"

# Ensure INSTALL_DIR is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "${YELLOW}⚠️  Adding $INSTALL_DIR to PATH${NC}"
    export PATH="$INSTALL_DIR:$PATH"
    echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "${HOME}/.bashrc"
fi

echo "================================================================"
echo "Rosetta-Ruchy Quality Tools Installer (OPTIMIZED)"
echo "Sprint 53: Enhanced with caching and progress reporting"
echo "================================================================"
echo ""
echo "Installation directory: $INSTALL_DIR"
echo "Temporary directory: $TEMP_DIR"
echo ""

# Sprint 53 Optimization: Cache helper functions (Sprint 49/51/52 pattern)

# Check if tool is available (cached)
check_tool_available() {
    local tool="$1"

    if [[ -n "${TOOL_CACHE[$tool]:-}" ]]; then
        CACHE_TOOL_HITS=$((CACHE_TOOL_HITS + 1))
        return "${TOOL_CACHE[$tool]}"
    fi

    if command -v "$tool" &>/dev/null; then
        TOOL_CACHE["$tool"]=0
        return 0
    else
        TOOL_CACHE["$tool"]=1
        return 1
    fi
}

# Check installation status (cached)
check_installation_status() {
    local tool="$1"

    if [[ -n "${INSTALL_CACHE[$tool]:-}" ]]; then
        CACHE_INSTALL_HITS=$((CACHE_INSTALL_HITS + 1))
        return "${INSTALL_CACHE[$tool]}"
    fi

    # Check if tool is available after installation attempt
    if check_tool_available "$tool"; then
        INSTALL_CACHE["$tool"]=0
        return 0
    else
        INSTALL_CACHE["$tool"]=1
        return 1
    fi
}

# Get command path (cached)
get_command_path() {
    local cmd="$1"

    if [[ -n "${COMMAND_CACHE[$cmd]:-}" ]]; then
        CACHE_COMMAND_HITS=$((CACHE_COMMAND_HITS + 1))
        echo "${COMMAND_CACHE[$cmd]}"
        return 0
    fi

    local path
    path=$(command -v "$cmd" 2>/dev/null)
    if [ -n "$path" ]; then
        COMMAND_CACHE["$cmd"]="$path"
        echo "$path"
        return 0
    fi
    return 1
}

# Function to check if tool is already installed (Sprint 53 Optimization: uses caching)
check_installed() {
    local tool=$1
    local version=$2

    # Sprint 53 Optimization: Use cached tool check
    if check_tool_available "$tool"; then
        local installed_version=$("$tool" --version 2>&1 | head -1 || echo "unknown")
        echo -e "${GREEN}✓${NC} $tool already installed: $installed_version"
        return 0
    else
        echo -e "${YELLOW}✗${NC} $tool not found, will install"
        echo -e "${BLUE}   └─${NC} Target version: $version"
        return 1
    fi
}

# Function to install Ruchy from GitHub releases
install_ruchy() {
    echo ""
    echo "================================================================"
    echo "Installing Ruchy v${RUCHY_VERSION}"
    echo "================================================================"

    if check_installed "ruchy" "$RUCHY_VERSION"; then
        return 0
    fi

    cd "$TEMP_DIR"

    # Try to download from GitHub releases (hypothetical URL structure)
    local ruchy_url="https://github.com/ruchy-lang/ruchy/releases/download/v${RUCHY_VERSION}/ruchy-linux-x86_64"

    echo "Attempting to download from: $ruchy_url"
    if curl -L -f -o ruchy "$ruchy_url" 2>/dev/null; then
        chmod +x ruchy
        mv ruchy "$INSTALL_DIR/"
        echo -e "${GREEN}✅ Ruchy v${RUCHY_VERSION} installed successfully${NC}"
        "$INSTALL_DIR/ruchy" --version
        return 0
    fi

    # Fallback: Try cargo install with network restrictions workaround
    echo -e "${YELLOW}⚠️  GitHub release not found, trying cargo install...${NC}"
    if cargo install ruchy --version "$RUCHY_VERSION" 2>/dev/null; then
        echo -e "${GREEN}✅ Ruchy v${RUCHY_VERSION} installed via cargo${NC}"
        return 0
    fi

    # Final fallback: Try latest version
    echo -e "${YELLOW}⚠️  Specific version unavailable, trying latest...${NC}"
    if cargo install ruchy 2>/dev/null; then
        echo -e "${YELLOW}⚠️  Installed latest Ruchy version (not ${RUCHY_VERSION})${NC}"
        return 0
    fi

    echo -e "${RED}❌ Failed to install Ruchy${NC}"
    echo "Please install manually:"
    echo "  cargo install ruchy --version ${RUCHY_VERSION}"
    return 1
}

# Function to install bashrs from GitHub releases
install_bashrs() {
    echo ""
    echo "================================================================"
    echo "Installing bashrs v${BASHRS_VERSION}"
    echo "================================================================"

    if check_installed "bashrs" "$BASHRS_VERSION"; then
        return 0
    fi

    cd "$TEMP_DIR"

    # Try to download from GitHub releases
    local bashrs_url="https://github.com/ruchy-lang/bashrs/releases/download/v${BASHRS_VERSION}/bashrs-linux-x86_64"

    echo "Attempting to download from: $bashrs_url"
    if curl -L -f -o bashrs "$bashrs_url" 2>/dev/null; then
        chmod +x bashrs
        mv bashrs "$INSTALL_DIR/"
        echo -e "${GREEN}✅ bashrs v${BASHRS_VERSION} installed successfully${NC}"
        "$INSTALL_DIR/bashrs" --version
        return 0
    fi

    # Fallback: Try cargo install
    echo -e "${YELLOW}⚠️  GitHub release not found, trying cargo install...${NC}"
    if cargo install bashrs --version "$BASHRS_VERSION" 2>/dev/null; then
        echo -e "${GREEN}✅ bashrs v${BASHRS_VERSION} installed via cargo${NC}"
        return 0
    fi

    echo -e "${YELLOW}⚠️  bashrs installation failed, will use alternative bash linting${NC}"
    return 1
}

# Function to install pmat from GitHub
install_pmat() {
    echo ""
    echo "================================================================"
    echo "Installing pmat v${PMAT_VERSION}"
    echo "================================================================"

    if check_installed "pmat" "$PMAT_VERSION"; then
        return 0
    fi

    cd "$TEMP_DIR"

    # Try to install from GitHub (git repository)
    echo "Attempting to install from GitHub repository..."
    if cargo install --git https://github.com/paiml/paiml-mcp-agent-toolkit.git pmat 2>/dev/null; then
        echo -e "${GREEN}✅ pmat installed successfully${NC}"
        return 0
    fi

    echo -e "${YELLOW}⚠️  pmat installation failed, will use pmat-style validation scripts${NC}"
    echo "Using: scripts/pmat-style-validation.sh as alternative"
    return 1
}

# Function to install shellcheck
install_shellcheck() {
    echo ""
    echo "================================================================"
    echo "Installing shellcheck"
    echo "================================================================"

    if check_installed "shellcheck" "$SHELLCHECK_VERSION"; then
        return 0
    fi

    # Try apt-get first (Ubuntu/Debian)
    if command -v apt-get &>/dev/null; then
        echo "Installing via apt-get..."
        if sudo apt-get update && sudo apt-get install -y shellcheck 2>/dev/null; then
            echo -e "${GREEN}✅ shellcheck installed successfully${NC}"
            return 0
        fi
    fi

    # Try downloading from GitHub releases
    cd "$TEMP_DIR"
    local shellcheck_version="v0.9.0"
    local shellcheck_url="https://github.com/koalaman/shellcheck/releases/download/${shellcheck_version}/shellcheck-${shellcheck_version}.linux.x86_64.tar.xz"

    echo "Downloading from: $shellcheck_url"
    if curl -L -f -o shellcheck.tar.xz "$shellcheck_url" 2>/dev/null; then
        tar -xf shellcheck.tar.xz
        mv "shellcheck-${shellcheck_version}/shellcheck" "$INSTALL_DIR/"
        chmod +x "$INSTALL_DIR/shellcheck"
        echo -e "${GREEN}✅ shellcheck installed successfully${NC}"
        return 0
    fi

    echo -e "${YELLOW}⚠️  shellcheck installation failed${NC}"
    return 1
}

# Function to install additional quality tools
install_additional_tools() {
    echo ""
    echo "================================================================"
    echo "Installing Additional Quality Tools"
    echo "================================================================"

    # Try to install useful tools (non-critical)
    local tools=(
        "cargo-audit"      # Security auditing
        "cargo-outdated"   # Dependency version checking
        "cargo-tarpaulin"  # Code coverage
    )

    for tool in "${tools[@]}"; do
        if command -v "$tool" &>/dev/null; then
            echo -e "${GREEN}✓${NC} $tool already installed"
        else
            echo "Attempting to install $tool..."
            if cargo install "$tool" 2>/dev/null; then
                echo -e "${GREEN}✅ $tool installed${NC}"
            else
                echo -e "${YELLOW}⚠️  $tool installation failed (non-critical)${NC}"
            fi
        fi
    done
}

# Main installation workflow
main() {
    echo "Starting installation..."
    echo ""

    # Sprint 53 Optimization: Enhanced progress reporting
    echo -e "${BLUE}Installing 4 core tools...${NC}"
    echo ""

    # Core tools (critical)
    echo -e "${BLUE}[1/4]${NC} Installing ruchy..."
    install_ruchy

    echo -e "${BLUE}[2/4]${NC} Installing bashrs..."
    install_bashrs

    echo -e "${BLUE}[3/4]${NC} Installing pmat..."
    install_pmat

    echo -e "${BLUE}[4/4]${NC} Installing shellcheck..."
    install_shellcheck

    # Additional tools (nice to have)
    install_additional_tools

    echo ""
    echo "================================================================"
    echo "Installation Summary"
    echo "================================================================"
    echo ""

    # Check what's installed
    local installed=0
    local failed=0

    for tool in ruchy bashrs pmat shellcheck cargo-audit; do
        if command -v "$tool" &>/dev/null; then
            echo -e "${GREEN}✓${NC} $tool: $(command -v $tool)"
            ((installed++))
        else
            echo -e "${RED}✗${NC} $tool: NOT INSTALLED"
            ((failed++))
        fi
    done

    echo ""
    echo "Installed: $installed"
    echo "Failed: $failed"
    echo ""

    if [ $failed -gt 0 ]; then
        echo -e "${YELLOW}⚠️  Some tools failed to install${NC}"
        echo "Alternative validation scripts are available in scripts/ directory:"
        echo "  - scripts/pmat-style-validation.sh (replaces pmat)"
        echo "  - scripts/test-documentation.sh (quality checks)"
        echo "  - scripts/validate-entire-book.sh (documentation validation)"
    fi

    echo ""
    echo -e "${GREEN}✅ Installation complete!${NC}"
    echo ""

    # Sprint 53 Optimization: Performance tracking and cache statistics
    END_TIME=$(date +%s.%N)
    EXECUTION_TIME=$(awk "BEGIN {printf \"%.3f\", $END_TIME - $START_TIME}")

    echo -e "${BLUE}Performance Metrics:${NC}"
    echo "  Execution time: ${EXECUTION_TIME}s"
    echo ""
    echo -e "${BLUE}Optimization Statistics:${NC}"
    echo "  Tool cache hits: $CACHE_TOOL_HITS"
    echo "  Installation cache hits: $CACHE_INSTALL_HITS"
    echo "  Command cache hits: $CACHE_COMMAND_HITS"
    local total_cache_hits=$((CACHE_TOOL_HITS + CACHE_INSTALL_HITS + CACHE_COMMAND_HITS))
    echo "  Total cache hits: $total_cache_hits"
    echo ""

    echo "Next steps:"
    echo "  1. Reload shell: source ~/.bashrc"
    echo "  2. Verify installation: make verify-tools"
    echo "  3. Run quality gates: make quality-gate"
    echo ""

    # Cleanup
    rm -rf "$TEMP_DIR"
}

# Run main installation
main "$@"
