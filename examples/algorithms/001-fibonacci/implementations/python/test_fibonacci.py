#!/usr/bin/env python3
"""
Test suite for Fibonacci implementations
"""

import unittest
from fibonacci import (
    fib_recursive, fib_iterative, fib_memoized, fib_lru_cached,
    fib_matrix, fib_matrix_numpy, fib_tail_recursive, fib_generator,
    FibonacciBenchmark
)


class TestFibonacci(unittest.TestCase):
    """Test all Fibonacci implementations"""
    
    def setUp(self):
        """Set up test cases"""
        self.test_cases = [
            (0, 0),
            (1, 1),
            (2, 1),
            (3, 2),
            (4, 3),
            (5, 5),
            (6, 8),
            (7, 13),
            (8, 21),
            (9, 34),
            (10, 55),
            (20, 6765),
            (30, 832040),
        ]
        
        self.large_test_cases = [
            (40, 102334155),
            (50, 12586269025),
        ]
    
    def test_recursive(self):
        """Test recursive implementation"""
        for n, expected in self.test_cases:
            if n <= 30:  # Avoid slow recursive calls
                self.assertEqual(fib_recursive(n), expected)
    
    def test_iterative(self):
        """Test iterative implementation"""
        for n, expected in self.test_cases + self.large_test_cases:
            self.assertEqual(fib_iterative(n), expected)
    
    def test_memoized(self):
        """Test memoized implementation"""
        for n, expected in self.test_cases + self.large_test_cases:
            self.assertEqual(fib_memoized(n), expected)
    
    def test_lru_cached(self):
        """Test LRU-cached implementation"""
        fib_lru_cached.cache_clear()
        for n, expected in self.test_cases + self.large_test_cases:
            self.assertEqual(fib_lru_cached(n), expected)
    
    def test_matrix(self):
        """Test matrix multiplication implementation"""
        for n, expected in self.test_cases + self.large_test_cases:
            self.assertEqual(fib_matrix(n), expected)
    
    def test_matrix_numpy(self):
        """Test NumPy matrix implementation"""
        try:
            for n, expected in self.test_cases + self.large_test_cases:
                self.assertEqual(fib_matrix_numpy(n), expected)
        except ImportError:
            self.skipTest("NumPy not available")
    
    def test_tail_recursive(self):
        """Test tail-recursive implementation"""
        for n, expected in self.test_cases:
            if n <= 30:  # Python doesn't optimize tail recursion
                self.assertEqual(fib_tail_recursive(n), expected)
    
    def test_generator(self):
        """Test generator implementation"""
        for n, expected in self.test_cases:
            gen = fib_generator(n)
            result = None
            for _ in range(n + 1):
                result = next(gen)
            self.assertEqual(result, expected)
    
    def test_large_numbers(self):
        """Test with very large numbers"""
        n = 100
        result = fib_iterative(n)
        self.assertEqual(result, 354224848179261915075)
        
        n = 200
        result = fib_iterative(n)
        self.assertEqual(result, 280571172992510140037611932413038677189525)
    
    def test_benchmark_runner(self):
        """Test the benchmark runner"""
        benchmark = FibonacciBenchmark()
        
        # Test single run
        result, duration_ns = benchmark.run(10, 'iterative')
        self.assertEqual(result, 55)
        self.assertGreater(duration_ns, 0)
        
        # Test all benchmarks
        results = benchmark.benchmark_all(20)
        self.assertIn('iterative', results)
        self.assertIn('memoized', results)
        
        # Check results are correct
        for variant, data in results.items():
            if 'error' not in data:
                self.assertEqual(data['result'], 6765)
    
    def test_consistency(self):
        """Test that all implementations give the same results"""
        benchmark = FibonacciBenchmark()
        
        for n in [10, 20, 30]:
            results = benchmark.benchmark_all(n)
            
            # Get reference result from iterative
            reference = results['iterative']['result']
            
            # Check all other implementations match
            for variant, data in results.items():
                if 'error' not in data:
                    self.assertEqual(
                        data['result'], 
                        reference, 
                        f"Mismatch in {variant} for n={n}"
                    )


class TestPerformance(unittest.TestCase):
    """Performance-related tests"""
    
    def test_performance_ordering(self):
        """Test that implementations have expected performance characteristics"""
        benchmark = FibonacciBenchmark()
        n = 25
        
        results = benchmark.benchmark_all(n)
        
        # Matrix should be faster than recursive for large n
        if 'recursive' in results and 'matrix' in results:
            self.assertLess(
                results['matrix']['time_ns'],
                results['recursive']['time_ns']
            )
        
        # Memoized should be faster than plain recursive
        if 'recursive' in results and 'memoized' in results:
            self.assertLess(
                results['memoized']['time_ns'],
                results['recursive']['time_ns']
            )


if __name__ == '__main__':
    unittest.main()