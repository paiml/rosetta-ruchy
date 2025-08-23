use quicksort_rust::*;
use std::time::Instant;

fn main() {
    let test_arrays = vec![
        vec![],
        vec![42],
        vec![3, 1, 4, 1, 5, 9, 2, 6],
        vec![5, 4, 3, 2, 1],
        vec![1, 2, 3, 4, 5],
        vec![5, 5, 5, 5, 5],
    ];
    
    println!("Quicksort Implementations Demo\n");
    
    for (i, arr) in test_arrays.iter().enumerate() {
        println!("Test case {}: {:?}", i + 1, arr);
        
        let mut arr1 = arr.clone();
        quicksort(&mut arr1);
        println!("  In-place:    {:?}", arr1);
        
        let arr2 = quicksort_functional(arr.clone());
        println!("  Functional:  {:?}", arr2);
        
        let mut arr3 = arr.clone();
        quicksort_three_way(&mut arr3);
        println!("  Three-way:   {:?}", arr3);
        
        #[cfg(feature = "parallel")]
        if !arr.is_empty() {
            let arr4 = quicksort_parallel(arr.clone());
            println!("  Parallel:    {:?}", arr4);
        }
        
        println!();
    }
    
    // Performance demonstration
    println!("Performance demonstration with large array:");
    let large_array: Vec<i32> = (0..10000)
        .map(|i| (i * 37 + 11) % 1000)
        .collect();
    
    println!("  Array size: {}", large_array.len());
    
    let start = Instant::now();
    let mut arr_copy = large_array.clone();
    quicksort(&mut arr_copy);
    let duration = start.elapsed();
    println!("  In-place time: {:?}", duration);
    
    let start = Instant::now();
    let _ = quicksort_functional(large_array.clone());
    let duration = start.elapsed();
    println!("  Functional time: {:?}", duration);
    
    #[cfg(feature = "parallel")]
    {
        let start = Instant::now();
        let _ = quicksort_parallel(large_array.clone());
        let duration = start.elapsed();
        println!("  Parallel time: {:?}", duration);
    }
}