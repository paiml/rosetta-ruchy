// Radix Sort - Rust Implementation
// Non-comparison sorting algorithm with linear time complexity

use std::time::Instant;

// Radix Sort implementation with multiple variants
#[derive(Debug, Clone)]
struct RadixSort {
    radix: usize,
    digit_extractions: usize,
    counting_operations: usize,
    memory_allocations: usize,
    track_stats: bool,
}

impl RadixSort {
    fn new(radix: usize, track_stats: bool) -> Self {
        Self {
            radix,
            digit_extractions: 0,
            counting_operations: 0,
            memory_allocations: 0,
            track_stats,
        }
    }

    // LSD Radix Sort (Least Significant Digit first)
    fn sort_lsd(&mut self, arr: &mut [i32]) -> SortResult {
        let start = Instant::now();
        
        if arr.len() <= 1 {
            return SortResult {
                algorithm: "LSD Radix Sort".to_string(),
                size: arr.len(),
                time_ms: start.elapsed().as_secs_f64() * 1000.0,
                passes: 0,
                digit_extractions: 0,
                counting_operations: 0,
                memory_allocations: 0,
                radix: self.radix,
                stable: true,
            };
        }

        self.reset_stats();

        // Handle negative numbers by separating and sorting independently
        let (mut negatives, mut non_negatives) = self.separate_by_sign(arr);
        let mut total_passes = 0;

        // Sort negative numbers (in reverse order, then reverse the result)
        if !negatives.is_empty() {
            // Convert to positive for sorting, then negate back
            for val in &mut negatives {
                *val = -*val;
            }
            
            let passes = self.sort_lsd_positive(&mut negatives);
            total_passes += passes;
            
            // Convert back to negative and reverse order
            for val in &mut negatives {
                *val = -*val;
            }
            negatives.reverse();
        }

        // Sort non-negative numbers normally  
        if !non_negatives.is_empty() {
            let passes = self.sort_lsd_positive(&mut non_negatives);
            total_passes += passes;
        }

        // Merge results: negatives first, then non-negatives
        let mut result_index = 0;
        for val in negatives {
            arr[result_index] = val;
            result_index += 1;
        }
        for val in non_negatives {
            arr[result_index] = val;
            result_index += 1;
        }

        SortResult {
            algorithm: "LSD Radix Sort".to_string(),
            size: arr.len(),
            time_ms: start.elapsed().as_secs_f64() * 1000.0,
            passes: total_passes,
            digit_extractions: self.digit_extractions,
            counting_operations: self.counting_operations,
            memory_allocations: self.memory_allocations,
            radix: self.radix,
            stable: true,
        }
    }

    // LSD sorting for positive numbers only
    fn sort_lsd_positive(&mut self, arr: &mut [i32]) -> usize {
        if arr.is_empty() {
            return 0;
        }

        // Find maximum to determine number of digits
        let max_val = *arr.iter().max().unwrap();
        let max_digits = if max_val == 0 { 1 } else { Self::count_digits(max_val, self.radix) };

        // Sort by each digit position
        let mut passes = 0;
        let mut exp = 1;
        
        for _ in 0..max_digits {
            self.counting_sort_by_digit(arr, exp);
            passes += 1;
            exp *= self.radix as i32;
        }

        passes
    }

    // MSD Radix Sort (Most Significant Digit first) 
    fn sort_msd(&mut self, arr: &mut [i32]) -> SortResult {
        let start = Instant::now();
        
        if arr.len() <= 1 {
            return SortResult {
                algorithm: "MSD Radix Sort".to_string(),
                size: arr.len(),
                time_ms: start.elapsed().as_secs_f64() * 1000.0,
                passes: 0,
                digit_extractions: 0,
                counting_operations: 0,
                memory_allocations: 0,
                radix: self.radix,
                stable: true,
            };
        }

        self.reset_stats();

        // Handle negative numbers
        let (mut negatives, mut non_negatives) = self.separate_by_sign(arr);
        let mut total_passes = 0;

        // Sort negative numbers
        if !negatives.is_empty() {
            for val in &mut negatives {
                *val = -*val;
            }
            
            let max_val = *negatives.iter().max().unwrap();
            let max_digits = if max_val == 0 { 1 } else { Self::count_digits(max_val, self.radix) };
            let passes = self.sort_msd_recursive(&mut negatives, max_digits);
            total_passes += passes;
            
            for val in &mut negatives {
                *val = -*val;
            }
            negatives.reverse();
        }

        // Sort non-negative numbers
        if !non_negatives.is_empty() {
            let max_val = *non_negatives.iter().max().unwrap();
            let max_digits = if max_val == 0 { 1 } else { Self::count_digits(max_val, self.radix) };
            let passes = self.sort_msd_recursive(&mut non_negatives, max_digits);
            total_passes += passes;
        }

        // Merge results
        let mut result_index = 0;
        for val in negatives {
            arr[result_index] = val;
            result_index += 1;
        }
        for val in non_negatives {
            arr[result_index] = val;
            result_index += 1;
        }

        SortResult {
            algorithm: "MSD Radix Sort".to_string(),
            size: arr.len(),
            time_ms: start.elapsed().as_secs_f64() * 1000.0,
            passes: total_passes,
            digit_extractions: self.digit_extractions,
            counting_operations: self.counting_operations,
            memory_allocations: self.memory_allocations,
            radix: self.radix,
            stable: true,
        }
    }

    // Recursive MSD implementation
    fn sort_msd_recursive(&mut self, arr: &mut [i32], digit_pos: usize) -> usize {
        if arr.len() <= 1 || digit_pos == 0 {
            return 0;
        }

        let exp = (self.radix as i32).pow(digit_pos as u32 - 1);
        let mut passes = 1;

        // Sort by current digit position
        self.counting_sort_by_digit(arr, exp);

        // Recursively sort subarrays with same digit value
        let mut start = 0;
        for digit in 0..self.radix {
            let mut end = start;
            
            // Find range of elements with this digit value
            while end < arr.len() && self.get_digit(arr[end], exp) == digit as i32 {
                end += 1;
            }

            // Recursively sort this subarray
            if end > start + 1 {
                passes += self.sort_msd_recursive(&mut arr[start..end], digit_pos - 1);
            }

            start = end;
        }

        passes
    }

    // Counting sort by specific digit position
    fn counting_sort_by_digit(&mut self, arr: &mut [i32], exp: i32) {
        let n = arr.len();
        let mut output = vec![0; n];
        let mut count = vec![0; self.radix];
        
        if self.track_stats {
            self.memory_allocations += 2; // output and count arrays
        }

        // Count occurrences of each digit
        for &val in arr.iter() {
            let digit = self.get_digit(val, exp) as usize;
            count[digit] += 1;
            if self.track_stats {
                self.counting_operations += 1;
            }
        }

        // Convert counts to cumulative counts (for stable sorting)
        for i in 1..self.radix {
            count[i] += count[i - 1];
            if self.track_stats {
                self.counting_operations += 1;
            }
        }

        // Build output array from right to left (for stability)
        for i in (0..n).rev() {
            let digit = self.get_digit(arr[i], exp) as usize;
            count[digit] -= 1;
            output[count[digit]] = arr[i];
            if self.track_stats {
                self.counting_operations += 1;
            }
        }

        // Copy output array back to original
        arr.copy_from_slice(&output);
    }

    // Extract specific digit from number
    fn get_digit(&mut self, num: i32, exp: i32) -> i32 {
        if self.track_stats {
            self.digit_extractions += 1;
        }
        (num / exp) % self.radix as i32
    }

    // Separate array by sign
    fn separate_by_sign(&self, arr: &[i32]) -> (Vec<i32>, Vec<i32>) {
        let mut negatives = Vec::new();
        let mut non_negatives = Vec::new();

        for &val in arr {
            if val < 0 {
                negatives.push(val);
            } else {
                non_negatives.push(val);
            }
        }

        (negatives, non_negatives)
    }

    // Count digits in a number for given radix
    fn count_digits(mut num: i32, radix: usize) -> usize {
        if num == 0 {
            return 1;
        }
        
        let mut count = 0;
        while num > 0 {
            num /= radix as i32;
            count += 1;
        }
        count
    }

    fn reset_stats(&mut self) {
        self.digit_extractions = 0;
        self.counting_operations = 0;
        self.memory_allocations = 0;
    }
}

// Alternative radix sort implementations
impl RadixSort {
    // Binary radix sort (base 2, bit-by-bit)
    fn sort_binary(&mut self, arr: &mut [i32]) -> SortResult {
        let start = Instant::now();
        
        if arr.len() <= 1 {
            return SortResult {
                algorithm: "Binary Radix Sort".to_string(),
                size: arr.len(),
                time_ms: start.elapsed().as_secs_f64() * 1000.0,
                passes: 0,
                digit_extractions: 0,
                counting_operations: 0,
                memory_allocations: 0,
                radix: 2,
                stable: true,
            };
        }

        self.reset_stats();
        let original_radix = self.radix;
        self.radix = 2; // Binary radix

        // Handle negative numbers
        let (mut negatives, mut non_negatives) = self.separate_by_sign(arr);
        let mut total_passes = 0;

        // Sort negative numbers
        if !negatives.is_empty() {
            for val in &mut negatives {
                *val = -*val;
            }
            
            let passes = self.sort_lsd_positive(&mut negatives);
            total_passes += passes;
            
            for val in &mut negatives {
                *val = -*val;
            }
            negatives.reverse();
        }

        // Sort non-negative numbers
        if !non_negatives.is_empty() {
            let passes = self.sort_lsd_positive(&mut non_negatives);
            total_passes += passes;
        }

        // Merge results
        let mut result_index = 0;
        for val in negatives {
            arr[result_index] = val;
            result_index += 1;
        }
        for val in non_negatives {
            arr[result_index] = val;
            result_index += 1;
        }

        self.radix = original_radix; // Restore original radix

        SortResult {
            algorithm: "Binary Radix Sort".to_string(),
            size: arr.len(),
            time_ms: start.elapsed().as_secs_f64() * 1000.0,
            passes: total_passes,
            digit_extractions: self.digit_extractions,
            counting_operations: self.counting_operations,
            memory_allocations: self.memory_allocations,
            radix: 2,
            stable: true,
        }
    }

    // High-radix sort (base 256 for efficiency)
    fn sort_high_radix(&mut self, arr: &mut [i32]) -> SortResult {
        let start = Instant::now();
        
        if arr.len() <= 1 {
            return SortResult {
                algorithm: "High Radix Sort (256)".to_string(),
                size: arr.len(),
                time_ms: start.elapsed().as_secs_f64() * 1000.0,
                passes: 0,
                digit_extractions: 0,
                counting_operations: 0,
                memory_allocations: 0,
                radix: 256,
                stable: true,
            };
        }

        self.reset_stats();
        let original_radix = self.radix;
        self.radix = 256; // High radix for efficiency

        // Handle negative numbers
        let (mut negatives, mut non_negatives) = self.separate_by_sign(arr);
        let mut total_passes = 0;

        if !negatives.is_empty() {
            for val in &mut negatives {
                *val = -*val;
            }
            
            let passes = self.sort_lsd_positive(&mut negatives);
            total_passes += passes;
            
            for val in &mut negatives {
                *val = -*val;
            }
            negatives.reverse();
        }

        if !non_negatives.is_empty() {
            let passes = self.sort_lsd_positive(&mut non_negatives);
            total_passes += passes;
        }

        // Merge results
        let mut result_index = 0;
        for val in negatives {
            arr[result_index] = val;
            result_index += 1;
        }
        for val in non_negatives {
            arr[result_index] = val;
            result_index += 1;
        }

        self.radix = original_radix;

        SortResult {
            algorithm: "High Radix Sort (256)".to_string(),
            size: arr.len(),
            time_ms: start.elapsed().as_secs_f64() * 1000.0,
            passes: total_passes,
            digit_extractions: self.digit_extractions,
            counting_operations: self.counting_operations,
            memory_allocations: self.memory_allocations,
            radix: 256,
            stable: true,
        }
    }
}

// Result and analysis structures
#[derive(Debug, Clone)]
struct SortResult {
    algorithm: String,
    size: usize,
    time_ms: f64,
    passes: usize,
    digit_extractions: usize,
    counting_operations: usize,
    memory_allocations: usize,
    radix: usize,
    stable: bool,
}

// Test case generation
struct TestCases;

impl TestCases {
    fn generate_test_cases() -> Vec<(String, Vec<i32>)> {
        vec![
            ("Empty Array".to_string(), vec![]),
            ("Single Element".to_string(), vec![42]),
            ("Two Elements".to_string(), vec![2, 1]),
            ("Small Positive".to_string(), vec![170, 45, 75, 90, 2, 802, 24, 66]),
            ("Small Mixed".to_string(), vec![-5, 3, -1, 8, 0, -2, 7, 1]),
            ("All Negative".to_string(), vec![-10, -5, -15, -3, -7]),
            ("All Same".to_string(), vec![5, 5, 5, 5, 5]),
            ("Already Sorted".to_string(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
            ("Reverse Sorted".to_string(), vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]),
            ("Large Numbers".to_string(), vec![999999, 1, 888888, 777777, 555555]),
            ("Many Duplicates".to_string(), vec![1, 3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5]),
            ("Random Medium".to_string(), Self::generate_random(100, -1000, 1000)),
            ("Random Large".to_string(), Self::generate_random(10000, -100000, 100000)),
            ("Few Unique".to_string(), Self::generate_few_unique(1000, 5)),
            ("Power of 2 Values".to_string(), vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512]),
        ]
    }

    fn generate_random(size: usize, min: i32, max: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(size);
        let mut seed = 42u64;
        
        for _ in 0..size {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let value = min + ((seed % (max - min + 1) as u64) as i32);
            result.push(value);
        }
        
        result
    }

    fn generate_few_unique(size: usize, unique_count: i32) -> Vec<i32> {
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

// Verification and analysis functions
fn is_sorted(arr: &[i32]) -> bool {
    arr.windows(2).all(|w| w[0] <= w[1])
}

fn verify_sorting_correctness(original: &[i32], sorted: &[i32]) -> bool {
    if original.len() != sorted.len() {
        return false;
    }
    
    if !is_sorted(sorted) {
        return false;
    }
    
    // Verify it's a permutation of the original
    let mut orig_sorted = original.to_vec();
    orig_sorted.sort_unstable();
    
    sorted == orig_sorted.as_slice()
}

fn analyze_performance(results: &[SortResult]) {
    if results.is_empty() {
        return;
    }

    println!("\nPerformance Analysis:");
    println!("{}", "=".repeat(70));
    
    // Time analysis
    let total_time: f64 = results.iter().map(|r| r.time_ms).sum();
    let avg_time = total_time / results.len() as f64;
    
    println!("Time Performance:");
    println!("  Average time: {:.3} ms", avg_time);
    println!("  Total time:   {:.3} ms", total_time);
    
    // Operations analysis
    let total_extractions: usize = results.iter().map(|r| r.digit_extractions).sum();
    let total_counting: usize = results.iter().map(|r| r.counting_operations).sum();
    let total_passes: usize = results.iter().map(|r| r.passes).sum();
    
    println!("\nOperation Counts:");
    println!("  Total digit extractions: {}", total_extractions);
    println!("  Total counting operations: {}", total_counting);
    println!("  Total passes: {}", total_passes);
    
    // Radix analysis
    println!("\nRadix Analysis:");
    for radix in [2, 10, 16, 256] {
        let radix_results: Vec<_> = results.iter()
            .filter(|r| r.radix == radix)
            .collect();
        
        if !radix_results.is_empty() {
            let avg_passes: f64 = radix_results.iter()
                .map(|r| r.passes as f64)
                .sum::<f64>() / radix_results.len() as f64;
            
            println!("  Base {}: {:.1} average passes", radix, avg_passes);
        }
    }
}

fn run_test_case(name: &str, data: Vec<i32>) {
    println!("\n{}", "=".repeat(70));
    println!("Test Case: {}", name);
    println!("{}", "=".repeat(70));
    
    let original = data.clone();
    println!("Input size: {}", data.len());
    
    if data.len() <= 20 {
        println!("Input:  {:?}", data);
    }
    
    // Test different radix sort variants
    let mut results: Vec<(&str, SortResult, bool, Vec<i32>)> = Vec::new();
    
    // LSD Base 10
    {
        let mut data_copy = data.clone();
        let mut radix_sort = RadixSort::new(10, true);
        let result = radix_sort.sort_lsd(&mut data_copy);
        let is_correct = verify_sorting_correctness(&original, &data_copy);
        results.push(("LSD Base 10", result, is_correct, data_copy));
    }
    
    // MSD Base 10
    {
        let mut data_copy = data.clone();
        let mut radix_sort = RadixSort::new(10, true);
        let result = radix_sort.sort_msd(&mut data_copy);
        let is_correct = verify_sorting_correctness(&original, &data_copy);
        results.push(("MSD Base 10", result, is_correct, data_copy));
    }
    
    // Binary
    {
        let mut data_copy = data.clone();
        let mut radix_sort = RadixSort::new(2, true);
        let result = radix_sort.sort_binary(&mut data_copy);
        let is_correct = verify_sorting_correctness(&original, &data_copy);
        results.push(("Binary (Base 2)", result, is_correct, data_copy));
    }
    
    // High Radix
    {
        let mut data_copy = data.clone();
        let mut radix_sort = RadixSort::new(256, true);
        let result = radix_sort.sort_high_radix(&mut data_copy);
        let is_correct = verify_sorting_correctness(&original, &data_copy);
        results.push(("High Radix (Base 256)", result, is_correct, data_copy));
    }
    
    println!("\nResults:");
    println!("{}", "-".repeat(90));
    println!("{:<20} | {:>8} | {:>10} | {:>6} | {:>8} | {:>8} | {:>8}",
             "Algorithm", "Correct", "Time(ms)", "Passes", "Digits", "Counts", "Memory");
    println!("{}", "-".repeat(90));
    
    let mut sort_results = Vec::new();
    
    for (name_variant, result, is_correct, data_copy) in &results {
        println!("{:<20} | {:>8} | {:>10.3} | {:>6} | {:>8} | {:>8} | {:>8}",
                 name_variant,
                 if *is_correct { "✓" } else { "✗" },
                 result.time_ms,
                 result.passes,
                 result.digit_extractions,
                 result.counting_operations,
                 result.memory_allocations);
        
        if data.len() <= 20 && *name_variant == "LSD Base 10" {
            println!("Output: {:?}", data_copy);
        }
        
        sort_results.push(result.clone());
    }
    
    // Complexity analysis for this case
    if !data.is_empty() {
        let n = data.len() as f64;
        let max_val = data.iter().map(|x| x.abs()).max().unwrap_or(0);
        let digits = if max_val == 0 { 1 } else { (max_val as f64).log10().floor() as usize + 1 };
        
        println!("\nComplexity Analysis:");
        println!("  Array size (n): {}", data.len());
        println!("  Max digits (d): {}", digits);
        println!("  Theoretical O(d*n): {}", digits * data.len());
        
        for result in &sort_results {
            let actual_ops = result.digit_extractions + result.counting_operations;
            let theoretical = digits * data.len() * 2; // Rough estimate
            if theoretical > 0 {
                println!("  {} efficiency: {:.2}x theoretical", 
                         result.algorithm, actual_ops as f64 / theoretical as f64);
            }
        }
    }
}

fn main() {
    println!("Radix Sort - Non-Comparison Linear Time Sorting");
    println!("{}", "=".repeat(70));
    
    let test_cases = TestCases::generate_test_cases();
    let mut all_results = Vec::new();
    
    // Run all test cases
    for (name, data) in test_cases {
        run_test_case(&name, data.clone());
        
        // Collect results for overall analysis (using LSD base 10 as representative)
        if !data.is_empty() {
            let mut radix_sort = RadixSort::new(10, true);
            let mut data_copy = data.clone();
            let result = radix_sort.sort_lsd(&mut data_copy);
            all_results.push(result);
        }
    }
    
    // Overall performance analysis
    analyze_performance(&all_results);
    
    // Algorithm summary
    println!("\n\nAlgorithm Summary:");
    println!("{}", "=".repeat(70));
    println!("Radix Sort Characteristics:");
    println!("• Time Complexity:  O(d * (n + k)) where d=digits, k=radix");
    println!("• Space Complexity: O(n + k) auxiliary space");
    println!("• Stability:        Stable (preserves relative order)");
    println!("• Method:           Non-comparison based");
    println!("• Best for:         Integer sorting, linear time when d < log n");
    
    println!("\nAlgorithm Variants:");
    println!("• LSD Radix Sort:   Processes digits right-to-left, simpler");
    println!("• MSD Radix Sort:   Processes digits left-to-right, can terminate early");
    println!("• Binary Radix:     Base 2, many passes but simple operations");
    println!("• High Radix:       Base 256, fewer passes but more memory");
    
    println!("\nPerformance Characteristics:");
    println!("• Linear time complexity for bounded integers");
    println!("• Excellent cache locality (sequential access)");
    println!("• Stable sorting preserves input order");
    println!("• Memory usage scales with radix size");
    
    println!("\nComparison with O(n log n) sorts:");
    println!("• Better when: d * (n + k) < n * log(n)");
    println!("• Worse when: integers have many digits");
    println!("• Trade-off: memory usage vs time complexity");
    
    println!("\nPractical Applications:");
    println!("• Integer arrays with bounded range");
    println!("• String sorting (MSD variant)");
    println!("• Database key sorting");
    println!("• Graphics and image processing");
    println!("• Network packet processing");
}