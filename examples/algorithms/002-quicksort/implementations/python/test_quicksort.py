#!/usr/bin/env python3

import unittest
import random
from typing import List
from quicksort import (
    quicksort_inplace, quicksort_functional, quicksort_three_way, 
    quicksort_parallel, SortableList
)

class TestQuicksort(unittest.TestCase):
    
    def test_empty_array(self):
        arr = []
        quicksort_inplace(arr)
        self.assertEqual(arr, [])
    
    def test_single_element(self):
        arr = [42]
        quicksort_inplace(arr)
        self.assertEqual(arr, [42])
    
    def test_sorted_array(self):
        arr = [1, 2, 3, 4, 5]
        quicksort_inplace(arr)
        self.assertEqual(arr, [1, 2, 3, 4, 5])
    
    def test_reverse_array(self):
        arr = [5, 4, 3, 2, 1]
        quicksort_inplace(arr)
        self.assertEqual(arr, [1, 2, 3, 4, 5])
    
    def test_random_array(self):
        arr = [3, 1, 4, 1, 5, 9, 2, 6]
        quicksort_inplace(arr)
        self.assertEqual(arr, [1, 1, 2, 3, 4, 5, 6, 9])
    
    def test_duplicates(self):
        arr = [5, 5, 5, 5, 5]
        quicksort_inplace(arr)
        self.assertEqual(arr, [5, 5, 5, 5, 5])
    
    def test_negative_numbers(self):
        arr = [3, -1, 4, -1, 5, 9, -2, 6]
        quicksort_inplace(arr)
        self.assertEqual(arr, [-2, -1, -1, 3, 4, 5, 6, 9])
    
    def test_functional_quicksort(self):
        arr = [3, 1, 4, 1, 5, 9, 2, 6]
        sorted_arr = quicksort_functional(arr)
        self.assertEqual(sorted_arr, [1, 1, 2, 3, 4, 5, 6, 9])
        # Original array should be unchanged
        self.assertEqual(arr, [3, 1, 4, 1, 5, 9, 2, 6])
    
    def test_three_way_quicksort(self):
        arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5]
        quicksort_three_way(arr)
        self.assertEqual(arr, [1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9])
    
    def test_parallel_quicksort(self):
        arr = [3, 1, 4, 1, 5, 9, 2, 6]
        sorted_arr = quicksort_parallel(arr)
        self.assertEqual(sorted_arr, [1, 1, 2, 3, 4, 5, 6, 9])
    
    def test_trait_implementation(self):
        arr = SortableList([3, 1, 4, 1, 5, 9, 2, 6])
        self.assertFalse(arr.is_sorted())
        arr.sort()
        self.assertTrue(arr.is_sorted())
        self.assertEqual(list(arr), [1, 1, 2, 3, 4, 5, 6, 9])
    
    def test_property_sorted_has_same_elements(self):
        """Property test: sorted array has same elements as original."""
        for _ in range(100):
            arr = [random.randint(-100, 100) for _ in range(random.randint(0, 50))]
            sorted_arr = arr.copy()
            quicksort_inplace(sorted_arr)
            
            orig_sorted = sorted(arr)
            self.assertEqual(sorted_arr, orig_sorted)
    
    def test_property_sorted_is_ordered(self):
        """Property test: sorted array is in order."""
        for _ in range(100):
            arr = [random.randint(-100, 100) for _ in range(random.randint(0, 50))]
            quicksort_inplace(arr)
            
            for i in range(len(arr) - 1):
                self.assertLessEqual(arr[i], arr[i + 1])
    
    def test_stability_preservation(self):
        """Test that all implementations produce same results."""
        for _ in range(50):
            arr = [random.randint(-50, 50) for _ in range(random.randint(0, 30))]
            
            arr1 = arr.copy()
            quicksort_inplace(arr1)
            
            arr2 = quicksort_functional(arr)
            
            arr3 = arr.copy()
            quicksort_three_way(arr3)
            
            arr4 = quicksort_parallel(arr)
            
            self.assertEqual(arr1, arr2)
            self.assertEqual(arr2, arr3)
            self.assertEqual(arr3, arr4)

if __name__ == '__main__':
    # Set random seed for reproducible tests
    random.seed(42)
    unittest.main()