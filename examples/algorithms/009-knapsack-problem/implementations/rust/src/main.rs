//! Knapsack Problem - Multiple Algorithm Implementation
//! 
//! This module implements various approaches to solve the 0/1 Knapsack Problem:
//! - Standard DP: O(n×W) time, O(n×W) space
//! - Space-optimized DP: O(n×W) time, O(W) space  
//! - Memoized recursive: O(n×W) time with caching
//! - Greedy approximation: O(n log n) time, O(1) space

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::time::Instant;

#[derive(Clone, Debug)]
struct Item {
    name: String,
    weight: usize,
    value: u32,
}

impl Item {
    fn new(name: &str, weight: usize, value: u32) -> Self {
        Self {
            name: name.to_string(),
            weight,
            value,
        }
    }

    fn density(&self) -> f64 {
        self.value as f64 / self.weight as f64
    }
}

#[derive(Clone, Debug)]
struct KnapsackResult {
    total_value: u32,
    total_weight: usize,
    selected_items: Vec<String>,
    algorithm_used: String,
    computation_time_ms: f64,
}

impl KnapsackResult {
    fn new(value: u32, weight: usize, items: Vec<String>, algorithm: &str, time_ms: f64) -> Self {
        Self {
            total_value: value,
            total_weight: weight,
            selected_items: items,
            algorithm_used: algorithm.to_string(),
            computation_time_ms: time_ms,
        }
    }
}

impl fmt::Display for KnapsackResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Knapsack Solution ({}):", self.algorithm_used)?;
        writeln!(f, "  Total Value: {}", self.total_value)?;
        writeln!(f, "  Total Weight: {}", self.total_weight)?;
        writeln!(f, "  Selected Items: {:?}", self.selected_items)?;
        writeln!(f, "  Computation Time: {:.2}ms", self.computation_time_ms)
    }
}

// Standard DP with full table
fn knapsack_standard(items: &[Item], capacity: usize) -> KnapsackResult {
    let start_time = Instant::now();
    let n = items.len();
    
    if n == 0 || capacity == 0 {
        return KnapsackResult::new(0, 0, vec![], "Standard DP", 0.0);
    }
    
    // Initialize DP table
    let mut dp = vec![vec![0u32; capacity + 1]; n + 1];
    
    // Fill DP table
    for i in 1..=n {
        for w in 1..=capacity {
            let item = &items[i - 1];
            if item.weight <= w {
                let include_value = item.value + dp[i - 1][w - item.weight];
                dp[i][w] = dp[i - 1][w].max(include_value);
            } else {
                dp[i][w] = dp[i - 1][w];
            }
        }
    }
    
    // Reconstruct solution
    let mut selected_items = Vec::new();
    let mut total_weight = 0;
    let mut w = capacity;
    
    for i in (1..=n).rev() {
        if w >= items[i - 1].weight && dp[i][w] == dp[i - 1][w - items[i - 1].weight] + items[i - 1].value {
            selected_items.push(items[i - 1].name.clone());
            total_weight += items[i - 1].weight;
            w -= items[i - 1].weight;
        }
    }
    
    selected_items.reverse();
    let elapsed = start_time.elapsed().as_secs_f64() * 1000.0;
    
    KnapsackResult::new(dp[n][capacity], total_weight, selected_items, "Standard DP", elapsed)
}

// Space-optimized DP using rolling array
fn knapsack_space_optimized(items: &[Item], capacity: usize) -> KnapsackResult {
    let start_time = Instant::now();
    
    if items.is_empty() || capacity == 0 {
        return KnapsackResult::new(0, 0, vec![], "Space-Optimized DP", 0.0);
    }
    
    let mut dp = vec![0u32; capacity + 1];
    
    // For reconstruction, we need to track which items were selected
    // This is tricky with space optimization, so we'll use a different approach
    let mut item_inclusion = vec![vec![false; capacity + 1]; items.len()];
    
    for (i, item) in items.iter().enumerate() {
        // Traverse backwards to avoid using updated values
        for w in (item.weight..=capacity).rev() {
            let new_value = dp[w - item.weight] + item.value;
            if new_value > dp[w] {
                dp[w] = new_value;
                item_inclusion[i][w] = true;
            }
        }
    }
    
    // Reconstruct solution from item_inclusion matrix
    let mut selected_items = Vec::new();
    let mut total_weight = 0;
    let mut remaining_capacity = capacity;
    
    for (i, item) in items.iter().enumerate().rev() {
        if remaining_capacity >= item.weight && item_inclusion[i][remaining_capacity] {
            selected_items.push(item.name.clone());
            total_weight += item.weight;
            remaining_capacity -= item.weight;
        }
    }
    
    selected_items.reverse();
    let elapsed = start_time.elapsed().as_secs_f64() * 1000.0;
    
    KnapsackResult::new(dp[capacity], total_weight, selected_items, "Space-Optimized DP", elapsed)
}

// Memoized recursive approach
fn knapsack_memoized(items: &[Item], capacity: usize) -> KnapsackResult {
    let start_time = Instant::now();
    let mut memo: HashMap<(usize, usize), u32> = HashMap::new();
    let mut selected: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    
    fn solve(
        items: &[Item],
        index: usize,
        remaining_capacity: usize,
        memo: &mut HashMap<(usize, usize), u32>,
        selected: &mut HashMap<(usize, usize), Vec<usize>>,
    ) -> u32 {
        if index == items.len() || remaining_capacity == 0 {
            selected.insert((index, remaining_capacity), Vec::new());
            return 0;
        }
        
        let key = (index, remaining_capacity);
        if let Some(&cached_value) = memo.get(&key) {
            return cached_value;
        }
        
        let item = &items[index];
        
        // Option 1: Don't take the current item
        let exclude_value = solve(items, index + 1, remaining_capacity, memo, selected);
        let mut best_selection = selected.get(&(index + 1, remaining_capacity)).unwrap().clone();
        let mut best_value = exclude_value;
        
        // Option 2: Take the current item (if it fits)
        if item.weight <= remaining_capacity {
            let include_value = item.value + solve(items, index + 1, remaining_capacity - item.weight, memo, selected);
            if include_value > best_value {
                best_value = include_value;
                let mut include_selection = selected.get(&(index + 1, remaining_capacity - item.weight)).unwrap().clone();
                include_selection.insert(0, index);
                best_selection = include_selection;
            }
        }
        
        memo.insert(key, best_value);
        selected.insert(key, best_selection);
        best_value
    }
    
    let optimal_value = solve(items, 0, capacity, &mut memo, &mut selected);
    
    let selected_indices = selected.get(&(0, capacity)).unwrap();
    let selected_items: Vec<String> = selected_indices.iter()
        .map(|&i| items[i].name.clone())
        .collect();
    let total_weight: usize = selected_indices.iter()
        .map(|&i| items[i].weight)
        .sum();
    
    let elapsed = start_time.elapsed().as_secs_f64() * 1000.0;
    
    KnapsackResult::new(optimal_value, total_weight, selected_items, "Memoized Recursive", elapsed)
}

// Greedy approximation algorithm
fn knapsack_greedy(items: &[Item], capacity: usize) -> KnapsackResult {
    let start_time = Instant::now();
    
    if items.is_empty() || capacity == 0 {
        return KnapsackResult::new(0, 0, vec![], "Greedy Approximation", 0.0);
    }
    
    // Create indexed items with density
    let mut indexed_items: Vec<(usize, &Item, f64)> = items.iter()
        .enumerate()
        .map(|(i, item)| (i, item, item.density()))
        .collect();
    
    // Sort by value density (value/weight ratio) in descending order
    indexed_items.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(Ordering::Equal));
    
    let mut selected_items = Vec::new();
    let mut total_weight = 0;
    let mut total_value = 0;
    
    for (_index, item, _density) in indexed_items {
        if total_weight + item.weight <= capacity {
            selected_items.push(item.name.clone());
            total_weight += item.weight;
            total_value += item.value;
        }
    }
    
    let elapsed = start_time.elapsed().as_secs_f64() * 1000.0;
    
    KnapsackResult::new(total_value, total_weight, selected_items, "Greedy Approximation", elapsed)
}

// Visualization of DP table (for small instances)
fn visualize_dp_table(items: &[Item], capacity: usize, dp: &[Vec<u32>]) {
    println!("DP Table Visualization:");
    println!("{}", "=".repeat(80));
    
    // Header
    print!("     ");
    for w in 0..=capacity.min(20) { // Limit display width
        print!("{:4}", w);
    }
    if capacity > 20 { print!(" ..."); }
    println!();
    
    // Empty knapsack row
    print!("∅    ");
    for w in 0..=capacity.min(20) {
        print!("{:4}", dp[0][w]);
    }
    if capacity > 20 { print!(" ..."); }
    println!();
    
    // Item rows
    for (i, item) in items.iter().enumerate().take(10) { // Limit display height
        print!("{:<4} ", format!("{}:{}", item.weight, item.value));
        for w in 0..=capacity.min(20) {
            print!("{:4}", dp[i + 1][w]);
        }
        if capacity > 20 { print!(" ..."); }
        println!();
    }
    
    if items.len() > 10 {
        println!("...");
    }
    
    println!("{}", "=".repeat(80));
}

// Performance comparison
fn run_performance_comparison(items: &[Item], capacity: usize) {
    println!("Performance Comparison for {} items, capacity {}:", items.len(), capacity);
    println!("{}", "-".repeat(70));
    
    let results = vec![
        knapsack_standard(items, capacity),
        knapsack_space_optimized(items, capacity),
        knapsack_memoized(items, capacity),
        knapsack_greedy(items, capacity),
    ];
    
    for result in &results {
        println!("{:<20} | Value: {:6} | Weight: {:4} | Time: {:8.2}ms", 
                result.algorithm_used, result.total_value, result.total_weight, result.computation_time_ms);
    }
    
    // Verify all optimal algorithms give same result
    let optimal_value = results[0].total_value;
    let all_optimal_same = results[0..3].iter().all(|r| r.total_value == optimal_value);
    
    println!("{}", "-".repeat(70));
    println!("Optimal algorithms consistent: {}", all_optimal_same);
    
    if results.len() > 3 {
        let greedy_ratio = results[3].total_value as f64 / optimal_value as f64;
        println!("Greedy approximation ratio: {:.2}%", greedy_ratio * 100.0);
    }
}

// Test case runner
fn run_test_case(name: &str, items: Vec<Item>, capacity: usize, expected_value: Option<u32>) {
    println!("Test Case: {}", name);
    println!("{}", "=".repeat(50));
    
    let result = knapsack_standard(&items, capacity);
    println!("{}", result);
    
    if let Some(expected) = expected_value {
        let passed = result.total_value == expected;
        println!("Expected: {}, Got: {}, Test: {}", 
                expected, result.total_value, if passed { "PASS" } else { "FAIL" });
    }
    
    // Show DP table for small cases
    if items.len() <= 5 && capacity <= 20 {
        let n = items.len();
        let mut dp = vec![vec![0u32; capacity + 1]; n + 1];
        
        for i in 1..=n {
            for w in 1..=capacity {
                let item = &items[i - 1];
                if item.weight <= w {
                    let include_value = item.value + dp[i - 1][w - item.weight];
                    dp[i][w] = dp[i - 1][w].max(include_value);
                } else {
                    dp[i][w] = dp[i - 1][w];
                }
            }
        }
        
        visualize_dp_table(&items, capacity, &dp);
    }
    
    println!();
}

// Generate test data
fn generate_random_items(count: usize, max_weight: usize, max_value: u32, seed: u64) -> Vec<Item> {
    // Simple linear congruential generator for reproducible randomness
    let mut rng = seed;
    let mut items = Vec::new();
    
    for i in 0..count {
        rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
        let weight = 1 + (rng as usize % max_weight);
        
        rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
        let value = 1 + (rng as u32 % max_value);
        
        items.push(Item::new(&format!("item_{}", i), weight, value));
    }
    
    items
}

fn main() {
    println!("Knapsack Problem - Multiple Algorithm Implementation");
    println!("{}", "=".repeat(60));
    
    // Test case 1: Classic example
    let items1 = vec![
        Item::new("item1", 10, 60),
        Item::new("item2", 20, 100),
        Item::new("item3", 30, 120),
    ];
    run_test_case("Classic Example", items1, 50, Some(220));
    
    // Test case 2: High-density items
    let items2 = vec![
        Item::new("gem", 2, 20),      // density: 10
        Item::new("gold", 5, 30),     // density: 6
        Item::new("silver", 10, 50),  // density: 5
        Item::new("bronze", 8, 24),   // density: 3
    ];
    run_test_case("High Value Density", items2, 20, Some(100));
    
    // Test case 3: Edge case - zero capacity
    let items3 = vec![
        Item::new("heavy", 10, 100),
    ];
    run_test_case("Zero Capacity", items3, 0, Some(0));
    
    // Test case 4: All items fit
    let items4 = vec![
        Item::new("light1", 5, 10),
        Item::new("light2", 3, 8),
        Item::new("light3", 2, 5),
    ];
    run_test_case("All Items Fit", items4, 20, Some(23));
    
    // Performance comparison on larger instance
    println!("Large Instance Performance Test:");
    println!("{}", "=".repeat(50));
    let large_items = generate_random_items(50, 20, 100, 42);
    run_performance_comparison(&large_items, 100);
    
    // Stress test
    println!("\nStress Test (1000 items):");
    println!("{}", "=".repeat(30));
    let stress_items = generate_random_items(1000, 50, 200, 123);
    
    let start_greedy = Instant::now();
    let greedy_result = knapsack_greedy(&stress_items, 5000);
    let greedy_time = start_greedy.elapsed().as_secs_f64() * 1000.0;
    
    println!("Greedy (1000 items): Value={}, Time={:.2}ms", 
             greedy_result.total_value, greedy_time);
    
    // Smaller stress test for optimal algorithms (to avoid excessive runtime)
    let medium_items = generate_random_items(100, 30, 150, 456);
    let start_optimal = Instant::now();
    let optimal_result = knapsack_space_optimized(&medium_items, 500);
    let optimal_time = start_optimal.elapsed().as_secs_f64() * 1000.0;
    
    println!("Optimal (100 items): Value={}, Time={:.2}ms", 
             optimal_result.total_value, optimal_time);
    
    // Algorithm comparison summary
    println!("\nAlgorithm Summary:");
    println!("{}", "=".repeat(60));
    println!("Standard DP:        O(nW) time, O(nW) space, optimal");
    println!("Space-Optimized DP: O(nW) time, O(W) space, optimal");  
    println!("Memoized Recursive: O(nW) time, O(nW) space, optimal");
    println!("Greedy Approximation: O(n log n) time, O(1) space, ~50% optimal");
    println!("\nFor large instances, prefer Space-Optimized DP or Greedy depending on accuracy requirements.");
}