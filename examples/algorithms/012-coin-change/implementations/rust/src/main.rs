//! Coin Change Problem - Multiple Algorithm Implementation
//!
//! This module implements various approaches to solve the Coin Change problem:
//! - Bottom-up DP: O(amount×coins) time, O(amount) space
//! - Top-down DP: O(amount×coins) time with memoization
//! - Greedy algorithm: O(coins log coins) time, works for canonical systems
//! - Naive recursive: O(coins^amount) time for educational purposes

use std::collections::HashMap;
use std::fmt;
use std::time::Instant;

#[derive(Clone, Debug)]
struct CoinChangeResult {
    min_coins: i32,
    coins_used: Vec<usize>,
    algorithm_used: String,
    computation_time_ms: f64,
    is_possible: bool,
}

impl CoinChangeResult {
    fn new(min_coins: i32, coins_used: Vec<usize>, algorithm: &str, time_ms: f64) -> Self {
        Self {
            min_coins,
            coins_used,
            algorithm_used: algorithm.to_string(),
            computation_time_ms: time_ms,
            is_possible: true,
        }
    }

    fn impossible(algorithm: &str, time_ms: f64) -> Self {
        Self {
            min_coins: -1,
            coins_used: Vec::new(),
            algorithm_used: algorithm.to_string(),
            computation_time_ms: time_ms,
            is_possible: false,
        }
    }
}

impl fmt::Display for CoinChangeResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Coin Change Result ({}):", self.algorithm_used)?;
        if self.is_possible {
            writeln!(f, "  Minimum Coins: {}", self.min_coins)?;
            writeln!(f, "  Coins Used: {:?}", self.coins_used)?;
            writeln!(
                f,
                "  Total Value: {}",
                self.coins_used.iter().sum::<usize>()
            )?;
        } else {
            writeln!(f, "  Result: No solution possible")?;
        }
        writeln!(f, "  Computation Time: {:.3}ms", self.computation_time_ms)
    }
}

// Bottom-up Dynamic Programming approach
fn coin_change_dp_bottom_up(coins: &[usize], amount: usize) -> CoinChangeResult {
    let start_time = Instant::now();

    if amount == 0 {
        return CoinChangeResult::new(0, Vec::new(), "Bottom-up DP", 0.0);
    }

    // dp[i] = minimum coins needed for amount i
    let mut dp = vec![i32::MAX; amount + 1];
    let mut parent = vec![None; amount + 1];
    dp[0] = 0;

    // Fill DP table
    for i in 1..=amount {
        for &coin in coins {
            if coin <= i && dp[i - coin] != i32::MAX {
                let new_count = dp[i - coin] + 1;
                if new_count < dp[i] {
                    dp[i] = new_count;
                    parent[i] = Some(coin);
                }
            }
        }
    }

    let elapsed = start_time.elapsed().as_secs_f64() * 1000.0;

    if dp[amount] == i32::MAX {
        CoinChangeResult::impossible("Bottom-up DP", elapsed)
    } else {
        let coins_used = reconstruct_solution(&parent, amount);
        CoinChangeResult::new(dp[amount], coins_used, "Bottom-up DP", elapsed)
    }
}

// Reconstruct the coin solution from parent pointers
fn reconstruct_solution(parent: &[Option<usize>], mut amount: usize) -> Vec<usize> {
    let mut coins = Vec::new();

    while amount > 0 {
        if let Some(coin) = parent[amount] {
            coins.push(coin);
            amount -= coin;
        } else {
            break;
        }
    }

    coins.sort();
    coins
}

// Top-down Dynamic Programming with memoization
fn coin_change_dp_top_down(coins: &[usize], amount: usize) -> CoinChangeResult {
    let start_time = Instant::now();

    if amount == 0 {
        return CoinChangeResult::new(0, Vec::new(), "Top-down DP", 0.0);
    }

    let mut memo = HashMap::new();

    fn solve(coins: &[usize], amount: usize, memo: &mut HashMap<usize, i32>) -> i32 {
        if amount == 0 {
            return 0;
        }

        if let Some(&cached) = memo.get(&amount) {
            return cached;
        }

        let mut min_coins = i32::MAX;
        for &coin in coins {
            if coin <= amount {
                let sub_result = solve(coins, amount - coin, memo);
                if sub_result != i32::MAX {
                    min_coins = min_coins.min(sub_result + 1);
                }
            }
        }

        memo.insert(amount, min_coins);
        min_coins
    }

    let result = solve(coins, amount, &mut memo);
    let elapsed = start_time.elapsed().as_secs_f64() * 1000.0;

    if result == i32::MAX {
        CoinChangeResult::impossible("Top-down DP", elapsed)
    } else {
        // For simplicity, we don't reconstruct coins in memoized version
        CoinChangeResult::new(result, Vec::new(), "Top-down DP", elapsed)
    }
}

// Greedy algorithm (works optimally only for canonical coin systems)
fn coin_change_greedy(coins: &[usize], amount: usize) -> CoinChangeResult {
    let start_time = Instant::now();

    if amount == 0 {
        return CoinChangeResult::new(0, Vec::new(), "Greedy", 0.0);
    }

    let mut sorted_coins = coins.to_vec();
    sorted_coins.sort_by(|a, b| b.cmp(a)); // Sort in descending order

    let mut remaining = amount;
    let mut coins_used = Vec::new();

    for &coin in &sorted_coins {
        while remaining >= coin {
            coins_used.push(coin);
            remaining -= coin;
        }
    }

    let elapsed = start_time.elapsed().as_secs_f64() * 1000.0;

    if remaining == 0 {
        CoinChangeResult::new(coins_used.len() as i32, coins_used, "Greedy", elapsed)
    } else {
        CoinChangeResult::impossible("Greedy", elapsed)
    }
}

// Naive recursive approach (exponential time - educational only)
fn coin_change_naive_recursive(coins: &[usize], amount: usize) -> CoinChangeResult {
    let start_time = Instant::now();

    if amount == 0 {
        return CoinChangeResult::new(0, Vec::new(), "Naive Recursive", 0.0);
    }

    fn solve_naive(coins: &[usize], amount: usize) -> i32 {
        if amount == 0 {
            return 0;
        }

        let mut min_coins = i32::MAX;
        for &coin in coins {
            if coin <= amount {
                let sub_result = solve_naive(coins, amount - coin);
                if sub_result != i32::MAX {
                    min_coins = min_coins.min(sub_result + 1);
                }
            }
        }

        min_coins
    }

    let result = solve_naive(coins, amount);
    let elapsed = start_time.elapsed().as_secs_f64() * 1000.0;

    if result == i32::MAX {
        CoinChangeResult::impossible("Naive Recursive", elapsed)
    } else {
        CoinChangeResult::new(result, Vec::new(), "Naive Recursive", elapsed)
    }
}

// Visualize DP table construction for small problems
fn visualize_dp_table(coins: &[usize], amount: usize) {
    println!("DP Table Construction:");
    println!("{}", "=".repeat(60));
    println!("Coins: {:?}", coins);
    println!("Target Amount: {}", amount);
    println!();

    // Build DP table step by step
    let mut dp = vec![i32::MAX; amount + 1];
    let mut parent = vec![None; amount + 1];
    dp[0] = 0;

    // Print header
    print!("Amount: ");
    for i in 0..=amount.min(20) {
        print!("{:4}", i);
    }
    if amount > 20 {
        print!(" ...");
    }
    println!();

    // Fill and display DP table
    for i in 1..=amount {
        for &coin in coins {
            if coin <= i && dp[i - coin] != i32::MAX {
                let new_count = dp[i - coin] + 1;
                if new_count < dp[i] {
                    dp[i] = new_count;
                    parent[i] = Some(coin);
                }
            }
        }
    }

    // Display final DP values
    print!("DP:     ");
    for i in 0..=amount.min(20) {
        if dp[i] == i32::MAX {
            print!("{:4}", "∞");
        } else {
            print!("{:4}", dp[i]);
        }
    }
    if amount > 20 {
        print!(" ...");
    }
    println!();

    // Display coin choices
    print!("Coin:   ");
    for i in 0..=amount.min(20) {
        if let Some(coin) = parent[i] {
            print!("{:4}", coin);
        } else {
            print!("{:4}", "-");
        }
    }
    if amount > 20 {
        print!(" ...");
    }
    println!();

    println!("{}", "=".repeat(60));
}

// Performance comparison between algorithms
fn run_performance_comparison(coins: &[usize], amount: usize) {
    println!(
        "Performance Comparison: coins={:?}, amount={}",
        coins, amount
    );
    println!("{}", "-".repeat(70));

    let results = vec![
        coin_change_dp_bottom_up(coins, amount),
        coin_change_dp_top_down(coins, amount),
        coin_change_greedy(coins, amount),
    ];

    // Only include naive recursive for very small amounts
    let mut all_results = results;
    if amount <= 20 {
        all_results.push(coin_change_naive_recursive(coins, amount));
    }

    for result in &all_results {
        let status = if result.is_possible { "✓" } else { "✗" };
        println!(
            "{} {:<15} | Coins: {:3} | Time: {:8.3}ms",
            status, result.algorithm_used, result.min_coins, result.computation_time_ms
        );
    }

    // Analyze results
    let optimal_results: Vec<_> = all_results
        .iter()
        .filter(|r| {
            r.is_possible
                && (r.algorithm_used.contains("DP") || r.algorithm_used == "Naive Recursive")
        })
        .collect();

    if !optimal_results.is_empty() {
        let expected_coins = optimal_results[0].min_coins;
        let all_optimal_consistent = optimal_results
            .iter()
            .all(|r| r.min_coins == expected_coins);
        println!("{}", "-".repeat(70));
        println!("Optimal algorithms consistent: {}", all_optimal_consistent);

        // Check if greedy is optimal
        let greedy_result = all_results.iter().find(|r| r.algorithm_used == "Greedy");
        if let Some(greedy) = greedy_result {
            if greedy.is_possible {
                let greedy_optimal = greedy.min_coins == expected_coins;
                println!(
                    "Greedy algorithm optimal: {} (used {} coins)",
                    greedy_optimal, greedy.min_coins
                );
                if !greedy_optimal {
                    println!("  → Coin system is non-canonical");
                }
            }
        }
    }
}

// Test case runner
fn run_test_case(name: &str, coins: Vec<usize>, amount: usize, expected_coins: Option<i32>) {
    println!("Test Case: {}", name);
    println!("{}", "=".repeat(50));

    let result = coin_change_dp_bottom_up(&coins, amount);
    println!("Coins: {:?}, Amount: {}", coins, amount);
    println!("{}", result);

    if let Some(expected) = expected_coins {
        let passed = if expected == -1 {
            !result.is_possible
        } else {
            result.is_possible && result.min_coins == expected
        };

        println!(
            "Expected: {}, Got: {}, Test: {}",
            if expected == -1 {
                "impossible".to_string()
            } else {
                expected.to_string()
            },
            if result.is_possible {
                result.min_coins.to_string()
            } else {
                "impossible".to_string()
            },
            if passed { "PASS" } else { "FAIL" }
        );
    }

    // Show DP table for small cases
    if amount <= 20 && coins.len() <= 5 {
        println!();
        visualize_dp_table(&coins, amount);
    }

    println!();
}

// Generate test data
fn generate_coin_system(system_type: &str) -> Vec<usize> {
    match system_type {
        "us" => vec![1, 5, 10, 25],
        "euro" => vec![1, 2, 5, 10, 20, 50, 100, 200],
        "binary" => vec![1, 2, 4, 8, 16, 32, 64],
        "fibonacci" => vec![1, 2, 3, 5, 8, 13, 21, 34],
        "prime" => vec![2, 3, 5, 7, 11, 13, 17, 19, 23],
        "non_canonical" => vec![1, 3, 4],
        _ => vec![1, 5, 10, 25],
    }
}

// Test canonical vs non-canonical systems
fn test_canonical_systems() {
    println!("Canonical vs Non-Canonical Coin Systems:");
    println!("{}", "=".repeat(60));

    let test_cases = [
        ("US Coins (Canonical)", generate_coin_system("us"), 67),
        ("Euro Coins (Canonical)", generate_coin_system("euro"), 47),
        (
            "Non-Canonical [1,3,4]",
            generate_coin_system("non_canonical"),
            6,
        ),
        ("Binary System", generate_coin_system("binary"), 100),
    ];

    for (name, coins, amount) in test_cases {
        println!("\n{}: {:?}, Amount: {}", name, coins, amount);

        let dp_result = coin_change_dp_bottom_up(&coins, amount);
        let greedy_result = coin_change_greedy(&coins, amount);

        if dp_result.is_possible && greedy_result.is_possible {
            let optimal = dp_result.min_coins;
            let greedy_coins = greedy_result.min_coins;
            let is_canonical = optimal == greedy_coins;

            println!("  DP Optimal: {} coins", optimal);
            println!("  Greedy: {} coins", greedy_coins);
            println!(
                "  System is {}",
                if is_canonical {
                    "CANONICAL"
                } else {
                    "NON-CANONICAL"
                }
            );

            if !is_canonical {
                println!("  → Greedy gives suboptimal solution");
            }
        }
    }
}

fn main() {
    println!("Coin Change Problem - Multiple Algorithm Implementation");
    println!("{}", "=".repeat(65));

    // Test case 1: Basic exact change
    run_test_case("Exact Change", vec![1, 5, 10, 25], 30, Some(2));

    // Test case 2: No change needed
    run_test_case("No Change Needed", vec![1, 5, 10, 25], 0, Some(0));

    // Test case 3: Impossible case
    run_test_case("Impossible Case", vec![3, 5], 1, Some(-1));

    // Test case 4: Greedy fails
    run_test_case("Greedy Fails", vec![1, 3, 4], 6, Some(2));

    // Test case 5: Large amount
    run_test_case("Large Amount", vec![1, 5, 10, 25], 67, Some(6));

    // Performance comparison on medium problem
    println!("Medium Problem Performance Test:");
    println!("{}", "=".repeat(50));
    run_performance_comparison(&[1, 5, 10, 21, 25], 63);

    // Large problem performance (DP algorithms only)
    println!("\nLarge Problem Performance Test:");
    println!("{}", "=".repeat(40));
    let large_coins = vec![1, 5, 10, 25, 50, 100];
    let large_amount = 2674;

    let start_time = Instant::now();
    let large_result = coin_change_dp_bottom_up(&large_coins, large_amount);
    let elapsed = start_time.elapsed().as_secs_f64() * 1000.0;

    println!("Coins: {:?}, Amount: {}", large_coins, large_amount);
    if large_result.is_possible {
        println!("Minimum coins: {}", large_result.min_coins);
        println!("Computation time: {:.2}ms", elapsed);
        println!("Subproblems solved: {}", large_amount);
    }

    // Stress test
    println!("\nStress Test:");
    println!("{}", "=".repeat(30));
    let stress_amount = 10000;
    let stress_coins = vec![1, 5, 10, 25];

    let start_stress = Instant::now();
    let stress_result = coin_change_dp_bottom_up(&stress_coins, stress_amount);
    let stress_elapsed = start_stress.elapsed().as_secs_f64() * 1000.0;

    println!("Large amount stress test (amount={}):", stress_amount);
    if stress_result.is_possible {
        println!("Minimum coins: {}", stress_result.min_coins);
        println!("Computation time: {:.2}ms", stress_elapsed);
        println!(
            "Problems per second: {:.0}",
            (stress_amount as f64) / (stress_elapsed / 1000.0)
        );
    }

    // Canonical system analysis
    test_canonical_systems();

    // Algorithm summary
    println!("\nAlgorithm Summary:");
    println!("{}", "=".repeat(60));
    println!("Bottom-up DP:      O(amount×coins) time, O(amount) space, optimal");
    println!("Top-down DP:       O(amount×coins) time, O(amount) space, optimal");
    println!(
        "Greedy Algorithm:   O(coins log coins) time, O(1) space, works for canonical systems"
    );
    println!("Naive Recursive:    O(coins^amount) time, O(amount) space, educational only");
    println!("\nFor practical use:");
    println!("- Use DP for guaranteed optimal solutions");
    println!("- Use Greedy only for known canonical coin systems (US, Euro)");
    println!("- Bottom-up DP preferred for coin reconstruction");
}
