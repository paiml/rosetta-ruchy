#!/usr/bin/env node
/**
 * QuickSort Benchmark - JavaScript Implementation
 * O(n log n) average case performance validation
 */

function quicksort(arr) {
    if (arr.length <= 1) return arr;
    
    const pivot = arr[Math.floor(arr.length / 2)];
    const left = arr.filter(x => x < pivot);
    const middle = arr.filter(x => x === pivot);
    const right = arr.filter(x => x > pivot);
    
    return [...quicksort(left), ...middle, ...quicksort(right)];
}

function quicksortInplace(arr, low = 0, high = null) {
    if (high === null) high = arr.length - 1;
    
    if (low < high) {
        const pi = partition(arr, low, high);
        quicksortInplace(arr, low, pi - 1);
        quicksortInplace(arr, pi + 1, high);
    }
    
    return arr;
}

function partition(arr, low, high) {
    const pivot = arr[high];
    let i = low - 1;
    
    for (let j = low; j < high; j++) {
        if (arr[j] <= pivot) {
            i++;
            [arr[i], arr[j]] = [arr[j], arr[i]];
        }
    }
    
    [arr[i + 1], arr[high]] = [arr[high], arr[i + 1]];
    return i + 1;
}

function generateTestArray(size, pattern) {
    switch (pattern) {
        case 'random': {
            const arr = Array.from({length: size}, (_, i) => i);
            // Fisher-Yates shuffle
            for (let i = arr.length - 1; i > 0; i--) {
                const j = Math.floor(Math.random() * (i + 1));
                [arr[i], arr[j]] = [arr[j], arr[i]];
            }
            return arr;
        }
        case 'sorted':
            return Array.from({length: size}, (_, i) => i);
        case 'reverse':
            return Array.from({length: size}, (_, i) => size - 1 - i);
        case 'partial': {
            const arr = Array.from({length: size}, (_, i) => i);
            // Shuffle last quarter
            const quarter = Math.floor(size / 4);
            const start = size - quarter;
            for (let i = size - 1; i >= start; i--) {
                const j = Math.floor(Math.random() * (i - start + 1)) + start;
                [arr[i], arr[j]] = [arr[j], arr[i]];
            }
            return arr;
        }
        default:
            return new Array(size).fill(0);
    }
}

function benchmarkQuicksort(iterations, size, pattern, inplace = true) {
    let totalTime = 0;
    
    for (let i = 0; i < iterations; i++) {
        const arr = generateTestArray(size, pattern);
        
        const startTime = process.hrtime.bigint();
        if (inplace) {
            quicksortInplace([...arr]); // Copy array to preserve original
        } else {
            quicksort(arr);
        }
        const endTime = process.hrtime.bigint();
        
        totalTime += Number(endTime - startTime);
    }
    
    return totalTime / iterations / 1000; // Convert to microseconds
}

function main() {
    // Parse command line arguments
    let iterations = 1000;
    const args = process.argv.slice(2);
    
    if (args.length >= 2 && args[0] === '--iterations') {
        iterations = parseInt(args[1]) || 1000;
    }
    
    const results = {
        algorithm: 'quicksort',
        language: 'javascript',
        iterations: iterations,
        results: []
    };
    
    const testSizes = [10, 50, 100, 500, 1000, 5000];
    const patterns = ['random', 'sorted', 'reverse', 'partial'];
    
    for (const size of testSizes) {
        for (const pattern of patterns) {
            const avgTime = benchmarkQuicksort(iterations, size, pattern);
            
            results.results.push({
                size: size,
                pattern: pattern,
                avg_time_us: Math.round(avgTime * 1000) / 1000
            });
        }
    }
    
    console.log(JSON.stringify(results, null, 2));
}

if (require.main === module) {
    main();
}