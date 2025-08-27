#!/bin/bash
# Ruchy Error Diagnosis Tool
# Sprint 39: Advanced debugging patterns

set -euo pipefail

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if file is provided
if [ $# -eq 0 ]; then
    echo "Usage: $0 <ruchy_file>"
    exit 1
fi

FILE="$1"

echo "🔍 Ruchy Error Diagnosis Tool"
echo "=============================="
echo ""
echo "Analyzing: $FILE"
echo ""

# Function to diagnose error
diagnose_error() {
    local error="$1"
    
    if [[ "$error" == *"var not found"* ]] || [[ "$error" == *"cannot find value \`var\`"* ]]; then
        echo -e "${YELLOW}⚠️ Diagnosis: var keyword issue${NC}"
        echo "  Problem: 'var' is not properly transpiled to Rust"
        echo "  Solution: Replace with recursive functions or let bindings"
        echo "  Example:"
        echo "    ❌ var x = 0;"
        echo "    ✅ fun loop(x: i32) -> i32 { if x < 10 { loop(x+1) } else { x } }"
        return 0
    fi
    
    if [[ "$error" == *"use of moved value"* ]]; then
        echo -e "${YELLOW}⚠️ Diagnosis: Ownership issue${NC}"
        echo "  Problem: Value moved and then used again"
        echo "  Solution: Clone the value or restructure logic"
        echo "  Example:"
        echo "    ❌ let x = vec![1,2,3]; process(x); use(x);"
        echo "    ✅ let x = vec![1,2,3]; process(x.clone()); use(x);"
        return 0
    fi
    
    if [[ "$error" == *"Expected RightParen"* ]]; then
        echo -e "${YELLOW}⚠️ Diagnosis: Syntax error${NC}"
        echo "  Problem: Mismatched parentheses or braces"
        echo "  Solution: Check bracket matching"
        echo "  Tip: Count opening and closing brackets"
        return 0
    fi
    
    if [[ "$error" == *"cannot infer type"* ]]; then
        echo -e "${YELLOW}⚠️ Diagnosis: Type inference failure${NC}"
        echo "  Problem: Compiler cannot determine type"
        echo "  Solution: Add explicit type annotations"
        echo "  Example:"
        echo "    ❌ let x = Vec::new();"
        echo "    ✅ let x: Vec<i32> = Vec::new();"
        return 0
    fi
    
    if [[ "$error" == *"not yet implemented"* ]]; then
        echo -e "${RED}❌ Diagnosis: Feature not implemented${NC}"
        echo "  Problem: This Ruchy feature is not yet available"
        echo "  Solution: Use alternative approaches or wait for update"
        return 0
    fi
    
    return 1
}

# Step 1: Syntax Check
echo "1. Syntax Check"
echo "---------------"
syntax_output=$(ruchy check "$FILE" 2>&1) || true

if [[ "$syntax_output" == *"Syntax is valid"* ]]; then
    echo -e "${GREEN}✅ Syntax valid${NC}"
else
    echo -e "${RED}❌ Syntax error detected${NC}"
    echo "$syntax_output" | head -5
    diagnose_error "$syntax_output" || echo "  No specific diagnosis available"
fi
echo ""

# Step 2: Provability Check
echo "2. Provability Analysis"
echo "----------------------"
provability_output=$(ruchy provability "$FILE" 2>&1) || true

if [[ "$provability_output" == *"Score:"* ]]; then
    score=$(echo "$provability_output" | grep -oP 'Score: \K[0-9.]+' || echo "0")
    echo -e "${GREEN}✅ Provability score: $score${NC}"
else
    echo -e "${YELLOW}⚠️ Could not determine provability${NC}"
fi
echo ""

# Step 3: Runtime Complexity
echo "3. Runtime Complexity"
echo "--------------------"
runtime_output=$(ruchy runtime "$FILE" 2>&1) || true

if [[ "$runtime_output" == *"Performance"* ]] || [[ "$runtime_output" == *"Complexity"* ]]; then
    echo -e "${GREEN}✅ Complexity analysis available${NC}"
    echo "$runtime_output" | grep -E "(O\(|Complexity)" | head -3 || true
else
    echo -e "${YELLOW}⚠️ No complexity information${NC}"
fi
echo ""

# Step 4: Quality Score
echo "4. Quality Score"
echo "---------------"
score_output=$(ruchy score "$FILE" 2>&1) || true

if [[ "$score_output" == *"Score:"* ]]; then
    quality=$(echo "$score_output" | grep -oP 'Score: \K[0-9.]+' || echo "0")
    echo -e "${GREEN}✅ Quality score: $quality${NC}"
    
    # Provide grade
    if (( $(echo "$quality >= 0.95" | bc -l) )); then
        echo "  Grade: A+"
    elif (( $(echo "$quality >= 0.90" | bc -l) )); then
        echo "  Grade: A"
    elif (( $(echo "$quality >= 0.85" | bc -l) )); then
        echo "  Grade: B+"
    elif (( $(echo "$quality >= 0.80" | bc -l) )); then
        echo "  Grade: B"
    else
        echo "  Grade: Below B (needs improvement)"
    fi
else
    echo -e "${YELLOW}⚠️ Could not determine quality score${NC}"
fi
echo ""

# Step 5: Compilation Test
echo "5. Compilation Test"
echo "------------------"
compile_output=$(ruchy compile "$FILE" --output /tmp/test_binary 2>&1) || true

if [[ "$compile_output" == *"Successfully compiled"* ]]; then
    echo -e "${GREEN}✅ Compilation successful${NC}"
    rm -f /tmp/test_binary
else
    echo -e "${RED}❌ Compilation failed${NC}"
    echo "$compile_output" | head -10
    echo ""
    echo "Analyzing compilation errors..."
    diagnose_error "$compile_output" || echo "  Multiple issues detected - see details above"
fi
echo ""

# Step 6: Parse Tree
echo "6. Parse Tree Analysis"
echo "---------------------"
parse_output=$(ruchy parse "$FILE" 2>&1) || true

if [[ "$parse_output" == *"Expr"* ]]; then
    echo -e "${GREEN}✅ Parse tree generated${NC}"
    echo "  Functions found:"
    echo "$parse_output" | grep -oP 'name: "\K[^"]+' | head -5 | sed 's/^/    - /'
else
    echo -e "${RED}❌ Parse failed${NC}"
    diagnose_error "$parse_output" || true
fi
echo ""

# Summary
echo "=============================="
echo "Summary"
echo "=============================="

total_checks=6
passed_checks=0

[[ "$syntax_output" == *"Syntax is valid"* ]] && ((passed_checks++))
[[ "$provability_output" == *"Score:"* ]] && ((passed_checks++))
[[ "$runtime_output" == *"Performance"* ]] || [[ "$runtime_output" == *"Complexity"* ]] && ((passed_checks++))
[[ "$score_output" == *"Score:"* ]] && ((passed_checks++))
[[ "$compile_output" == *"Successfully compiled"* ]] && ((passed_checks++))
[[ "$parse_output" == *"Expr"* ]] && ((passed_checks++))

echo "Checks passed: $passed_checks/$total_checks"

if [ $passed_checks -eq $total_checks ]; then
    echo -e "${GREEN}✅ All checks passed! File is production ready.${NC}"
elif [ $passed_checks -ge 4 ]; then
    echo -e "${YELLOW}⚠️ Most checks passed. Minor issues to address.${NC}"
else
    echo -e "${RED}❌ Multiple issues detected. Review diagnoses above.${NC}"
fi

echo ""
echo "Diagnosis complete!"