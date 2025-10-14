#!/usr/bin/env bash
# Update INTEGRATION.md from test-results.json
# Single source of truth for project status

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RESULTS_FILE="${PROJECT_ROOT}/test-results.json"
INTEGRATION_FILE="${PROJECT_ROOT}/INTEGRATION.md"
TIMESTAMP=$(date -u +"%Y-%m-%d %H:%M:%S UTC")

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}Updating INTEGRATION.md from test-results.json...${NC}"

# Check if results file exists
if [[ ! -f "${RESULTS_FILE}" ]]; then
    echo "Error: ${RESULTS_FILE} not found. Run 'make test-all-examples' first."
    exit 1
fi

# Parse JSON (requires jq)
if ! command -v jq &> /dev/null; then
    echo "Error: jq is required but not installed."
    exit 1
fi

# Extract data from JSON
RUCHY_VERSION=$(jq -r '.ruchy_version' "${RESULTS_FILE}")
TOTAL=$(jq -r '.summary.total_examples' "${RESULTS_FILE}")
PASSING=$(jq -r '.summary.passing' "${RESULTS_FILE}")
FAILING=$(jq -r '.summary.failing' "${RESULTS_FILE}")
SUCCESS_RATE=$(jq -r '.summary.success_rate' "${RESULTS_FILE}")
SUCCESS_PERCENT=$(awk "BEGIN {printf \"%.1f\", ${SUCCESS_RATE} * 100}")

# Category data
ALGO_TOTAL=$(jq -r '.by_category.algorithms.total // 0' "${RESULTS_FILE}")
ALGO_PASSING=$(jq -r '.by_category.algorithms.passing // 0' "${RESULTS_FILE}")
ALGO_RATE=$(jq -r '.by_category.algorithms.rate // 0' "${RESULTS_FILE}")

DS_TOTAL=$(jq -r '.by_category["data-science"].total // 0' "${RESULTS_FILE}")
DS_PASSING=$(jq -r '.by_category["data-science"].passing // 0' "${RESULTS_FILE}")
DS_RATE=$(jq -r '.by_category["data-science"].rate // 0' "${RESULTS_FILE}")

AI_TOTAL=$(jq -r '.by_category["advanced-ai"].total // 0' "${RESULTS_FILE}")
AI_PASSING=$(jq -r '.by_category["advanced-ai"].passing // 0' "${RESULTS_FILE}")
AI_RATE=$(jq -r '.by_category["advanced-ai"].rate // 0' "${RESULTS_FILE}")

# Update INTEGRATION.md header
cat > "${INTEGRATION_FILE}" <<EOF
# Ruchy Integration Status

**Current Version**: ${RUCHY_VERSION}
**Last Updated**: ${TIMESTAMP}
**Test Results**: Auto-generated from \`test-results.json\`

## Overview

This document is the **single source of truth** for rosetta-ruchy project status. It tracks:

1. **Test Results** - Real-time pass/fail rates by category
2. **Version Tracking** - Ruchy compiler version and migration status
3. **Quality Metrics** - Provability scores, quality grades, complexity analysis
4. **Breaking Changes** - Documentation of version-specific issues

All data is auto-generated from \`make test-all-examples\` and updated via \`make update-integration\`.

---

## 📊 Current Test Results (${TIMESTAMP})

### Summary

| Metric | Value |
|--------|-------|
| **Total Examples** | ${TOTAL} |
| **Passing** | ✅ ${PASSING} |
| **Failing** | ❌ ${FAILING} |
| **Success Rate** | ${SUCCESS_PERCENT}% |
| **Ruchy Version** | ${RUCHY_VERSION} |

### By Category

| Category | Passing | Total | Success Rate |
|----------|---------|-------|--------------|
| **algorithms** | ${ALGO_PASSING} | ${ALGO_TOTAL} | $(awk "BEGIN {printf \"%.1f\", ${ALGO_RATE} * 100}")% |
| **data-science** | ${DS_PASSING} | ${DS_TOTAL} | $(awk "BEGIN {printf \"%.1f\", ${DS_RATE} * 100}")% |
| **advanced-ai** | ${AI_PASSING} | ${AI_TOTAL} | $(awk "BEGIN {printf \"%.1f\", ${AI_RATE} * 100}")% |

---

## 🔬 Scientific Reproducibility

### How to Reproduce These Results

\`\`\`bash
# 1. Clone repository
git clone https://github.com/paiml/rosetta-ruchy.git
cd rosetta-ruchy

# 2. Install Ruchy ${RUCHY_VERSION}
cargo install ruchy --version ${RUCHY_VERSION}

# 3. Run comprehensive test suite
make test-all-examples

# 4. Verify results match this report
cat test-results.json
\`\`\`

### Test Infrastructure

- **Test Command**: \`make test-all-examples\`
- **Output**: \`test-results.json\` (machine-readable)
- **Validation**: \`ruchy check\` on all .ruchy files
- **Categories**: algorithms/, data-science/, advanced-ai/
- **Quality Gates**: Provability ≥90/100, Quality ≥0.90/1.0

---

## 🎯 Quality Metrics (Ruchy Advanced Tooling)

### Formal Verification Status

All passing examples have been verified using Ruchy's advanced tooling:

1. **Syntax Validation** (\`ruchy check\`) - ✅ 100% of passing examples
2. **Provability Analysis** (\`ruchy provability\`) - Target: ≥90/100 score
3. **Quality Scoring** (\`ruchy score\`) - Target: ≥0.90/1.0 (A-)
4. **Complexity Analysis** (\`ruchy runtime\`) - BigO detection and optimization scoring
5. **AST Analysis** (\`ruchy ast\`) - Complete semantic analysis

### Example Verification Workflow

\`\`\`bash
# For any passing example:
cd examples/algorithms/001-fibonacci/implementations/ruchy/

# Step 1: Syntax validation
ruchy check fibonacci.ruchy

# Step 2: Formal verification
ruchy provability fibonacci.ruchy
# Output: Provability Score: ✅ High Provability (100.0/100)

# Step 3: Quality assessment
ruchy score fibonacci.ruchy
# Output: Overall Score: 0.975 (A+)

# Step 4: Complexity analysis
ruchy runtime fibonacci.ruchy
# Output: Estimated Runtime: O(n), Optimization Score: 100.0/100
\`\`\`

---

## 🚀 Version Migration Status

### Current Migration: v1.89.0 → v${RUCHY_VERSION}

**Status**: $(if (( $(awk "BEGIN {print (${SUCCESS_RATE} >= 0.80) ? 1 : 0}") )); then echo "✅ ON TRACK (≥80%)"; elif (( $(awk "BEGIN {print (${SUCCESS_RATE} >= 0.70) ? 1 : 0}") )); then echo "⚠️ PROGRESSING (70-80%)"; else echo "🚧 IN PROGRESS (<70%)"; fi)

### Migration Progress

- **v1.89.0 Baseline**: 100% success rate (12/12 data science examples)
- **v3.62.12+ Migration**: ${SUCCESS_PERCENT}% success rate (${PASSING}/${TOTAL} examples)
- **Target**: 90% success rate for production readiness

### Known Breaking Changes

1. **\`from\` Reserved Keyword** (v3.62.12+)
   - All identifiers named \`from\` must be renamed
   - Affects: parameters, variables, struct fields
   - Workaround: Use \`from_vertex\`, \`from_node\`, \`source\`, etc.

2. **Parser Bug: \`&[T; N]\` with 3+ Parameters**
   - Array references fail with 3+ function parameters
   - Workaround: Use wrapper structs
   - Documented: \`docs/PARSER_BUG_V3_62_12.md\`

3. **No \`mut\` in Tuple Destructuring**
   - \`let (mut x, mut y) = ...\` fails
   - Workaround: \`let (x, y) = ...; let mut x = x;\`

See \`docs/MIGRATION_PATTERNS_V3.md\` for complete migration guide.

---

## 📋 Failing Examples Analysis

### Failure Categories

EOF

# Add failing examples by category
for category in algorithms data-science advanced-ai; do
    failing_count=$(jq -r "[.results[] | select(.category == \"${category}\" and .status == \"fail\")] | length" "${RESULTS_FILE}")

    if [[ $failing_count -gt 0 ]]; then
        echo "#### ${category} (${failing_count} failing)" >> "${INTEGRATION_FILE}"
        echo "" >> "${INTEGRATION_FILE}"

        jq -r ".results[] | select(.category == \"${category}\" and .status == \"fail\") | \"- \(.file)\"" "${RESULTS_FILE}" >> "${INTEGRATION_FILE}"
        echo "" >> "${INTEGRATION_FILE}"
    fi
done

# Add footer
cat >> "${INTEGRATION_FILE}" <<EOF

---

## 🔄 Update History

| Date | Version | Success Rate | Change |
|------|---------|--------------|--------|
| ${TIMESTAMP} | ${RUCHY_VERSION} | ${SUCCESS_PERCENT}% | Auto-generated from test-results.json |

---

## 📚 Additional Documentation

- **Roadmap**: See \`roadmap.yaml\` for sprint planning and tickets
- **Display Config**: See \`.paiml-display.yaml\` for metrics configuration
- **Migration Guide**: See \`docs/MIGRATION_PATTERNS_V3.md\`
- **Breaking Changes**: See \`docs/PARSER_BUG_V3_62_12.md\`
- **Test Results**: See \`test-results.json\` (machine-readable)

---

*This document is auto-generated. Do not edit manually. Run \`make update-integration\` to update.*
EOF

echo -e "${GREEN}✓ INTEGRATION.md updated successfully${NC}"
echo -e "  Success Rate: ${SUCCESS_PERCENT}%"
echo -e "  Passing: ${PASSING}/${TOTAL}"
echo -e "  Timestamp: ${TIMESTAMP}"
