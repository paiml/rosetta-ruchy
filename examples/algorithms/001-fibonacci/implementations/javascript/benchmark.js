#!/usr/bin/env node
/**
 * Benchmark script for JavaScript Fibonacci implementations
 * Outputs results in JSON format compatible with Rosetta Ruchy
 */

const { FibonacciBenchmark } = require('./fibonacci');
const fs = require('fs');
const path = require('path');

function runBenchmarkSuite() {
    const benchmark = new FibonacciBenchmark();
    
    // Test configurations from spec.toml
    const testConfigs = [
        { name: 'tiny', n: 5, expected: 5, variants: ['all'] },
        { name: 'small', n: 10, expected: 55, variants: ['all'] },
        { name: 'medium_recursive', n: 30, expected: 832040,
          variants: ['recursive', 'memoized', 'closure'] },
        { name: 'medium_iterative', n: 40, expected: 102334155,
          variants: ['iterative', 'memoized', 'closure', 'matrix', 'tailRecursive'] },
        { name: 'large', n: 100, expected: '354224848179261915075',
          variants: ['iterative', 'memoized', 'matrix', 'bigint'] },
    ];
    
    const results = {
        language: 'javascript',
        version: process.version,
        timestamp: Date.now(),
        variants: {}
    };
    
    for (const config of testConfigs) {
        const { name: testName, n, expected, variants } = config;
        
        // Determine which variants to test
        const variantsToTest = variants[0] === 'all' 
            ? Object.keys(benchmark.implementations).filter(v => 
                n <= 35 || v !== 'recursive')
            : variants;
        
        for (const variant of variantsToTest) {
            if (!results.variants[variant]) {
                results.variants[variant] = { testCases: {} };
            }
            
            // Run multiple iterations for statistics
            const times = [];
            const iterations = n < 30 ? 10 : 5;
            let result = null;
            let correct = false;
            
            for (let i = 0; i < iterations; i++) {
                try {
                    const runResult = benchmark.run(n, variant);
                    result = runResult.result;
                    times.push(runResult.timeNs);
                    correct = (result == expected || result == expected.toString());
                } catch (error) {
                    result = null;
                    correct = false;
                    times.push(Infinity);
                    break;
                }
            }
            
            // Calculate statistics
            const validTimes = times.filter(t => t !== Infinity);
            const stats = validTimes.length > 0 ? {
                mean: validTimes.reduce((a, b) => a + b, 0) / validTimes.length,
                median: validTimes.sort((a, b) => a - b)[Math.floor(validTimes.length / 2)],
                min: Math.min(...validTimes),
                max: Math.max(...validTimes),
                stdDev: calculateStdDev(validTimes)
            } : null;
            
            results.variants[variant].testCases[testName] = {
                input: n,
                output: result,
                expected: expected.toString(),
                correct,
                timeNs: stats ? Math.floor(stats.median) : null,
                iterations: validTimes.length,
                statistics: stats
            };
        }
    }
    
    return results;
}

function calculateStdDev(values) {
    const mean = values.reduce((a, b) => a + b, 0) / values.length;
    const squaredDiffs = values.map(v => Math.pow(v - mean, 2));
    const variance = squaredDiffs.reduce((a, b) => a + b, 0) / values.length;
    return Math.sqrt(variance);
}

function main() {
    console.log('Running JavaScript Fibonacci benchmarks...');
    const results = runBenchmarkSuite();
    
    // Output JSON results
    console.log(JSON.stringify(results, null, 2));
    
    // Create results directory if it doesn't exist
    const resultsDir = path.join(__dirname, 'results');
    if (!fs.existsSync(resultsDir)) {
        fs.mkdirSync(resultsDir);
    }
    
    // Save to file
    const outputPath = path.join(resultsDir, 'javascript_benchmark.json');
    fs.writeFileSync(outputPath, JSON.stringify(results, null, 2));
    
    console.log(`\nBenchmark complete. Results saved to ${outputPath}`);
}

// Run if executed directly
if (require.main === module) {
    main();
}