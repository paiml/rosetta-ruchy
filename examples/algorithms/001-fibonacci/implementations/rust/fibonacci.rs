// Fibonacci Sequence - Rust Implementation
// Pure recursive for fair comparison

/// Pure recursive Fibonacci
/// Time: O(2^n), Space: O(n)
fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fibonacci(20), 6765);
    }
}

fn main() {
    println!("Fibonacci Sequence (Rust):");
    for i in 0..10 {
        println!("F({}) = {}", i, fibonacci(i));
    }
}