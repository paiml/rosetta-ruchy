// Bucket Sort - Rust Implementation
// Distribution-based sorting with linear average-case complexity

use std::time::Instant;

// Bucket Sort implementation with multiple strategies
#[derive(Debug, Clone)]
struct BucketSort {
    bucket_count: usize,
    distribution_strategy: DistributionStrategy,
    subroutine_algorithm: SubroutineAlgorithm,
    bucket_assignments: usize,
    subroutine_calls: usize,
    total_subroutine_operations: usize,
    track_stats: bool,
}

#[derive(Debug, Clone, PartialEq)]
enum DistributionStrategy {
    Linear,        // Linear mapping based on min/max range
    Logarithmic,   // For exponentially distributed data
    Quantile,      // Use data quantiles for balanced buckets
    Hash,          // Custom hash function distribution
}

#[derive(Debug, Clone, PartialEq)]
enum SubroutineAlgorithm {
    InsertionSort, // O(n²), good for small buckets
    QuickSort,     // O(n log n), general purpose
    MergeSort,     // O(n log n), stable
    RadixSort,     // O(d*n), for integers
}

impl BucketSort {
    fn new(bucket_count: usize, strategy: DistributionStrategy, subroutine: SubroutineAlgorithm, track_stats: bool) -> Self {
        Self {
            bucket_count,
            distribution_strategy: strategy,
            subroutine_algorithm: subroutine,
            bucket_assignments: 0,
            subroutine_calls: 0,
            total_subroutine_operations: 0,
            track_stats,
        }
    }

    // Main bucket sort algorithm
    fn sort(&mut self, arr: &mut [f64]) -> SortResult {
        let start = Instant::now();
        
        if arr.len() <= 1 {
            return SortResult {
                algorithm: format!("Bucket Sort ({:?})", self.subroutine_algorithm),
                size: arr.len(),
                time_ms: start.elapsed().as_secs_f64() * 1000.0,
                bucket_count: self.bucket_count,
                bucket_assignments: 0,
                subroutine_calls: 0,
                total_subroutine_operations: 0,
                distribution_strategy: self.distribution_strategy.clone(),
                average_bucket_size: 0.0,
                max_bucket_size: 0,
                empty_buckets: self.bucket_count,
                load_factor: 0.0,
            };
        }

        self.reset_stats();

        // Create buckets
        let mut buckets: Vec<Vec<f64>> = vec![Vec::new(); self.bucket_count];
        
        // Phase 1: Distribute elements into buckets
        self.distribute_elements(arr, &mut buckets);
        
        // Phase 2: Sort individual buckets
        self.sort_buckets(&mut buckets);
        
        // Phase 3: Concatenate sorted buckets
        self.concatenate_buckets(&buckets, arr);
        
        // Calculate statistics
        let bucket_sizes: Vec<usize> = buckets.iter().map(|b| b.len()).collect();
        let non_empty_buckets = bucket_sizes.iter().filter(|&&size| size > 0).count();
        let empty_buckets = self.bucket_count - non_empty_buckets;
        let max_bucket_size = bucket_sizes.iter().max().cloned().unwrap_or(0);
        let average_bucket_size = if non_empty_buckets > 0 {
            arr.len() as f64 / non_empty_buckets as f64
        } else {
            0.0
        };
        let load_factor = if average_bucket_size > 0.0 {
            max_bucket_size as f64 / average_bucket_size
        } else {
            0.0
        };

        SortResult {
            algorithm: format!("Bucket Sort ({:?})", self.subroutine_algorithm),
            size: arr.len(),
            time_ms: start.elapsed().as_secs_f64() * 1000.0,
            bucket_count: self.bucket_count,
            bucket_assignments: self.bucket_assignments,
            subroutine_calls: self.subroutine_calls,
            total_subroutine_operations: self.total_subroutine_operations,
            distribution_strategy: self.distribution_strategy.clone(),
            average_bucket_size,
            max_bucket_size,
            empty_buckets,
            load_factor,
        }
    }

    // Integer bucket sort variant
    fn sort_integers(&mut self, arr: &mut [i32]) -> SortResult {
        let start = Instant::now();
        
        if arr.len() <= 1 {
            return SortResult {
                algorithm: format!("Integer Bucket Sort ({:?})", self.subroutine_algorithm),
                size: arr.len(),
                time_ms: start.elapsed().as_secs_f64() * 1000.0,
                bucket_count: self.bucket_count,
                bucket_assignments: 0,
                subroutine_calls: 0,
                total_subroutine_operations: 0,
                distribution_strategy: self.distribution_strategy.clone(),
                average_bucket_size: 0.0,
                max_bucket_size: 0,
                empty_buckets: self.bucket_count,
                load_factor: 0.0,
            };
        }

        self.reset_stats();

        // Find min and max for range calculation
        let min_val = *arr.iter().min().unwrap();
        let max_val = *arr.iter().max().unwrap();
        let range = (max_val - min_val + 1) as f64;

        // Create buckets
        let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); self.bucket_count];
        
        // Phase 1: Distribute elements
        for &val in arr.iter() {
            let bucket_index = self.get_integer_bucket_index(val, min_val, range);
            buckets[bucket_index].push(val);
            if self.track_stats {
                self.bucket_assignments += 1;
            }
        }
        
        // Phase 2: Sort individual buckets
        for bucket in buckets.iter_mut() {
            if !bucket.is_empty() {
                self.sort_integer_bucket(bucket);
                if self.track_stats {
                    self.subroutine_calls += 1;
                    self.total_subroutine_operations += bucket.len() * bucket.len().ilog2() as usize; // Approximation
                }
            }
        }
        
        // Phase 3: Concatenate
        let mut index = 0;
        for bucket in buckets.iter() {
            for &val in bucket {
                arr[index] = val;
                index += 1;
            }
        }
        
        // Calculate statistics
        let bucket_sizes: Vec<usize> = buckets.iter().map(|b| b.len()).collect();
        let non_empty_buckets = bucket_sizes.iter().filter(|&&size| size > 0).count();
        let empty_buckets = self.bucket_count - non_empty_buckets;
        let max_bucket_size = bucket_sizes.iter().max().cloned().unwrap_or(0);
        let average_bucket_size = if non_empty_buckets > 0 {
            arr.len() as f64 / non_empty_buckets as f64
        } else {
            0.0
        };
        let load_factor = if average_bucket_size > 0.0 {
            max_bucket_size as f64 / average_bucket_size
        } else {
            0.0
        };

        SortResult {
            algorithm: format!("Integer Bucket Sort ({:?})", self.subroutine_algorithm),
            size: arr.len(),
            time_ms: start.elapsed().as_secs_f64() * 1000.0,
            bucket_count: self.bucket_count,
            bucket_assignments: self.bucket_assignments,
            subroutine_calls: self.subroutine_calls,
            total_subroutine_operations: self.total_subroutine_operations,
            distribution_strategy: self.distribution_strategy.clone(),
            average_bucket_size,
            max_bucket_size,
            empty_buckets,
            load_factor,
        }
    }

    // Distribution phase: scatter elements into buckets
    fn distribute_elements(&mut self, arr: &[f64], buckets: &mut [Vec<f64>]) {
        let min_val = arr.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_val = arr.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let range = max_val - min_val;

        for &val in arr {
            let bucket_index = self.get_bucket_index(val, min_val, range);
            buckets[bucket_index].push(val);
            if self.track_stats {
                self.bucket_assignments += 1;
            }
        }
    }

    // Get bucket index based on distribution strategy
    fn get_bucket_index(&self, value: f64, min_val: f64, range: f64) -> usize {
        match self.distribution_strategy {
            DistributionStrategy::Linear => {
                if range == 0.0 {
                    0
                } else {
                    let normalized = (value - min_val) / range;
                    let index = (normalized * self.bucket_count as f64).floor() as usize;
                    index.min(self.bucket_count - 1)
                }
            }
            DistributionStrategy::Logarithmic => {
                if value <= 0.0 {
                    0
                } else {
                    let log_val = value.ln();
                    let log_min = (min_val + 1e-10).ln(); // Avoid log(0)
                    let log_max = (min_val + range + 1e-10).ln();
                    let log_range = log_max - log_min;
                    
                    if log_range == 0.0 {
                        0
                    } else {
                        let normalized = (log_val - log_min) / log_range;
                        let index = (normalized * self.bucket_count as f64).floor() as usize;
                        index.min(self.bucket_count - 1)
                    }
                }
            }
            DistributionStrategy::Hash => {
                // Simple hash function for demonstration
                let hash = ((value * 31.0) as u64).wrapping_mul(2654435761);
                (hash as usize) % self.bucket_count
            }
            DistributionStrategy::Quantile => {
                // For simplicity, fall back to linear. In practice, would pre-compute quantiles
                if range == 0.0 {
                    0
                } else {
                    let normalized = (value - min_val) / range;
                    let index = (normalized * self.bucket_count as f64).floor() as usize;
                    index.min(self.bucket_count - 1)
                }
            }
        }
    }

    // Get bucket index for integers
    fn get_integer_bucket_index(&self, value: i32, min_val: i32, range: f64) -> usize {
        if range == 0.0 {
            0
        } else {
            let normalized = (value - min_val) as f64 / range;
            let index = (normalized * self.bucket_count as f64).floor() as usize;
            index.min(self.bucket_count - 1)
        }
    }

    // Sort individual buckets using chosen subroutine
    fn sort_buckets(&mut self, buckets: &mut [Vec<f64>]) {
        for bucket in buckets.iter_mut() {
            if !bucket.is_empty() {
                self.sort_bucket(bucket);
                if self.track_stats {
                    self.subroutine_calls += 1;
                    self.total_subroutine_operations += self.estimate_operations(bucket.len());
                }
            }
        }
    }

    // Sort single bucket using selected algorithm
    fn sort_bucket(&self, bucket: &mut [f64]) {
        match self.subroutine_algorithm {
            SubroutineAlgorithm::InsertionSort => {
                self.insertion_sort(bucket);
            }
            SubroutineAlgorithm::QuickSort => {
                if bucket.len() > 1 {
                    self.quicksort(bucket, 0, bucket.len() - 1);
                }
            }
            SubroutineAlgorithm::MergeSort => {
                if bucket.len() > 1 {
                    let mut temp = vec![0.0; bucket.len()];
                    self.merge_sort(bucket, &mut temp, 0, bucket.len() - 1);
                }
            }
            SubroutineAlgorithm::RadixSort => {
                // Convert to integers for radix sort (multiply by 1000 for precision)
                let mut int_bucket: Vec<i32> = bucket.iter()
                    .map(|&x| (x * 1000.0).round() as i32)
                    .collect();
                self.radix_sort_integers(&mut int_bucket);
                
                // Convert back to floats
                for (i, &val) in int_bucket.iter().enumerate() {
                    bucket[i] = val as f64 / 1000.0;
                }
            }
        }
    }

    // Sort integer bucket
    fn sort_integer_bucket(&self, bucket: &mut [i32]) {
        match self.subroutine_algorithm {
            SubroutineAlgorithm::InsertionSort => {
                self.insertion_sort_integers(bucket);
            }
            SubroutineAlgorithm::QuickSort => {
                if bucket.len() > 1 {
                    self.quicksort_integers(bucket, 0, bucket.len() - 1);
                }
            }
            SubroutineAlgorithm::MergeSort => {
                if bucket.len() > 1 {
                    let mut temp = vec![0; bucket.len()];
                    self.merge_sort_integers(bucket, &mut temp, 0, bucket.len() - 1);
                }
            }
            SubroutineAlgorithm::RadixSort => {
                self.radix_sort_integers(bucket);
            }
        }
    }

    // Concatenate sorted buckets back into original array
    fn concatenate_buckets(&self, buckets: &[Vec<f64>], arr: &mut [f64]) {
        let mut index = 0;
        for bucket in buckets {
            for &val in bucket {
                arr[index] = val;
                index += 1;
            }
        }
    }

    // Estimate operations for complexity analysis
    fn estimate_operations(&self, n: usize) -> usize {
        match self.subroutine_algorithm {
            SubroutineAlgorithm::InsertionSort => n * n / 2, // O(n²)
            SubroutineAlgorithm::QuickSort => n * n.ilog2() as usize, // O(n log n)
            SubroutineAlgorithm::MergeSort => n * n.ilog2() as usize, // O(n log n)
            SubroutineAlgorithm::RadixSort => n * 10, // O(d*n), assume d=10 for floats
        }
    }

    // Subroutine implementations
    fn insertion_sort(&self, arr: &mut [f64]) {
        for i in 1..arr.len() {
            let key = arr[i];
            let mut j = i;
            while j > 0 && arr[j - 1] > key {
                arr[j] = arr[j - 1];
                j -= 1;
            }
            arr[j] = key;
        }
    }

    fn insertion_sort_integers(&self, arr: &mut [i32]) {
        for i in 1..arr.len() {
            let key = arr[i];
            let mut j = i;
            while j > 0 && arr[j - 1] > key {
                arr[j] = arr[j - 1];
                j -= 1;
            }
            arr[j] = key;
        }
    }

    fn quicksort(&self, arr: &mut [f64], low: usize, high: usize) {
        if low < high {
            let pi = self.partition(arr, low, high);
            if pi > 0 {
                self.quicksort(arr, low, pi - 1);
            }
            self.quicksort(arr, pi + 1, high);
        }
    }

    fn quicksort_integers(&self, arr: &mut [i32], low: usize, high: usize) {
        if low < high {
            let pi = self.partition_integers(arr, low, high);
            if pi > 0 {
                self.quicksort_integers(arr, low, pi - 1);
            }
            self.quicksort_integers(arr, pi + 1, high);
        }
    }

    fn partition(&self, arr: &mut [f64], low: usize, high: usize) -> usize {
        let pivot = arr[high];
        let mut i = low;
        
        for j in low..high {
            if arr[j] <= pivot {
                arr.swap(i, j);
                i += 1;
            }
        }
        arr.swap(i, high);
        i
    }

    fn partition_integers(&self, arr: &mut [i32], low: usize, high: usize) -> usize {
        let pivot = arr[high];
        let mut i = low;
        
        for j in low..high {
            if arr[j] <= pivot {
                arr.swap(i, j);
                i += 1;
            }
        }
        arr.swap(i, high);
        i
    }

    fn merge_sort(&self, arr: &mut [f64], temp: &mut [f64], left: usize, right: usize) {
        if left < right {
            let mid = left + (right - left) / 2;
            self.merge_sort(arr, temp, left, mid);
            self.merge_sort(arr, temp, mid + 1, right);
            self.merge(arr, temp, left, mid, right);
        }
    }

    fn merge_sort_integers(&self, arr: &mut [i32], temp: &mut [i32], left: usize, right: usize) {
        if left < right {
            let mid = left + (right - left) / 2;
            self.merge_sort_integers(arr, temp, left, mid);
            self.merge_sort_integers(arr, temp, mid + 1, right);
            self.merge_integers(arr, temp, left, mid, right);
        }
    }

    fn merge(&self, arr: &mut [f64], temp: &mut [f64], left: usize, mid: usize, right: usize) {
        let mut i = left;
        let mut j = mid + 1;
        let mut k = left;

        while i <= mid && j <= right {
            if arr[i] <= arr[j] {
                temp[k] = arr[i];
                i += 1;
            } else {
                temp[k] = arr[j];
                j += 1;
            }
            k += 1;
        }

        while i <= mid {
            temp[k] = arr[i];
            i += 1;
            k += 1;
        }

        while j <= right {
            temp[k] = arr[j];
            j += 1;
            k += 1;
        }

        for i in left..=right {
            arr[i] = temp[i];
        }
    }

    fn merge_integers(&self, arr: &mut [i32], temp: &mut [i32], left: usize, mid: usize, right: usize) {
        let mut i = left;
        let mut j = mid + 1;
        let mut k = left;

        while i <= mid && j <= right {
            if arr[i] <= arr[j] {
                temp[k] = arr[i];
                i += 1;
            } else {
                temp[k] = arr[j];
                j += 1;
            }
            k += 1;
        }

        while i <= mid {
            temp[k] = arr[i];
            i += 1;
            k += 1;
        }

        while j <= right {
            temp[k] = arr[j];
            j += 1;
            k += 1;
        }

        for i in left..=right {
            arr[i] = temp[i];
        }
    }

    // Simple radix sort for integers (LSD)
    fn radix_sort_integers(&self, arr: &mut [i32]) {
        if arr.is_empty() {
            return;
        }

        let max_val = *arr.iter().max().unwrap();
        let mut exp = 1;

        while max_val / exp > 0 {
            self.counting_sort_by_digit(arr, exp);
            exp *= 10;
        }
    }

    fn counting_sort_by_digit(&self, arr: &mut [i32], exp: i32) {
        let n = arr.len();
        let mut output = vec![0; n];
        let mut count = vec![0; 10];

        // Count occurrences
        for &val in arr.iter() {
            count[((val / exp) % 10) as usize] += 1;
        }

        // Convert to cumulative counts
        for i in 1..10 {
            count[i] += count[i - 1];
        }

        // Build output array
        for i in (0..n).rev() {
            let digit = ((arr[i] / exp) % 10) as usize;
            count[digit] -= 1;
            output[count[digit]] = arr[i];
        }

        // Copy back
        arr.copy_from_slice(&output);
    }

    fn reset_stats(&mut self) {
        self.bucket_assignments = 0;
        self.subroutine_calls = 0;
        self.total_subroutine_operations = 0;
    }
}

// Result and analysis structures
#[derive(Debug, Clone)]
struct SortResult {
    algorithm: String,
    size: usize,
    time_ms: f64,
    bucket_count: usize,
    bucket_assignments: usize,
    subroutine_calls: usize,
    total_subroutine_operations: usize,
    distribution_strategy: DistributionStrategy,
    average_bucket_size: f64,
    max_bucket_size: usize,
    empty_buckets: usize,
    load_factor: f64,
}

// Test case generation
struct TestCases;

impl TestCases {
    fn generate_float_test_cases() -> Vec<(String, Vec<f64>)> {
        vec![
            ("Empty Array".to_string(), vec![]),
            ("Single Element".to_string(), vec![0.5]),
            ("Two Elements".to_string(), vec![0.3, 0.7]),
            ("Uniform [0,1)".to_string(), Self::generate_uniform_float(100, 0.0, 1.0)),
            ("Uniform [0,10)".to_string(), Self::generate_uniform_float(50, 0.0, 10.0)),
            ("Normal Distribution".to_string(), Self::generate_normal_float(100, 5.0, 2.0)),
            ("Exponential".to_string(), Self::generate_exponential_float(80, 2.0)),
            ("Already Sorted".to_string(), Self::generate_sorted_float(30, 0.0, 5.0)),
            ("Reverse Sorted".to_string(), Self::generate_reverse_sorted_float(30, 0.0, 5.0)),
            ("Few Unique Values".to_string(), Self::generate_few_unique_float(100, 5)),
            ("Skewed Distribution".to_string(), Self::generate_skewed_float(100)),
            ("Large Uniform".to_string(), Self::generate_uniform_float(10000, 0.0, 100.0)),
        ]
    }

    fn generate_integer_test_cases() -> Vec<(String, Vec<i32>)> {
        vec![
            ("Empty Array".to_string(), vec![]),
            ("Single Element".to_string(), vec![42]),
            ("Small Range".to_string(), vec![3, 1, 4, 1, 5, 9, 2, 6]),
            ("Large Range".to_string(), vec![100, 5, 999, 234, 567, 1, 789]),
            ("Negative Numbers".to_string(), vec![-5, 3, -1, 8, 0, -10, 7]),
            ("Already Sorted".to_string(), (1..=50).collect()),
            ("Reverse Sorted".to_string(), (1..=50).rev().collect()),
            ("Many Duplicates".to_string(), vec![1, 3, 1, 3, 1, 3, 2, 2, 2]),
            ("Uniform Distribution".to_string(), Self::generate_uniform_int(100, 1, 1000)),
            ("Large Dataset".to_string(), Self::generate_uniform_int(5000, 1, 10000)),
        ]
    }

    fn generate_uniform_float(size: usize, min: f64, max: f64) -> Vec<f64> {
        let mut result = Vec::with_capacity(size);
        let mut seed = 42u64;
        let range = max - min;
        
        for _ in 0..size {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let normalized = (seed as f64) / (u64::MAX as f64);
            result.push(min + normalized * range);
        }
        
        result
    }

    fn generate_uniform_int(size: usize, min: i32, max: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(size);
        let mut seed = 42u64;
        let range = (max - min + 1) as u64;
        
        for _ in 0..size {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let value = min + ((seed % range) as i32);
            result.push(value);
        }
        
        result
    }

    fn generate_normal_float(size: usize, mean: f64, std_dev: f64) -> Vec<f64> {
        let mut result = Vec::with_capacity(size);
        let mut seed = 42u64;
        
        for i in 0..size {
            // Box-Muller transform for normal distribution
            if i % 2 == 0 {
                seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
                let u1 = (seed as f64) / (u64::MAX as f64);
                seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
                let u2 = (seed as f64) / (u64::MAX as f64);
                
                let z0 = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
                result.push(mean + std_dev * z0);
            }
        }
        
        // Fill remaining if odd size
        while result.len() < size {
            result.push(mean);
        }
        
        result
    }

    fn generate_exponential_float(size: usize, lambda: f64) -> Vec<f64> {
        let mut result = Vec::with_capacity(size);
        let mut seed = 42u64;
        
        for _ in 0..size {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let u = (seed as f64) / (u64::MAX as f64);
            let value = -u.ln() / lambda;
            result.push(value);
        }
        
        result
    }

    fn generate_sorted_float(size: usize, min: f64, max: f64) -> Vec<f64> {
        let mut result = Self::generate_uniform_float(size, min, max);
        result.sort_by(|a, b| a.partial_cmp(b).unwrap());
        result
    }

    fn generate_reverse_sorted_float(size: usize, min: f64, max: f64) -> Vec<f64> {
        let mut result = Self::generate_uniform_float(size, min, max);
        result.sort_by(|a, b| b.partial_cmp(a).unwrap());
        result
    }

    fn generate_few_unique_float(size: usize, unique_count: usize) -> Vec<f64> {
        let unique_values: Vec<f64> = (0..unique_count).map(|i| i as f64).collect();
        let mut result = Vec::with_capacity(size);
        let mut seed = 42u64;
        
        for _ in 0..size {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let index = (seed % unique_count as u64) as usize;
            result.push(unique_values[index]);
        }
        
        result
    }

    fn generate_skewed_float(size: usize) -> Vec<f64> {
        let mut result = Vec::with_capacity(size);
        
        // 80% of elements in first 20% of range
        for i in 0..(size * 4 / 5) {
            result.push((i as f64 / size as f64) * 0.2);
        }
        
        // 20% of elements in remaining 80% of range
        for i in 0..(size / 5) {
            result.push(0.2 + (i as f64 / (size / 5) as f64) * 0.8);
        }
        
        result
    }
}

// Verification and analysis functions
fn is_sorted_float(arr: &[f64]) -> bool {
    arr.windows(2).all(|w| w[0] <= w[1])
}

fn is_sorted_int(arr: &[i32]) -> bool {
    arr.windows(2).all(|w| w[0] <= w[1])
}

fn verify_sorting_correctness_float(original: &[f64], sorted: &[f64]) -> bool {
    if original.len() != sorted.len() {
        return false;
    }
    
    if !is_sorted_float(sorted) {
        return false;
    }
    
    // Verify it's a permutation (same elements, same counts)
    let mut orig_sorted = original.to_vec();
    orig_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    sorted.len() == orig_sorted.len() && 
    sorted.iter().zip(orig_sorted.iter()).all(|(a, b)| (a - b).abs() < f64::EPSILON)
}

fn verify_sorting_correctness_int(original: &[i32], sorted: &[i32]) -> bool {
    if original.len() != sorted.len() {
        return false;
    }
    
    if !is_sorted_int(sorted) {
        return false;
    }
    
    let mut orig_sorted = original.to_vec();
    orig_sorted.sort_unstable();
    
    sorted == orig_sorted.as_slice()
}

fn analyze_distribution(data: &[f64]) -> DistributionAnalysis {
    if data.is_empty() {
        return DistributionAnalysis {
            mean: 0.0,
            variance: 0.0,
            min: 0.0,
            max: 0.0,
            range: 0.0,
            uniformity_score: 0.0,
        };
    }

    let mean = data.iter().sum::<f64>() / data.len() as f64;
    let variance = data.iter()
        .map(|x| (x - mean).powi(2))
        .sum::<f64>() / data.len() as f64;
    
    let min = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let range = max - min;
    
    // Simple uniformity score based on variance (lower variance = more uniform)
    let uniformity_score = if range > 0.0 {
        1.0 - (variance.sqrt() / range).min(1.0)
    } else {
        1.0
    };

    DistributionAnalysis {
        mean,
        variance,
        min,
        max,
        range,
        uniformity_score,
    }
}

#[derive(Debug)]
struct DistributionAnalysis {
    mean: f64,
    variance: f64,
    min: f64,
    max: f64,
    range: f64,
    uniformity_score: f64, // 0.0 to 1.0, higher = more uniform
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
    
    // Bucket analysis
    let avg_bucket_count: f64 = results.iter().map(|r| r.bucket_count as f64).sum::<f64>() / results.len() as f64;
    let avg_load_factor: f64 = results.iter().map(|r| r.load_factor).sum::<f64>() / results.len() as f64;
    let avg_empty_buckets: f64 = results.iter().map(|r| r.empty_buckets as f64).sum::<f64>() / results.len() as f64;
    
    println!("\nBucket Statistics:");
    println!("  Average bucket count: {:.1}", avg_bucket_count);
    println!("  Average load factor: {:.2}", avg_load_factor);
    println!("  Average empty buckets: {:.1}", avg_empty_buckets);
    
    // Operations analysis
    let total_assignments: usize = results.iter().map(|r| r.bucket_assignments).sum();
    let total_subroutine_calls: usize = results.iter().map(|r| r.subroutine_calls).sum();
    let total_subroutine_ops: usize = results.iter().map(|r| r.total_subroutine_operations).sum();
    
    println!("\nOperation Counts:");
    println!("  Total bucket assignments: {}", total_assignments);
    println!("  Total subroutine calls: {}", total_subroutine_calls);
    println!("  Total subroutine operations: {}", total_subroutine_ops);
}

fn run_float_test_case(name: &str, data: Vec<f64>) {
    println!("\n{}", "=".repeat(70));
    println!("Float Test Case: {}", name);
    println!("{}", "=".repeat(70));
    
    let original = data.clone();
    println!("Input size: {}", data.len());
    
    if data.len() <= 15 {
        println!("Input:  {:?}", data.iter().map(|x| format!("{:.2}", x)).collect::<Vec<_>>());
    }
    
    // Analyze data distribution
    if !data.is_empty() {
        let dist_analysis = analyze_distribution(&data);
        println!("Distribution: mean={:.2}, variance={:.3}, range={:.2}, uniformity={:.2}", 
                 dist_analysis.mean, dist_analysis.variance, dist_analysis.range, dist_analysis.uniformity_score);
        
        // Choose bucket count based on data size (rule of thumb: sqrt(n))
        let bucket_count = (data.len() as f64).sqrt().ceil() as usize;
        
        println!("\nResults:");
        println!("{}", "-".repeat(100));
        println!("{:<25} | {:>8} | {:>10} | {:>8} | {:>8} | {:>8} | {:>8} | {:>8}",
                 "Algorithm", "Correct", "Time(ms)", "Buckets", "LoadFac", "Empty", "Assign", "SubOps");
        println!("{}", "-".repeat(100));
        
        // Test different subroutine algorithms
        let subroutines = [
            SubroutineAlgorithm::InsertionSort,
            SubroutineAlgorithm::QuickSort,
            SubroutineAlgorithm::MergeSort,
        ];
        
        for subroutine in &subroutines {
            let mut data_copy = data.clone();
            let mut bucket_sort = BucketSort::new(bucket_count, DistributionStrategy::Linear, subroutine.clone(), true);
            
            let result = bucket_sort.sort(&mut data_copy);
            let is_correct = verify_sorting_correctness_float(&original, &data_copy);
            
            println!("{:<25} | {:>8} | {:>10.3} | {:>8} | {:>8.2} | {:>8} | {:>8} | {:>8}",
                     format!("{:?}", subroutine),
                     if is_correct { "✓" } else { "✗" },
                     result.time_ms,
                     result.bucket_count,
                     result.load_factor,
                     result.empty_buckets,
                     result.bucket_assignments,
                     result.total_subroutine_operations);
            
            if data.len() <= 15 && subroutine == &SubroutineAlgorithm::QuickSort {
                println!("Output: {:?}", data_copy.iter().map(|x| format!("{:.2}", x)).collect::<Vec<_>>());
            }
        }
    }
}

fn run_integer_test_case(name: &str, data: Vec<i32>) {
    println!("\n{}", "=".repeat(70));
    println!("Integer Test Case: {}", name);
    println!("{}", "=".repeat(70));
    
    let original = data.clone();
    println!("Input size: {}", data.len());
    
    if data.len() <= 20 {
        println!("Input:  {:?}", data);
    }
    
    if !data.is_empty() {
        let bucket_count = (data.len() as f64).sqrt().ceil() as usize;
        
        println!("\nResults:");
        println!("{}", "-".repeat(100));
        println!("{:<25} | {:>8} | {:>10} | {:>8} | {:>8} | {:>8} | {:>8} | {:>8}",
                 "Algorithm", "Correct", "Time(ms)", "Buckets", "LoadFac", "Empty", "Assign", "SubOps");
        println!("{}", "-".repeat(100));
        
        let subroutines = [
            SubroutineAlgorithm::InsertionSort,
            SubroutineAlgorithm::QuickSort,
            SubroutineAlgorithm::RadixSort,
        ];
        
        for subroutine in &subroutines {
            let mut data_copy = data.clone();
            let mut bucket_sort = BucketSort::new(bucket_count, DistributionStrategy::Linear, subroutine.clone(), true);
            
            let result = bucket_sort.sort_integers(&mut data_copy);
            let is_correct = verify_sorting_correctness_int(&original, &data_copy);
            
            println!("{:<25} | {:>8} | {:>10.3} | {:>8} | {:>8.2} | {:>8} | {:>8} | {:>8}",
                     format!("{:?}", subroutine),
                     if is_correct { "✓" } else { "✗" },
                     result.time_ms,
                     result.bucket_count,
                     result.load_factor,
                     result.empty_buckets,
                     result.bucket_assignments,
                     result.total_subroutine_operations);
            
            if data.len() <= 20 && subroutine == &SubroutineAlgorithm::QuickSort {
                println!("Output: {:?}", data_copy);
            }
        }
    }
}

fn main() {
    println!("Bucket Sort - Distribution-Based Linear Average-Case Sorting");
    println!("{}", "=".repeat(70));
    
    let float_test_cases = TestCases::generate_float_test_cases();
    let integer_test_cases = TestCases::generate_integer_test_cases();
    let mut all_results = Vec::new();
    
    // Run float test cases
    println!("\n🔢 FLOATING-POINT TEST CASES");
    println!("{}", "=".repeat(70));
    
    for (name, data) in float_test_cases {
        run_float_test_case(&name, data.clone());
        
        // Collect results for overall analysis
        if !data.is_empty() {
            let bucket_count = (data.len() as f64).sqrt().ceil() as usize;
            let mut bucket_sort = BucketSort::new(bucket_count, DistributionStrategy::Linear, SubroutineAlgorithm::QuickSort, true);
            let mut data_copy = data.clone();
            let result = bucket_sort.sort(&mut data_copy);
            all_results.push(result);
        }
    }
    
    // Run integer test cases
    println!("\n\n🔢 INTEGER TEST CASES");
    println!("{}", "=".repeat(70));
    
    for (name, data) in integer_test_cases {
        run_integer_test_case(&name, data.clone());
        
        // Collect results for overall analysis
        if !data.is_empty() {
            let bucket_count = (data.len() as f64).sqrt().ceil() as usize;
            let mut bucket_sort = BucketSort::new(bucket_count, DistributionStrategy::Linear, SubroutineAlgorithm::QuickSort, true);
            let mut data_copy = data.clone();
            let result = bucket_sort.sort_integers(&mut data_copy);
            all_results.push(result);
        }
    }
    
    // Overall performance analysis
    analyze_performance(&all_results);
    
    // Algorithm summary
    println!("\n\nAlgorithm Summary:");
    println!("{}", "=".repeat(70));
    println!("Bucket Sort Characteristics:");
    println!("• Time Complexity:  O(n + k) average, O(n²) worst case");
    println!("• Space Complexity: O(n + k) auxiliary space");
    println!("• Stability:        Depends on subroutine (can be stable)");
    println!("• Method:           Distribution-based sorting");
    println!("• Best for:         Uniformly distributed data with known range");
    
    println!("\nDistribution Strategies:");
    println!("• Linear:           Uniform distribution across range");
    println!("• Logarithmic:      For exponentially distributed data");
    println!("• Hash-based:       Custom distribution functions");
    println!("• Quantile-based:   Balanced buckets using data percentiles");
    
    println!("\nSubroutine Algorithms:");
    println!("• Insertion Sort:   O(m²), optimal for small buckets (m < 10)");
    println!("• Quick Sort:       O(m log m), general purpose, fast");
    println!("• Merge Sort:       O(m log m), stable, predictable");
    println!("• Radix Sort:       O(d*m), linear for bounded integers");
    
    println!("\nPerformance Factors:");
    println!("• Distribution uniformity is critical for good performance");
    println!("• Bucket count affects memory usage vs bucket sorting time");
    println!("• Subroutine choice matters for bucket sorting efficiency");
    println!("• Load balancing determines worst-case behavior");
    
    println!("\nComparison with Other Algorithms:");
    println!("• vs Radix Sort: More flexible range handling, works with floats");
    println!("• vs Quick Sort: Linear time with uniform data, but less consistent");
    println!("• vs Counting Sort: Better for large ranges, uses less memory");
    println!("• vs Merge Sort: Can be faster, but distribution-dependent");
    
    println!("\nPractical Applications:");
    println!("• Graphics: Color component sorting, pixel processing");
    println!("• Databases: Range partitioning, histogram construction");
    println!("• Scientific computing: Measurement data with known ranges");
    println!("• External sorting: First phase of out-of-core algorithms");
    println!("• Parallel processing: Natural parallelization across buckets");
}