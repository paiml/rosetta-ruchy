use std::cmp::Ordering;

pub trait Sortable<T> {
    fn sort(&mut self);
    fn is_sorted(&self) -> bool;
}

impl<T: Ord + Clone> Sortable<T> for Vec<T> {
    fn sort(&mut self) {
        quicksort(self);
    }
    
    fn is_sorted(&self) -> bool {
        self.windows(2).all(|w| w[0] <= w[1])
    }
}

pub fn quicksort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len > 1 {
        quicksort_range(arr, 0, len - 1);
    }
}

fn quicksort_range<T: Ord>(arr: &mut [T], low: usize, high: usize) {
    if low < high {
        let pivot_index = partition(arr, low, high);
        
        if pivot_index > 0 && pivot_index.saturating_sub(1) >= low {
            quicksort_range(arr, low, pivot_index - 1);
        }
        if pivot_index + 1 <= high {
            quicksort_range(arr, pivot_index + 1, high);
        }
    }
}

fn partition<T: Ord>(arr: &mut [T], low: usize, high: usize) -> usize {
    let mut i = low;
    
    for j in low..high {
        if arr[j] <= arr[high] {
            arr.swap(i, j);
            i += 1;
        }
    }
    
    arr.swap(i, high);
    i
}

pub fn quicksort_functional<T: Ord + Clone>(mut arr: Vec<T>) -> Vec<T> {
    match arr.len() {
        0 | 1 => arr,
        _ => {
            let pivot_index = arr.len() / 2;
            let pivot = arr.remove(pivot_index);
            
            let (mut less, mut greater): (Vec<_>, Vec<_>) = arr
                .into_iter()
                .partition(|x| x < &pivot);
            
            less = quicksort_functional(less);
            greater = quicksort_functional(greater);
            
            less.push(pivot);
            less.extend(greater);
            less
        }
    }
}

pub fn quicksort_three_way<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    three_way_partition_sort(arr, 0, arr.len() - 1);
}

fn three_way_partition_sort<T: Ord>(arr: &mut [T], low: usize, high: usize) {
    if low >= high {
        return;
    }
    
    let mut lt = low;
    let mut gt = high;
    let mut i = low + 1;
    
    while i <= gt {
        match arr[i].cmp(&arr[low]) {
            Ordering::Less => {
                arr.swap(i, lt);
                lt += 1;
                i += 1;
            }
            Ordering::Greater => {
                arr.swap(i, gt);
                if gt > 0 {
                    gt -= 1;
                } else {
                    break;
                }
            }
            Ordering::Equal => {
                i += 1;
            }
        }
    }
    
    if lt > 0 {
        three_way_partition_sort(arr, low, lt.saturating_sub(1));
    }
    three_way_partition_sort(arr, gt + 1, high);
}

#[cfg(feature = "parallel")]
pub fn quicksort_parallel<T: Ord + Clone + Send>(arr: Vec<T>) -> Vec<T> {
    use rayon::prelude::*;
    
    const PARALLEL_THRESHOLD: usize = 10000;
    
    if arr.len() <= PARALLEL_THRESHOLD {
        return quicksort_functional(arr);
    }
    
    match arr.len() {
        0 | 1 => arr,
        _ => {
            let pivot_index = arr.len() / 2;
            let pivot = arr[pivot_index].clone();
            
            let (less, equal_and_greater): (Vec<_>, Vec<_>) = arr
                .into_par_iter()
                .partition(|x| x < &pivot);
            
            let (equal, greater): (Vec<_>, Vec<_>) = equal_and_greater
                .into_par_iter()
                .partition(|x| x == &pivot);
            
            let (sorted_less, sorted_greater) = rayon::join(
                || quicksort_parallel(less),
                || quicksort_parallel(greater),
            );
            
            let mut result = sorted_less;
            result.extend(equal);
            result.extend(sorted_greater);
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;
    
    #[test]
    fn test_empty_array() {
        let mut arr: Vec<i32> = vec![];
        quicksort(&mut arr);
        assert_eq!(arr, vec![]);
    }
    
    #[test]
    fn test_single_element() {
        let mut arr = vec![42];
        quicksort(&mut arr);
        assert_eq!(arr, vec![42]);
    }
    
    #[test]
    fn test_sorted_array() {
        let mut arr = vec![1, 2, 3, 4, 5];
        quicksort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }
    
    #[test]
    fn test_reverse_array() {
        let mut arr = vec![5, 4, 3, 2, 1];
        quicksort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }
    
    #[test]
    fn test_random_array() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        quicksort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }
    
    #[test]
    fn test_duplicates() {
        let mut arr = vec![5, 5, 5, 5, 5];
        quicksort(&mut arr);
        assert_eq!(arr, vec![5, 5, 5, 5, 5]);
    }
    
    #[test]
    fn test_negative_numbers() {
        let mut arr = vec![3, -1, 4, -1, 5, 9, -2, 6];
        quicksort(&mut arr);
        assert_eq!(arr, vec![-2, -1, -1, 3, 4, 5, 6, 9]);
    }
    
    #[test]
    fn test_functional_quicksort() {
        let arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let sorted = quicksort_functional(arr);
        assert_eq!(sorted, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }
    
    #[test]
    fn test_three_way_quicksort() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        quicksort_three_way(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
    }
    
    #[cfg(feature = "parallel")]
    #[test]
    fn test_parallel_quicksort() {
        let arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let sorted = quicksort_parallel(arr);
        assert_eq!(sorted, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }
    
    #[test]
    fn test_trait_implementation() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        assert!(!arr.is_sorted());
        arr.sort();
        assert!(arr.is_sorted());
        assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }
    
    #[quickcheck]
    fn prop_sorted_has_same_elements(arr: Vec<i32>) -> bool {
        let mut sorted = arr.clone();
        quicksort(&mut sorted);
        
        let mut orig_sorted = arr.clone();
        orig_sorted.sort();
        
        sorted == orig_sorted
    }
    
    #[quickcheck]
    fn prop_sorted_is_ordered(arr: Vec<i32>) -> bool {
        let mut sorted = arr.clone();
        quicksort(&mut sorted);
        
        sorted.windows(2).all(|w| w[0] <= w[1])
    }
}