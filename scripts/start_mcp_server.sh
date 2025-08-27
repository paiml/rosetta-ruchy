#!/bin/bash
# Sprint 38: MCP Server Deployment Script
# Real-time quality monitoring for Ruchy projects

echo "🚀 Starting Ruchy MCP Quality Monitoring Server"
echo "=============================================="
echo ""

# Configuration
SERVER_NAME="rosetta-ruchy-monitor"
MIN_SCORE="0.85"
MAX_COMPLEXITY="20"
TIMEOUT="3600"

# Check if MCP server is available
if ! ruchy mcp --help >/dev/null 2>&1; then
    echo "❌ Error: ruchy mcp command not available"
    exit 1
fi

echo "Configuration:"
echo "  Server Name: $SERVER_NAME"
echo "  Min Quality Score: $MIN_SCORE"
echo "  Max Complexity: $MAX_COMPLEXITY"
echo "  Session Timeout: ${TIMEOUT}s"
echo ""

# Start the server
echo "Starting server..."
echo "Press Ctrl+C to stop"
echo ""

ruchy mcp \
    --name "$SERVER_NAME" \
    --min-score "$MIN_SCORE" \
    --max-complexity "$MAX_COMPLEXITY" \
    --timeout "$TIMEOUT" \
    --streaming \
    --verbose