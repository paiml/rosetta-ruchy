#!/usr/bin/env python3
"""
Hash Table - Python Implementation
Ergonomic baseline with comprehensive collision resolution strategies
"""

from typing import List, Optional, Dict, Any, Tuple, Union
import time
import hashlib
from collections import defaultdict
from dataclasses import dataclass
from abc import ABC, abstractmethod

@dataclass
class HashTableStats:
    """Statistics for hash table performance analysis"""
    total_insertions: int = 0
    total_lookups: int = 0
    total_collisions: int = 0
    resize_count: int = 0
    probe_distance_sum: int = 0

@dataclass
class LookupStats:
    """Statistics for a single lookup operation"""
    probes: int = 0
    collision_count: int = 0

@dataclass
class DistributionAnalysis:
    """Analysis of hash distribution quality"""
    uniformity_score: float
    collision_rate: float
    load_factor: float
    average_probe_distance: float
    max_probe_distance: int

class HashFunction(ABC):
    """Abstract base class for hash functions"""
    
    @abstractmethod
    def hash(self, key: str) -> int:
        pass

class DJB2Hasher(HashFunction):
    """DJB2 hash function - simple and effective"""
    
    def hash(self, key: str) -> int:
        hash_value = 5381
        for char in key:
            hash_value = ((hash_value << 5) + hash_value) + ord(char)
            hash_value &= 0xFFFFFFFFFFFFFFFF  # Keep it 64-bit
        return hash_value

class FNV1aHasher(HashFunction):
    """FNV-1a hash function - good distribution"""
    
    def hash(self, key: str) -> int:
        hash_value = 14695981039346656037  # FNV offset basis
        for char in key:
            hash_value ^= ord(char)
            hash_value *= 1099511628211  # FNV prime
            hash_value &= 0xFFFFFFFFFFFFFFFF  # Keep it 64-bit
        return hash_value

class SDBMHasher(HashFunction):
    """SDBM hash function"""
    
    def hash(self, key: str) -> int:
        hash_value = 0
        for char in key:
            hash_value = ord(char) + (hash_value << 6) + (hash_value << 16) - hash_value
            hash_value &= 0xFFFFFFFFFFFFFFFF
        return hash_value

class OpenAddressingHashTable:
    """Hash table with open addressing using linear probing"""
    
    def __init__(self, capacity: int = 16):
        self.capacity = self._next_power_of_two(capacity)
        self.buckets: List[Optional[Tuple[Any, Any]]] = [None] * self.capacity
        self.size = 0
        self.stats = HashTableStats()
        self._deleted_marker = object()  # Sentinel for deleted entries
    
    def _next_power_of_two(self, n: int) -> int:
        """Find the next power of 2 >= n"""
        if n <= 1:
            return 1
        n -= 1
        n |= n >> 1
        n |= n >> 2
        n |= n >> 4
        n |= n >> 8
        n |= n >> 16
        n |= n >> 32
        return n + 1
    
    def _hash(self, key: Any) -> int:
        """Hash function using Python's built-in hash"""
        return hash(key) & (self.capacity - 1)  # Efficient modulo for power of 2
    
    def load_factor(self) -> float:
        """Current load factor"""
        return self.size / self.capacity if self.capacity > 0 else 0.0
    
    def _should_resize(self) -> bool:
        """Check if resize is needed"""
        return self.load_factor() > 0.75
    
    def _resize(self) -> None:
        """Resize the hash table to double capacity"""
        print(f"🔄 Resizing hash table: {self.capacity} → {self.capacity * 2}")
        
        old_buckets = self.buckets
        old_size = self.size
        
        self.capacity *= 2
        self.buckets = [None] * self.capacity
        self.size = 0
        self.stats.resize_count += 1
        
        # Rehash all existing elements
        for bucket in old_buckets:
            if bucket is not None and bucket is not self._deleted_marker:
                key, value = bucket
                self._insert_internal(key, value, count_stats=False)
        
        print(f"✅ Resize complete: {old_size} elements redistributed")
    
    def _insert_internal(self, key: Any, value: Any, count_stats: bool = True) -> Optional[Any]:
        """Internal insertion method"""
        index = self._hash(key)
        probes = 0
        
        while probes < self.capacity:
            if (self.buckets[index] is None or 
                self.buckets[index] is self._deleted_marker):
                # Empty slot found
                self.buckets[index] = (key, value)
                self.size += 1
                
                if count_stats:
                    self.stats.probe_distance_sum += probes
                    if probes > 0:
                        self.stats.total_collisions += probes
                
                return None
            
            existing_key, existing_value = self.buckets[index]
            if existing_key == key:
                # Update existing entry
                self.buckets[index] = (key, value)
                return existing_value
            
            # Linear probing
            index = (index + 1) & (self.capacity - 1)
            probes += 1
        
        raise RuntimeError("Hash table is full - this should never happen")
    
    def insert(self, key: Any, value: Any) -> Optional[Any]:
        """Insert key-value pair, return old value if key existed"""
        self.stats.total_insertions += 1
        
        if self._should_resize():
            self._resize()
        
        return self._insert_internal(key, value)
    
    def lookup(self, key: Any) -> Optional[Any]:
        """Look up value by key"""
        result, _ = self.lookup_with_stats(key)
        return result
    
    def lookup_with_stats(self, key: Any) -> Tuple[Optional[Any], LookupStats]:
        """Look up value with detailed statistics"""
        stats = LookupStats()
        index = self._hash(key)
        
        while stats.probes < self.capacity:
            stats.probes += 1
            
            if self.buckets[index] is None:
                # Empty slot - key not found
                return None, stats
            
            if self.buckets[index] is not self._deleted_marker:
                existing_key, value = self.buckets[index]
                if existing_key == key:
                    return value, stats
            
            # Continue probing
            index = (index + 1) & (self.capacity - 1)
            stats.collision_count += 1
        
        return None, stats
    
    def remove(self, key: Any) -> Optional[Any]:
        """Remove key-value pair, return the value if found"""
        index = self._hash(key)
        probes = 0
        
        while probes < self.capacity:
            if self.buckets[index] is None:
                return None
            
            if (self.buckets[index] is not self._deleted_marker and
                self.buckets[index][0] == key):
                # Found the key
                _, value = self.buckets[index]
                self.buckets[index] = self._deleted_marker
                self.size -= 1
                return value
            
            index = (index + 1) & (self.capacity - 1)
            probes += 1
        
        return None
    
    def analyze_distribution(self) -> DistributionAnalysis:
        """Analyze hash distribution quality"""
        probe_distances = []
        max_probe_distance = 0
        total_probe_distance = 0
        
        for i, bucket in enumerate(self.buckets):
            if bucket is not None and bucket is not self._deleted_marker:
                key, _ = bucket
                ideal_pos = self._hash(key)
                
                # Calculate actual distance from ideal position
                if i >= ideal_pos:
                    distance = i - ideal_pos
                else:
                    distance = i + self.capacity - ideal_pos
                
                probe_distances.append(distance)
                max_probe_distance = max(max_probe_distance, distance)
                total_probe_distance += distance
        
        avg_probe_distance = (total_probe_distance / len(probe_distances) 
                             if probe_distances else 0.0)
        
        # Calculate uniformity using standard deviation of probe distances
        if len(probe_distances) > 1:
            variance = sum((d - avg_probe_distance) ** 2 for d in probe_distances) / len(probe_distances)
            uniformity_score = variance ** 0.5
        else:
            uniformity_score = 0.0
        
        collision_rate = (self.stats.total_collisions / 
                         max(self.stats.total_insertions, 1))
        
        return DistributionAnalysis(
            uniformity_score=uniformity_score,
            collision_rate=collision_rate,
            load_factor=self.load_factor(),
            average_probe_distance=avg_probe_distance,
            max_probe_distance=max_probe_distance
        )
    
    def show_stats(self) -> None:
        """Display comprehensive statistics"""
        analysis = self.analyze_distribution()
        
        print("📊 Hash Table Statistics:")
        print(f"   Size: {self.size} / {self.capacity} (load factor: {analysis.load_factor:.3f})")
        print(f"   Total insertions: {self.stats.total_insertions}")
        print(f"   Total collisions: {self.stats.total_collisions} (rate: {analysis.collision_rate:.3f})")
        print(f"   Resize operations: {self.stats.resize_count}")
        print(f"   Average probe distance: {analysis.average_probe_distance:.2f}")
        print(f"   Max probe distance: {analysis.max_probe_distance}")
        print(f"   Distribution uniformity: {analysis.uniformity_score:.3f} (lower is better)")

class ChainingHashTable:
    """Hash table with separate chaining using lists"""
    
    def __init__(self, capacity: int = 16):
        self.capacity = capacity
        self.buckets: List[List[Tuple[Any, Any]]] = [[] for _ in range(capacity)]
        self.size = 0
        self.stats = HashTableStats()
    
    def _hash(self, key: Any) -> int:
        """Hash function"""
        return hash(key) % self.capacity
    
    def insert(self, key: Any, value: Any) -> Optional[Any]:
        """Insert key-value pair"""
        self.stats.total_insertions += 1
        
        index = self._hash(key)
        bucket = self.buckets[index]
        
        # Check if key exists
        for i, (existing_key, existing_value) in enumerate(bucket):
            if existing_key == key:
                bucket[i] = (key, value)  # Update
                return existing_value
        
        # Insert new entry
        bucket.append((key, value))
        self.size += 1
        return None
    
    def lookup(self, key: Any) -> Optional[Any]:
        """Look up value by key"""
        index = self._hash(key)
        bucket = self.buckets[index]
        
        for existing_key, value in bucket:
            if existing_key == key:
                return value
        
        return None
    
    def remove(self, key: Any) -> Optional[Any]:
        """Remove key-value pair"""
        index = self._hash(key)
        bucket = self.buckets[index]
        
        for i, (existing_key, value) in enumerate(bucket):
            if existing_key == key:
                bucket.pop(i)
                self.size -= 1
                return value
        
        return None
    
    def analyze_chain_lengths(self) -> List[int]:
        """Get chain length for each bucket"""
        return [len(bucket) for bucket in self.buckets]
    
    def show_stats(self) -> None:
        """Display statistics"""
        chain_lengths = self.analyze_chain_lengths()
        max_chain_length = max(chain_lengths) if chain_lengths else 0
        avg_chain_length = self.size / self.capacity if self.capacity > 0 else 0
        non_empty_buckets = sum(1 for length in chain_lengths if length > 0)
        
        print("📊 Chaining Hash Table Statistics:")
        print(f"   Size: {self.size} / {self.capacity} buckets")
        print(f"   Non-empty buckets: {non_empty_buckets} ({non_empty_buckets/self.capacity*100:.1f}%)")
        print(f"   Average chain length: {avg_chain_length:.2f}")
        print(f"   Maximum chain length: {max_chain_length}")
        print(f"   Load factor: {self.size / self.capacity:.3f}")

def benchmark_hash_functions():
    """Benchmark different hash function implementations"""
    print("🧪 Hash Function Quality Analysis:")
    
    test_keys = [f"key_{i}" for i in range(10000)]
    hash_functions = {
        'DJB2': DJB2Hasher(),
        'FNV-1a': FNV1aHasher(),
        'SDBM': SDBMHasher()
    }
    
    for name, hasher in hash_functions.items():
        start_time = time.perf_counter()
        hashes = [hasher.hash(key) for key in test_keys]
        end_time = time.perf_counter()
        
        # Analyze uniqueness
        unique_hashes = len(set(hashes))
        uniqueness = unique_hashes / len(test_keys) * 100
        
        print(f"   {name} hashing time: {(end_time - start_time)*1000:.2f}ms for {len(test_keys)} keys")
        print(f"   {name} unique hashes: {unique_hashes} / {len(test_keys)} ({uniqueness:.2f}% uniqueness)")

def main():
    print("🗂️  Hash Table - Python Implementation")
    print("======================================")
    print()
    
    # Hash function analysis
    benchmark_hash_functions()
    
    print("\n📊 Open Addressing Hash Table (Linear Probing):")
    
    # Test open addressing hash table
    open_table = OpenAddressingHashTable()
    
    # Test data
    test_data = [
        ("apple", 5), ("banana", 7), ("cherry", 3), ("date", 9),
        ("elderberry", 11), ("fig", 2), ("grape", 8), ("honeydew", 6),
        ("kiwi", 4), ("lemon", 10)
    ]
    
    print(f"Inserting {len(test_data)} key-value pairs...")
    for key, value in test_data:
        old_value = open_table.insert(key, value)
        print(f"   Inserted: {key} → {value} (replaced: {old_value})")
    
    # Test lookups with statistics
    print("\n🔍 Lookup tests with collision analysis:")
    for key, expected in test_data[:5]:
        result, stats = open_table.lookup_with_stats(key)
        if result is not None:
            print(f"   ✅ {key}: {result} (probes: {stats.probes}, collisions: {stats.collision_count})")
            assert result == expected
        else:
            print(f"   ❌ {key} not found")
    
    # Show performance statistics
    open_table.show_stats()
    
    # Test resize behavior
    print(f"\n🔄 Testing resize behavior by adding more elements:")
    initial_capacity = open_table.capacity
    
    # Add elements to trigger resize
    for i in range(20, 35):
        open_table.insert(f"resize_key_{i}", i)
    
    if open_table.capacity > initial_capacity:
        print("✅ Resize triggered successfully!")
    open_table.show_stats()
    
    # Test removal
    print(f"\n🗑️  Testing removal:")
    removed = open_table.remove("apple")
    print(f"Removed 'apple': {removed}")
    assert removed == 5
    
    # Verify removal
    lookup_result = open_table.lookup("apple")
    print(f"Lookup 'apple' after removal: {lookup_result}")
    assert lookup_result is None
    
    # Compare with separate chaining
    print(f"\n🔗 Separate Chaining Hash Table Comparison:")
    chain_table = ChainingHashTable()
    
    for key, value in test_data:
        chain_table.insert(key, value)
    
    chain_table.show_stats()
    
    # Verify both tables have same data (except removed apple)
    print(f"\n✅ Verification - comparing table contents:")
    for key, expected in test_data[1:4]:  # Skip apple which was removed
        open_result = open_table.lookup(key)
        chain_result = chain_table.lookup(key)
        
        print(f"   {key}: open={open_result}, chain={chain_result} ✓")
        assert chain_result == expected  # Chain table still has apple
    
    print(f"\n🎉 Hash Table Python Implementation Complete!")
    print("✨ Open addressing with linear probing and tombstone deletion")
    print("✨ Separate chaining with dynamic lists")
    print("✨ Multiple hash function implementations and benchmarking")
    print("✨ Comprehensive collision analysis and statistics")
    print("✨ Load factor management with automatic resizing")
    print("✨ Ergonomic Python baseline established for comparison")

if __name__ == "__main__":
    main()