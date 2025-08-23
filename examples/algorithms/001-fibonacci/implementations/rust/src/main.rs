//! Fibonacci benchmark runner

use fibonacci_rust::*;
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <n> [variant]", args[0]);
        println!("Variants: recursive, iterative, memoized, matrix, tail");
        return;
    }
    
    let n: u32 = args[1].parse().expect("Invalid number");
    let variant = args.get(2).map(|s| s.as_str()).unwrap_or("iterative");
    
    let start = Instant::now();
    let result = match variant {
        "recursive" if n <= 40 => fib_recursive(n).to_string(),
        "iterative" => {
            if n <= 92 {
                fib_iterative(n).to_string()
            } else {
                fib_iterative_big(n).to_string()
            }
        }
        "memoized" => fib_memoized(n.min(92)).to_string(),
        "matrix" => fib_matrix(n.min(92)).to_string(),
        "tail" => fib_tail_recursive(n.min(92)).to_string(),
        _ => {
            eprintln!("Unknown variant: {}", variant);
            return;
        }
    };
    let duration = start.elapsed();
    
    println!("fib({}) = {}", n, result);
    println!("Time: {:?}", duration);
}