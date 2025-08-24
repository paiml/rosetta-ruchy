//! Rod Cutting Problem - Multiple Algorithm Implementation
//!
//! This module implements various approaches to solve the Rod Cutting problem:
//! - Bottom-up DP: O(n²) time, O(n) space
//! - Top-down DP: O(n²) time with memoization
//! - Greedy heuristic: O(n log n) time, approximation algorithm
//! - Naive recursive: O(2ⁿ) time for educational purposes

use std::collections::HashMap;
use std::fmt;
use std::time::Instant;

#[derive(Clone, Debug)]
struct RodCuttingResult {
    max_revenue: u32,
    cuts: Vec<usize>,
    algorithm_used: String,
    computation_time_ms: f64,
}

impl RodCuttingResult {
    fn new(revenue: u32, cuts: Vec<usize>, algorithm: &str, time_ms: f64) -> Self {
        Self {
            max_revenue: revenue,
            cuts,
            algorithm_used: algorithm.to_string(),
            computation_time_ms: time_ms,
        }
    }
}

impl fmt::Display for RodCuttingResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Rod Cutting Result ({}):", self.algorithm_used)?;
        writeln!(f, "  Maximum Revenue: {}", self.max_revenue)?;
        writeln!(f, "  Optimal Cuts: {:?}", self.cuts)?;
        writeln!(f, "  Total Cut Length: {}", self.cuts.iter().sum::<usize>())?;
        writeln!(f, "  Number of Pieces: {}", self.cuts.len())?;
        writeln!(f, "  Computation Time: {:.3}ms", self.computation_time_ms)
    }
}

// Bottom-up Dynamic Programming approach
fn rod_cutting_dp_bottom_up(prices: &[u32], length: usize) -> RodCuttingResult {
    let start_time = Instant::now();

    if length == 0 {
        return RodCuttingResult::new(0, Vec::new(), "Bottom-up DP", 0.0);
    }

    // revenue[i] = maximum revenue for rod of length i
    let mut revenue = vec![0; length + 1];
    let mut first_cut = vec![0; length + 1];

    // Fill DP table
    for i in 1..=length {
        for j in 1..=i.min(prices.len()) {
            let new_revenue = prices[j - 1] + revenue[i - j];
            if new_revenue > revenue[i] {
                revenue[i] = new_revenue;
                first_cut[i] = j;
            }
        }
    }

    // Reconstruct cutting sequence
    let cuts = reconstruct_cuts(&first_cut, length);
    let elapsed = start_time.elapsed().as_secs_f64() * 1000.0;

    RodCuttingResult::new(revenue[length], cuts, "Bottom-up DP", elapsed)
}

// Reconstruct the optimal cutting sequence
fn reconstruct_cuts(first_cut: &[usize], mut length: usize) -> Vec<usize> {
    let mut cuts = Vec::new();

    while length > 0 {
        let cut_length = first_cut[length];
        cuts.push(cut_length);
        length -= cut_length;
    }

    cuts.sort();
    cuts
}

// Top-down Dynamic Programming with memoization
fn rod_cutting_dp_top_down(prices: &[u32], length: usize) -> RodCuttingResult {
    let start_time = Instant::now();

    if length == 0 {
        return RodCuttingResult::new(0, Vec::new(), "Top-down DP", 0.0);
    }

    let mut memo = HashMap::new();

    fn solve(prices: &[u32], n: usize, memo: &mut HashMap<usize, u32>) -> u32 {
        if n == 0 {
            return 0;
        }

        if let Some(&cached) = memo.get(&n) {
            return cached;
        }

        let mut max_revenue = 0;
        for i in 1..=n.min(prices.len()) {
            let revenue = prices[i - 1] + solve(prices, n - i, memo);
            max_revenue = max_revenue.max(revenue);
        }

        memo.insert(n, max_revenue);
        max_revenue
    }

    let max_revenue = solve(prices, length, &mut memo);
    let elapsed = start_time.elapsed().as_secs_f64() * 1000.0;

    // For simplicity, we don't reconstruct cuts in memoized version
    RodCuttingResult::new(max_revenue, Vec::new(), "Top-down DP", elapsed)
}

// Greedy heuristic based on unit price
fn rod_cutting_greedy(prices: &[u32], length: usize) -> RodCuttingResult {
    let start_time = Instant::now();

    if length == 0 {
        return RodCuttingResult::new(0, Vec::new(), "Greedy Heuristic", 0.0);
    }

    // Calculate unit prices and create indexed array
    let mut unit_prices: Vec<(usize, f64)> = prices
        .iter()
        .enumerate()
        .map(|(i, &price)| (i + 1, price as f64 / (i + 1) as f64))
        .collect();

    // Sort by unit price in descending order
    unit_prices.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let mut remaining = length;
    let mut cuts = Vec::new();
    let mut total_revenue = 0;

    // Greedily select pieces with highest unit price
    for (piece_length, _unit_price) in unit_prices {
        while remaining >= piece_length && piece_length <= prices.len() {
            cuts.push(piece_length);
            total_revenue += prices[piece_length - 1];
            remaining -= piece_length;
        }
        if remaining == 0 {
            break;
        }
    }

    cuts.sort();
    let elapsed = start_time.elapsed().as_secs_f64() * 1000.0;

    RodCuttingResult::new(total_revenue, cuts, "Greedy Heuristic", elapsed)
}

// Naive recursive approach (exponential time - educational only)
fn rod_cutting_naive_recursive(prices: &[u32], length: usize) -> RodCuttingResult {
    let start_time = Instant::now();

    if length == 0 {
        return RodCuttingResult::new(0, Vec::new(), "Naive Recursive", 0.0);
    }

    fn solve_naive(prices: &[u32], n: usize) -> u32 {
        if n == 0 {
            return 0;
        }

        let mut max_revenue = 0;
        for i in 1..=n.min(prices.len()) {
            let revenue = prices[i - 1] + solve_naive(prices, n - i);
            max_revenue = max_revenue.max(revenue);
        }

        max_revenue
    }

    let max_revenue = solve_naive(prices, length);
    let elapsed = start_time.elapsed().as_secs_f64() * 1000.0;

    // For naive approach, we don't reconstruct cuts for simplicity
    RodCuttingResult::new(max_revenue, Vec::new(), "Naive Recursive", elapsed)
}

// Visualize DP table construction for small problems
fn visualize_dp_table(prices: &[u32], length: usize) {
    println!("DP Table Construction:");
    println!("{}", "=".repeat(60));
    println!("Prices: {:?}", prices);
    println!("Rod Length: {}", length);
    println!();

    // Build DP table step by step
    let mut revenue = vec![0; length + 1];
    let mut first_cut = vec![0; length + 1];

    // Print header
    print!("Length: ");
    for i in 0..=length.min(15) {
        print!("{:4}", i);
    }
    if length > 15 {
        print!(" ...");
    }
    println!();

    // Fill and display DP table
    for i in 1..=length {
        for j in 1..=i.min(prices.len()) {
            let new_revenue = prices[j - 1] + revenue[i - j];
            if new_revenue > revenue[i] {
                revenue[i] = new_revenue;
                first_cut[i] = j;
            }
        }
    }

    // Display revenue values
    print!("Revenue:");
    for i in 0..=length.min(15) {
        print!("{:4}", revenue[i]);
    }
    if length > 15 {
        print!(" ...");
    }
    println!();

    // Display first cut choices
    print!("Cut:    ");
    for i in 0..=length.min(15) {
        if i == 0 {
            print!("{:4}", "-");
        } else {
            print!("{:4}", first_cut[i]);
        }
    }
    if length > 15 {
        print!(" ...");
    }
    println!();

    println!("{}", "=".repeat(60));
}

// Performance comparison between algorithms
fn run_performance_comparison(prices: &[u32], length: usize) {
    println!(
        "Performance Comparison: length={}, prices={:?}",
        length, prices
    );
    println!("{}", "-".repeat(70));

    let results = vec![
        rod_cutting_dp_bottom_up(prices, length),
        rod_cutting_dp_top_down(prices, length),
        rod_cutting_greedy(prices, length),
    ];

    // Only include naive recursive for very small lengths
    let mut all_results = results;
    if length <= 15 {
        all_results.push(rod_cutting_naive_recursive(prices, length));
    }

    for result in &all_results {
        println!(
            "{:<18} | Revenue: {:4} | Time: {:8.3}ms",
            result.algorithm_used, result.max_revenue, result.computation_time_ms
        );
    }

    // Analyze results
    let optimal_results: Vec<_> = all_results
        .iter()
        .filter(|r| r.algorithm_used.contains("DP") || r.algorithm_used == "Naive Recursive")
        .collect();

    if !optimal_results.is_empty() {
        let expected_revenue = optimal_results[0].max_revenue;
        let all_optimal_consistent = optimal_results
            .iter()
            .all(|r| r.max_revenue == expected_revenue);

        println!("{}", "-".repeat(70));
        println!("Optimal algorithms consistent: {}", all_optimal_consistent);

        // Check greedy performance
        let greedy_result = all_results
            .iter()
            .find(|r| r.algorithm_used == "Greedy Heuristic");
        if let Some(greedy) = greedy_result {
            let approximation_ratio = greedy.max_revenue as f64 / expected_revenue as f64;
            println!(
                "Greedy approximation: {} / {} = {:.2}% of optimal",
                greedy.max_revenue,
                expected_revenue,
                approximation_ratio * 100.0
            );
        }
    }
}

// Test case runner
fn run_test_case(name: &str, prices: Vec<u32>, length: usize, expected_revenue: Option<u32>) {
    println!("Test Case: {}", name);
    println!("{}", "=".repeat(50));

    let result = rod_cutting_dp_bottom_up(&prices, length);
    println!("Prices: {:?}, Length: {}", prices, length);
    println!("{}", result);

    if let Some(expected) = expected_revenue {
        let passed = result.max_revenue == expected;
        println!(
            "Expected: {}, Got: {}, Test: {}",
            expected,
            result.max_revenue,
            if passed { "PASS" } else { "FAIL" }
        );
    }

    // Show DP table for small cases
    if length <= 15 && prices.len() <= 8 {
        println!();
        visualize_dp_table(&prices, length);
    }

    println!();
}

// Generate test data
fn generate_prices(pattern: &str, length: usize, seed: u64) -> Vec<u32> {
    let mut prices = Vec::with_capacity(length);
    let mut rng = seed;

    match pattern {
        "linear" => {
            for i in 1..=length {
                prices.push(i as u32);
            }
        }
        "quadratic" => {
            for i in 1..=length {
                prices.push((i * i) as u32);
            }
        }
        "fibonacci" => {
            let mut a = 1u32;
            let mut b = 1u32;
            prices.push(a);
            if length > 1 {
                prices.push(b);
            }

            for _ in 2..length {
                let c = a + b;
                prices.push(c);
                a = b;
                b = c;
            }
        }
        "random" => {
            for _ in 0..length {
                rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
                prices.push(1 + (rng as u32 % 50));
            }
        }
        "diminishing" => {
            for i in 1..=length {
                // Unit price decreases as length increases
                let unit_price = 100 / i as u32;
                prices.push(unit_price * i as u32);
            }
        }
        _ => {
            // Default to linear
            for i in 1..=length {
                prices.push(i as u32);
            }
        }
    }

    prices
}

// Analyze unit prices and cutting efficiency
fn analyze_cutting_efficiency(prices: &[u32]) {
    println!("Unit Price Analysis:");
    println!("{}", "-".repeat(40));
    println!(
        "{:<8} {:<8} {:<12} {:<10}",
        "Length", "Price", "Unit Price", "Efficiency"
    );

    let mut unit_prices: Vec<(usize, u32, f64)> = prices
        .iter()
        .enumerate()
        .map(|(i, &price)| {
            let length = i + 1;
            let unit_price = price as f64 / length as f64;
            (length, price, unit_price)
        })
        .collect();

    for (length, price, unit_price) in &unit_prices {
        let efficiency = if *unit_price > 0.0 { "Good" } else { "Poor" };
        println!(
            "{:<8} {:<8} {:<12.2} {:<10}",
            length, price, unit_price, efficiency
        );
    }

    // Find best unit price
    unit_prices.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
    if let Some((best_length, best_price, best_unit_price)) = unit_prices.first() {
        println!();
        println!(
            "Best unit price: length {} with {:.2} per unit (price {})",
            best_length, best_unit_price, best_price
        );
    }
}

fn main() {
    println!("Rod Cutting Problem - Multiple Algorithm Implementation");
    println!("{}", "=".repeat(65));

    // Test case 1: Simple case
    run_test_case("Simple Cut", vec![1, 5], 2, Some(5));

    // Test case 2: Classic textbook example
    run_test_case(
        "Classic Example",
        vec![1, 5, 8, 9, 10, 17, 17, 20],
        8,
        Some(22),
    );

    // Test case 3: No cutting optimal
    run_test_case("No Cutting Optimal", vec![1, 5, 8], 3, Some(8));

    // Test case 4: Many small pieces optimal
    run_test_case("Many Small Pieces", vec![10, 15, 18, 20, 21], 5, Some(50));

    // Test case 5: Greedy fails case
    run_test_case("Greedy Fails", vec![1, 4, 6, 7], 4, Some(8));

    // Performance comparison on medium problem
    println!("Medium Problem Performance Test:");
    println!("{}", "=".repeat(50));
    let medium_prices = vec![2, 5, 7, 8, 10, 12, 14, 15, 16, 17];
    run_performance_comparison(&medium_prices, 10);

    // Large problem performance (DP algorithms only)
    println!("\nLarge Problem Performance Test:");
    println!("{}", "=".repeat(40));
    let large_prices = generate_prices("random", 30, 42);

    let start_time = Instant::now();
    let large_result = rod_cutting_dp_bottom_up(&large_prices, 50);
    let elapsed = start_time.elapsed().as_secs_f64() * 1000.0;

    println!("Random prices, rod length 50:");
    println!("Maximum revenue: {}", large_result.max_revenue);
    println!("Computation time: {:.2}ms", elapsed);
    println!("Subproblems solved: {}", (50 * 50) / 2); // Approximate O(n²)

    // Stress test
    println!("\nStress Test:");
    println!("{}", "=".repeat(30));
    let stress_prices = generate_prices("fibonacci", 25, 0);

    let start_stress = Instant::now();
    let stress_result = rod_cutting_dp_bottom_up(&stress_prices, 100);
    let stress_elapsed = start_stress.elapsed().as_secs_f64() * 1000.0;

    println!("Fibonacci prices, rod length 100:");
    println!("Maximum revenue: {}", stress_result.max_revenue);
    println!("Computation time: {:.2}ms", stress_elapsed);
    println!(
        "Subproblems per second: {:.0}",
        (100.0 * 100.0) / (stress_elapsed / 1000.0)
    );

    // Unit price analysis
    println!("\nUnit Price Analysis Example:");
    println!("{}", "=".repeat(45));
    let analysis_prices = vec![1, 5, 8, 9, 10, 17, 17, 20];
    analyze_cutting_efficiency(&analysis_prices);

    // Algorithm summary
    println!("\nAlgorithm Summary:");
    println!("{}", "=".repeat(60));
    println!("Bottom-up DP:      O(n²) time, O(n) space, optimal");
    println!("Top-down DP:       O(n²) time, O(n) space, optimal");
    println!("Greedy Heuristic:  O(n log n) time, O(1) space, approximation");
    println!("Naive Recursive:   O(2ⁿ) time, O(n) space, educational only");
    println!("\nFor practical use:");
    println!("- Use DP for guaranteed optimal solutions and cut reconstruction");
    println!("- Use Greedy for fast approximations when near-optimal is sufficient");
    println!("- Bottom-up DP preferred for its predictable performance");
}
