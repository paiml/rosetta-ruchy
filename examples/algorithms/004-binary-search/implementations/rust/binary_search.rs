// Binary Search - Rust Implementation
// Baseline performance reference for Ruchy comparison

use std::collections::HashMap;
use std::cmp::Ordering;
use std::time::Instant;

// Basic binary search function
fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        
        match arr[mid].cmp(target) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid,
        }
    }
    
    None
}

// Enhanced binary search with caching for performance comparison
struct BinarySearcher<T: Ord + Clone + std::hash::Hash> {
    cache: HashMap<T, Option<usize>>,
    data: Vec<T>,
    stats: SearchStats,
}

#[derive(Debug, Default)]
struct SearchStats {
    cache_hits: usize,
    cache_misses: usize,
    total_comparisons: usize,
}

impl<T: Ord + Clone + std::hash::Hash + std::fmt::Debug> BinarySearcher<T> {
    fn new(data: Vec<T>) -> Self {
        Self {
            cache: HashMap::new(),
            data,
            stats: SearchStats::default(),
        }
    }
    
    fn search_with_cache(&mut self, target: &T) -> Option<usize> {
        // Check cache first
        if let Some(&cached_result) = self.cache.get(target) {
            self.stats.cache_hits += 1;
            println!("🎯 Cache hit for target: {:?}", target);
            return cached_result;
        }
        
        // Cache miss - perform search with comparison counting
        self.stats.cache_misses += 1;
        let result = self.binary_search_with_stats(target);
        
        // Cache the result
        self.cache.insert(target.clone(), result);
        println!("💾 Cached result for {:?}: {:?}", target, result);
        
        result
    }
    
    fn binary_search_with_stats(&mut self, target: &T) -> Option<usize> {
        let mut left = 0;
        let mut right = self.data.len();
        
        while left < right {
            self.stats.total_comparisons += 1;
            let mid = left + (right - left) / 2;
            
            match self.data[mid].cmp(target) {
                Ordering::Equal => return Some(mid),
                Ordering::Less => left = mid + 1,
                Ordering::Greater => right = mid,
            }
        }
        
        None
    }
    
    fn show_performance(&self) {
        let total_searches = self.stats.cache_hits + self.stats.cache_misses;
        let cache_hit_rate = if total_searches > 0 {
            self.stats.cache_hits as f64 / total_searches as f64 * 100.0
        } else {
            0.0
        };
        
        println!("\n📊 Performance Statistics:");
        println!("   Total searches: {}", total_searches);
        println!("   Cache hits: {} ({:.1}%)", self.stats.cache_hits, cache_hit_rate);
        println!("   Cache misses: {}", self.stats.cache_misses);
        println!("   Total comparisons: {}", self.stats.total_comparisons);
        if self.stats.cache_misses > 0 {
            println!("   Avg comparisons per search: {:.1}", 
                     self.stats.total_comparisons as f64 / self.stats.cache_misses as f64);
        }
    }
}

// Specialized search variants for comprehensive comparison
fn binary_search_leftmost<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        
        if arr[mid] < *target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    
    if left < arr.len() && arr[left] == *target {
        Some(left)
    } else {
        None
    }
}

fn binary_search_rightmost<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        
        if arr[mid] <= *target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    
    if left > 0 && arr[left - 1] == *target {
        Some(left - 1)
    } else {
        None
    }
}

fn benchmark_search(data: &[i32], targets: &[i32], iterations: usize) -> f64 {
    let start = Instant::now();
    
    for _ in 0..iterations {
        for &target in targets {
            binary_search(data, &target);
        }
    }
    
    start.elapsed().as_nanos() as f64 / (iterations * targets.len()) as f64
}

fn main() {
    println!("🔍 Binary Search - Rust Baseline Implementation");
    println!("===============================================");
    
    let data = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 23, 29, 31, 37, 41, 43, 47];
    println!("📊 Test data ({} elements): {:?}", data.len(), data);
    println!();
    
    // Test basic binary search
    println!("🔍 Basic binary search tests:");
    for target in &[5, 11, 20, 1, 47, 25] {
        match binary_search(&data, target) {
            Some(idx) => println!("   ✅ Found {} at index {}", target, idx),
            None => println!("   ❌ {} not found", target),
        }
    }
    
    // Test specialized searches for duplicates
    println!("\n🔄 Boundary search tests (with duplicates):");
    let duplicates = vec![1, 2, 2, 2, 3, 5, 5, 7, 7, 7, 9];
    println!("   Data with duplicates: {:?}", duplicates);
    
    for target in &[2, 5, 7] {
        let leftmost = binary_search_leftmost(&duplicates, target);
        let rightmost = binary_search_rightmost(&duplicates, target);
        println!("   {} -> leftmost: {:?}, rightmost: {:?}", target, leftmost, rightmost);
    }
    
    println!("\n🚀 Enhanced search with caching:");
    
    // Test enhanced searcher with caching
    let mut searcher = BinarySearcher::new(data.clone());
    
    // Demonstrate caching behavior
    let test_targets = vec![7, 15, 7, 23, 15, 7, 41, 23];
    for target in test_targets {
        match searcher.search_with_cache(&target) {
            Some(idx) => println!("   ✅ Found {} at index {}", target, idx),
            None => println!("   ❌ {} not found", target),
        }
    }
    
    // Show performance statistics
    searcher.show_performance();
    
    // Benchmark performance
    println!("\n⚡ Performance benchmark:");
    let benchmark_targets = vec![5, 11, 15, 23, 29, 37, 99]; // Mix of found/not found
    let avg_time = benchmark_search(&data, &benchmark_targets, 10000);
    println!("   Average search time: {:.2} ns per operation", avg_time);
    
    println!("\n✅ Rust binary search demonstration complete");
    println!("🎯 Baseline established for Ruchy performance comparison");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search_basic() {
        let arr = [1, 3, 5, 7, 9];
        
        assert_eq!(binary_search(&arr, &5), Some(2));
        assert_eq!(binary_search(&arr, &1), Some(0));
        assert_eq!(binary_search(&arr, &9), Some(4));
        assert_eq!(binary_search(&arr, &4), None);
        assert_eq!(binary_search(&arr, &10), None);
        
        // Test empty array
        let empty: [i32; 0] = [];
        assert_eq!(binary_search(&empty, &5), None);
    }
    
    #[test]
    fn test_boundary_searches() {
        let arr = [1, 2, 2, 2, 3, 5, 5];
        
        assert_eq!(binary_search_leftmost(&arr, &2), Some(1));
        assert_eq!(binary_search_rightmost(&arr, &2), Some(3));
        assert_eq!(binary_search_leftmost(&arr, &5), Some(5));
        assert_eq!(binary_search_rightmost(&arr, &5), Some(6));
        assert_eq!(binary_search_leftmost(&arr, &4), None);
    }
    
    #[test]
    fn test_cached_search() {
        let data = vec![1, 3, 5, 7, 9];
        let mut searcher = BinarySearcher::new(data);
        
        // First search should be cache miss
        assert_eq!(searcher.search_with_cache(&5), Some(2));
        assert_eq!(searcher.stats.cache_misses, 1);
        assert_eq!(searcher.stats.cache_hits, 0);
        
        // Second search should be cache hit
        assert_eq!(searcher.search_with_cache(&5), Some(2));
        assert_eq!(searcher.stats.cache_misses, 1);
        assert_eq!(searcher.stats.cache_hits, 1);
    }
}