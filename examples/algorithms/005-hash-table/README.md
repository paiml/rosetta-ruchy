# 005-hash-table

## Problem Statement

Implement comprehensive hash table data structure with multiple collision resolution strategies:
- **Open Addressing**: Linear probing, quadratic probing, double hashing
- **Separate Chaining**: Linked lists, dynamic arrays, balanced trees
- **Dynamic Resizing**: Load factor management with automatic growth/shrinking
- **Hash Functions**: Multiple algorithms with quality analysis
- **Collision Analysis**: Distribution testing and performance measurement

## Algorithm Complexity

- **Time**: O(1) average case, O(n) worst case for all operations
- **Space**: O(n) for n key-value pairs
- **Load Factor**: α ≤ 0.75 for optimal performance
- **Resize**: Amortized O(1) with proper load factor management

## Test Cases

### Basic Operations
- Empty table operations
- Single insert/delete/lookup
- Missing key lookups
- Value updates and overwrites

### Collision Scenarios
- **Linear Probing**: Sequential collision resolution
- **Chaining**: Multiple items per bucket
- **Hash Quality**: Distribution analysis across buckets
- **Worst Case**: Pathological key patterns

### Load Factor Management
- **Resize Trigger**: Growth when α > 0.75
- **Shrink Trigger**: Reduction when α < 0.25
- **Capacity**: Powers-of-2 sizing for efficient modulo
- **Rehashing**: All elements redistributed during resize

## Performance Targets

| Metric | Target |
|--------|---------|
| 100K insertions/sec | > 500K ops/sec |
| 100K lookups/sec | > 1M ops/sec |  
| Memory overhead | < 25% |
| Cache miss ratio | < 5% |
| Hash distribution | Chi-squared p < 0.05 |

## Hash Function Analysis

### String Hashing
- **DJB2**: `hash = hash * 33 + char` - simple and effective
- **FNV-1a**: `hash = (hash ^ char) * prime` - good distribution  
- **SDBM**: `hash = char + (hash << 6) + (hash << 16) - hash`
- **MurmurHash3**: Cryptographically strong, excellent distribution

### Integer Hashing
- **Division Method**: `h(k) = k mod m` - simple but can cluster
- **Multiplication Method**: `h(k) = floor(m * (k * A mod 1))` - better distribution
- **Universal Hashing**: Randomized function family - theoretical guarantees

## Ruchy v1.5.0 Advanced Features

### Self-Hosting Hash Generation
Use Ruchy's self-hosting compiler to generate optimized hash functions at runtime:
```ruchy
// Generate hash function using v1.5.0 self-hosting
let optimized_hash = compiler.generate_hash_function("string", "djb2")?;
```

### Algorithm W Type Inference
Complex generic constraints automatically resolved:
```ruchy
trait HashTable<K, V> where K: Hash + Eq + Clone, V: Clone {
    // Algorithm W infers all constraint propagation automatically
    fn insert_with_analysis(&mut self, key: K, value: V) -> InsertResult<V>;
}
```

### Advanced Hash Analysis
- **Runtime Quality Metrics**: Distribution uniformity, collision rates
- **Formal Verification**: SMT solver verification of hash properties
- **Performance Profiling**: Cache efficiency, memory access patterns
- **Self-Optimization**: Adaptive hash function selection

## Implementation Variants

### Open Addressing Strategies
- **Linear Probing**: `h(k, i) = (h(k) + i) mod m`
- **Quadratic Probing**: `h(k, i) = (h(k) + c₁i + c₂i²) mod m`
- **Double Hashing**: `h(k, i) = (h₁(k) + i·h₂(k)) mod m`

### Separate Chaining Options
- **Linked Lists**: Simple implementation, good for low load factors
- **Dynamic Arrays**: Better cache locality, supports binary search
- **Balanced Trees**: O(log n) worst-case guarantees

### Resizing Policies
- **Growth**: Double capacity when α > 0.75
- **Shrinking**: Halve capacity when α < 0.25  
- **Hysteresis**: Prevent oscillation with different grow/shrink thresholds
- **Incremental**: Spread rehashing across multiple operations

### Hash Quality Metrics
- **Uniformity**: Chi-squared goodness-of-fit test
- **Independence**: Correlation between hash values
- **Avalanche Effect**: Bit change propagation
- **Collision Rate**: Measured vs theoretical expectations