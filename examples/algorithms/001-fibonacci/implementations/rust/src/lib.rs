//! Fibonacci sequence implementations in Rust
//! Performance baseline for Rosetta Ruchy benchmarks

use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::collections::HashMap;

/// Recursive Fibonacci (exponential complexity)
pub fn fib_recursive(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib_recursive(n - 1) + fib_recursive(n - 2),
    }
}

/// Iterative Fibonacci (linear complexity)
pub fn fib_iterative(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    
    let mut prev = 0u64;
    let mut curr = 1u64;
    
    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    
    curr
}

/// Memoized Fibonacci (linear complexity with caching)
pub fn fib_memoized(n: u32) -> u64 {
    fn fib_memo_helper(n: u32, cache: &mut HashMap<u32, u64>) -> u64 {
        if let Some(&result) = cache.get(&n) {
            return result;
        }
        
        let result = match n {
            0 => 0,
            1 => 1,
            _ => fib_memo_helper(n - 1, cache) + fib_memo_helper(n - 2, cache),
        };
        
        cache.insert(n, result);
        result
    }
    
    let mut cache = HashMap::new();
    fib_memo_helper(n, &mut cache)
}

/// Matrix multiplication Fibonacci (logarithmic complexity)
pub fn fib_matrix(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    
    fn matrix_mult(a: [[u64; 2]; 2], b: [[u64; 2]; 2]) -> [[u64; 2]; 2] {
        [
            [
                a[0][0] * b[0][0] + a[0][1] * b[1][0],
                a[0][0] * b[0][1] + a[0][1] * b[1][1],
            ],
            [
                a[1][0] * b[0][0] + a[1][1] * b[1][0],
                a[1][0] * b[0][1] + a[1][1] * b[1][1],
            ],
        ]
    }
    
    fn matrix_pow(mat: [[u64; 2]; 2], n: u32) -> [[u64; 2]; 2] {
        if n == 1 {
            return mat;
        }
        
        if n % 2 == 0 {
            let half = matrix_pow(mat, n / 2);
            matrix_mult(half, half)
        } else {
            matrix_mult(mat, matrix_pow(mat, n - 1))
        }
    }
    
    let base_matrix = [[1, 1], [1, 0]];
    let result = matrix_pow(base_matrix, n);
    result[0][1]
}

/// Iterative Fibonacci for large numbers using BigUint
pub fn fib_iterative_big(n: u32) -> BigUint {
    if n == 0 {
        return BigUint::zero();
    }
    if n == 1 {
        return BigUint::one();
    }
    
    let mut prev = BigUint::zero();
    let mut curr = BigUint::one();
    
    for _ in 2..=n {
        let next = &prev + &curr;
        prev = curr;
        curr = next;
    }
    
    curr
}

/// Tail-recursive Fibonacci (optimized recursive)
pub fn fib_tail_recursive(n: u32) -> u64 {
    fn fib_tail_helper(n: u32, prev: u64, curr: u64) -> u64 {
        match n {
            0 => prev,
            _ => fib_tail_helper(n - 1, curr, prev + curr),
        }
    }
    
    fib_tail_helper(n, 0, 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_recursive() {
        assert_eq!(fib_recursive(0), 0);
        assert_eq!(fib_recursive(1), 1);
        assert_eq!(fib_recursive(5), 5);
        assert_eq!(fib_recursive(10), 55);
        assert_eq!(fib_recursive(30), 832040);
    }
    
    #[test]
    fn test_iterative() {
        assert_eq!(fib_iterative(0), 0);
        assert_eq!(fib_iterative(1), 1);
        assert_eq!(fib_iterative(5), 5);
        assert_eq!(fib_iterative(10), 55);
        assert_eq!(fib_iterative(40), 102334155);
    }
    
    #[test]
    fn test_memoized() {
        assert_eq!(fib_memoized(0), 0);
        assert_eq!(fib_memoized(1), 1);
        assert_eq!(fib_memoized(5), 5);
        assert_eq!(fib_memoized(10), 55);
        assert_eq!(fib_memoized(40), 102334155);
    }
    
    #[test]
    fn test_matrix() {
        assert_eq!(fib_matrix(0), 0);
        assert_eq!(fib_matrix(1), 1);
        assert_eq!(fib_matrix(5), 5);
        assert_eq!(fib_matrix(10), 55);
        assert_eq!(fib_matrix(40), 102334155);
    }
    
    #[test]
    fn test_tail_recursive() {
        assert_eq!(fib_tail_recursive(0), 0);
        assert_eq!(fib_tail_recursive(1), 1);
        assert_eq!(fib_tail_recursive(5), 5);
        assert_eq!(fib_tail_recursive(10), 55);
        assert_eq!(fib_tail_recursive(40), 102334155);
    }
    
    #[test]
    fn test_big_numbers() {
        let result = fib_iterative_big(1000);
        let expected_start = "434665576869374564356885276750406258025646605173717804024817290895365554179490518904038798400792551692959225930803226347752096896232398733224711616429964409065331879382989696499285160037044761377951668492288";
        assert!(result.to_string().starts_with(expected_start));
    }
}