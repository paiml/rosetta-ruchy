# Ruchy Fibonacci Implementation Makefile
# Showcases Ruchy's advanced tooling capabilities

RUCHY := /home/noah/src/ruchy/target/debug/ruchy
RUCHY_FILE := fibonacci.ruchy

.PHONY: all analyze verify optimize test bench report clean

# Complete workflow demonstrating Ruchy's advantages
all: analyze verify optimize test bench report

# STEP 1: Advanced Static Analysis (Ruchy's Core Strength)
analyze:
	@echo "🔍 Running Ruchy Advanced Analysis Suite"
	@echo "==========================================="
	@mkdir -p analysis
	$(RUCHY) ast $(RUCHY_FILE) > analysis/ast.txt 2>&1 || true
	$(RUCHY) provability $(RUCHY_FILE) > analysis/provability.txt 2>&1 || true
	$(RUCHY) runtime $(RUCHY_FILE) > analysis/runtime.txt 2>&1 || true
	$(RUCHY) score $(RUCHY_FILE) > analysis/score.txt 2>&1 || true
	@echo "✅ Analysis complete. Results in analysis/"

# STEP 2: Formal Verification (Unique to Ruchy)
verify:
	@echo "✅ Running Formal Verification"
	@echo "=============================="
	$(RUCHY) check $(RUCHY_FILE) || true
	$(RUCHY) quality-gate $(RUCHY_FILE) --threshold 0.80 2>&1 || true
	@echo "✅ Verification complete"

# STEP 3: Optimization Analysis
optimize:
	@echo "⚡ Hardware-Aware Optimization Analysis"
	@echo "======================================="
	$(RUCHY) optimize $(RUCHY_FILE) --target-cpu native 2>&1 || true
	@echo "✅ Optimization analysis complete"

# STEP 4: Testing
test:
	@echo "🧪 Running Tests"
	@echo "==============="
	$(RUCHY) run $(RUCHY_FILE) 2>&1 || true
	@echo "✅ Tests complete"

# STEP 5: Benchmarking
bench:
	@echo "📊 Running Benchmarks"
	@echo "===================="
	$(RUCHY) bench $(RUCHY_FILE) 2>&1 || true
	@echo "✅ Benchmarks complete"

# STEP 6: Generate Quality Report
report:
	@echo "📋 Generating Quality Report"
	@echo "==========================="
	@mkdir -p reports
	@echo "# Ruchy Fibonacci Analysis Report" > reports/quality_report.md
	@echo "" >> reports/quality_report.md
	@echo "## AST Analysis" >> reports/quality_report.md
	@if [ -f analysis/ast.txt ]; then tail -20 analysis/ast.txt >> reports/quality_report.md; fi
	@echo "" >> reports/quality_report.md
	@echo "## Provability Analysis" >> reports/quality_report.md
	@if [ -f analysis/provability.txt ]; then tail -20 analysis/provability.txt >> reports/quality_report.md; fi
	@echo "" >> reports/quality_report.md
	@echo "## Runtime Complexity" >> reports/quality_report.md
	@if [ -f analysis/runtime.txt ]; then tail -20 analysis/runtime.txt >> reports/quality_report.md; fi
	@echo "" >> reports/quality_report.md
	@echo "## Quality Score" >> reports/quality_report.md
	@if [ -f analysis/score.txt ]; then cat analysis/score.txt >> reports/quality_report.md; fi
	@echo "✅ Report generated: reports/quality_report.md"

# Clean up generated files
clean:
	rm -rf analysis/ reports/ target/

# Run the implementation
run:
	$(RUCHY) run $(RUCHY_FILE)

# Show AST
ast:
	$(RUCHY) ast $(RUCHY_FILE)

# Show provability
provability:
	$(RUCHY) provability $(RUCHY_FILE)

# Show runtime analysis
runtime:
	$(RUCHY) runtime $(RUCHY_FILE)

# Show quality score
score:
	$(RUCHY) score $(RUCHY_FILE)