// Fibonacci Benchmark - Rust Baseline Implementation
// Performance comparison baseline for Ruchy benchmarks

use std::env;
use std::time::{Duration, Instant};

fn fibonacci_recursive(n: i32) -> i32 {
    if n <= 1 {
        n
    } else {
        fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
    }
}

fn fibonacci_iterative(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    
    let mut prev = 0;
    let mut curr = 1;
    
    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    
    curr
}

fn benchmark_fibonacci(iterations: usize, n: i32, use_recursive: bool) -> Duration {
    // Warmup phase
    let warmup = iterations / 10;
    for _ in 0..warmup {
        if use_recursive && n <= 20 {
            fibonacci_recursive(n);
        } else {
            fibonacci_iterative(n);
        }
    }
    
    // Benchmark phase
    let start = Instant::now();
    
    for _ in 0..iterations {
        if use_recursive && n <= 20 {
            fibonacci_recursive(n);
        } else {
            fibonacci_iterative(n);
        }
    }
    
    start.elapsed()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let iterations = if args.len() > 2 && args[1] == "--iterations" {
        args[2].parse::<usize>().unwrap_or(1000)
    } else {
        1000
    };
    
    println!("{{");
    println!("  \"algorithm\": \"fibonacci\",");
    println!("  \"language\": \"rust\",");
    println!("  \"iterations\": {},", iterations);
    println!("  \"results\": [");
    
    let test_sizes = vec![5, 10, 15, 20, 25, 30, 35, 40];
    
    for (i, &n) in test_sizes.iter().enumerate() {
        // Benchmark iterative version
        let iter_duration = benchmark_fibonacci(iterations, n, false);
        let iter_time_us = iter_duration.as_micros() as f64 / iterations as f64;
        
        // Benchmark recursive version (only for small n)
        let rec_time_us = if n <= 20 {
            let rec_duration = benchmark_fibonacci(iterations, n, true);
            rec_duration.as_micros() as f64 / iterations as f64
        } else {
            -1.0
        };
        
        println!("    {{");
        println!("      \"n\": {},", n);
        println!("      \"iterative_time_us\": {:.3},", iter_time_us);
        println!("      \"recursive_time_us\": {:.3}", rec_time_us);
        
        if i < test_sizes.len() - 1 {
            println!("    }},");
        } else {
            println!("    }}");
        }
    }
    
    println!("  ]");
    println!("}}");
}