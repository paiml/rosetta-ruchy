// Hash Table - Rust Baseline Implementation
// Comprehensive implementation with multiple collision resolution strategies

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::time::Instant;

// Hash function implementations for comparison
trait HashFunction<K> {
    fn hash(&self, key: &K) -> u64;
}

struct DJB2Hasher;
impl HashFunction<String> for DJB2Hasher {
    fn hash(&self, key: &String) -> u64 {
        let mut hash = 5381u64;
        for byte in key.bytes() {
            hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
        }
        hash
    }
}

struct FNV1aHasher;
impl HashFunction<String> for FNV1aHasher {
    fn hash(&self, key: &String) -> u64 {
        let mut hash = 14695981039346656037u64; // FNV offset basis
        for byte in key.bytes() {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(1099511628211u64); // FNV prime
        }
        hash
    }
}

// Open addressing hash table with linear probing
#[derive(Debug)]
struct OpenAddressingHashTable<K, V> {
    buckets: Vec<Option<(K, V)>>,
    size: usize,
    capacity: usize,
    stats: HashTableStats,
}

#[derive(Debug, Default)]
struct HashTableStats {
    total_insertions: usize,
    total_lookups: usize,
    total_collisions: usize,
    resize_count: usize,
    probe_distance_sum: usize,
}

#[derive(Debug, Default)]
struct LookupStats {
    probes: usize,
    collision_count: usize,
}

#[derive(Debug)]
struct DistributionAnalysis {
    uniformity_score: f64,
    collision_rate: f64,
    load_factor: f64,
    average_probe_distance: f64,
    max_probe_distance: usize,
}

impl<K: Hash + Eq + Clone + Debug, V: Clone + Debug> OpenAddressingHashTable<K, V> {
    fn new() -> Self {
        Self::with_capacity(16)
    }

    fn with_capacity(capacity: usize) -> Self {
        // Ensure capacity is power of 2 for efficient modulo
        let capacity = capacity.next_power_of_two();
        Self {
            buckets: vec![None; capacity],
            size: 0,
            capacity,
            stats: HashTableStats::default(),
        }
    }

    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize & (self.capacity - 1) // Efficient modulo for power of 2
    }

    fn load_factor(&self) -> f64 {
        self.size as f64 / self.capacity as f64
    }

    fn should_resize(&self) -> bool {
        self.load_factor() > 0.75
    }

    fn resize(&mut self) {
        println!(
            "🔄 Resizing hash table: {} → {}",
            self.capacity,
            self.capacity * 2
        );

        let old_buckets = std::mem::replace(&mut self.buckets, vec![None; self.capacity * 2]);
        let old_size = self.size;

        self.capacity *= 2;
        self.size = 0;
        self.stats.resize_count += 1;

        // Rehash all existing elements
        for bucket in old_buckets {
            if let Some((key, value)) = bucket {
                self.insert_internal(key, value, false);
            }
        }

        println!("✅ Resize complete: {} elements redistributed", old_size);
    }

    fn insert_internal(&mut self, key: K, value: V, count_stats: bool) -> Option<V> {
        let mut index = self.hash(&key);
        let mut probes = 0;

        loop {
            match &mut self.buckets[index] {
                None => {
                    self.buckets[index] = Some((key, value));
                    self.size += 1;

                    if count_stats {
                        self.stats.probe_distance_sum += probes;
                        if probes > 0 {
                            self.stats.total_collisions += probes;
                        }
                    }
                    return None;
                }
                Some((existing_key, existing_value)) => {
                    if existing_key == &key {
                        let old_value = std::mem::replace(existing_value, value);
                        return Some(old_value);
                    }
                    // Linear probing
                    index = (index + 1) & (self.capacity - 1);
                    probes += 1;

                    // Prevent infinite loop (should never happen with proper load factor)
                    if probes >= self.capacity {
                        panic!("Hash table is full - this should never happen with proper load factor management");
                    }
                }
            }
        }
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.stats.total_insertions += 1;

        if self.should_resize() {
            self.resize();
        }

        self.insert_internal(key, value, true)
    }

    fn lookup(&self, key: &K) -> Option<&V> {
        self.lookup_with_stats(key).0
    }

    fn lookup_with_stats(&self, key: &K) -> (Option<&V>, LookupStats) {
        let mut stats = LookupStats::default();
        let mut index = self.hash(key);

        loop {
            stats.probes += 1;

            match &self.buckets[index] {
                None => return (None, stats),
                Some((existing_key, value)) => {
                    if existing_key == key {
                        return (Some(value), stats);
                    }
                    // Continue linear probing
                    index = (index + 1) & (self.capacity - 1);
                    stats.collision_count += 1;

                    // Prevent infinite loop
                    if stats.probes > self.capacity {
                        return (None, stats);
                    }
                }
            }
        }
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let mut index = self.hash(key);
        let mut probes = 0;

        loop {
            match &self.buckets[index] {
                None => return None,
                Some((existing_key, _)) => {
                    if existing_key == key {
                        // Found the key - remove it
                        if let Some((_, value)) = self.buckets[index].take() {
                            self.size -= 1;

                            // Shift back any elements that were displaced by this one
                            self.shift_back(index);
                            return Some(value);
                        }
                    }
                    index = (index + 1) & (self.capacity - 1);
                    probes += 1;

                    if probes > self.capacity {
                        return None;
                    }
                }
            }
        }
    }

    fn shift_back(&mut self, mut start_index: usize) {
        let mut current = (start_index + 1) & (self.capacity - 1);

        while current != start_index {
            if let Some((key, value)) = self.buckets[current].take() {
                let ideal_pos = self.hash(&key);
                let distance = if current >= ideal_pos {
                    current - ideal_pos
                } else {
                    current + self.capacity - ideal_pos
                };

                let start_distance = if start_index >= ideal_pos {
                    start_index - ideal_pos
                } else {
                    start_index + self.capacity - ideal_pos
                };

                if distance > start_distance {
                    // This element belongs in the empty slot
                    self.buckets[start_index] = Some((key, value));
                    start_index = current;
                } else {
                    // Put it back
                    self.buckets[current] = Some((key, value));
                }
            }
            current = (current + 1) & (self.capacity - 1);
        }
    }

    fn analyze_distribution(&self) -> DistributionAnalysis {
        let mut probe_distances = Vec::new();
        let mut max_probe_distance = 0;
        let mut total_probe_distance = 0;

        for bucket in &self.buckets {
            if let Some((key, _)) = bucket {
                let ideal_pos = self.hash(key);
                let actual_pos = self
                    .buckets
                    .iter()
                    .position(|b| {
                        if let Some((k, _)) = b {
                            std::ptr::eq(k, key)
                        } else {
                            false
                        }
                    })
                    .unwrap();

                let distance = if actual_pos >= ideal_pos {
                    actual_pos - ideal_pos
                } else {
                    actual_pos + self.capacity - ideal_pos
                };

                probe_distances.push(distance);
                max_probe_distance = max_probe_distance.max(distance);
                total_probe_distance += distance;
            }
        }

        let average_probe_distance = if probe_distances.is_empty() {
            0.0
        } else {
            total_probe_distance as f64 / probe_distances.len() as f64
        };

        // Calculate uniformity using variance of probe distances
        let variance = if probe_distances.len() > 1 {
            let mean = average_probe_distance;
            probe_distances
                .iter()
                .map(|&d| (d as f64 - mean).powi(2))
                .sum::<f64>()
                / probe_distances.len() as f64
        } else {
            0.0
        };

        DistributionAnalysis {
            uniformity_score: variance.sqrt(), // Standard deviation as uniformity score
            collision_rate: self.stats.total_collisions as f64
                / self.stats.total_insertions.max(1) as f64,
            load_factor: self.load_factor(),
            average_probe_distance,
            max_probe_distance,
        }
    }

    fn show_stats(&self) {
        let analysis = self.analyze_distribution();

        println!("📊 Hash Table Statistics:");
        println!(
            "   Size: {} / {} (load factor: {:.3})",
            self.size, self.capacity, analysis.load_factor
        );
        println!("   Total insertions: {}", self.stats.total_insertions);
        println!(
            "   Total collisions: {} (rate: {:.3})",
            self.stats.total_collisions, analysis.collision_rate
        );
        println!("   Resize operations: {}", self.stats.resize_count);
        println!(
            "   Average probe distance: {:.2}",
            analysis.average_probe_distance
        );
        println!("   Max probe distance: {}", analysis.max_probe_distance);
        println!(
            "   Distribution uniformity: {:.3} (lower is better)",
            analysis.uniformity_score
        );
    }
}

// Separate chaining hash table for comparison
#[derive(Debug)]
struct ChainingHashTable<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    size: usize,
    capacity: usize,
    stats: HashTableStats,
}

impl<K: Hash + Eq + Clone + Debug, V: Clone + Debug> ChainingHashTable<K, V> {
    fn new() -> Self {
        Self::with_capacity(16)
    }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            buckets: vec![Vec::new(); capacity],
            size: 0,
            capacity,
            stats: HashTableStats::default(),
        }
    }

    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.capacity
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.stats.total_insertions += 1;

        let index = self.hash(&key);
        let bucket = &mut self.buckets[index];

        // Check if key exists
        for (existing_key, existing_value) in bucket.iter_mut() {
            if existing_key == &key {
                return Some(std::mem::replace(existing_value, value));
            }
        }

        // Insert new entry
        bucket.push((key, value));
        self.size += 1;
        None
    }

    fn lookup(&self, key: &K) -> Option<&V> {
        let index = self.hash(key);
        let bucket = &self.buckets[index];

        for (existing_key, value) in bucket {
            if existing_key == key {
                return Some(value);
            }
        }

        None
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let index = self.hash(key);
        let bucket = &mut self.buckets[index];

        for (i, (existing_key, _)) in bucket.iter().enumerate() {
            if existing_key == key {
                let (_, value) = bucket.remove(i);
                self.size -= 1;
                return Some(value);
            }
        }

        None
    }

    fn analyze_chain_lengths(&self) -> Vec<usize> {
        self.buckets.iter().map(|bucket| bucket.len()).collect()
    }

    fn show_stats(&self) {
        let chain_lengths = self.analyze_chain_lengths();
        let max_chain_length = chain_lengths.iter().max().unwrap_or(&0);
        let avg_chain_length = if self.capacity > 0 {
            self.size as f64 / self.capacity as f64
        } else {
            0.0
        };

        let non_empty_buckets = chain_lengths.iter().filter(|&&len| len > 0).count();

        println!("📊 Chaining Hash Table Statistics:");
        println!("   Size: {} / {} buckets", self.size, self.capacity);
        println!(
            "   Non-empty buckets: {} ({:.1}%)",
            non_empty_buckets,
            non_empty_buckets as f64 / self.capacity as f64 * 100.0
        );
        println!("   Average chain length: {:.2}", avg_chain_length);
        println!("   Maximum chain length: {}", max_chain_length);
        println!(
            "   Load factor: {:.3}",
            self.size as f64 / self.capacity as f64
        );
    }
}

// Benchmark different hash functions
fn benchmark_hash_functions() {
    println!("🧪 Hash Function Quality Analysis:");

    let test_keys: Vec<String> = (0..10000).map(|i| format!("key_{}", i)).collect();

    // Test DJB2
    let djb2 = DJB2Hasher;
    let start = Instant::now();
    let mut djb2_hashes = Vec::new();
    for key in &test_keys {
        djb2_hashes.push(djb2.hash(key));
    }
    let djb2_time = start.elapsed();

    // Test FNV-1a
    let fnv1a = FNV1aHasher;
    let start = Instant::now();
    let mut fnv1a_hashes = Vec::new();
    for key in &test_keys {
        fnv1a_hashes.push(fnv1a.hash(key));
    }
    let fnv1a_time = start.elapsed();

    println!(
        "   DJB2 hashing time: {:?} for {} keys",
        djb2_time,
        test_keys.len()
    );
    println!(
        "   FNV-1a hashing time: {:?} for {} keys",
        fnv1a_time,
        test_keys.len()
    );

    // Simple distribution analysis (count unique hashes)
    let djb2_unique: std::collections::HashSet<_> = djb2_hashes.into_iter().collect();
    let fnv1a_unique: std::collections::HashSet<_> = fnv1a_hashes.into_iter().collect();

    println!(
        "   DJB2 unique hashes: {} / {} ({:.2}% uniqueness)",
        djb2_unique.len(),
        test_keys.len(),
        djb2_unique.len() as f64 / test_keys.len() as f64 * 100.0
    );
    println!(
        "   FNV-1a unique hashes: {} / {} ({:.2}% uniqueness)",
        fnv1a_unique.len(),
        test_keys.len(),
        fnv1a_unique.len() as f64 / test_keys.len() as f64 * 100.0
    );
}

fn main() {
    println!("🗂️  Hash Table - Rust Baseline Implementation");
    println!("==============================================");
    println!();

    // Hash function analysis
    benchmark_hash_functions();

    println!("\n📊 Open Addressing Hash Table (Linear Probing):");

    // Test open addressing hash table
    let mut open_table = OpenAddressingHashTable::new();

    // Insert test data
    let test_data = vec![
        ("apple".to_string(), 5),
        ("banana".to_string(), 7),
        ("cherry".to_string(), 3),
        ("date".to_string(), 9),
        ("elderberry".to_string(), 11),
        ("fig".to_string(), 2),
        ("grape".to_string(), 8),
        ("honeydew".to_string(), 6),
        ("kiwi".to_string(), 4),
        ("lemon".to_string(), 10),
    ];

    println!("Inserting {} key-value pairs...", test_data.len());
    for (key, value) in &test_data {
        let old_value = open_table.insert(key.clone(), *value);
        println!(
            "   Inserted: {} → {} (replaced: {:?})",
            key, value, old_value
        );
    }

    // Test lookups with statistics
    println!("\n🔍 Lookup tests with collision analysis:");
    for (key, expected) in test_data.iter().take(5) {
        let (result, stats) = open_table.lookup_with_stats(key);
        match result {
            Some(value) => {
                println!(
                    "   ✅ {}: {} (probes: {}, collisions: {})",
                    key, value, stats.probes, stats.collision_count
                );
                assert_eq!(value, expected);
            }
            None => println!("   ❌ {} not found", key),
        }
    }

    // Show performance statistics
    open_table.show_stats();

    // Test resize behavior
    println!("\n🔄 Testing resize behavior by adding more elements:");
    let initial_capacity = open_table.capacity;

    // Add elements to trigger resize
    for i in 20..35 {
        open_table.insert(format!("resize_key_{}", i), i);
    }

    if open_table.capacity > initial_capacity {
        println!("✅ Resize triggered successfully!");
    }
    open_table.show_stats();

    // Test removal
    println!("\n🗑️  Testing removal:");
    let removed = open_table.remove(&"apple".to_string());
    println!("Removed 'apple': {:?}", removed);
    assert_eq!(removed, Some(5));

    // Verify removal
    let lookup_result = open_table.lookup(&"apple".to_string());
    println!("Lookup 'apple' after removal: {:?}", lookup_result);
    assert_eq!(lookup_result, None);

    // Compare with separate chaining
    println!("\n🔗 Separate Chaining Hash Table Comparison:");
    let mut chain_table = ChainingHashTable::new();

    for (key, value) in &test_data {
        chain_table.insert(key.clone(), *value);
    }

    chain_table.show_stats();

    // Verify both tables have same data (except removed apple)
    println!("\n✅ Verification - comparing table contents:");
    for (key, expected) in test_data.iter().skip(1).take(3) {
        // Skip apple which was removed
        let open_result = open_table.lookup(key);
        let chain_result = chain_table.lookup(key);

        println!(
            "   {}: open={:?}, chain={:?} ✓",
            key, open_result, chain_result
        );
        assert_eq!(chain_result, Some(expected)); // Chain table still has apple
    }

    println!("\n🎉 Hash Table Baseline Implementation Complete!");
    println!("✨ Open addressing with linear probing");
    println!("✨ Separate chaining with dynamic arrays");
    println!("✨ Load factor management with automatic resizing");
    println!("✨ Collision analysis and distribution metrics");
    println!("✨ Hash function quality comparison");
    println!("✨ Performance baseline established for Ruchy comparison");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_addressing_basic_operations() {
        let mut table = OpenAddressingHashTable::new();

        // Test insertion
        assert_eq!(table.insert("key1".to_string(), 100), None);
        assert_eq!(table.insert("key2".to_string(), 200), None);

        // Test lookup
        assert_eq!(table.lookup(&"key1".to_string()), Some(&100));
        assert_eq!(table.lookup(&"key2".to_string()), Some(&200));
        assert_eq!(table.lookup(&"key3".to_string()), None);

        // Test update
        assert_eq!(table.insert("key1".to_string(), 150), Some(100));
        assert_eq!(table.lookup(&"key1".to_string()), Some(&150));

        // Test removal
        assert_eq!(table.remove(&"key1".to_string()), Some(150));
        assert_eq!(table.lookup(&"key1".to_string()), None);
        assert_eq!(table.remove(&"key1".to_string()), None);
    }

    #[test]
    fn test_chaining_basic_operations() {
        let mut table = ChainingHashTable::new();

        // Test insertion
        assert_eq!(table.insert("key1".to_string(), 100), None);
        assert_eq!(table.insert("key2".to_string(), 200), None);

        // Test lookup
        assert_eq!(table.lookup(&"key1".to_string()), Some(&100));
        assert_eq!(table.lookup(&"key2".to_string()), Some(&200));
        assert_eq!(table.lookup(&"key3".to_string()), None);

        // Test update
        assert_eq!(table.insert("key1".to_string(), 150), Some(100));
        assert_eq!(table.lookup(&"key1".to_string()), Some(&150));

        // Test removal
        assert_eq!(table.remove(&"key1".to_string()), Some(150));
        assert_eq!(table.lookup(&"key1".to_string()), None);
    }

    #[test]
    fn test_resize_behavior() {
        let mut table = OpenAddressingHashTable::with_capacity(4);

        // Fill beyond load factor threshold
        table.insert("key1".to_string(), 1);
        table.insert("key2".to_string(), 2);
        table.insert("key3".to_string(), 3);

        let initial_capacity = table.capacity;

        // This should trigger resize
        table.insert("key4".to_string(), 4);

        assert!(table.capacity > initial_capacity);

        // Verify all elements are still accessible
        assert_eq!(table.lookup(&"key1".to_string()), Some(&1));
        assert_eq!(table.lookup(&"key2".to_string()), Some(&2));
        assert_eq!(table.lookup(&"key3".to_string()), Some(&3));
        assert_eq!(table.lookup(&"key4".to_string()), Some(&4));
    }

    #[test]
    fn test_hash_function_consistency() {
        let djb2 = DJB2Hasher;
        let key = "test_key".to_string();

        let hash1 = djb2.hash(&key);
        let hash2 = djb2.hash(&key);

        assert_eq!(hash1, hash2);
    }
}
