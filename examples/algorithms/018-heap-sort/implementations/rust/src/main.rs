// Heap Sort - Rust Implementation
// Comprehensive heap-based sorting with performance analysis

use std::time::Instant;

// Heap Sort implementation with comprehensive analysis
#[derive(Debug, Clone)]
struct HeapSort {
    comparisons: usize,
    swaps: usize,
    heap_size: usize,
    track_stats: bool,
}

impl HeapSort {
    fn new(track_stats: bool) -> Self {
        Self {
            comparisons: 0,
            swaps: 0,
            heap_size: 0,
            track_stats,
        }
    }

    // Main heap sort algorithm
    fn sort(&mut self, arr: &mut [i32]) -> SortResult {
        let start = Instant::now();
        
        if arr.len() <= 1 {
            return SortResult {
                algorithm: "Heap Sort".to_string(),
                size: arr.len(),
                time_ms: start.elapsed().as_secs_f64() * 1000.0,
                comparisons: 0,
                swaps: 0,
                is_stable: false,
                is_in_place: true,
            };
        }

        self.reset_stats();
        self.heap_size = arr.len();

        // Phase 1: Build max heap (bottom-up approach)
        self.build_max_heap(arr);

        // Phase 2: Extract elements one by one
        for i in (1..arr.len()).rev() {
            // Move current root to end
            self.swap(arr, 0, i);
            
            // Reduce heap size and restore heap property
            self.heap_size = i;
            self.heapify_recursive(arr, 0);
        }

        SortResult {
            algorithm: "Heap Sort".to_string(),
            size: arr.len(),
            time_ms: start.elapsed().as_secs_f64() * 1000.0,
            comparisons: self.comparisons,
            swaps: self.swaps,
            is_stable: false,
            is_in_place: true,
        }
    }

    // Build max heap from unsorted array (O(n) approach)
    fn build_max_heap(&mut self, arr: &mut [i32]) {
        // Start from last non-leaf node and heapify each node
        let last_non_leaf = (self.heap_size / 2).saturating_sub(1);
        
        for i in (0..=last_non_leaf).rev() {
            self.heapify_recursive(arr, i);
        }
    }

    // Recursive heapify - restore heap property at index i
    fn heapify_recursive(&mut self, arr: &mut [i32], i: usize) {
        let left = 2 * i + 1;
        let right = 2 * i + 2;
        let mut largest = i;

        // Find largest among root, left child, and right child
        if left < self.heap_size {
            self.increment_comparison();
            if arr[left] > arr[largest] {
                largest = left;
            }
        }

        if right < self.heap_size {
            self.increment_comparison();
            if arr[right] > arr[largest] {
                largest = right;
            }
        }

        // If largest is not root, swap and continue heapifying
        if largest != i {
            self.swap(arr, i, largest);
            self.heapify_recursive(arr, largest);
        }
    }

    // Iterative heapify implementation (alternative to recursive)
    fn heapify_iterative(&mut self, arr: &mut [i32], mut i: usize) {
        loop {
            let left = 2 * i + 1;
            let right = 2 * i + 2;
            let mut largest = i;

            if left < self.heap_size {
                self.increment_comparison();
                if arr[left] > arr[largest] {
                    largest = left;
                }
            }

            if right < self.heap_size {
                self.increment_comparison();
                if arr[right] > arr[largest] {
                    largest = right;
                }
            }

            if largest == i {
                break; // Heap property satisfied
            }

            self.swap(arr, i, largest);
            i = largest; // Continue with the affected subtree
        }
    }

    // Alternative sorting using iterative heapify
    fn sort_iterative(&mut self, arr: &mut [i32]) -> SortResult {
        let start = Instant::now();
        
        if arr.len() <= 1 {
            return SortResult {
                algorithm: "Heap Sort (Iterative)".to_string(),
                size: arr.len(),
                time_ms: start.elapsed().as_secs_f64() * 1000.0,
                comparisons: 0,
                swaps: 0,
                is_stable: false,
                is_in_place: true,
            };
        }

        self.reset_stats();
        self.heap_size = arr.len();

        // Build heap using iterative heapify
        let last_non_leaf = (self.heap_size / 2).saturating_sub(1);
        for i in (0..=last_non_leaf).rev() {
            self.heapify_iterative(arr, i);
        }

        // Extract elements
        for i in (1..arr.len()).rev() {
            self.swap(arr, 0, i);
            self.heap_size = i;
            self.heapify_iterative(arr, 0);
        }

        SortResult {
            algorithm: "Heap Sort (Iterative)".to_string(),
            size: arr.len(),
            time_ms: start.elapsed().as_secs_f64() * 1000.0,
            comparisons: self.comparisons,
            swaps: self.swaps,
            is_stable: false,
            is_in_place: true,
        }
    }

    // Utility functions
    fn swap(&mut self, arr: &mut [i32], i: usize, j: usize) {
        if i != j {
            arr.swap(i, j);
            self.increment_swap();
        }
    }

    fn increment_comparison(&mut self) {
        if self.track_stats {
            self.comparisons += 1;
        }
    }

    fn increment_swap(&mut self) {
        if self.track_stats {
            self.swaps += 1;
        }
    }

    fn reset_stats(&mut self) {
        self.comparisons = 0;
        self.swaps = 0;
    }
}

// Heap visualization and analysis tools
struct HeapAnalyzer;

impl HeapAnalyzer {
    // Verify heap property
    fn is_max_heap(arr: &[i32]) -> bool {
        for i in 0..arr.len() / 2 {
            let left = 2 * i + 1;
            let right = 2 * i + 2;

            if left < arr.len() && arr[i] < arr[left] {
                return false;
            }
            if right < arr.len() && arr[i] < arr[right] {
                return false;
            }
        }
        true
    }

    // Calculate heap height
    fn heap_height(size: usize) -> usize {
        if size == 0 {
            0
        } else {
            (size as f64).log2().floor() as usize
        }
    }

    // Visualize heap structure (for small heaps)
    fn visualize_heap(arr: &[i32]) {
        if arr.is_empty() {
            println!("Empty heap");
            return;
        }

        let height = Self::heap_height(arr.len());
        let mut level = 0;
        let mut level_start = 0;

        while level <= height && level_start < arr.len() {
            let level_size = (1 << level).min(arr.len() - level_start);
            let spacing = " ".repeat((1 << (height - level)) - 1);
            
            print!("{}", spacing);
            for i in 0..level_size {
                if level_start + i < arr.len() {
                    print!("{:3}", arr[level_start + i]);
                    if i < level_size - 1 {
                        print!("{}", " ".repeat((1 << (height - level + 1)) - 1));
                    }
                }
            }
            println!();
            
            level_start += level_size;
            level += 1;
        }
    }

    // Analyze heap properties
    fn analyze_heap(arr: &[i32]) -> HeapProperties {
        HeapProperties {
            size: arr.len(),
            height: Self::heap_height(arr.len()),
            is_max_heap: Self::is_max_heap(arr),
            max_element: arr.first().cloned(),
            levels: Self::count_levels(arr.len()),
        }
    }

    fn count_levels(size: usize) -> Vec<usize> {
        let height = Self::heap_height(size);
        let mut levels = Vec::new();
        let mut remaining = size;
        
        for level in 0..=height {
            let level_capacity = 1 << level;
            let level_size = level_capacity.min(remaining);
            levels.push(level_size);
            remaining -= level_size;
            if remaining == 0 {
                break;
            }
        }
        
        levels
    }
}

// Result structures
#[derive(Debug, Clone)]
struct SortResult {
    algorithm: String,
    size: usize,
    time_ms: f64,
    comparisons: usize,
    swaps: usize,
    is_stable: bool,
    is_in_place: bool,
}

#[derive(Debug)]
struct HeapProperties {
    size: usize,
    height: usize,
    is_max_heap: bool,
    max_element: Option<i32>,
    levels: Vec<usize>,
}

// Test case generator
struct TestCases;

impl TestCases {
    fn generate_test_cases() -> Vec<(String, Vec<i32>)> {
        vec![
            ("Empty Array".to_string(), vec![]),
            ("Single Element".to_string(), vec![42]),
            ("Two Elements (Sorted)".to_string(), vec![1, 2]),
            ("Two Elements (Reverse)".to_string(), vec![2, 1]),
            ("Already Sorted".to_string(), vec![1, 2, 3, 4, 5, 6, 7, 8]),
            ("Reverse Sorted".to_string(), vec![8, 7, 6, 5, 4, 3, 2, 1]),
            ("Random Small".to_string(), vec![4, 2, 7, 1, 9, 3, 6, 5]),
            ("All Same".to_string(), vec![5, 5, 5, 5, 5, 5, 5]),
            ("Duplicates Mixed".to_string(), vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3]),
            ("Large Random".to_string(), Self::generate_random_array(1000, 1, 1000)),
            ("Nearly Sorted".to_string(), Self::generate_nearly_sorted(100)),
            ("Few Unique".to_string(), Self::generate_few_unique(50, 3)),
        ]
    }

    fn generate_random_array(size: usize, min: i32, max: i32) -> Vec<i32> {
        
        let mut result = Vec::with_capacity(size);
        let mut seed = 42u64;
        
        for _ in 0..size {
            // Simple LCG for deterministic "random" numbers
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let value = min + ((seed % (max - min + 1) as u64) as i32);
            result.push(value);
        }
        
        result
    }

    fn generate_nearly_sorted(size: usize) -> Vec<i32> {
        let mut arr: Vec<i32> = (1..=size as i32).collect();
        
        // Swap a few random elements
        let swaps = size / 10;
        let mut seed = 42u64;
        
        for _ in 0..swaps {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let i = (seed % size as u64) as usize;
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let j = (seed % size as u64) as usize;
            arr.swap(i, j);
        }
        
        arr
    }

    fn generate_few_unique(size: usize, unique_count: usize) -> Vec<i32> {
        let mut result = Vec::with_capacity(size);
        let mut seed = 42u64;
        
        for _ in 0..size {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let value = ((seed % unique_count as u64) + 1) as i32;
            result.push(value);
        }
        
        result
    }
}

// Verification functions
fn is_sorted(arr: &[i32]) -> bool {
    arr.windows(2).all(|w| w[0] <= w[1])
}

fn verify_sorting_correctness(original: &[i32], sorted: &[i32]) -> bool {
    if original.len() != sorted.len() {
        return false;
    }
    
    // Check if sorted array is actually sorted
    if !is_sorted(sorted) {
        return false;
    }
    
    // Check if it's a permutation of original (same elements, same counts)
    let mut orig_counts = std::collections::HashMap::new();
    let mut sort_counts = std::collections::HashMap::new();
    
    for &val in original {
        *orig_counts.entry(val).or_insert(0) += 1;
    }
    
    for &val in sorted {
        *sort_counts.entry(val).or_insert(0) += 1;
    }
    
    orig_counts == sort_counts
}

// Performance analysis
fn analyze_performance(results: &[SortResult]) {
    if results.is_empty() {
        return;
    }

    println!("\nPerformance Analysis:");
    println!("{}", "=".repeat(50));
    
    // Time analysis
    let total_time: f64 = results.iter().map(|r| r.time_ms).sum();
    let avg_time = total_time / results.len() as f64;
    let min_time = results.iter().map(|r| r.time_ms).fold(f64::INFINITY, f64::min);
    let max_time = results.iter().map(|r| r.time_ms).fold(0.0, f64::max);
    
    println!("Time (ms):");
    println!("  Average: {:.3}", avg_time);
    println!("  Min:     {:.3}", min_time);
    println!("  Max:     {:.3}", max_time);
    
    // Operations analysis
    let total_comparisons: usize = results.iter().map(|r| r.comparisons).sum();
    let total_swaps: usize = results.iter().map(|r| r.swaps).sum();
    
    println!("\nOperations:");
    println!("  Total Comparisons: {}", total_comparisons);
    println!("  Total Swaps:       {}", total_swaps);
    
    // Complexity analysis
    println!("\nComplexity Analysis:");
    if results.len() > 1 {
        // Find results with different sizes for complexity analysis
        let size_time_pairs: Vec<_> = results.iter()
            .map(|r| (r.size as f64, r.time_ms))
            .collect();
        
        println!("  Size vs Time (for complexity verification):");
        for (size, time) in &size_time_pairs {
            let theoretical_nlogn = size * size.log2();
            let ratio = time / theoretical_nlogn;
            println!("    Size: {}, Time: {:.3}ms, Ratio: {:.6}", 
                     *size as usize, time, ratio);
        }
    }
}

// Main test runner
fn run_test_case(name: &str, data: Vec<i32>) {
    println!("\n{}", "=".repeat(70));
    println!("Test Case: {}", name);
    println!("{}", "=".repeat(70));
    
    let original = data.clone();
    println!("Input size: {}", data.len());
    
    if data.len() <= 20 {
        println!("Input:  {:?}", data);
        
        // Show initial heap visualization if small enough
        if data.len() <= 15 && !data.is_empty() {
            println!("\nInitial array as heap structure:");
            HeapAnalyzer::visualize_heap(&data);
            
            let props = HeapAnalyzer::analyze_heap(&data);
            println!("Initial heap properties: {:?}", props);
        }
    }
    
    // Test recursive heap sort
    let mut heap_sort = HeapSort::new(true);
    let mut data_copy = data.clone();
    let result_recursive = heap_sort.sort(&mut data_copy);
    
    // Verify correctness
    let is_correct = verify_sorting_correctness(&original, &data_copy);
    
    if data.len() <= 20 {
        println!("Output: {:?}", data_copy);
    }
    
    // Test iterative heap sort for comparison
    let mut heap_sort_iter = HeapSort::new(true);
    let mut data_copy2 = data.clone();
    let result_iterative = heap_sort_iter.sort_iterative(&mut data_copy2);
    
    // Display results
    println!("\nResults:");
    println!("{}", "-".repeat(70));
    println!("{:<20} | {:>8} | {:>10} | {:>8} | {:>8} | {:>8}",
             "Algorithm", "Correct", "Time(ms)", "Compares", "Swaps", "Size");
    println!("{}", "-".repeat(70));
    
    println!("{:<20} | {:>8} | {:>10.3} | {:>8} | {:>8} | {:>8}",
             result_recursive.algorithm,
             if is_correct { "✓" } else { "✗" },
             result_recursive.time_ms,
             result_recursive.comparisons,
             result_recursive.swaps,
             result_recursive.size);
             
    println!("{:<20} | {:>8} | {:>10.3} | {:>8} | {:>8} | {:>8}",
             result_iterative.algorithm,
             if verify_sorting_correctness(&original, &data_copy2) { "✓" } else { "✗" },
             result_iterative.time_ms,
             result_iterative.comparisons,
             result_iterative.swaps,
             result_iterative.size);
    
    // Complexity analysis for this case
    if !data.is_empty() {
        let n = data.len() as f64;
        let theoretical_comparisons = n * n.log2(); // Approximate for heap sort
        let actual_comparisons = result_recursive.comparisons as f64;
        
        println!("\nComplexity Analysis for this case:");
        println!("  Theoretical O(n log n): {:.0} comparisons", theoretical_comparisons);
        println!("  Actual comparisons: {}", result_recursive.comparisons);
        println!("  Efficiency ratio: {:.2}", actual_comparisons / theoretical_comparisons);
    }
    
    // Show final heap properties if small enough
    if data.len() <= 15 && !data.is_empty() {
        println!("\nFinal sorted array:");
        if data.len() <= 10 {
            HeapAnalyzer::visualize_heap(&data_copy);
        }
        
        let final_props = HeapAnalyzer::analyze_heap(&data_copy);
        println!("Final properties: {:?}", final_props);
    }
}

fn main() {
    println!("Heap Sort - Comprehensive Implementation and Analysis");
    println!("{}", "=".repeat(70));
    
    let test_cases = TestCases::generate_test_cases();
    let mut all_results = Vec::new();
    
    // Run all test cases
    for (name, data) in test_cases {
        run_test_case(&name, data.clone());
        
        // Collect results for overall analysis
        if !data.is_empty() {
            let mut heap_sort = HeapSort::new(true);
            let mut data_copy = data.clone();
            let result = heap_sort.sort(&mut data_copy);
            all_results.push(result);
        }
    }
    
    // Overall performance analysis
    analyze_performance(&all_results);
    
    // Algorithm summary
    println!("\n\nAlgorithm Summary:");
    println!("{}", "=".repeat(70));
    println!("Heap Sort Characteristics:");
    println!("• Time Complexity:  O(n log n) in all cases");
    println!("• Space Complexity: O(1) auxiliary space (in-place)");
    println!("• Stability:        Not stable");
    println!("• Method:           Comparison-based");
    println!("• Best for:         Guaranteed O(n log n), memory-constrained environments");
    
    println!("\nHeap Operations:");
    println!("• Build Heap:       O(n) time");
    println!("• Heapify:          O(log n) time");
    println!("• Extract Max:      O(log n) time");
    println!("• Insert:           O(log n) time");
    
    println!("\nPractical Considerations:");
    println!("• Poor cache performance due to non-sequential access");
    println!("• Good worst-case guarantee (unlike QuickSort)");
    println!("• Useful for priority queue implementations");
    println!("• Often used in hybrid sorting algorithms");
    println!("• Suitable for external sorting applications");
    
    println!("\nImplementation Variants Demonstrated:");
    println!("• Recursive heapify (standard approach)");
    println!("• Iterative heapify (stack-efficient)");
    println!("• Bottom-up heap construction");
    println!("• Comprehensive performance tracking");
    println!("• Heap property verification");
    println!("• Visual heap representation");
}