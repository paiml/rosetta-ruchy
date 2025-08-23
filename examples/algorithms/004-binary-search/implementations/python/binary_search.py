#!/usr/bin/env python3
"""
Binary Search - Python Implementation
Ergonomic baseline for Ruchy comparison
"""

from typing import List, Optional, TypeVar, Generic
import time
from collections import defaultdict

T = TypeVar('T')

class BinarySearcher(Generic[T]):
    """Enhanced binary search with caching and performance tracking"""
    
    def __init__(self, data: List[T]):
        self.data = sorted(data)  # Ensure data is sorted
        self.cache = {}
        self.stats = {
            'cache_hits': 0,
            'cache_misses': 0,
            'total_comparisons': 0,
            'total_searches': 0
        }
    
    def binary_search(self, target: T) -> Optional[int]:
        """Basic binary search implementation"""
        left, right = 0, len(self.data)
        comparisons = 0
        
        while left < right:
            comparisons += 1
            mid = left + (right - left) // 2
            
            if self.data[mid] == target:
                self.stats['total_comparisons'] += comparisons
                return mid
            elif self.data[mid] < target:
                left = mid + 1
            else:
                right = mid
        
        self.stats['total_comparisons'] += comparisons
        return None
    
    def search_with_cache(self, target: T) -> Optional[int]:
        """Enhanced search with caching"""
        self.stats['total_searches'] += 1
        
        # Check cache first
        if target in self.cache:
            self.stats['cache_hits'] += 1
            print(f"🎯 Cache hit for target: {target}")
            return self.cache[target]
        
        # Cache miss - perform search
        self.stats['cache_misses'] += 1
        result = self.binary_search(target)
        
        # Cache the result
        self.cache[target] = result
        print(f"💾 Cached result for {target}: {result}")
        
        return result
    
    def binary_search_leftmost(self, target: T) -> Optional[int]:
        """Find leftmost occurrence of target"""
        left, right = 0, len(self.data)
        
        while left < right:
            mid = left + (right - left) // 2
            
            if self.data[mid] < target:
                left = mid + 1
            else:
                right = mid
        
        if left < len(self.data) and self.data[left] == target:
            return left
        return None
    
    def binary_search_rightmost(self, target: T) -> Optional[int]:
        """Find rightmost occurrence of target"""
        left, right = 0, len(self.data)
        
        while left < right:
            mid = left + (right - left) // 2
            
            if self.data[mid] <= target:
                left = mid + 1
            else:
                right = mid
        
        if left > 0 and self.data[left - 1] == target:
            return left - 1
        return None
    
    def search_range(self, start: T, end: T) -> List[int]:
        """Find all indices in range [start, end]"""
        left = self.binary_search_leftmost(start)
        right = self.binary_search_rightmost(end)
        
        if left is None or right is None:
            return []
        
        return list(range(left, right + 1))
    
    def show_performance(self):
        """Display performance statistics"""
        total = self.stats['total_searches']
        hits = self.stats['cache_hits']
        misses = self.stats['cache_misses']
        comparisons = self.stats['total_comparisons']
        
        hit_rate = (hits / total * 100) if total > 0 else 0
        avg_comparisons = (comparisons / misses) if misses > 0 else 0
        
        print(f"\n📊 Performance Statistics:")
        print(f"   Total searches: {total}")
        print(f"   Cache hits: {hits} ({hit_rate:.1f}%)")
        print(f"   Cache misses: {misses}")
        print(f"   Total comparisons: {comparisons}")
        if misses > 0:
            print(f"   Avg comparisons per search: {avg_comparisons:.1f}")

def binary_search_simple(arr: List[T], target: T) -> Optional[int]:
    """Standalone binary search function"""
    left, right = 0, len(arr)
    
    while left < right:
        mid = left + (right - left) // 2
        
        if arr[mid] == target:
            return mid
        elif arr[mid] < target:
            left = mid + 1
        else:
            right = mid
    
    return None

def benchmark_search(data: List[int], targets: List[int], iterations: int = 10000) -> float:
    """Benchmark binary search performance"""
    start_time = time.perf_counter()
    
    for _ in range(iterations):
        for target in targets:
            binary_search_simple(data, target)
    
    end_time = time.perf_counter()
    total_operations = iterations * len(targets)
    
    return (end_time - start_time) * 1_000_000_000 / total_operations  # ns per operation

def interpolation_search(arr: List[int], target: int) -> Optional[int]:
    """Interpolation search for uniformly distributed data"""
    left, right = 0, len(arr) - 1
    
    while left <= right and target >= arr[left] and target <= arr[right]:
        if left == right:
            return left if arr[left] == target else None
        
        # Interpolation formula
        pos = left + int(((target - arr[left]) * (right - left)) / (arr[right] - arr[left]))
        pos = max(left, min(pos, right))  # Ensure pos is within bounds
        
        if arr[pos] == target:
            return pos
        elif arr[pos] < target:
            left = pos + 1
        else:
            right = pos - 1
    
    return None

def main():
    print("🔍 Binary Search - Python Implementation")
    print("========================================")
    
    # Test data
    data = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 23, 29, 31, 37, 41, 43, 47]
    print(f"📊 Test data ({len(data)} elements): {data}")
    print()
    
    # Test basic binary search
    print("🔍 Basic binary search tests:")
    for target in [5, 11, 20, 1, 47, 25]:
        result = binary_search_simple(data, target)
        if result is not None:
            print(f"   ✅ Found {target} at index {result}")
        else:
            print(f"   ❌ {target} not found")
    
    # Test boundary searches with duplicates
    print("\n🔄 Boundary search tests (with duplicates):")
    duplicates = [1, 2, 2, 2, 3, 5, 5, 7, 7, 7, 9]
    searcher_dup = BinarySearcher(duplicates)
    print(f"   Data with duplicates: {duplicates}")
    
    for target in [2, 5, 7]:
        leftmost = searcher_dup.binary_search_leftmost(target)
        rightmost = searcher_dup.binary_search_rightmost(target)
        print(f"   {target} -> leftmost: {leftmost}, rightmost: {rightmost}")
    
    # Test range queries
    print(f"\n📏 Range query tests:")
    range_indices = searcher_dup.search_range(2, 7)
    print(f"   Elements in range [2, 7]: indices {range_indices}")
    range_values = [duplicates[i] for i in range_indices]
    print(f"   Values: {range_values}")
    
    print("\n🚀 Enhanced search with caching:")
    
    # Test enhanced searcher with caching
    searcher = BinarySearcher(data)
    
    # Demonstrate caching behavior
    test_targets = [7, 15, 7, 23, 15, 7, 41, 23]
    for target in test_targets:
        result = searcher.search_with_cache(target)
        if result is not None:
            print(f"   ✅ Found {target} at index {result}")
        else:
            print(f"   ❌ {target} not found")
    
    # Show performance statistics
    searcher.show_performance()
    
    # Test interpolation search
    print(f"\n🔬 Interpolation search test:")
    uniform_data = list(range(0, 100, 2))  # [0, 2, 4, 6, ..., 98]
    print(f"   Uniform data sample: {uniform_data[:10]} ... {uniform_data[-10:]}")
    
    for target in [20, 50, 77, 99]:
        result = interpolation_search(uniform_data, target)
        if result is not None:
            print(f"   ✅ Interpolation found {target} at index {result}")
        else:
            print(f"   ❌ Interpolation: {target} not found")
    
    # Benchmark performance
    print(f"\n⚡ Performance benchmark:")
    benchmark_targets = [5, 11, 15, 23, 29, 37, 99]  # Mix of found/not found
    avg_time = benchmark_search(data, benchmark_targets, 10000)
    print(f"   Average search time: {avg_time:.2f} ns per operation")
    
    print(f"\n✅ Python binary search demonstration complete")
    print(f"🐍 Ergonomic baseline established for Ruchy comparison")

if __name__ == "__main__":
    main()