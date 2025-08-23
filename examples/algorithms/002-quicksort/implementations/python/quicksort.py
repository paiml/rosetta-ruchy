#!/usr/bin/env python3

import time
from typing import List, TypeVar
from abc import ABC, abstractmethod
from concurrent.futures import ThreadPoolExecutor
import random

T = TypeVar('T')

class Sortable(ABC):
    @abstractmethod
    def sort(self) -> None:
        pass
    
    @abstractmethod
    def is_sorted(self) -> bool:
        pass

class SortableList(list):
    def sort(self) -> None:
        quicksort_inplace(self)
    
    def is_sorted(self) -> bool:
        return all(self[i] <= self[i + 1] for i in range(len(self) - 1))

def quicksort_inplace(arr: List[T]) -> None:
    """In-place quicksort implementation."""
    if len(arr) > 1:
        _quicksort_range(arr, 0, len(arr) - 1)

def _quicksort_range(arr: List[T], low: int, high: int) -> None:
    """Recursive helper for in-place quicksort."""
    if low < high:
        pivot_index = _partition(arr, low, high)
        
        if pivot_index > 0:
            _quicksort_range(arr, low, pivot_index - 1)
        if pivot_index + 1 <= high:
            _quicksort_range(arr, pivot_index + 1, high)

def _partition(arr: List[T], low: int, high: int) -> int:
    """Lomuto partition scheme."""
    pivot = arr[high]
    i = low
    
    for j in range(low, high):
        if arr[j] <= pivot:
            arr[i], arr[j] = arr[j], arr[i]
            i += 1
    
    arr[i], arr[high] = arr[high], arr[i]
    return i

def quicksort_functional(arr: List[T]) -> List[T]:
    """Functional quicksort implementation."""
    if len(arr) <= 1:
        return arr.copy()
    
    pivot = arr[len(arr) // 2]
    less = [x for x in arr if x < pivot]
    equal = [x for x in arr if x == pivot]
    greater = [x for x in arr if x > pivot]
    
    return quicksort_functional(less) + equal + quicksort_functional(greater)

def quicksort_three_way(arr: List[T]) -> None:
    """Three-way quicksort for arrays with many duplicates."""
    if len(arr) <= 1:
        return
    _three_way_partition_sort(arr, 0, len(arr) - 1)

def _three_way_partition_sort(arr: List[T], low: int, high: int) -> None:
    """Three-way partitioning quicksort implementation."""
    if low >= high:
        return
    
    pivot = arr[low]
    lt = low
    gt = high
    i = low + 1
    
    while i <= gt:
        if arr[i] < pivot:
            arr[i], arr[lt] = arr[lt], arr[i]
            lt += 1
            i += 1
        elif arr[i] > pivot:
            arr[i], arr[gt] = arr[gt], arr[i]
            gt -= 1
        else:
            i += 1
    
    if lt > 0:
        _three_way_partition_sort(arr, low, lt - 1)
    _three_way_partition_sort(arr, gt + 1, high)

def quicksort_parallel(arr: List[T], threshold: int = 10000) -> List[T]:
    """Parallel quicksort using ThreadPoolExecutor."""
    if len(arr) <= threshold:
        return quicksort_functional(arr)
    
    if len(arr) <= 1:
        return arr.copy()
    
    pivot = arr[len(arr) // 2]
    less = [x for x in arr if x < pivot]
    equal = [x for x in arr if x == pivot]
    greater = [x for x in arr if x > pivot]
    
    with ThreadPoolExecutor(max_workers=2) as executor:
        future_less = executor.submit(quicksort_parallel, less, threshold)
        future_greater = executor.submit(quicksort_parallel, greater, threshold)
        
        sorted_less = future_less.result()
        sorted_greater = future_greater.result()
    
    return sorted_less + equal + sorted_greater

def main():
    test_arrays = [
        [],
        [42],
        [3, 1, 4, 1, 5, 9, 2, 6],
        [5, 4, 3, 2, 1],
        [1, 2, 3, 4, 5],
        [5, 5, 5, 5, 5],
    ]
    
    print("Quicksort Implementations Demo\n")
    
    for i, arr in enumerate(test_arrays):
        print(f"Test case {i + 1}: {arr}")
        
        arr1 = arr.copy()
        quicksort_inplace(arr1)
        print(f"  In-place:    {arr1}")
        
        arr2 = quicksort_functional(arr)
        print(f"  Functional:  {arr2}")
        
        arr3 = arr.copy()
        quicksort_three_way(arr3)
        print(f"  Three-way:   {arr3}")
        
        if arr:
            arr4 = quicksort_parallel(arr)
            print(f"  Parallel:    {arr4}")
        
        print()
    
    # Performance demonstration
    print("Performance demonstration with large array:")
    large_array = [(i * 37 + 11) % 1000 for i in range(10000)]
    
    print(f"  Array size: {len(large_array)}")
    
    start = time.time()
    arr_copy = large_array.copy()
    quicksort_inplace(arr_copy)
    duration = time.time() - start
    print(f"  In-place time: {duration:.4f}s")
    
    start = time.time()
    _ = quicksort_functional(large_array)
    duration = time.time() - start
    print(f"  Functional time: {duration:.4f}s")
    
    start = time.time()
    _ = quicksort_parallel(large_array)
    duration = time.time() - start
    print(f"  Parallel time: {duration:.4f}s")

if __name__ == "__main__":
    main()