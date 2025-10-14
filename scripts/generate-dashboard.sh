#!/usr/bin/env bash
# Generate HTML Dashboard from test-results.json
# Uses .paiml-display.yaml configuration

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RESULTS_FILE="${PROJECT_ROOT}/test-results.json"
OUTPUT_FILE="${PROJECT_ROOT}/reports/dashboard.html"
TIMESTAMP=$(date -u +"%Y-%m-%d %H:%M:%S UTC")

echo "🎨 Generating HTML Dashboard..."

# Check if results file exists
if [[ ! -f "${RESULTS_FILE}" ]]; then
    echo "Error: ${RESULTS_FILE} not found. Run 'make test-all-examples' first."
    exit 1
fi

# Create reports directory
mkdir -p "${PROJECT_ROOT}/reports"

# Parse test results
if ! command -v jq &> /dev/null; then
    echo "Error: jq is required but not installed"
    exit 1
fi

# Extract data
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

# Generate HTML
cat > "${OUTPUT_FILE}" <<'HTML_START'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Rosetta Ruchy - Quality Dashboard</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            padding: 20px;
            color: #333;
        }
        .container {
            max-width: 1200px;
            margin: 0 auto;
        }
        .header {
            background: white;
            border-radius: 12px;
            padding: 30px;
            margin-bottom: 20px;
            box-shadow: 0 10px 40px rgba(0,0,0,0.1);
        }
        .header h1 {
            font-size: 2.5rem;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            margin-bottom: 10px;
        }
        .header .subtitle {
            color: #666;
            font-size: 1.1rem;
        }
        .stats-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 20px;
            margin-bottom: 20px;
        }
        .stat-card {
            background: white;
            border-radius: 12px;
            padding: 25px;
            box-shadow: 0 10px 40px rgba(0,0,0,0.1);
            transition: transform 0.3s ease;
        }
        .stat-card:hover {
            transform: translateY(-5px);
        }
        .stat-card .label {
            color: #666;
            font-size: 0.9rem;
            text-transform: uppercase;
            letter-spacing: 1px;
            margin-bottom: 10px;
        }
        .stat-card .value {
            font-size: 3rem;
            font-weight: bold;
            color: #333;
        }
        .stat-card .subvalue {
            color: #999;
            font-size: 1rem;
            margin-top: 5px;
        }
        .success-rate-chart {
            background: white;
            border-radius: 12px;
            padding: 30px;
            margin-bottom: 20px;
            box-shadow: 0 10px 40px rgba(0,0,0,0.1);
        }
        .chart-title {
            font-size: 1.5rem;
            margin-bottom: 20px;
            color: #333;
        }
        .progress-bar {
            background: #f0f0f0;
            border-radius: 10px;
            height: 40px;
            overflow: hidden;
            margin-bottom: 15px;
        }
        .progress-fill {
            height: 100%;
            transition: width 1s ease;
            display: flex;
            align-items: center;
            padding-left: 15px;
            color: white;
            font-weight: bold;
        }
        .category-card {
            background: white;
            border-radius: 12px;
            padding: 25px;
            margin-bottom: 20px;
            box-shadow: 0 10px 40px rgba(0,0,0,0.1);
        }
        .category-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 15px;
        }
        .category-name {
            font-size: 1.3rem;
            font-weight: bold;
            color: #333;
        }
        .category-stats {
            color: #666;
        }
        .footer {
            text-align: center;
            color: white;
            margin-top: 40px;
            padding: 20px;
        }
        .badge {
            display: inline-block;
            padding: 5px 12px;
            border-radius: 20px;
            font-size: 0.85rem;
            font-weight: bold;
            margin-left: 10px;
        }
        .badge-success { background: #10b981; color: white; }
        .badge-warning { background: #f59e0b; color: white; }
        .badge-danger { background: #ef4444; color: white; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🌸 Rosetta Ruchy</h1>
            <p class="subtitle">Quality Dashboard - Test-Driven Scientific Validation</p>
        </div>
HTML_START

# Add dynamic content
cat >> "${OUTPUT_FILE}" <<HTML_DYNAMIC
        <div class="stats-grid">
            <div class="stat-card">
                <div class="label">Success Rate</div>
                <div class="value">${SUCCESS_PERCENT}%</div>
                <div class="subvalue">Target: 90%</div>
            </div>
            <div class="stat-card">
                <div class="label">Passing Examples</div>
                <div class="value">${PASSING}</div>
                <div class="subvalue">out of ${TOTAL} total</div>
            </div>
            <div class="stat-card">
                <div class="label">Failing Examples</div>
                <div class="value">${FAILING}</div>
                <div class="subvalue">need attention</div>
            </div>
            <div class="stat-card">
                <div class="label">Ruchy Version</div>
                <div class="value" style="font-size: 2rem;">${RUCHY_VERSION}</div>
                <div class="subvalue">compiler</div>
            </div>
        </div>

        <div class="success-rate-chart">
            <h2 class="chart-title">Overall Success Rate</h2>
            <div class="progress-bar">
                <div class="progress-fill" style="width: ${SUCCESS_PERCENT}%; background: $(if (( $(echo "${SUCCESS_RATE} >= 0.90" | bc -l) )); then echo "linear-gradient(135deg, #10b981 0%, #059669 100%)"; elif (( $(echo "${SUCCESS_RATE} >= 0.80" | bc -l) )); then echo "linear-gradient(135deg, #3b82f6 0%, #2563eb 100%)"; elif (( $(echo "${SUCCESS_RATE} >= 0.70" | bc -l) )); then echo "linear-gradient(135deg, #f59e0b 0%, #d97706 100%)"; else echo "linear-gradient(135deg, #ef4444 0%, #dc2626 100%)"; fi);">
                    ${SUCCESS_PERCENT}%
                </div>
            </div>
            <p style="color: #666; font-size: 0.9rem;">Last updated: ${TIMESTAMP}</p>
        </div>

        <div class="category-card">
            <div class="category-header">
                <span class="category-name">🧮 Algorithms</span>
                <span class="category-stats">${ALGO_PASSING}/${ALGO_TOTAL} <span class="badge $(if (( $(echo "${ALGO_RATE} >= 0.80" | bc -l) )); then echo "badge-success"; elif (( $(echo "${ALGO_RATE} >= 0.70" | bc -l) )); then echo "badge-warning"; else echo "badge-danger"; fi)">$(awk "BEGIN {printf \"%.1f\", ${ALGO_RATE} * 100}")%</span></span>
            </div>
            <div class="progress-bar">
                <div class="progress-fill" style="width: $(awk "BEGIN {printf \"%.1f\", ${ALGO_RATE} * 100}")%; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);">
                    $(awk "BEGIN {printf \"%.1f\", ${ALGO_RATE} * 100}")%
                </div>
            </div>
        </div>

        <div class="category-card">
            <div class="category-header">
                <span class="category-name">📊 Data Science</span>
                <span class="category-stats">${DS_PASSING}/${DS_TOTAL} <span class="badge $(if (( $(echo "${DS_RATE} >= 0.80" | bc -l) )); then echo "badge-success"; elif (( $(echo "${DS_RATE} >= 0.70" | bc -l) )); then echo "badge-warning"; else echo "badge-danger"; fi)">$(awk "BEGIN {printf \"%.1f\", ${DS_RATE} * 100}")%</span></span>
            </div>
            <div class="progress-bar">
                <div class="progress-fill" style="width: $(awk "BEGIN {printf \"%.1f\", ${DS_RATE} * 100}")%; background: linear-gradient(135deg, #06b6d4 0%, #0891b2 100%);">
                    $(awk "BEGIN {printf \"%.1f\", ${DS_RATE} * 100}")%
                </div>
            </div>
        </div>

        <div class="category-card">
            <div class="category-header">
                <span class="category-name">🤖 Advanced AI</span>
                <span class="category-stats">${AI_PASSING}/${AI_TOTAL} <span class="badge $(if (( $(echo "${AI_RATE} >= 0.80" | bc -l) )); then echo "badge-success"; elif (( $(echo "${AI_RATE} >= 0.70" | bc -l) )); then echo "badge-warning"; else echo "badge-danger"; fi)">$(awk "BEGIN {printf \"%.1f\", ${AI_RATE} * 100}")%</span></span>
            </div>
            <div class="progress-bar">
                <div class="progress-fill" style="width: $(awk "BEGIN {printf \"%.1f\", ${AI_RATE} * 100}")%; background: linear-gradient(135deg, #8b5cf6 0%, #7c3aed 100%);">
                    $(awk "BEGIN {printf \"%.1f\", ${AI_RATE} * 100}")%
                </div>
            </div>
        </div>

        <div class="footer">
            <p>🌸 Built with Toyota Way principles: Quality built-in, not bolted-on</p>
            <p style="margin-top: 10px; font-size: 0.9rem;">Generated: ${TIMESTAMP}</p>
            <p style="margin-top: 5px;"><a href="https://github.com/paiml/rosetta-ruchy" style="color: white;">View on GitHub</a></p>
        </div>
    </div>
</body>
</html>
HTML_DYNAMIC

echo "✅ Dashboard generated: ${OUTPUT_FILE}"
echo "   Success Rate: ${SUCCESS_PERCENT}%"
echo "   View with: open ${OUTPUT_FILE}"
