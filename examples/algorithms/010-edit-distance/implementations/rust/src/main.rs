//! Edit Distance (Levenshtein Distance) - Multiple Algorithm Implementation
//!
//! This module implements various approaches to solve the Edit Distance problem:
//! - Standard DP: O(m×n) time, O(m×n) space  
//! - Space-optimized DP: O(m×n) time, O(min(m,n)) space
//! - Memoized recursive: O(m×n) time with caching
//! - Naive recursive: O(3^max(m,n)) time for educational purposes

use std::collections::HashMap;
use std::fmt;
use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
enum EditOperation {
    Match(char),
    Substitute(char, char), // from, to
    Insert(char),
    Delete(char),
}

impl fmt::Display for EditOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EditOperation::Match(ch) => write!(f, "match '{}'", ch),
            EditOperation::Substitute(from, to) => write!(f, "substitute '{}' → '{}'", from, to),
            EditOperation::Insert(ch) => write!(f, "insert '{}'", ch),
            EditOperation::Delete(ch) => write!(f, "delete '{}'", ch),
        }
    }
}

#[derive(Clone, Debug)]
struct EditDistanceResult {
    distance: usize,
    operations: Vec<EditOperation>,
    algorithm_used: String,
    computation_time_ms: f64,
}

impl EditDistanceResult {
    fn new(distance: usize, operations: Vec<EditOperation>, algorithm: &str, time_ms: f64) -> Self {
        Self {
            distance,
            operations,
            algorithm_used: algorithm.to_string(),
            computation_time_ms: time_ms,
        }
    }
}

impl fmt::Display for EditDistanceResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Edit Distance Result ({}):", self.algorithm_used)?;
        writeln!(f, "  Distance: {}", self.distance)?;
        writeln!(f, "  Operations: {}", self.operations.len())?;
        for (i, op) in self.operations.iter().enumerate() {
            writeln!(f, "    {}: {}", i + 1, op)?;
        }
        writeln!(f, "  Computation Time: {:.2}ms", self.computation_time_ms)
    }
}

// Standard DP with full table
fn edit_distance_standard(str1: &str, str2: &str) -> EditDistanceResult {
    let start_time = Instant::now();
    let (m, n) = (str1.len(), str2.len());
    let chars1: Vec<char> = str1.chars().collect();
    let chars2: Vec<char> = str2.chars().collect();

    // Initialize DP table
    let mut dp = vec![vec![0usize; n + 1]; m + 1];

    // Base cases: transforming empty string
    for i in 0..=m {
        dp[i][0] = i; // Delete all characters
    }
    for j in 0..=n {
        dp[0][j] = j; // Insert all characters
    }

    // Fill DP table
    for i in 1..=m {
        for j in 1..=n {
            if chars1[i - 1] == chars2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1]; // No operation needed
            } else {
                dp[i][j] = 1 + dp[i - 1][j - 1] // Substitute
                    .min(dp[i - 1][j]) // Delete
                    .min(dp[i][j - 1]); // Insert
            }
        }
    }

    // Reconstruct operations
    let operations = reconstruct_operations(&chars1, &chars2, &dp);
    let elapsed = start_time.elapsed().as_secs_f64() * 1000.0;

    EditDistanceResult::new(dp[m][n], operations, "Standard DP", elapsed)
}

// Reconstruct the sequence of edit operations
fn reconstruct_operations(
    chars1: &[char],
    chars2: &[char],
    dp: &[Vec<usize>],
) -> Vec<EditOperation> {
    let mut operations = Vec::new();
    let (mut i, mut j) = (chars1.len(), chars2.len());

    while i > 0 || j > 0 {
        if i > 0 && j > 0 && chars1[i - 1] == chars2[j - 1] {
            // Characters match
            operations.push(EditOperation::Match(chars1[i - 1]));
            i -= 1;
            j -= 1;
        } else if i > 0 && j > 0 && dp[i][j] == dp[i - 1][j - 1] + 1 {
            // Substitution
            operations.push(EditOperation::Substitute(chars1[i - 1], chars2[j - 1]));
            i -= 1;
            j -= 1;
        } else if i > 0 && dp[i][j] == dp[i - 1][j] + 1 {
            // Deletion
            operations.push(EditOperation::Delete(chars1[i - 1]));
            i -= 1;
        } else if j > 0 {
            // Insertion
            operations.push(EditOperation::Insert(chars2[j - 1]));
            j -= 1;
        }
    }

    operations.reverse();
    operations
}

// Space-optimized DP using rolling array
fn edit_distance_space_optimized(str1: &str, str2: &str) -> EditDistanceResult {
    let start_time = Instant::now();
    let (m, n) = (str1.len(), str2.len());

    // Ensure we use the shorter string for the columns
    let (shorter, longer, _swapped) = if m < n {
        (str1, str2, false)
    } else {
        (str2, str1, true)
    };

    let chars_short: Vec<char> = shorter.chars().collect();
    let chars_long: Vec<char> = longer.chars().collect();

    // Use two arrays: previous row and current row
    let mut prev = (0..=chars_short.len()).collect::<Vec<usize>>();
    let mut curr = vec![0; chars_short.len() + 1];

    for (i, ch_long) in chars_long.iter().enumerate() {
        curr[0] = i + 1; // Cost of deleting all characters so far

        for (j, ch_short) in chars_short.iter().enumerate() {
            curr[j + 1] = if ch_long == ch_short {
                prev[j] // No change needed
            } else {
                1 + prev[j] // Substitute
                    .min(prev[j + 1]) // Delete
                    .min(curr[j]) // Insert
            };
        }

        std::mem::swap(&mut prev, &mut curr);
    }

    let distance = prev[chars_short.len()];

    // Note: Cannot reconstruct operations with space optimization
    let operations = vec![]; // Would need additional data structure
    let elapsed = start_time.elapsed().as_secs_f64() * 1000.0;

    EditDistanceResult::new(distance, operations, "Space-Optimized DP", elapsed)
}

// Memoized recursive approach
fn edit_distance_memoized(str1: &str, str2: &str) -> EditDistanceResult {
    let start_time = Instant::now();
    let chars1: Vec<char> = str1.chars().collect();
    let chars2: Vec<char> = str2.chars().collect();
    let mut memo: HashMap<(usize, usize), usize> = HashMap::new();

    fn solve(
        chars1: &[char],
        chars2: &[char],
        i: usize,
        j: usize,
        memo: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        // Base cases
        if i == 0 {
            return j;
        }
        if j == 0 {
            return i;
        }

        let key = (i, j);
        if let Some(&cached_result) = memo.get(&key) {
            return cached_result;
        }

        let result = if chars1[i - 1] == chars2[j - 1] {
            // Characters match, no edit needed
            solve(chars1, chars2, i - 1, j - 1, memo)
        } else {
            // Try all three operations and pick minimum
            1 + solve(chars1, chars2, i - 1, j - 1, memo) // Substitute
                .min(solve(chars1, chars2, i - 1, j, memo)) // Delete
                .min(solve(chars1, chars2, i, j - 1, memo)) // Insert
        };

        memo.insert(key, result);
        result
    }

    let distance = solve(&chars1, &chars2, chars1.len(), chars2.len(), &mut memo);

    // For simplicity, we won't reconstruct operations in memoized version
    let operations = vec![];
    let elapsed = start_time.elapsed().as_secs_f64() * 1000.0;

    EditDistanceResult::new(distance, operations, "Memoized Recursive", elapsed)
}

// Naive recursive approach (exponential time - for educational purposes only)
fn edit_distance_naive_recursive(str1: &str, str2: &str) -> EditDistanceResult {
    let start_time = Instant::now();
    let chars1: Vec<char> = str1.chars().collect();
    let chars2: Vec<char> = str2.chars().collect();

    fn solve(chars1: &[char], chars2: &[char], i: usize, j: usize) -> usize {
        // Base cases
        if i == 0 {
            return j;
        }
        if j == 0 {
            return i;
        }

        if chars1[i - 1] == chars2[j - 1] {
            // Characters match, no edit needed
            solve(chars1, chars2, i - 1, j - 1)
        } else {
            // Try all three operations and pick minimum
            1 + solve(chars1, chars2, i - 1, j - 1) // Substitute
                .min(solve(chars1, chars2, i - 1, j)) // Delete
                .min(solve(chars1, chars2, i, j - 1)) // Insert
        }
    }

    let distance = solve(&chars1, &chars2, chars1.len(), chars2.len());
    let operations = vec![]; // Too expensive to reconstruct
    let elapsed = start_time.elapsed().as_secs_f64() * 1000.0;

    EditDistanceResult::new(distance, operations, "Naive Recursive", elapsed)
}

// Visualize DP table for small inputs
fn visualize_dp_table(str1: &str, str2: &str, dp: &[Vec<usize>]) {
    let chars1: Vec<char> = str1.chars().collect();
    let chars2: Vec<char> = str2.chars().collect();

    println!("DP Table Visualization:");
    println!("{}", "=".repeat(60));

    // Header row
    print!("     \"\"");
    for ch in &chars2 {
        print!("  {}", ch);
    }
    println!();

    // Empty string row
    print!("\"\"   ");
    for j in 0..=chars2.len() {
        print!("{:3}", dp[0][j]);
    }
    println!();

    // Character rows
    for (i, ch) in chars1.iter().enumerate() {
        print!("{}    ", ch);
        for j in 0..=chars2.len() {
            print!("{:3}", dp[i + 1][j]);
        }
        println!();
    }

    println!("{}", "=".repeat(60));
}

// Performance comparison
fn run_performance_comparison(str1: &str, str2: &str) {
    println!("Performance Comparison: \"{}\" → \"{}\"", str1, str2);
    println!("{}", "-".repeat(70));

    let results = vec![
        edit_distance_standard(str1, str2),
        edit_distance_space_optimized(str1, str2),
        edit_distance_memoized(str1, str2),
    ];

    // Only include naive recursive for very small inputs
    let mut all_results = results;
    if str1.len() <= 8 && str2.len() <= 8 {
        all_results.push(edit_distance_naive_recursive(str1, str2));
    }

    for result in &all_results {
        println!(
            "{:<20} | Distance: {:3} | Time: {:8.2}ms",
            result.algorithm_used, result.distance, result.computation_time_ms
        );
    }

    // Verify all algorithms give same result
    let expected_distance = all_results[0].distance;
    let all_consistent = all_results.iter().all(|r| r.distance == expected_distance);

    println!("{}", "-".repeat(70));
    println!("All algorithms consistent: {}", all_consistent);

    if all_results.len() > 3 {
        let speedup = all_results[3].computation_time_ms / all_results[1].computation_time_ms;
        println!("Memoized vs Naive speedup: {:.1}x", speedup);
    }
}

// Test case runner
fn run_test_case(name: &str, str1: &str, str2: &str, expected_distance: Option<usize>) {
    println!("Test Case: {}", name);
    println!("{}", "=".repeat(50));

    let result = edit_distance_standard(str1, str2);
    println!("Input: \"{}\" → \"{}\"", str1, str2);
    println!("{}", result);

    if let Some(expected) = expected_distance {
        let passed = result.distance == expected;
        println!(
            "Expected: {}, Got: {}, Test: {}",
            expected,
            result.distance,
            if passed { "PASS" } else { "FAIL" }
        );
    }

    // Show DP table for small cases
    if str1.len() <= 8 && str2.len() <= 8 {
        let chars1: Vec<char> = str1.chars().collect();
        let chars2: Vec<char> = str2.chars().collect();
        let (m, n) = (chars1.len(), chars2.len());
        let mut dp = vec![vec![0; n + 1]; m + 1];

        // Rebuild DP table for visualization
        for i in 0..=m {
            dp[i][0] = i;
        }
        for j in 0..=n {
            dp[0][j] = j;
        }

        for i in 1..=m {
            for j in 1..=n {
                if chars1[i - 1] == chars2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = 1 + dp[i - 1][j - 1].min(dp[i - 1][j]).min(dp[i][j - 1]);
                }
            }
        }

        visualize_dp_table(str1, str2, &dp);
    }

    println!();
}

// Generate test strings for performance testing
fn generate_random_string(length: usize, alphabet: &[char], seed: u64) -> String {
    let mut rng = seed;
    let mut result = String::new();

    for _ in 0..length {
        rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
        let index = (rng as usize) % alphabet.len();
        result.push(alphabet[index]);
    }

    result
}

// Mutate a string with given mutation rate
fn mutate_string(original: &str, mutation_rate: f64, seed: u64) -> String {
    let mut rng = seed;
    let mut result = String::new();
    let chars: Vec<char> = original.chars().collect();
    let alphabet = ['A', 'T', 'C', 'G']; // DNA alphabet

    for &ch in &chars {
        rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
        let random_val = (rng as f64) / (u64::MAX as f64);

        if random_val < mutation_rate {
            // Mutate this character
            rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
            let new_char = alphabet[(rng as usize) % alphabet.len()];
            result.push(new_char);
        } else {
            result.push(ch);
        }
    }

    result
}

fn main() {
    println!("Edit Distance - Multiple Algorithm Implementation");
    println!("{}", "=".repeat(60));

    // Test case 1: Classic example
    run_test_case("Classic Example", "kitten", "sitting", Some(3));

    // Test case 2: Empty strings
    run_test_case("Empty Strings", "", "", Some(0));

    // Test case 3: One empty
    run_test_case("One Empty", "hello", "", Some(5));

    // Test case 4: Identical strings
    run_test_case("Identical Strings", "same", "same", Some(0));

    // Test case 5: DNA mutation
    run_test_case("DNA Mutation", "ATCG", "ATGG", Some(1));

    // Test case 6: Single operations
    run_test_case("Single Substitution", "abc", "axc", Some(1));
    run_test_case("Single Insertion", "abc", "abxc", Some(1));
    run_test_case("Single Deletion", "abxc", "abc", Some(1));

    // Performance comparison on medium-sized strings
    println!("Medium String Performance Test:");
    println!("{}", "=".repeat(50));
    run_performance_comparison("programming", "algorithm");

    // Large string performance (space-optimized only)
    println!("Large String Performance Test:");
    println!("{}", "=".repeat(40));
    let dna1 = generate_random_string(100, &['A', 'T', 'C', 'G'], 42);
    let dna2 = mutate_string(&dna1, 0.1, 123);

    let start_time = Instant::now();
    let large_result = edit_distance_space_optimized(&dna1, &dna2);
    let elapsed = start_time.elapsed().as_secs_f64() * 1000.0;

    println!("Large DNA sequences (100 chars, 10% mutation):");
    println!(
        "Distance: {}, Time: {:.2}ms",
        large_result.distance, elapsed
    );
    println!(
        "Mutation rate achieved: {:.1}%",
        (large_result.distance as f64 / 100.0) * 100.0
    );

    // Stress test with space-optimized algorithm
    println!("\nStress Test (Space-Optimized only):");
    println!("{}", "=".repeat(40));
    let stress1 = generate_random_string(1000, &['A', 'B'], 789);
    let stress2 = generate_random_string(1000, &['A', 'B'], 456);

    let start_stress = Instant::now();
    let stress_result = edit_distance_space_optimized(&stress1, &stress2);
    let stress_elapsed = start_stress.elapsed().as_secs_f64() * 1000.0;

    println!("Binary strings (1000×1000 chars):");
    println!(
        "Distance: {}, Time: {:.2}ms",
        stress_result.distance, stress_elapsed
    );
    println!(
        "Operations per second: {:.0}",
        (1000.0 * 1000.0) / (stress_elapsed / 1000.0)
    );

    // Algorithm summary
    println!("\nAlgorithm Summary:");
    println!("{}", "=".repeat(60));
    println!("Standard DP:        O(mn) time, O(mn) space, with operations");
    println!("Space-Optimized DP: O(mn) time, O(min(m,n)) space, distance only");
    println!("Memoized Recursive: O(mn) time, O(mn) space, sparse optimization");
    println!("Naive Recursive:    O(3^max(m,n)) time, educational only");
    println!("\nFor large strings, use Space-Optimized DP for best memory efficiency.");
    println!("For operation reconstruction, use Standard DP with full table.");
}
