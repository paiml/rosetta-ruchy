// Selection Sort Algorithm Implementation in Rust
// Demonstrates O(n²) quadratic complexity with empirical verification

use std::time::Instant;

/// Result structure for selection sort analysis
#[derive(Debug)]
struct SelectionSortResult {
    comparisons: usize,
    swaps: usize,
    time_ns: u128,
    space_complexity: String,
    complexity_verified: bool,
}

/// Selection sort implementation with detailed complexity tracking
/// 
/// Time Complexity: O(n²) - Always performs n(n-1)/2 comparisons
/// Space Complexity: O(1) - In-place sorting with constant extra space
/// 
/// Algorithm properties:
/// - Not stable (does not preserve relative order of equal elements)  
/// - Not adaptive (same performance regardless of input order)
/// - Minimal swaps (at most n-1 swaps)
/// - In-place sorting
fn selection_sort(arr: &mut [i32]) -> SelectionSortResult {
    let start_time = Instant::now();
    let n = arr.len();
    let mut comparisons = 0;
    let mut swaps = 0;
    
    // Edge case: arrays of size 0 or 1 are already sorted
    if n <= 1 {
        return SelectionSortResult {
            comparisons: 0,
            swaps: 0,
            time_ns: start_time.elapsed().as_nanos(),
            space_complexity: "O(1)".to_string(),
            complexity_verified: true,
        };
    }
    
    // Main selection sort algorithm
    for i in 0..n-1 {
        let mut min_idx = i;
        
        // Find the minimum element in the unsorted portion
        for j in (i+1)..n {
            comparisons += 1;
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        
        // Swap the found minimum element with the first element of unsorted portion
        if min_idx != i {
            arr.swap(i, min_idx);
            swaps += 1;
        }
        
        // Invariant check: elements 0..=i should be in sorted order
        debug_assert!(is_sorted_up_to(arr, i + 1), "Invariant violated: sorted portion not sorted");
        
        // Invariant check: sorted portion should be <= unsorted portion
        if i + 1 < n {
            debug_assert!(
                arr[i] <= *arr[i+1..].iter().min().unwrap_or(&arr[i]),
                "Invariant violated: sorted max > unsorted min"
            );
        }
    }
    
    let elapsed = start_time.elapsed().as_nanos();
    
    // Verify O(n²) complexity
    let expected_comparisons = n * (n - 1) / 2;
    let complexity_verified = verify_quadratic_complexity(n, comparisons, swaps);
    
    SelectionSortResult {
        comparisons,
        swaps,
        time_ns: elapsed,
        space_complexity: "O(1)".to_string(),
        complexity_verified,
    }
}

/// Verify that the algorithm achieves theoretical O(n²) complexity bounds
fn verify_quadratic_complexity(n: usize, comparisons: usize, swaps: usize) -> bool {
    let expected_comparisons = n * (n - 1) / 2;
    
    // Verify exact comparison count (selection sort always does n(n-1)/2 comparisons)
    let comparisons_correct = comparisons == expected_comparisons;
    
    // Verify swap count bounds (at most n-1 swaps)
    let swaps_correct = swaps <= n.saturating_sub(1);
    
    comparisons_correct && swaps_correct
}

/// Check if array is sorted up to index `up_to` (exclusive)
fn is_sorted_up_to(arr: &[i32], up_to: usize) -> bool {
    if up_to <= 1 { return true; }
    
    for i in 1..up_to.min(arr.len()) {
        if arr[i] < arr[i-1] {
            return false;
        }
    }
    true
}

/// Check if the entire array is sorted
fn is_sorted(arr: &[i32]) -> bool {
    is_sorted_up_to(arr, arr.len())
}

/// Comprehensive test suite for selection sort with complexity verification
fn run_complexity_tests() -> bool {
    let test_cases = vec![
        vec![],                        // Empty array
        vec![42],                      // Single element
        vec![5, 2, 8, 1, 9],          // Random case
        vec![1, 2, 3, 4, 5],          // Already sorted (best case)
        vec![5, 4, 3, 2, 1],          // Reverse sorted (worst case)  
        vec![3, 3, 3, 3, 3],          // All equal elements
        vec![2, 1],                    // Two elements
        vec![1, 3, 2, 4, 3, 5, 4],   // Duplicates mixed
    ];
    
    for (i, mut test_case) in test_cases.into_iter().enumerate() {
        let original = test_case.clone();
        let n = test_case.len();
        
        let result = selection_sort(&mut test_case);
        
        // Verify sorting correctness
        if !is_sorted(&test_case) {
            eprintln!("❌ Test case {} failed: array not sorted", i);
            eprintln!("Original: {:?}", original);
            eprintln!("Result: {:?}", test_case);
            return false;
        }
        
        // Verify O(n²) complexity
        if !result.complexity_verified {
            eprintln!("❌ Test case {} failed: complexity verification failed", i);
            eprintln!("Size: {}, Comparisons: {}, Swaps: {}", n, result.comparisons, result.swaps);
            return false;
        }
        
        // Verify expected comparison count for non-empty arrays
        if n > 1 {
            let expected_comparisons = n * (n - 1) / 2;
            if result.comparisons != expected_comparisons {
                eprintln!("❌ Test case {} failed: expected {} comparisons, got {}", 
                         i, expected_comparisons, result.comparisons);
                return false;
            }
        }
        
        println!("✅ Test case {}: n={}, comparisons={}, swaps={}, time={}ns", 
                 i, n, result.comparisons, result.swaps, result.time_ns);
    }
    
    true
}

/// Empirical complexity analysis with larger datasets
fn analyze_complexity() {
    println!("\n📊 EMPIRICAL COMPLEXITY ANALYSIS");
    println!("Size\tComparisons\tSwaps\tTime(ns)\tComplexity");
    println!("----\t-----------\t-----\t--------\t----------");
    
    let sizes = vec![10, 20, 50, 100];
    
    for &n in &sizes {
        // Generate random test data
        let mut data: Vec<i32> = (0..n as i32).rev().collect(); // Worst case: reverse sorted
        
        let result = selection_sort(&mut data);
        
        // Calculate complexity ratio compared to n²
        let theoretical_ops = n * (n - 1) / 2;
        let complexity_ratio = if theoretical_ops > 0 {
            result.comparisons as f64 / theoretical_ops as f64
        } else {
            0.0
        };
        
        println!("{}\t{}\t\t{}\t{}\t{:.3}x",
                 n, result.comparisons, result.swaps, result.time_ns, complexity_ratio);
        
        assert!(is_sorted(&data), "Array not properly sorted for size {}", n);
        assert!(result.complexity_verified, "Complexity verification failed for size {}", n);
    }
}

fn main() {
    println!("🎯 SELECTION SORT O(n²) COMPLEXITY VERIFICATION");
    println!("================================================");
    
    // Run comprehensive test suite
    if !run_complexity_tests() {
        eprintln!("❌ Test suite failed!");
        std::process::exit(1);
    }
    
    // Run empirical complexity analysis
    analyze_complexity();
    
    // Demonstration with sample data
    println!("\n🔍 DEMONSTRATION");
    let mut demo_data = vec![64, 34, 25, 12, 22, 11, 90];
    println!("Original: {:?}", demo_data);
    
    let result = selection_sort(&mut demo_data);
    
    println!("Sorted:   {:?}", demo_data);
    println!("\n📈 COMPLEXITY ANALYSIS");
    println!("Array size: {}", demo_data.len());
    println!("Comparisons: {} (expected: {})", result.comparisons, demo_data.len() * (demo_data.len() - 1) / 2);
    println!("Swaps: {} (max possible: {})", result.swaps, demo_data.len().saturating_sub(1));
    println!("Time: {} nanoseconds", result.time_ns);
    println!("Space complexity: {}", result.space_complexity);
    println!("Complexity verified: {}", result.complexity_verified);
    
    if result.complexity_verified && is_sorted(&demo_data) {
        println!("\n🎯 COMPLEXITY VERIFICATION SUCCESS: ✅ O(n²) time complexity mathematically verified");
        println!("Selection sort maintains exactly n(n-1)/2 comparisons regardless of input distribution");
        std::process::exit(0);
    } else {
        println!("\n❌ COMPLEXITY VERIFICATION FAILED");
        std::process::exit(1);
    }
}