#!/usr/bin/env node
/**
 * Fibonacci Benchmark - JavaScript Implementation
 * Performance comparison for Ruchy benchmarks
 */

function fibonacciRecursive(n) {
    if (n <= 1) return n;
    return fibonacciRecursive(n - 1) + fibonacciRecursive(n - 2);
}

function fibonacciIterative(n) {
    if (n <= 1) return n;
    
    let prev = 0;
    let curr = 1;
    
    for (let i = 2; i <= n; i++) {
        const next = prev + curr;
        prev = curr;
        curr = next;
    }
    
    return curr;
}

function fibonacciMemoized(n, memo = {}) {
    if (n <= 1) return n;
    
    if (!(n in memo)) {
        memo[n] = fibonacciMemoized(n - 1, memo) + fibonacciMemoized(n - 2, memo);
    }
    
    return memo[n];
}

function benchmarkFibonacci(iterations, n, method = 'iterative') {
    // Warmup phase
    const warmup = Math.floor(iterations / 10);
    for (let i = 0; i < warmup; i++) {
        if (method === 'recursive' && n <= 20) {
            fibonacciRecursive(n);
        } else if (method === 'memoized') {
            fibonacciMemoized(n);
        } else {
            fibonacciIterative(n);
        }
    }
    
    // Benchmark phase
    const startTime = process.hrtime.bigint();
    
    for (let i = 0; i < iterations; i++) {
        if (method === 'recursive' && n <= 20) {
            fibonacciRecursive(n);
        } else if (method === 'memoized') {
            fibonacciMemoized(n, {}); // Fresh memo for each iteration
        } else {
            fibonacciIterative(n);
        }
    }
    
    const endTime = process.hrtime.bigint();
    const elapsedNs = Number(endTime - startTime);
    return elapsedNs / iterations / 1000; // Convert to microseconds
}

function main() {
    // Parse command line arguments
    let iterations = 1000;
    const args = process.argv.slice(2);
    
    if (args.length >= 2 && args[0] === '--iterations') {
        iterations = parseInt(args[1]) || 1000;
    }
    
    const results = {
        algorithm: 'fibonacci',
        language: 'javascript',
        iterations: iterations,
        results: []
    };
    
    const testSizes = [5, 10, 15, 20, 25, 30, 35, 40];
    
    for (const n of testSizes) {
        // Benchmark iterative version
        const iterTime = benchmarkFibonacci(iterations, n, 'iterative');
        
        // Benchmark recursive version (only for small n)
        let recTime = -1;
        if (n <= 20) {
            recTime = benchmarkFibonacci(iterations, n, 'recursive');
        }
        
        // Benchmark memoized version
        const memoTime = benchmarkFibonacci(iterations, n, 'memoized');
        
        results.results.push({
            n: n,
            iterative_time_us: Math.round(iterTime * 1000) / 1000,
            recursive_time_us: Math.round(recTime * 1000) / 1000,
            memoized_time_us: Math.round(memoTime * 1000) / 1000
        });
    }
    
    console.log(JSON.stringify(results, null, 2));
}

if (require.main === module) {
    main();
}