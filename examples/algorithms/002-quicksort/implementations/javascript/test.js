#!/usr/bin/env node

const {
    Sortable,
    quicksortInplace,
    quicksortFunctional,
    quicksortThreeWay,
    quicksortParallel
} = require('./quicksort');

function runTests() {
    let passed = 0;
    let total = 0;
    
    function test(name, fn) {
        total++;
        try {
            fn();
            console.log(`✓ ${name}`);
            passed++;
        } catch (error) {
            console.log(`✗ ${name}: ${error.message}`);
        }
    }
    
    function assertEqual(actual, expected, message = '') {
        const actualStr = JSON.stringify(actual);
        const expectedStr = JSON.stringify(expected);
        if (actualStr !== expectedStr) {
            throw new Error(`Expected ${expectedStr}, got ${actualStr}. ${message}`);
        }
    }
    
    function assertTrue(condition, message = '') {
        if (!condition) {
            throw new Error(`Expected true, got false. ${message}`);
        }
    }
    
    function assertFalse(condition, message = '') {
        if (condition) {
            throw new Error(`Expected false, got true. ${message}`);
        }
    }
    
    // Test empty array
    test('Empty array', () => {
        const arr = [];
        quicksortInplace(arr);
        assertEqual(arr, []);
    });
    
    // Test single element
    test('Single element', () => {
        const arr = [42];
        quicksortInplace(arr);
        assertEqual(arr, [42]);
    });
    
    // Test sorted array
    test('Already sorted array', () => {
        const arr = [1, 2, 3, 4, 5];
        quicksortInplace(arr);
        assertEqual(arr, [1, 2, 3, 4, 5]);
    });
    
    // Test reverse array
    test('Reverse sorted array', () => {
        const arr = [5, 4, 3, 2, 1];
        quicksortInplace(arr);
        assertEqual(arr, [1, 2, 3, 4, 5]);
    });
    
    // Test random array
    test('Random array', () => {
        const arr = [3, 1, 4, 1, 5, 9, 2, 6];
        quicksortInplace(arr);
        assertEqual(arr, [1, 1, 2, 3, 4, 5, 6, 9]);
    });
    
    // Test duplicates
    test('Array with duplicates', () => {
        const arr = [5, 5, 5, 5, 5];
        quicksortInplace(arr);
        assertEqual(arr, [5, 5, 5, 5, 5]);
    });
    
    // Test negative numbers
    test('Array with negative numbers', () => {
        const arr = [3, -1, 4, -1, 5, 9, -2, 6];
        quicksortInplace(arr);
        assertEqual(arr, [-2, -1, -1, 3, 4, 5, 6, 9]);
    });
    
    // Test functional quicksort
    test('Functional quicksort', () => {
        const arr = [3, 1, 4, 1, 5, 9, 2, 6];
        const sorted = quicksortFunctional(arr);
        assertEqual(sorted, [1, 1, 2, 3, 4, 5, 6, 9]);
        // Original array should be unchanged
        assertEqual(arr, [3, 1, 4, 1, 5, 9, 2, 6]);
    });
    
    // Test three-way quicksort
    test('Three-way quicksort', () => {
        const arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        quicksortThreeWay(arr);
        assertEqual(arr, [1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
    });
    
    // Test parallel quicksort
    test('Parallel quicksort', async () => {
        const arr = [3, 1, 4, 1, 5, 9, 2, 6];
        const sorted = await quicksortParallel(arr);
        assertEqual(sorted, [1, 1, 2, 3, 4, 5, 6, 9]);
    });
    
    // Test Sortable class
    test('Sortable class', () => {
        const sortable = new Sortable([3, 1, 4, 1, 5, 9, 2, 6]);
        assertFalse(sortable.isSorted());
        sortable.sort();
        assertTrue(sortable.isSorted());
        assertEqual(sortable.toArray(), [1, 1, 2, 3, 4, 5, 6, 9]);
    });
    
    // Property tests
    test('Property: sorted array has same elements', () => {
        for (let i = 0; i < 100; i++) {
            const arr = Array.from({ length: Math.floor(Math.random() * 50) }, 
                                  () => Math.floor(Math.random() * 200) - 100);
            const sorted = [...arr];
            quicksortInplace(sorted);
            
            const origSorted = [...arr].sort((a, b) => a - b);
            assertEqual(sorted, origSorted);
        }
    });
    
    test('Property: sorted array is ordered', () => {
        for (let i = 0; i < 100; i++) {
            const arr = Array.from({ length: Math.floor(Math.random() * 50) }, 
                                  () => Math.floor(Math.random() * 200) - 100);
            quicksortInplace(arr);
            
            for (let j = 0; j < arr.length - 1; j++) {
                if (arr[j] > arr[j + 1]) {
                    throw new Error(`Array not sorted at indices ${j}, ${j + 1}: ${arr[j]} > ${arr[j + 1]}`);
                }
            }
        }
    });
    
    test('Implementation consistency', async () => {
        for (let i = 0; i < 50; i++) {
            const arr = Array.from({ length: Math.floor(Math.random() * 30) }, 
                                  () => Math.floor(Math.random() * 100) - 50);
            
            const arr1 = [...arr];
            quicksortInplace(arr1);
            
            const arr2 = quicksortFunctional(arr);
            
            const arr3 = [...arr];
            quicksortThreeWay(arr3);
            
            const arr4 = await quicksortParallel(arr);
            
            assertEqual(arr1, arr2, 'In-place vs functional');
            assertEqual(arr2, arr3, 'Functional vs three-way');
            assertEqual(arr3, arr4, 'Three-way vs parallel');
        }
    });
    
    console.log(`\nTests completed: ${passed}/${total} passed`);
    return passed === total;
}

async function main() {
    console.log('Running Quicksort Tests...\n');
    const success = await runTests();
    process.exit(success ? 0 : 1);
}

if (require.main === module) {
    main().catch(console.error);
}