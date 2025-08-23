#!/bin/bash
# Rosetta Ruchy MCP Server Installation Script
# Downloads and installs the latest release for your platform

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
INSTALL_DIR="${HOME}/.local/bin"
VERSION="v1.0.0"
BINARY_NAME="rosetta-ruchy-mcp"

# Platform detection
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case $OS in
    "linux")
        case $ARCH in
            "x86_64")
                PLATFORM="x86_64-unknown-linux-gnu"
                ARCHIVE_EXT="tar.gz"
                ;;
            *)
                echo -e "${RED}Error: Unsupported architecture $ARCH for Linux${NC}"
                exit 1
                ;;
        esac
        ;;
    "darwin")
        case $ARCH in
            "x86_64")
                PLATFORM="x86_64-apple-darwin"
                ARCHIVE_EXT="tar.gz"
                ;;
            "arm64")
                PLATFORM="aarch64-apple-darwin"
                ARCHIVE_EXT="tar.gz"
                ;;
            *)
                echo -e "${RED}Error: Unsupported architecture $ARCH for macOS${NC}"
                exit 1
                ;;
        esac
        ;;
    *)
        echo -e "${RED}Error: Unsupported operating system $OS${NC}"
        echo "Please download the appropriate binary manually from:"
        echo "https://github.com/paiml/rosetta-ruchy/releases"
        exit 1
        ;;
esac

echo -e "${BLUE}Rosetta Ruchy MCP Server Installer${NC}"
echo "=================================="
echo "Platform: $OS-$ARCH ($PLATFORM)"
echo "Version: $VERSION"
echo "Install directory: $INSTALL_DIR"
echo ""

# Create install directory
mkdir -p "$INSTALL_DIR"

# Download URL
ARCHIVE_NAME="${BINARY_NAME}-${PLATFORM}.${ARCHIVE_EXT}"
DOWNLOAD_URL="https://github.com/paiml/rosetta-ruchy/releases/download/${VERSION}/${ARCHIVE_NAME}"

echo -e "${YELLOW}Downloading $ARCHIVE_NAME...${NC}"
TEMP_DIR=$(mktemp -d)
cd "$TEMP_DIR"

if command -v curl >/dev/null 2>&1; then
    curl -fsSL "$DOWNLOAD_URL" -o "$ARCHIVE_NAME"
elif command -v wget >/dev/null 2>&1; then
    wget -q "$DOWNLOAD_URL" -O "$ARCHIVE_NAME"
else
    echo -e "${RED}Error: curl or wget is required${NC}"
    exit 1
fi

echo -e "${YELLOW}Extracting archive...${NC}"
tar -xzf "$ARCHIVE_NAME"

echo -e "${YELLOW}Installing binary...${NC}"
cp "$BINARY_NAME" "$INSTALL_DIR/"
chmod +x "$INSTALL_DIR/$BINARY_NAME"

# Cleanup
cd - > /dev/null
rm -rf "$TEMP_DIR"

echo -e "${GREEN}✅ Installation complete!${NC}"
echo ""
echo "The binary has been installed to: $INSTALL_DIR/$BINARY_NAME"
echo ""

# Check if install dir is in PATH
if [[ ":$PATH:" == *":$INSTALL_DIR:"* ]]; then
    echo -e "${GREEN}✅ $INSTALL_DIR is already in your PATH${NC}"
else
    echo -e "${YELLOW}⚠️  $INSTALL_DIR is not in your PATH${NC}"
    echo "Add this to your shell profile (.bashrc, .zshrc, etc.):"
    echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
    echo ""
fi

echo "Usage:"
echo "  $BINARY_NAME --help                    # Show help"
echo "  $BINARY_NAME --host 127.0.0.1 --port 8080  # Start MCP server"
echo ""
echo "API Example:"
echo "  curl -X POST http://localhost:8080/api/v1/translate \\"
echo "    -H \"Content-Type: application/json\" \\"
echo "    -d '{\"source_code\": \"def hello(): print(\\\"Hello!\\\")\", \"source_language\": \"python\"}'"
echo ""
echo -e "${GREEN}🚀 Happy translating!${NC}"