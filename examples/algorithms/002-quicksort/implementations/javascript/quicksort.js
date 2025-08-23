#!/usr/bin/env node

class Sortable {
    constructor(arr) {
        this.data = arr;
    }
    
    sort() {
        quicksortInplace(this.data);
    }
    
    isSorted() {
        for (let i = 0; i < this.data.length - 1; i++) {
            if (this.data[i] > this.data[i + 1]) {
                return false;
            }
        }
        return true;
    }
    
    toArray() {
        return [...this.data];
    }
}

function quicksortInplace(arr) {
    if (arr.length > 1) {
        quicksortRange(arr, 0, arr.length - 1);
    }
}

function quicksortRange(arr, low, high) {
    if (low < high) {
        const pivotIndex = partition(arr, low, high);
        
        if (pivotIndex > 0) {
            quicksortRange(arr, low, pivotIndex - 1);
        }
        if (pivotIndex + 1 <= high) {
            quicksortRange(arr, pivotIndex + 1, high);
        }
    }
}

function partition(arr, low, high) {
    const pivot = arr[high];
    let i = low;
    
    for (let j = low; j < high; j++) {
        if (arr[j] <= pivot) {
            [arr[i], arr[j]] = [arr[j], arr[i]];
            i++;
        }
    }
    
    [arr[i], arr[high]] = [arr[high], arr[i]];
    return i;
}

function quicksortFunctional(arr) {
    if (arr.length <= 1) {
        return [...arr];
    }
    
    const pivot = arr[Math.floor(arr.length / 2)];
    const less = arr.filter(x => x < pivot);
    const equal = arr.filter(x => x === pivot);
    const greater = arr.filter(x => x > pivot);
    
    return [
        ...quicksortFunctional(less),
        ...equal,
        ...quicksortFunctional(greater)
    ];
}

function quicksortThreeWay(arr) {
    if (arr.length <= 1) {
        return;
    }
    threeWayPartitionSort(arr, 0, arr.length - 1);
}

function threeWayPartitionSort(arr, low, high) {
    if (low >= high) {
        return;
    }
    
    const pivot = arr[low];
    let lt = low;
    let gt = high;
    let i = low + 1;
    
    while (i <= gt) {
        if (arr[i] < pivot) {
            [arr[i], arr[lt]] = [arr[lt], arr[i]];
            lt++;
            i++;
        } else if (arr[i] > pivot) {
            [arr[i], arr[gt]] = [arr[gt], arr[i]];
            gt--;
        } else {
            i++;
        }
    }
    
    if (lt > 0) {
        threeWayPartitionSort(arr, low, lt - 1);
    }
    threeWayPartitionSort(arr, gt + 1, high);
}

async function quicksortParallel(arr, threshold = 10000) {
    if (arr.length <= threshold) {
        return quicksortFunctional(arr);
    }
    
    if (arr.length <= 1) {
        return [...arr];
    }
    
    const pivot = arr[Math.floor(arr.length / 2)];
    const less = arr.filter(x => x < pivot);
    const equal = arr.filter(x => x === pivot);
    const greater = arr.filter(x => x > pivot);
    
    // Use Promise.all for parallel execution
    const [sortedLess, sortedGreater] = await Promise.all([
        quicksortParallel(less, threshold),
        quicksortParallel(greater, threshold)
    ]);
    
    return [...sortedLess, ...equal, ...sortedGreater];
}

// Web Worker implementation for true parallel processing
function quicksortWebWorker(arr) {
    return new Promise((resolve, reject) => {
        if (typeof Worker === 'undefined') {
            // Fallback to regular quicksort if Web Workers not available
            resolve(quicksortFunctional(arr));
            return;
        }
        
        const workerCode = `
            self.onmessage = function(e) {
                const arr = e.data;
                const result = quicksortFunctional(arr);
                self.postMessage(result);
            };
            
            ${quicksortFunctional.toString()}
        `;
        
        const blob = new Blob([workerCode], { type: 'application/javascript' });
        const worker = new Worker(URL.createObjectURL(blob));
        
        worker.onmessage = function(e) {
            resolve(e.data);
            worker.terminate();
        };
        
        worker.onerror = function(error) {
            reject(error);
            worker.terminate();
        };
        
        worker.postMessage(arr);
    });
}

async function main() {
    const testArrays = [
        [],
        [42],
        [3, 1, 4, 1, 5, 9, 2, 6],
        [5, 4, 3, 2, 1],
        [1, 2, 3, 4, 5],
        [5, 5, 5, 5, 5],
    ];
    
    console.log("Quicksort Implementations Demo\n");
    
    for (let i = 0; i < testArrays.length; i++) {
        const arr = testArrays[i];
        console.log(`Test case ${i + 1}: [${arr.join(', ')}]`);
        
        const arr1 = [...arr];
        quicksortInplace(arr1);
        console.log(`  In-place:    [${arr1.join(', ')}]`);
        
        const arr2 = quicksortFunctional(arr);
        console.log(`  Functional:  [${arr2.join(', ')}]`);
        
        const arr3 = [...arr];
        quicksortThreeWay(arr3);
        console.log(`  Three-way:   [${arr3.join(', ')}]`);
        
        if (arr.length > 0) {
            const arr4 = await quicksortParallel(arr);
            console.log(`  Parallel:    [${arr4.join(', ')}]`);
        }
        
        console.log();
    }
    
    // Performance demonstration
    console.log("Performance demonstration with large array:");
    const largeArray = Array.from({ length: 10000 }, (_, i) => (i * 37 + 11) % 1000);
    
    console.log(`  Array size: ${largeArray.length}`);
    
    let start = performance.now();
    const arrCopy = [...largeArray];
    quicksortInplace(arrCopy);
    let duration = performance.now() - start;
    console.log(`  In-place time: ${duration.toFixed(4)}ms`);
    
    start = performance.now();
    quicksortFunctional(largeArray);
    duration = performance.now() - start;
    console.log(`  Functional time: ${duration.toFixed(4)}ms`);
    
    start = performance.now();
    await quicksortParallel(largeArray);
    duration = performance.now() - start;
    console.log(`  Parallel time: ${duration.toFixed(4)}ms`);
}

// Export for testing
if (typeof module !== 'undefined' && module.exports) {
    module.exports = {
        Sortable,
        quicksortInplace,
        quicksortFunctional,
        quicksortThreeWay,
        quicksortParallel,
        quicksortWebWorker
    };
}

// Run main if this is the entry point
if (require.main === module) {
    main().catch(console.error);
}