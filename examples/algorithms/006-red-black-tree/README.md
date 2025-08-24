# 006-red-black-tree

## Problem Statement

Implement a self-balancing Red-Black tree with comprehensive operations:
- **Insertion**: Maintain balance through rotations and recoloring
- **Deletion**: Complex rebalancing with double-black resolution
- **Search**: Guaranteed O(log n) through height constraints
- **Traversal**: In-order, pre-order, post-order, level-order
- **Invariant Maintenance**: Enforce all five RB-tree properties

## Red-Black Tree Properties

1. **Node Color**: Every node is either red or black
2. **Root Property**: The root is always black
3. **Red Property**: Red nodes cannot have red children (no consecutive reds)
4. **Black Height**: All paths from root to NIL have same number of black nodes
5. **Leaf Property**: All leaves (NIL nodes) are black

## Algorithm Complexity

- **Time Complexity**: O(log n) for all operations (guaranteed)
- **Space Complexity**: O(n) for n nodes
- **Height Guarantee**: h ≤ 2 * log₂(n + 1)
- **Rotation Frequency**: ≤ 2 rotations per insertion, ≤ 3 per deletion

## Implementation Details

### Node Structure
```
struct RBNode<T> {
    value: T,
    color: Color (Red | Black),
    left: Option<Box<RBNode<T>>>,
    right: Option<Box<RBNode<T>>>,
    parent: Weak<RBNode<T>>,  // For easier rotations
}
```

### Insertion Algorithm
1. Standard BST insertion
2. Color new node RED
3. Fix violations bottom-up:
   - Case 1: Uncle is red → Recolor
   - Case 2: Triangle → Rotate to make line
   - Case 3: Line → Rotate and recolor

### Deletion Algorithm
1. Standard BST deletion
2. If deleted node was black, fix double-black:
   - Case 1: Sibling is red → Rotate and recolor
   - Case 2: Sibling is black with black children → Recolor
   - Case 3: Sibling is black with red child → Rotate and recolor

### Rotation Operations
- **Left Rotation**: Promote right child, demote parent
- **Right Rotation**: Promote left child, demote parent
- **Recoloring**: Swap colors to maintain invariants

## Test Cases

### Basic Operations
- Empty tree handling
- Single node insertion/deletion
- Small tree rotations (3-5 nodes)

### Rotation Scenarios
- **Simple Left/Right**: Single rotation cases
- **Double Rotation**: LR and RL cases
- **Recoloring Cascades**: Uncle-red propagation

### Deletion Complexity
- Delete red leaf (trivial)
- Delete black node with red child (simple)
- Double-black resolution (complex)
- Root deletion

### Stress Testing
- Sequential insertion (ascending/descending)
- Random insertion/deletion patterns
- Pathological alternating patterns
- 100K+ node trees

## Performance Targets

| Metric | Target |
|--------|---------|
| Insert rate | > 200K ops/sec |
| Search rate | > 500K ops/sec |
| Height ratio | h/log₂(n) < 1.5 |
| Rotation rate | < 30% of insertions |
| Memory overhead | < 20% vs unbalanced |

## Ruchy v1.5.0 Advanced Features

### Self-Hosting Rotation Generation
Use Ruchy's self-hosting compiler to generate optimized rotation code:
```rust
// Generate rotation logic at compile-time
let rotation_code = compiler.generate_rotation("left", node_type)?;
```

### Algorithm W Type Inference
Complex trait bounds automatically resolved:
```rust
trait BalancedTree<T> where T: Ord + Clone {
    type Node: TreeNode<T>;
    // Algorithm W infers all associated type constraints
}
```

### Pattern Matching Exhaustiveness
Ensure all color/rotation cases are handled:
```rust
match (node.color, uncle_color, parent_position) {
    (Red, Red, Left) => recolor_case(),
    (Red, Black, Left) if is_line() => rotate_case(),
    // Compiler ensures exhaustiveness
}
```

### Concurrent Operations
Safe concurrent tree access with formal verification:
```rust
async fn concurrent_insert<T>(&self, value: T) -> Result<(), TreeError> {
    // Lock-free insertion with invariant preservation
}
```

## Comparison with Other Trees

### vs AVL Trees
- **Height**: RB ≤ 2*log(n), AVL ≤ 1.44*log(n)
- **Rotations**: RB requires fewer rotations (amortized)
- **Complexity**: RB simpler deletion algorithm
- **Use Case**: RB better for frequent insertions/deletions

### vs B-Trees
- **Structure**: RB binary, B-tree multi-way
- **Cache**: B-tree better cache locality
- **Use Case**: RB for in-memory, B-tree for disk

### vs Splay Trees
- **Balance**: RB guaranteed, Splay amortized
- **Access Pattern**: Splay better for skewed access
- **Predictability**: RB more predictable performance

## Visualization

### ASCII Art Representation
```
        10(B)
       /      \
     5(R)     15(R)
    /   \     /    \
  3(B) 7(B) 12(B) 20(B)
```

### Properties Display
- Node color: (B)lack or (R)ed
- Black height: Number shown in brackets
- NIL nodes: Usually hidden for clarity
- Violations: Highlighted in output

## Formal Verification

### Invariant Checks
- After every operation
- Periodic full tree validation
- Property preservation proofs

### SMT Solver Integration
- Prove height bounds
- Verify rotation correctness
- Validate color invariants

### Coverage Requirements
- 95% code coverage
- All rotation cases tested
- All deletion scenarios covered