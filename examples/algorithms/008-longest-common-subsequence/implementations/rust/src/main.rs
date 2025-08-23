// Longest Common Subsequence - Rust Baseline Implementation
// Dynamic programming with multiple optimization variants

use std::collections::HashMap;
use std::time::Instant;
use std::fmt::{self, Display};

#[derive(Debug, Clone)]
struct LCSResult {
    length: usize,
    sequence: String,
    computation_time: std::time::Duration,
    memory_used: usize, // Approximate bytes
}

impl Display for LCSResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LCS: \"{}\" (length: {}, time: {:?})", 
               self.sequence, self.length, self.computation_time)
    }
}

// Standard bottom-up DP approach
fn lcs_standard(s1: &str, s2: &str) -> LCSResult {
    let start = Instant::now();
    let (m, n) = (s1.len(), s2.len());
    
    // Create DP table
    let mut dp = vec![vec![0; n + 1]; m + 1];
    
    // Fill the DP table
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();
    
    for i in 1..=m {
        for j in 1..=n {
            if chars1[i-1] == chars2[j-1] {
                dp[i][j] = dp[i-1][j-1] + 1;
            } else {
                dp[i][j] = dp[i-1][j].max(dp[i][j-1]);
            }
        }
    }
    
    // Reconstruct the LCS
    let lcs = reconstruct_lcs_from_table(&chars1, &chars2, &dp);
    let memory_used = (m + 1) * (n + 1) * std::mem::size_of::<usize>();
    
    LCSResult {
        length: dp[m][n],
        sequence: lcs,
        computation_time: start.elapsed(),
        memory_used,
    }
}

// Reconstruct LCS from DP table
fn reconstruct_lcs_from_table(chars1: &[char], chars2: &[char], dp: &[Vec<usize>]) -> String {
    let mut lcs = Vec::new();
    let (mut i, mut j) = (chars1.len(), chars2.len());
    
    while i > 0 && j > 0 {
        if chars1[i-1] == chars2[j-1] {
            lcs.push(chars1[i-1]);
            i -= 1;
            j -= 1;
        } else if dp[i-1][j] > dp[i][j-1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }
    
    lcs.reverse();
    lcs.into_iter().collect()
}

// Space-optimized version using rolling arrays
fn lcs_space_optimized(s1: &str, s2: &str) -> LCSResult {
    let start = Instant::now();
    let (m, n) = (s1.len(), s2.len());
    
    // Choose shorter string for space optimization
    let (shorter, longer, _swapped) = if m <= n {
        (s1, s2, false)
    } else {
        (s2, s1, true)
    };
    
    let chars_short: Vec<char> = shorter.chars().collect();
    let chars_long: Vec<char> = longer.chars().collect();
    
    let mut prev = vec![0; chars_short.len() + 1];
    let mut curr = vec![0; chars_short.len() + 1];
    
    for ch_long in chars_long.iter() {
        for (j, ch_short) in chars_short.iter().enumerate() {
            curr[j + 1] = if ch_long == ch_short {
                prev[j] + 1
            } else {
                curr[j].max(prev[j + 1])
            };
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    
    let length = prev[chars_short.len()];
    let memory_used = 2 * (chars_short.len() + 1) * std::mem::size_of::<usize>();
    
    // Note: Space-optimized version doesn't reconstruct sequence by default
    // Would need additional computation for sequence reconstruction
    
    LCSResult {
        length,
        sequence: format!("(length {})", length), // Placeholder
        computation_time: start.elapsed(),
        memory_used,
    }
}

// Memoized recursive approach
struct LCSMemo {
    memo: HashMap<(usize, usize), usize>,
    chars1: Vec<char>,
    chars2: Vec<char>,
}

impl LCSMemo {
    fn new(s1: &str, s2: &str) -> Self {
        LCSMemo {
            memo: HashMap::new(),
            chars1: s1.chars().collect(),
            chars2: s2.chars().collect(),
        }
    }
    
    fn lcs_length(&mut self, i: usize, j: usize) -> usize {
        if i == 0 || j == 0 {
            return 0;
        }
        
        if let Some(&result) = self.memo.get(&(i, j)) {
            return result;
        }
        
        let result = if self.chars1[i-1] == self.chars2[j-1] {
            1 + self.lcs_length(i - 1, j - 1)
        } else {
            self.lcs_length(i - 1, j).max(self.lcs_length(i, j - 1))
        };
        
        self.memo.insert((i, j), result);
        result
    }
    
    fn reconstruct_lcs(&self, i: usize, j: usize) -> String {
        if i == 0 || j == 0 {
            return String::new();
        }
        
        if self.chars1[i-1] == self.chars2[j-1] {
            let mut result = self.reconstruct_lcs(i - 1, j - 1);
            result.push(self.chars1[i-1]);
            result
        } else {
            let len_up = self.memo.get(&(i-1, j)).copied().unwrap_or(0);
            let len_left = self.memo.get(&(i, j-1)).copied().unwrap_or(0);
            
            if len_up > len_left {
                self.reconstruct_lcs(i - 1, j)
            } else {
                self.reconstruct_lcs(i, j - 1)
            }
        }
    }
}

fn lcs_memoized(s1: &str, s2: &str) -> LCSResult {
    let start = Instant::now();
    let mut memo_solver = LCSMemo::new(s1, s2);
    
    let length = memo_solver.lcs_length(s1.len(), s2.len());
    let sequence = memo_solver.reconstruct_lcs(s1.len(), s2.len());
    let memory_used = memo_solver.memo.len() * std::mem::size_of::<((usize, usize), usize)>();
    
    LCSResult {
        length,
        sequence,
        computation_time: start.elapsed(),
        memory_used,
    }
}

// Enhanced space-optimized with sequence reconstruction (Hirschberg-style)
fn lcs_hirschberg(s1: &str, s2: &str) -> LCSResult {
    let start = Instant::now();
    
    fn lcs_length_forward(s1: &[char], s2: &[char]) -> Vec<usize> {
        let mut prev = vec![0; s2.len() + 1];
        let mut curr = vec![0; s2.len() + 1];
        
        for ch1 in s1.iter() {
            for (j, ch2) in s2.iter().enumerate() {
                curr[j + 1] = if ch1 == ch2 {
                    prev[j] + 1
                } else {
                    curr[j].max(prev[j + 1])
                };
            }
            std::mem::swap(&mut prev, &mut curr);
        }
        
        prev
    }
    
    fn lcs_length_backward(s1: &[char], s2: &[char]) -> Vec<usize> {
        let mut prev = vec![0; s2.len() + 1];
        let mut curr = vec![0; s2.len() + 1];
        
        for ch1 in s1.iter().rev() {
            for (j, ch2) in s2.iter().enumerate().rev() {
                curr[j] = if ch1 == ch2 {
                    prev[j + 1] + 1
                } else {
                    curr[j + 1].max(prev[j])
                };
            }
            std::mem::swap(&mut prev, &mut curr);
        }
        
        prev
    }
    
    fn hirschberg_lcs(s1: &[char], s2: &[char]) -> String {
        if s1.is_empty() || s2.is_empty() {
            return String::new();
        }
        
        if s1.len() == 1 {
            if s2.contains(&s1[0]) {
                return s1[0].to_string();
            } else {
                return String::new();
            }
        }
        
        let mid = s1.len() / 2;
        let left_lengths = lcs_length_forward(&s1[..mid], s2);
        let right_lengths = lcs_length_backward(&s1[mid..], s2);
        
        // Find the optimal split point
        let mut max_length = 0;
        let mut best_k = 0;
        
        for k in 0..=s2.len() {
            let total_length = left_lengths[k] + right_lengths[k];
            if total_length > max_length {
                max_length = total_length;
                best_k = k;
            }
        }
        
        // Recursively solve subproblems
        let left_lcs = hirschberg_lcs(&s1[..mid], &s2[..best_k]);
        let right_lcs = hirschberg_lcs(&s1[mid..], &s2[best_k..]);
        
        left_lcs + &right_lcs
    }
    
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();
    
    let sequence = hirschberg_lcs(&chars1, &chars2);
    let length = sequence.chars().count();
    
    // Hirschberg uses O(min(m,n)) space
    let memory_used = 2 * chars2.len().min(chars1.len()) * std::mem::size_of::<usize>();
    
    LCSResult {
        length,
        sequence,
        computation_time: start.elapsed(),
        memory_used,
    }
}

// Visualization of DP table construction
fn visualize_dp_construction(s1: &str, s2: &str) {
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();
    let (m, n) = (chars1.len(), chars2.len());
    
    let mut dp = vec![vec![0; n + 1]; m + 1];
    
    println!("\n🎯 DP Table Construction for \"{}\" vs \"{}\":", s1, s2);
    println!("{}", "=".repeat(50));
    
    // Print header
    print!("     \"\" ");
    for ch in &chars2 {
        print!(" {} ", ch);
    }
    println!();
    
    // Fill and display table step by step
    for i in 0..=m {
        if i == 0 {
            print!("\"\" ");
        } else {
            print!(" {}  ", chars1[i-1]);
        }
        
        for j in 0..=n {
            if i == 0 || j == 0 {
                dp[i][j] = 0;
            } else if chars1[i-1] == chars2[j-1] {
                dp[i][j] = dp[i-1][j-1] + 1;
            } else {
                dp[i][j] = dp[i-1][j].max(dp[i][j-1]);
            }
            
            print!("{:2} ", dp[i][j]);
        }
        println!();
    }
    
    // Show LCS reconstruction path
    println!("\n📍 LCS Reconstruction Path:");
    let mut path = Vec::new();
    let (mut i, mut j) = (m, n);
    path.push((i, j));
    
    while i > 0 && j > 0 {
        if chars1[i-1] == chars2[j-1] {
            i -= 1;
            j -= 1;
        } else if dp[i-1][j] > dp[i][j-1] {
            i -= 1;
        } else {
            j -= 1;
        }
        path.push((i, j));
    }
    
    path.reverse();
    println!("Path: {:?}", path);
    
    let lcs = reconstruct_lcs_from_table(&chars1, &chars2, &dp);
    println!("LCS: \"{}\" (length: {})", lcs, dp[m][n]);
}

// Benchmark different algorithms
fn benchmark_lcs_algorithms(s1: &str, s2: &str, name: &str) {
    println!("\n📊 Benchmarking: {}", name);
    println!("Strings: \"{}\" vs \"{}\"", s1, s2);
    
    // Standard DP
    let result_standard = lcs_standard(s1, s2);
    println!("Standard DP:     {}", result_standard);
    println!("                 Memory: {} bytes", result_standard.memory_used);
    
    // Space-optimized
    let result_optimized = lcs_space_optimized(s1, s2);
    println!("Space-optimized: Length: {}, Time: {:?}", 
             result_optimized.length, result_optimized.computation_time);
    println!("                 Memory: {} bytes ({:.1}% of standard)", 
             result_optimized.memory_used, 
             result_optimized.memory_used as f64 / result_standard.memory_used as f64 * 100.0);
    
    // Memoized
    let result_memoized = lcs_memoized(s1, s2);
    println!("Memoized:        {}", result_memoized);
    println!("                 Memory: {} bytes, Cache entries: {}", 
             result_memoized.memory_used,
             result_memoized.memory_used / std::mem::size_of::<((usize, usize), usize)>());
    
    // Hirschberg (space-optimal with reconstruction)
    let result_hirschberg = lcs_hirschberg(s1, s2);
    println!("Hirschberg:      {}", result_hirschberg);
    println!("                 Memory: {} bytes", result_hirschberg.memory_used);
}

// Generate test strings of various types
fn generate_dna_sequence(length: usize, seed: u64) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let bases = ['A', 'T', 'C', 'G'];
    let mut result = String::with_capacity(length);
    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    
    for i in 0..length {
        (seed + i as u64).hash(&mut hasher);
        let index = (hasher.finish() % 4) as usize;
        result.push(bases[index]);
    }
    
    result
}

fn mutate_sequence(original: &str, mutation_rate: f64, seed: u64) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let bases = ['A', 'T', 'C', 'G'];
    let mut result = String::with_capacity(original.len());
    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    
    for (i, ch) in original.char_indices() {
        (seed + i as u64).hash(&mut hasher);
        let random_val = (hasher.finish() % 1000) as f64 / 1000.0;
        
        if random_val < mutation_rate {
            let new_base_idx = (hasher.finish() % 4) as usize;
            result.push(bases[new_base_idx]);
        } else {
            result.push(ch);
        }
    }
    
    result
}

fn main() {
    println!("🧬 Longest Common Subsequence - Rust Baseline Implementation");
    println!("============================================================");
    
    // Example 1: Classic textbook example
    println!("\n📍 Example 1: Classic Cases");
    let examples = vec![
        ("ABCDGH", "AEDFHR"),
        ("AGGTAB", "GXTXAYB"),
        ("programming", "algorithm"),
        ("HELLO", "HELLO"),
        ("ABC", "DEF"),
    ];
    
    for (s1, s2) in examples {
        let result = lcs_standard(s1, s2);
        println!("\"{}\" vs \"{}\" → \"{}\" (length: {})", 
                 s1, s2, result.sequence, result.length);
    }
    
    // Example 2: Visualization of DP table
    println!("\n📍 Example 2: DP Table Visualization");
    visualize_dp_construction("ABCD", "ACBD");
    
    // Example 3: Algorithm comparison
    println!("\n📍 Example 3: Algorithm Comparison");
    benchmark_lcs_algorithms("ABCDGH", "AEDFHR", "Short strings");
    benchmark_lcs_algorithms(
        &"ABCDEFGHIJKLMNOP".repeat(5),
        &"ACEGIKMOQSUWY".repeat(7),
        "Medium strings"
    );
    
    // Example 4: DNA sequence analysis
    println!("\n📍 Example 4: DNA Sequence Analysis");
    let dna1 = generate_dna_sequence(100, 42);
    let dna2 = mutate_sequence(&dna1, 0.2, 123);
    
    println!("Original: {}...", &dna1[..20]);
    println!("Mutated:  {}...", &dna2[..20]);
    
    let result = lcs_standard(&dna1, &dna2);
    println!("DNA LCS length: {} / {} ({:.1}% similarity)", 
             result.length, dna1.len(),
             result.length as f64 / dna1.len() as f64 * 100.0);
    
    // Performance stress tests
    println!("\n📍 Example 5: Performance Stress Tests");
    
    let lengths = [10, 50, 100, 200];
    for &len in &lengths {
        let s1 = "A".repeat(len);
        let s2 = "A".repeat(len);
        
        let start = Instant::now();
        let _result = lcs_standard(&s1, &s2);
        let duration = start.elapsed();
        
        println!("{}×{} identical strings: {} ms", 
                 len, len, duration.as_millis());
    }
    
    // Pathological case: completely different strings
    println!("\n🔍 Pathological Case: No common characters");
    let s1 = "A".repeat(100);
    let s2 = "B".repeat(100);
    let result = lcs_standard(&s1, &s2);
    println!("\"{}...\" vs \"{}...\" → length: {} (time: {:?})",
             &s1[..10], &s2[..10], result.length, result.computation_time);
    
    println!("\n✅ LCS algorithm baseline established");
    println!("🎯 Ready for comparison with other implementations");
    println!("📊 Memory efficiency: Up to 99% reduction with space optimization");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_empty_strings() {
        let result = lcs_standard("", "");
        assert_eq!(result.length, 0);
        assert_eq!(result.sequence, "");
    }
    
    #[test]
    fn test_one_empty() {
        let result = lcs_standard("ABC", "");
        assert_eq!(result.length, 0);
        assert_eq!(result.sequence, "");
    }
    
    #[test]
    fn test_identical_strings() {
        let result = lcs_standard("HELLO", "HELLO");
        assert_eq!(result.length, 5);
        assert_eq!(result.sequence, "HELLO");
    }
    
    #[test]
    fn test_classic_example() {
        let result = lcs_standard("ABCDGH", "AEDFHR");
        assert_eq!(result.length, 3);
        assert_eq!(result.sequence, "ADH");
    }
    
    #[test]
    fn test_dna_example() {
        let result = lcs_standard("AGGTAB", "GXTXAYB");
        assert_eq!(result.length, 4);
        assert_eq!(result.sequence, "GTAB");
    }
    
    #[test]
    fn test_no_common_chars() {
        let result = lcs_standard("ABC", "DEF");
        assert_eq!(result.length, 0);
        assert_eq!(result.sequence, "");
    }
    
    #[test]
    fn test_space_optimized_consistency() {
        let test_cases = vec![
            ("ABCDGH", "AEDFHR"),
            ("AGGTAB", "GXTXAYB"),
            ("HELLO", "WORLD"),
            ("", "ABC"),
            ("ABC", ""),
        ];
        
        for (s1, s2) in test_cases {
            let standard = lcs_standard(s1, s2);
            let optimized = lcs_space_optimized(s1, s2);
            assert_eq!(standard.length, optimized.length,
                      "Length mismatch for '{}' vs '{}'", s1, s2);
        }
    }
    
    #[test]
    fn test_memoized_consistency() {
        let test_cases = vec![
            ("ABCD", "ACBD"),
            ("programming", "algorithm"),
            ("test", "best"),
        ];
        
        for (s1, s2) in test_cases {
            let standard = lcs_standard(s1, s2);
            let memoized = lcs_memoized(s1, s2);
            assert_eq!(standard.length, memoized.length);
            assert_eq!(standard.sequence, memoized.sequence);
        }
    }
}