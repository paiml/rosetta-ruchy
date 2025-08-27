#!/bin/bash
# Sprint 37: Test advanced Ruchy tools systematically

echo "=== Testing Advanced Ruchy Tools (Sprint 37) ==="
echo ""

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test file
TEST_FILE="examples/advanced-ai/001-deep-learning/implementations/ruchy/deep_learning.ruchy"

# Counter for working tools
WORKING=0
TOTAL=0

# Function to test a tool
test_tool() {
    local cmd="$1"
    local name="$2"
    TOTAL=$((TOTAL + 1))
    
    echo -n "Testing $name... "
    
    # Run command and check output
    output=$(ruchy $cmd $TEST_FILE 2>&1)
    
    if [[ "$output" == *"not yet implemented"* ]]; then
        echo -e "${RED}❌ Not implemented${NC}"
    elif [[ "$output" == *"error"* ]] || [[ "$output" == *"Error"* ]]; then
        echo -e "${YELLOW}⚠️ Error: ${output:0:50}${NC}"
    else
        echo -e "${GREEN}✅ Working${NC}"
        WORKING=$((WORKING + 1))
    fi
}

# Test each advanced tool
echo "Phase 1: Documentation & Analysis"
test_tool "doc" "ruchy doc"
test_tool "bench" "ruchy bench"
test_tool "optimize" "ruchy optimize"

echo ""
echo "Phase 2: Compilation Targets"
test_tool "compile" "ruchy compile"
test_tool "transpile" "ruchy transpile"
test_tool "wasm" "ruchy wasm"

echo ""
echo "Phase 3: Advanced Debugging"
test_tool "actor:observe" "ruchy actor:observe"
test_tool "dataflow:debug" "ruchy dataflow:debug"

echo ""
echo "Phase 4: Interactive Tools"
test_tool "prove --help" "ruchy prove (interactive)"

# MCP server test (special case)
echo ""
echo -n "Testing ruchy mcp (server)... "
mcp_help=$(ruchy mcp --help 2>&1)
if [[ "$mcp_help" == *"Start MCP server"* ]]; then
    echo -e "${GREEN}✅ Available${NC}"
    WORKING=$((WORKING + 1))
else
    echo -e "${RED}❌ Not available${NC}"
fi
TOTAL=$((TOTAL + 1))

# Summary
echo ""
echo "==================================="
echo "Summary: $WORKING/$TOTAL tools available"
PERCENTAGE=$((WORKING * 100 / TOTAL))
echo "Tool availability: ${PERCENTAGE}%"

if [ $WORKING -eq $TOTAL ]; then
    echo -e "${GREEN}✅ All tools working!${NC}"
elif [ $WORKING -gt $((TOTAL / 2)) ]; then
    echo -e "${YELLOW}⚠️ Some tools need implementation${NC}"
else
    echo -e "${RED}❌ Most tools not yet implemented${NC}"
fi