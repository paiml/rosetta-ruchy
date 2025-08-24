# Binary Search Tree (BST) Operations

## Problem Statement

A Binary Search Tree is a hierarchical data structure where each node has at most two children, and for every node:
- All values in the left subtree are less than the node's value
- All values in the right subtree are greater than the node's value
- Both subtrees are also binary search trees

This implementation provides core BST operations with comprehensive analysis.

## Complexity Analysis

### Basic Operations
- **Search**: O(log n) average, O(n) worst case
- **Insert**: O(log n) average, O(n) worst case  
- **Delete**: O(log n) average, O(n) worst case
- **Space**: O(n) for tree storage

### Traversal Operations
- **Inorder**: O(n) time, O(h) space (where h = height)
- **Preorder**: O(n) time, O(h) space
- **Postorder**: O(n) time, O(h) space
- **Level-order**: O(n) time, O(w) space (where w = width)

### Tree Properties
- **Height**: O(log n) balanced, O(n) degenerate
- **Minimum/Maximum**: O(log n) average, O(n) worst case

## Operations Implemented

### 1. Insertion
- Recursive and iterative approaches
- Maintains BST property
- Handles duplicates (reject or allow)

### 2. Search
- Efficient logarithmic search
- Returns node or indicates absence
- Path tracking for analysis

### 3. Deletion
- Three cases:
  - Leaf node: Simple removal
  - One child: Replace with child
  - Two children: Replace with inorder successor/predecessor

### 4. Traversals
- **Inorder**: Left → Root → Right (sorted order)
- **Preorder**: Root → Left → Right (prefix notation)
- **Postorder**: Left → Right → Root (postfix notation)
- **Level-order**: Breadth-first traversal

### 5. Tree Analysis
- Height calculation
- Node counting
- Balance factor analysis
- Path lengths

## Test Cases

1. **Empty Tree Operations**
   - Search, insert, delete on empty tree
   - Edge case handling

2. **Single Node Tree**
   - All operations on trivial tree
   - Root-only scenarios

3. **Balanced Tree Construction**
   - Optimal insertion order
   - Logarithmic height verification

4. **Degenerate Tree (Linked List)**
   - Worst-case insertion order
   - Linear height verification

5. **Random Operations**
   - Mixed insert/delete/search
   - Tree property maintenance

6. **Large Tree Performance**
   - Scalability testing
   - Performance degradation analysis

## Advanced Features

### Tree Validation
- BST property verification
- Structure integrity checks
- Recursive validation

### Performance Metrics
- Operation counts
- Tree height tracking
- Balance analysis

### Visualization
- ASCII tree representation
- Structure analysis
- Path visualization

## Applications

- **Database Indexing**: Efficient data retrieval
- **Expression Parsing**: Syntax tree construction  
- **File Systems**: Directory hierarchies
- **Priority Queues**: Ordered data access
- **Symbol Tables**: Variable storage in compilers
- **Range Queries**: Efficient range searches