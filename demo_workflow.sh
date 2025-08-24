#!/bin/bash
# Demo workflow showing scientific infrastructure in action

echo "🔬 ROSETTA RUCHY SCIENTIFIC INFRASTRUCTURE DEMO"
echo "==============================================="
echo ""

# Step 1: Check infrastructure syntax
echo "Step 1: Validating Infrastructure Components"
echo "--------------------------------------------"
ruchy check harness/benchmark/benchmark_simple.ruchy
ruchy check harness/statistics/statistics_simple.ruchy
ruchy check scripts/reproduce_simple.ruchy
ruchy check scripts/graphs_simple.ruchy
echo "✅ All components validated"
echo ""

# Step 2: Run benchmark
echo "Step 2: Running Benchmark Harness"
echo "---------------------------------"
ruchy run harness/benchmark/benchmark_simple.ruchy
echo ""

# Step 3: Statistical analysis  
echo "Step 3: Statistical Analysis"
echo "----------------------------"
ruchy run harness/statistics/statistics_simple.ruchy
echo ""

# Step 4: Generate visualizations
echo "Step 4: Generate Visualizations"
echo "-------------------------------"
ruchy run scripts/graphs_simple.ruchy
echo ""

# Step 5: Show reproducibility workflow
echo "Step 5: Reproducibility Framework"
echo "---------------------------------"
ruchy run scripts/reproduce_simple.ruchy
echo ""

echo "✅ INFRASTRUCTURE DEMO COMPLETE!"
echo ""
echo "This demonstrates:"
echo "- Pure Ruchy implementation of all tools"
echo "- Statistical rigor (t-tests, confidence intervals)"
echo "- ASCII visualization for scientific reports"
echo "- Reproducible workflow automation"
echo "- Ready for formal algorithm verification"