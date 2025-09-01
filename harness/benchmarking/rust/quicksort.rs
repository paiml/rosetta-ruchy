// QuickSort Benchmark - Rust Baseline Implementation
// O(n log n) average case performance validation

use std::env;
use std::time::{Duration, Instant};
use rand::prelude::*;

fn quicksort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    
    let pivot_index = partition(arr);
    let (left, right) = arr.split_at_mut(pivot_index);
    quicksort(left);
    quicksort(&mut right[1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot_index = len / 2;
    arr.swap(pivot_index, len - 1);
    
    let mut i = 0;
    for j in 0..len - 1 {
        if arr[j] <= arr[len - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }
    
    arr.swap(i, len - 1);
    i
}

fn generate_test_array(size: usize, pattern: &str) -> Vec<i32> {
    let mut rng = thread_rng();
    match pattern {
        "random" => {
            let mut arr: Vec<i32> = (0..size as i32).collect();
            arr.shuffle(&mut rng);
            arr
        },
        "sorted" => (0..size as i32).collect(),
        "reverse" => (0..size as i32).rev().collect(),
        "partial" => {
            let mut arr: Vec<i32> = (0..size as i32).collect();
            // Shuffle only the last quarter
            let quarter = size / 4;
            arr[size - quarter..].shuffle(&mut rng);
            arr
        },
        _ => vec![0; size]
    }
}

fn benchmark_quicksort(iterations: usize, size: usize, pattern: &str) -> Duration {
    let mut total_duration = Duration::ZERO;
    
    for _ in 0..iterations {
        let mut arr = generate_test_array(size, pattern);
        
        let start = Instant::now();
        quicksort(&mut arr);
        total_duration += start.elapsed();
        
        // Verify sorting
        debug_assert!(arr.windows(2).all(|w| w[0] <= w[1]));
    }
    
    total_duration
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let iterations = if args.len() > 2 && args[1] == "--iterations" {
        args[2].parse::<usize>().unwrap_or(1000)
    } else {
        1000
    };
    
    println!("{{");
    println!("  \"algorithm\": \"quicksort\",");
    println!("  \"language\": \"rust\",");
    println!("  \"iterations\": {},", iterations);
    println!("  \"results\": [");
    
    let test_sizes = vec![10, 50, 100, 500, 1000, 5000];
    let patterns = vec!["random", "sorted", "reverse", "partial"];
    
    let mut first = true;
    for &size in &test_sizes {
        for &pattern in &patterns {
            if !first {
                println!(",");
            }
            first = false;
            
            let duration = benchmark_quicksort(iterations, size, pattern);
            let avg_time_us = duration.as_micros() as f64 / iterations as f64;
            
            print!("    {{");
            print!("\"size\": {}, ", size);
            print!("\"pattern\": \"{}\", ", pattern);
            print!("\"avg_time_us\": {:.3}", avg_time_us);
            print!("}}");
        }
    }
    
    println!("");
    println!("  ]");
    println!("}}");
}