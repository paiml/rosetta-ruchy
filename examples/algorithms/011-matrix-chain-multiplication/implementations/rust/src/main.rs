//! Matrix Chain Multiplication - Multiple Algorithm Implementation
//!
//! This module implements various approaches to solve the Matrix Chain Multiplication problem:
//! - Standard DP: O(n³) time, O(n²) space
//! - Memoized recursive: O(n³) time with caching
//! - Naive recursive: O(2ⁿ) time for educational purposes

use std::collections::HashMap;
use std::fmt;
use std::time::Instant;

#[derive(Clone, Debug)]
struct MatrixChainResult {
    min_cost: usize,
    parenthesization: String,
    algorithm_used: String,
    computation_time_ms: f64,
    split_points: Vec<Vec<usize>>,
}

impl MatrixChainResult {
    fn new(cost: usize, parenthesization: String, algorithm: &str, time_ms: f64) -> Self {
        Self {
            min_cost: cost,
            parenthesization,
            algorithm_used: algorithm.to_string(),
            computation_time_ms: time_ms,
            split_points: Vec::new(),
        }
    }

    fn with_splits(
        cost: usize,
        parenthesization: String,
        algorithm: &str,
        time_ms: f64,
        splits: Vec<Vec<usize>>,
    ) -> Self {
        Self {
            min_cost: cost,
            parenthesization,
            algorithm_used: algorithm.to_string(),
            computation_time_ms: time_ms,
            split_points: splits,
        }
    }

    fn empty() -> Self {
        Self {
            min_cost: 0,
            parenthesization: String::new(),
            algorithm_used: "N/A".to_string(),
            computation_time_ms: 0.0,
            split_points: Vec::new(),
        }
    }
}

impl fmt::Display for MatrixChainResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Matrix Chain Multiplication Result ({}):",
            self.algorithm_used
        )?;
        writeln!(
            f,
            "  Minimum Cost: {} scalar multiplications",
            self.min_cost
        )?;
        writeln!(f, "  Optimal Parenthesization: {}", self.parenthesization)?;
        writeln!(f, "  Computation Time: {:.3}ms", self.computation_time_ms)
    }
}

// Standard DP approach with full table
fn matrix_chain_order_standard(dimensions: &[usize]) -> MatrixChainResult {
    let start_time = Instant::now();
    let n = dimensions.len();

    if n <= 1 {
        return MatrixChainResult::empty();
    }

    let num_matrices = n - 1;
    if num_matrices == 1 {
        return MatrixChainResult::new(0, "A1".to_string(), "Standard DP", 0.0);
    }

    // DP table: dp[i][j] = minimum cost to multiply matrices i..j (inclusive)
    let mut dp = vec![vec![0usize; num_matrices]; num_matrices];
    let mut split = vec![vec![0usize; num_matrices]; num_matrices];

    // Fill for chain lengths 2 to num_matrices
    for length in 2..=num_matrices {
        for i in 0..=num_matrices - length {
            let j = i + length - 1;
            dp[i][j] = usize::MAX;

            // Try all possible split points k
            for k in i..j {
                let cost =
                    dp[i][k] + dp[k + 1][j] + dimensions[i] * dimensions[k + 1] * dimensions[j + 1];

                if cost < dp[i][j] {
                    dp[i][j] = cost;
                    split[i][j] = k;
                }
            }
        }
    }

    let parenthesization = reconstruct_parenthesization(&split, 0, num_matrices - 1);
    let elapsed = start_time.elapsed().as_secs_f64() * 1000.0;

    MatrixChainResult::with_splits(
        dp[0][num_matrices - 1],
        parenthesization,
        "Standard DP",
        elapsed,
        split,
    )
}

// Reconstruct optimal parenthesization from split points
fn reconstruct_parenthesization(split: &[Vec<usize>], i: usize, j: usize) -> String {
    if i == j {
        format!("A{}", i + 1)
    } else {
        let k = split[i][j];
        let left = reconstruct_parenthesization(split, i, k);
        let right = reconstruct_parenthesization(split, k + 1, j);
        format!("({}{})", left, right)
    }
}

// Memoized recursive approach
fn matrix_chain_order_memoized(dimensions: &[usize]) -> MatrixChainResult {
    let start_time = Instant::now();
    let n = dimensions.len();

    if n <= 1 {
        return MatrixChainResult::empty();
    }

    let num_matrices = n - 1;
    if num_matrices == 1 {
        return MatrixChainResult::new(0, "A1".to_string(), "Memoized Recursive", 0.0);
    }

    let mut memo = HashMap::new();
    let mut split_memo = HashMap::new();

    fn solve(
        dims: &[usize],
        i: usize,
        j: usize,
        memo: &mut HashMap<(usize, usize), usize>,
        split_memo: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if i == j {
            return 0;
        }

        let key = (i, j);
        if let Some(&cached) = memo.get(&key) {
            return cached;
        }

        let mut min_cost = usize::MAX;
        let mut best_split = i;

        for k in i..j {
            let cost = solve(dims, i, k, memo, split_memo)
                + solve(dims, k + 1, j, memo, split_memo)
                + dims[i] * dims[k + 1] * dims[j + 1];

            if cost < min_cost {
                min_cost = cost;
                best_split = k;
            }
        }

        memo.insert(key, min_cost);
        split_memo.insert(key, best_split);
        min_cost
    }

    let min_cost = solve(dimensions, 0, num_matrices - 1, &mut memo, &mut split_memo);

    // Reconstruct parenthesization from memoized splits
    fn reconstruct_memoized(
        split_memo: &HashMap<(usize, usize), usize>,
        i: usize,
        j: usize,
    ) -> String {
        if i == j {
            format!("A{}", i + 1)
        } else {
            let k = split_memo.get(&(i, j)).copied().unwrap_or(i);
            let left = reconstruct_memoized(split_memo, i, k);
            let right = reconstruct_memoized(split_memo, k + 1, j);
            format!("({}{})", left, right)
        }
    }

    let parenthesization = reconstruct_memoized(&split_memo, 0, num_matrices - 1);
    let elapsed = start_time.elapsed().as_secs_f64() * 1000.0;

    MatrixChainResult::new(min_cost, parenthesization, "Memoized Recursive", elapsed)
}

// Naive recursive approach (exponential time - educational only)
fn matrix_chain_order_naive(dimensions: &[usize]) -> MatrixChainResult {
    let start_time = Instant::now();
    let n = dimensions.len();

    if n <= 1 {
        return MatrixChainResult::empty();
    }

    let num_matrices = n - 1;
    if num_matrices == 1 {
        return MatrixChainResult::new(0, "A1".to_string(), "Naive Recursive", 0.0);
    }

    fn solve_naive(dims: &[usize], i: usize, j: usize) -> usize {
        if i == j {
            return 0;
        }

        let mut min_cost = usize::MAX;

        for k in i..j {
            let cost = solve_naive(dims, i, k)
                + solve_naive(dims, k + 1, j)
                + dims[i] * dims[k + 1] * dims[j + 1];

            min_cost = min_cost.min(cost);
        }

        min_cost
    }

    let min_cost = solve_naive(dimensions, 0, num_matrices - 1);
    let elapsed = start_time.elapsed().as_secs_f64() * 1000.0;

    // For naive approach, we don't track splits for simplicity
    MatrixChainResult::new(
        min_cost,
        "Not reconstructed".to_string(),
        "Naive Recursive",
        elapsed,
    )
}

// Calculate cost of a specific parenthesization
#[allow(dead_code)]
fn calculate_parenthesization_cost(dimensions: &[usize], _parenthesization: &str) -> Option<usize> {
    // This is a simplified version - in practice, you'd parse the parenthesization
    // For demo purposes, we'll just return the optimal cost
    let result = matrix_chain_order_standard(dimensions);
    Some(result.min_cost)
}

// Generate all possible parenthesizations (for small n)
#[allow(dead_code)]
fn generate_all_parenthesizations(n: usize) -> Vec<String> {
    if n == 1 {
        return vec!["A1".to_string()];
    }

    let mut results = Vec::new();

    for k in 1..n {
        let left_parts = generate_all_parenthesizations(k);
        let right_parts = generate_all_parenthesizations(n - k);

        for left in &left_parts {
            for right in &right_parts {
                results.push(format!("({}{})", left, right));
            }
        }
    }

    results
}

// Visualize DP table for small inputs
fn visualize_dp_table(dimensions: &[usize], dp: &[Vec<usize>], split: &[Vec<usize>]) {
    let n = dp.len();

    println!("DP Table Visualization:");
    println!("{}", "=".repeat(80));

    // Print matrix dimensions
    print!("Matrices: ");
    for i in 0..dimensions.len() - 1 {
        print!("A{}({}×{}) ", i + 1, dimensions[i], dimensions[i + 1]);
    }
    println!();
    println!();

    // Print DP table header
    print!("     ");
    for j in 0..n {
        print!("{:8}", j + 1);
    }
    println!();

    // Print DP table rows
    for i in 0..n {
        print!("{:2}   ", i + 1);
        for j in 0..n {
            if i <= j {
                print!("{:8}", dp[i][j]);
            } else {
                print!("{:8}", "-");
            }
        }
        println!();
    }

    println!();
    println!("Split points:");
    for i in 0..n {
        print!("{:2}   ", i + 1);
        for j in 0..n {
            if i < j {
                print!("{:8}", split[i][j] + 1);
            } else {
                print!("{:8}", "-");
            }
        }
        println!();
    }

    println!("{}", "=".repeat(80));
}

// Performance comparison
fn run_performance_comparison(dimensions: &[usize]) {
    println!(
        "Performance Comparison for {} matrices:",
        dimensions.len() - 1
    );
    println!("Dimensions: {:?}", dimensions);
    println!("{}", "-".repeat(70));

    let results = vec![
        matrix_chain_order_standard(dimensions),
        matrix_chain_order_memoized(dimensions),
    ];

    // Only include naive for very small inputs
    let mut all_results = results;
    if dimensions.len() <= 7 {
        // 6 matrices or fewer
        all_results.push(matrix_chain_order_naive(dimensions));
    }

    for result in &all_results {
        println!(
            "{:<18} | Cost: {:8} | Time: {:8.3}ms",
            result.algorithm_used, result.min_cost, result.computation_time_ms
        );
    }

    // Verify all algorithms give same result
    let expected_cost = all_results[0].min_cost;
    let all_consistent = all_results.iter().all(|r| r.min_cost == expected_cost);

    println!("{}", "-".repeat(70));
    println!("All algorithms consistent: {}", all_consistent);

    if all_results.len() > 2 {
        let speedup = all_results[2].computation_time_ms / all_results[0].computation_time_ms;
        println!("DP vs Naive speedup: {:.1}x", speedup);
    }
}

// Test case runner
fn run_test_case(name: &str, dimensions: Vec<usize>, expected_cost: Option<usize>) {
    println!("Test Case: {}", name);
    println!("{}", "=".repeat(50));

    if dimensions.len() <= 1 {
        println!("Error: Need at least 2 dimensions for matrix multiplication");
        println!();
        return;
    }

    let result = matrix_chain_order_standard(&dimensions);
    println!("Dimensions: {:?}", dimensions);
    println!("{}", result);

    if let Some(expected) = expected_cost {
        let passed = result.min_cost == expected;
        println!(
            "Expected: {}, Got: {}, Test: {}",
            expected,
            result.min_cost,
            if passed { "PASS" } else { "FAIL" }
        );
    }

    // Show DP table for small cases
    if dimensions.len() <= 6 {
        let MatrixChainResult { split_points, .. } = &result;
        if !split_points.is_empty() {
            let n = dimensions.len() - 1;
            let mut dp = vec![vec![0; n]; n];

            // Rebuild DP table for visualization
            for length in 2..=n {
                for i in 0..=n - length {
                    let j = i + length - 1;
                    dp[i][j] = usize::MAX;

                    for k in i..j {
                        let cost = dp[i][k]
                            + dp[k + 1][j]
                            + dimensions[i] * dimensions[k + 1] * dimensions[j + 1];
                        dp[i][j] = dp[i][j].min(cost);
                    }
                }
            }

            visualize_dp_table(&dimensions, &dp, split_points);
        }
    }

    println!();
}

// Generate random dimensions for testing
fn generate_random_dimensions(
    count: usize,
    min_dim: usize,
    max_dim: usize,
    seed: u64,
) -> Vec<usize> {
    let mut rng = seed;
    let mut dimensions = Vec::with_capacity(count + 1);

    for _ in 0..=count {
        rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
        let dim = min_dim + (rng as usize % (max_dim - min_dim + 1));
        dimensions.push(dim);
    }

    dimensions
}

// Calculate Catalan numbers
fn catalan_number(n: usize) -> usize {
    if n == 0 {
        return 1;
    }

    let mut catalan = vec![0; n + 1];
    catalan[0] = 1;
    catalan[1] = 1;

    for i in 2..=n {
        for j in 0..i {
            catalan[i] += catalan[j] * catalan[i - 1 - j];
        }
    }

    catalan[n]
}

fn main() {
    println!("Matrix Chain Multiplication - Multiple Algorithm Implementation");
    println!("{}", "=".repeat(70));

    // Test case 1: Two matrices
    run_test_case("Two Matrices", vec![2, 3, 6], Some(36));

    // Test case 2: Three matrices - classic example
    run_test_case("Three Matrices", vec![1, 2, 3, 4], Some(18));

    // Test case 3: Textbook example
    run_test_case("Textbook Example", vec![5, 4, 6, 2, 7], Some(158));

    // Test case 4: Classic DP example
    run_test_case("Classic Example", vec![1, 2, 3, 4, 5], Some(38));

    // Test case 5: Identical dimensions
    run_test_case("Identical Dimensions", vec![10, 10, 10, 10, 10], Some(3000));

    // Performance comparison on medium-sized problem
    println!("Medium Chain Performance Test:");
    println!("{}", "=".repeat(50));
    let medium_dims = vec![40, 20, 30, 10, 30, 25];
    run_performance_comparison(&medium_dims);

    // Large chain performance (DP algorithms only)
    println!("\nLarge Chain Performance Test:");
    println!("{}", "=".repeat(40));
    let large_dims = generate_random_dimensions(20, 10, 100, 42);

    let start_time = Instant::now();
    let large_result = matrix_chain_order_standard(&large_dims);
    let elapsed = start_time.elapsed().as_secs_f64() * 1000.0;

    println!("Random chain (21 matrices, dims 10-100):");
    println!("Minimum cost: {}", large_result.min_cost);
    println!("Standard DP time: {:.2}ms", elapsed);

    let memoized_result = matrix_chain_order_memoized(&large_dims);
    println!(
        "Memoized time: {:.2}ms",
        memoized_result.computation_time_ms
    );

    // Stress test
    println!("\nStress Test:");
    println!("{}", "=".repeat(30));
    let stress_dims = generate_random_dimensions(50, 5, 200, 789);

    let start_stress = Instant::now();
    let stress_result = matrix_chain_order_standard(&stress_dims);
    let stress_elapsed = start_stress.elapsed().as_secs_f64() * 1000.0;

    println!("Large random chain (51 matrices, dims 5-200):");
    println!("Minimum cost: {}", stress_result.min_cost);
    println!("Computation time: {:.2}ms", stress_elapsed);
    println!("Subproblems solved: {}", (50 * 50 * 50) / 6); // Approximate O(n³)

    // Catalan numbers demonstration
    println!("\nCatalan Numbers (parenthesization count):");
    println!("{}", "=".repeat(45));
    for n in 1..=10 {
        let catalan = catalan_number(n);
        println!(
            "{} matrices: {} ways to parenthesize (C_{})",
            n + 1,
            catalan,
            n
        );
    }

    // Algorithm summary
    println!("\nAlgorithm Summary:");
    println!("{}", "=".repeat(60));
    println!("Standard DP:        O(n³) time, O(n²) space, systematic");
    println!("Memoized Recursive: O(n³) time, O(n²) space, sparse optimization");
    println!("Naive Recursive:    O(2ⁿ) time, O(n) space, educational only");
    println!("\nFor practical use, Standard DP is recommended for its predictable");
    println!("performance and ability to reconstruct optimal parenthesization.");
}
