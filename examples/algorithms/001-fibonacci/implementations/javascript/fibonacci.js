#!/usr/bin/env node
/**
 * Fibonacci sequence implementations in JavaScript
 * Ecosystem comparison for Rosetta Ruchy benchmarks
 */

/**
 * Recursive Fibonacci (exponential complexity)
 */
function fibRecursive(n) {
    if (n <= 1) return n;
    return fibRecursive(n - 1) + fibRecursive(n - 2);
}

/**
 * Iterative Fibonacci (linear complexity)
 */
function fibIterative(n) {
    if (n <= 1) return n;
    
    let prev = 0;
    let curr = 1;
    
    for (let i = 2; i <= n; i++) {
        [prev, curr] = [curr, prev + curr];
    }
    
    return curr;
}

/**
 * Memoized Fibonacci (linear complexity with caching)
 */
function fibMemoized(n, cache = {}) {
    if (n in cache) return cache[n];
    
    if (n <= 1) {
        cache[n] = n;
    } else {
        cache[n] = fibMemoized(n - 1, cache) + fibMemoized(n - 2, cache);
    }
    
    return cache[n];
}

/**
 * Fibonacci with closure-based memoization
 */
const fibClosure = (() => {
    const cache = {};
    
    return function fib(n) {
        if (n in cache) return cache[n];
        
        if (n <= 1) {
            cache[n] = n;
        } else {
            cache[n] = fib(n - 1) + fib(n - 2);
        }
        
        return cache[n];
    };
})();

/**
 * Matrix multiplication Fibonacci (logarithmic complexity)
 */
function fibMatrix(n) {
    if (n === 0) return 0;
    if (n === 1) return 1;
    
    function matrixMult(a, b) {
        return [
            [a[0][0] * b[0][0] + a[0][1] * b[1][0],
             a[0][0] * b[0][1] + a[0][1] * b[1][1]],
            [a[1][0] * b[0][0] + a[1][1] * b[1][0],
             a[1][0] * b[0][1] + a[1][1] * b[1][1]]
        ];
    }
    
    function matrixPow(mat, p) {
        if (p === 1) return mat;
        
        if (p % 2 === 0) {
            const half = matrixPow(mat, p / 2);
            return matrixMult(half, half);
        } else {
            return matrixMult(mat, matrixPow(mat, p - 1));
        }
    }
    
    const baseMatrix = [[1, 1], [1, 0]];
    const result = matrixPow(baseMatrix, n);
    return result[0][1];
}

/**
 * Generator-based Fibonacci
 */
function* fibGenerator() {
    let prev = 0;
    let curr = 1;
    
    yield prev;
    yield curr;
    
    while (true) {
        [prev, curr] = [curr, prev + curr];
        yield curr;
    }
}

/**
 * Tail-recursive Fibonacci (optimized in some JS engines)
 */
function fibTailRecursive(n, prev = 0, curr = 1) {
    if (n === 0) return prev;
    return fibTailRecursive(n - 1, curr, prev + curr);
}

/**
 * BigInt Fibonacci for large numbers
 */
function fibBigInt(n) {
    if (n <= 1) return BigInt(n);
    
    let prev = 0n;
    let curr = 1n;
    
    for (let i = 2; i <= n; i++) {
        [prev, curr] = [curr, prev + curr];
    }
    
    return curr;
}

/**
 * Async Fibonacci (for demonstration)
 */
async function fibAsync(n) {
    return new Promise((resolve) => {
        setImmediate(() => {
            resolve(fibIterative(n));
        });
    });
}

/**
 * Worker-based Fibonacci (for parallel processing demonstration)
 */
function fibWorker(n) {
    // Note: This is a simplified version
    // Real implementation would use Worker threads
    return fibIterative(n);
}

/**
 * Benchmark class for running performance tests
 */
class FibonacciBenchmark {
    constructor() {
        this.implementations = {
            recursive: fibRecursive,
            iterative: fibIterative,
            memoized: fibMemoized,
            closure: fibClosure,
            matrix: fibMatrix,
            tailRecursive: fibTailRecursive,
            bigint: fibBigInt,
        };
    }
    
    run(n, variant = 'iterative') {
        const func = this.implementations[variant];
        if (!func) {
            throw new Error(`Unknown variant: ${variant}`);
        }
        
        // Clear closure cache if needed
        if (variant === 'closure') {
            // Reset cache by recreating the function
            // (Not easily done with closure pattern)
        }
        
        const start = process.hrtime.bigint();
        const result = func(n);
        const duration = process.hrtime.bigint() - start;
        
        return {
            result: result.toString(),
            timeNs: Number(duration)
        };
    }
    
    benchmarkAll(n) {
        const results = {};
        
        for (const [variant, func] of Object.entries(this.implementations)) {
            // Skip recursive for large n
            if (variant === 'recursive' && n > 35) continue;
            
            try {
                const { result, timeNs } = this.run(n, variant);
                results[variant] = {
                    result,
                    timeNs,
                    timeMs: timeNs / 1_000_000
                };
            } catch (error) {
                results[variant] = { error: error.message };
            }
        }
        
        return results;
    }
}

// Command-line interface
if (require.main === module) {
    const args = process.argv.slice(2);
    
    if (args.length < 1) {
        console.log('Usage: node fibonacci.js <n> [variant]');
        console.log('Variants: recursive, iterative, memoized, closure, matrix, tailRecursive, bigint');
        process.exit(1);
    }
    
    const n = parseInt(args[0]);
    const variant = args[1] || 'iterative';
    
    const benchmark = new FibonacciBenchmark();
    
    try {
        const { result, timeNs } = benchmark.run(n, variant);
        console.log(`fib(${n}) = ${result}`);
        console.log(`Time: ${timeNs} ns (${(timeNs / 1_000_000).toFixed(3)} ms)`);
    } catch (error) {
        console.error(`Error: ${error.message}`);
        process.exit(1);
    }
}

// Export for testing
module.exports = {
    fibRecursive,
    fibIterative,
    fibMemoized,
    fibClosure,
    fibMatrix,
    fibGenerator,
    fibTailRecursive,
    fibBigInt,
    fibAsync,
    fibWorker,
    FibonacciBenchmark
};