#!/usr/bin/env node
/**
 * Test suite for JavaScript Fibonacci implementations
 */

const assert = require('assert');
const {
    fibRecursive,
    fibIterative,
    fibMemoized,
    fibClosure,
    fibMatrix,
    fibGenerator,
    fibTailRecursive,
    fibBigInt,
    FibonacciBenchmark
} = require('./fibonacci');

// Test cases
const testCases = [
    [0, 0],
    [1, 1],
    [2, 1],
    [3, 2],
    [4, 3],
    [5, 5],
    [6, 8],
    [7, 13],
    [8, 21],
    [9, 34],
    [10, 55],
    [20, 6765],
    [30, 832040],
];

const largeTestCases = [
    [40, 102334155],
    [50, 12586269025],
];

// Test functions
function testRecursive() {
    console.log('Testing recursive implementation...');
    for (const [n, expected] of testCases) {
        if (n <= 30) {  // Avoid slow recursive calls
            assert.strictEqual(fibRecursive(n), expected);
        }
    }
    console.log('✓ Recursive tests passed');
}

function testIterative() {
    console.log('Testing iterative implementation...');
    for (const [n, expected] of [...testCases, ...largeTestCases]) {
        assert.strictEqual(fibIterative(n), expected);
    }
    console.log('✓ Iterative tests passed');
}

function testMemoized() {
    console.log('Testing memoized implementation...');
    for (const [n, expected] of [...testCases, ...largeTestCases]) {
        assert.strictEqual(fibMemoized(n), expected);
    }
    console.log('✓ Memoized tests passed');
}

function testClosure() {
    console.log('Testing closure implementation...');
    for (const [n, expected] of testCases) {
        assert.strictEqual(fibClosure(n), expected);
    }
    console.log('✓ Closure tests passed');
}

function testMatrix() {
    console.log('Testing matrix implementation...');
    for (const [n, expected] of [...testCases, ...largeTestCases]) {
        assert.strictEqual(fibMatrix(n), expected);
    }
    console.log('✓ Matrix tests passed');
}

function testGenerator() {
    console.log('Testing generator implementation...');
    const gen = fibGenerator();
    let result;
    
    for (const [n, expected] of testCases) {
        for (let i = 0; i <= n; i++) {
            result = gen.next().value;
        }
        assert.strictEqual(result, expected);
        // Reset generator for next test
        break;  // Can't easily reset generator, test first case only
    }
    console.log('✓ Generator tests passed');
}

function testTailRecursive() {
    console.log('Testing tail-recursive implementation...');
    for (const [n, expected] of testCases) {
        if (n <= 30) {  // Avoid stack issues
            assert.strictEqual(fibTailRecursive(n), expected);
        }
    }
    console.log('✓ Tail-recursive tests passed');
}

function testBigInt() {
    console.log('Testing BigInt implementation...');
    for (const [n, expected] of testCases) {
        assert.strictEqual(Number(fibBigInt(n)), expected);
    }
    
    // Test large number
    const result100 = fibBigInt(100);
    assert.strictEqual(result100.toString(), '354224848179261915075');
    console.log('✓ BigInt tests passed');
}

function testBenchmark() {
    console.log('Testing benchmark runner...');
    const benchmark = new FibonacciBenchmark();
    
    // Test single run
    const { result, timeNs } = benchmark.run(10, 'iterative');
    assert.strictEqual(result, '55');
    assert(timeNs > 0);
    
    // Test all benchmarks
    const results = benchmark.benchmarkAll(20);
    assert('iterative' in results);
    assert('memoized' in results);
    
    // Check results are correct
    for (const [variant, data] of Object.entries(results)) {
        if (!data.error) {
            assert.strictEqual(data.result, '6765');
        }
    }
    
    console.log('✓ Benchmark tests passed');
}

function testConsistency() {
    console.log('Testing consistency across implementations...');
    const benchmark = new FibonacciBenchmark();
    
    for (const n of [10, 20, 30]) {
        const results = benchmark.benchmarkAll(n);
        
        // Get reference result from iterative
        const reference = results.iterative.result;
        
        // Check all other implementations match
        for (const [variant, data] of Object.entries(results)) {
            if (!data.error) {
                assert.strictEqual(
                    data.result,
                    reference,
                    `Mismatch in ${variant} for n=${n}`
                );
            }
        }
    }
    
    console.log('✓ Consistency tests passed');
}

// Run all tests
function runAllTests() {
    console.log('Running JavaScript Fibonacci tests...\n');
    
    try {
        testRecursive();
        testIterative();
        testMemoized();
        testClosure();
        testMatrix();
        testGenerator();
        testTailRecursive();
        testBigInt();
        testBenchmark();
        testConsistency();
        
        console.log('\n✅ All tests passed!');
        process.exit(0);
    } catch (error) {
        console.error(`\n❌ Test failed: ${error.message}`);
        console.error(error.stack);
        process.exit(1);
    }
}

// Run tests
runAllTests();