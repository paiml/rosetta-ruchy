// Binary Search Tree - Rust Implementation
// Comprehensive BST operations with performance analysis

use std::collections::VecDeque;
use std::time::Instant;

// BST Node structure
#[derive(Debug, Clone)]
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(value: i32) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    fn new_boxed(value: i32) -> Box<Self> {
        Box::new(Self::new(value))
    }
}

// Binary Search Tree implementation
#[derive(Debug, Clone)]
struct BinarySearchTree {
    root: Option<Box<TreeNode>>,
    size: usize,
    allow_duplicates: bool,
}

impl BinarySearchTree {
    fn new() -> Self {
        Self {
            root: None,
            size: 0,
            allow_duplicates: false,
        }
    }

    fn new_with_duplicates(allow_duplicates: bool) -> Self {
        Self {
            root: None,
            size: 0,
            allow_duplicates,
        }
    }

    // Insertion operations
    fn insert(&mut self, value: i32) -> bool {
        let inserted = Self::insert_recursive(&mut self.root, value, self.allow_duplicates);
        if inserted {
            self.size += 1;
        }
        inserted
    }

    fn insert_recursive(node: &mut Option<Box<TreeNode>>, value: i32, allow_duplicates: bool) -> bool {
        match node {
            None => {
                *node = Some(TreeNode::new_boxed(value));
                true
            }
            Some(current) => {
                if value < current.value {
                    Self::insert_recursive(&mut current.left, value, allow_duplicates)
                } else if value > current.value {
                    Self::insert_recursive(&mut current.right, value, allow_duplicates)
                } else {
                    // Value already exists
                    if allow_duplicates {
                        Self::insert_recursive(&mut current.right, value, allow_duplicates)
                    } else {
                        false // Reject duplicate
                    }
                }
            }
        }
    }

    fn insert_iterative(&mut self, value: i32) -> bool {
        if self.root.is_none() {
            self.root = Some(TreeNode::new_boxed(value));
            self.size += 1;
            return true;
        }

        let mut current = &mut self.root;
        loop {
            match current {
                Some(node) => {
                    if value < node.value {
                        if node.left.is_none() {
                            node.left = Some(TreeNode::new_boxed(value));
                            self.size += 1;
                            return true;
                        }
                        current = &mut node.left;
                    } else if value > node.value {
                        if node.right.is_none() {
                            node.right = Some(TreeNode::new_boxed(value));
                            self.size += 1;
                            return true;
                        }
                        current = &mut node.right;
                    } else {
                        // Duplicate found
                        if self.allow_duplicates {
                            if node.right.is_none() {
                                node.right = Some(TreeNode::new_boxed(value));
                                self.size += 1;
                                return true;
                            }
                            current = &mut node.right;
                        } else {
                            return false;
                        }
                    }
                }
                None => unreachable!(),
            }
        }
    }

    // Search operations
    fn search(&self, value: i32) -> bool {
        self.search_recursive(&self.root, value)
    }

    fn search_recursive(&self, node: &Option<Box<TreeNode>>, value: i32) -> bool {
        match node {
            None => false,
            Some(current) => {
                if value == current.value {
                    true
                } else if value < current.value {
                    self.search_recursive(&current.left, value)
                } else {
                    self.search_recursive(&current.right, value)
                }
            }
        }
    }

    fn search_iterative(&self, value: i32) -> bool {
        let mut current = &self.root;
        while let Some(node) = current {
            if value == node.value {
                return true;
            } else if value < node.value {
                current = &node.left;
            } else {
                current = &node.right;
            }
        }
        false
    }

    // Deletion operation
    fn delete(&mut self, value: i32) -> bool {
        let deleted = Self::delete_recursive(&mut self.root, value);
        if deleted {
            self.size -= 1;
        }
        deleted
    }

    fn delete_recursive(node: &mut Option<Box<TreeNode>>, value: i32) -> bool {
        match node {
            None => false,
            Some(current) => {
                if value < current.value {
                    Self::delete_recursive(&mut current.left, value)
                } else if value > current.value {
                    Self::delete_recursive(&mut current.right, value)
                } else {
                    // Found node to delete
                    match (&mut current.left, &mut current.right) {
                        (None, None) => {
                            // Leaf node
                            *node = None;
                        }
                        (Some(_), None) => {
                            // Only left child
                            *node = current.left.take();
                        }
                        (None, Some(_)) => {
                            // Only right child
                            *node = current.right.take();
                        }
                        (Some(_), Some(_)) => {
                            // Two children - replace with inorder successor
                            let successor_value = Self::find_min_value(&current.right);
                            current.value = successor_value;
                            Self::delete_recursive(&mut current.right, successor_value);
                            return true; // Already decremented size in recursive call
                        }
                    }
                    true
                }
            }
        }
    }

    fn find_min_value(node: &Option<Box<TreeNode>>) -> i32 {
        match node {
            Some(current) => {
                if current.left.is_none() {
                    current.value
                } else {
                    Self::find_min_value(&current.left)
                }
            }
            None => panic!("Called find_min_value on empty subtree"),
        }
    }

    // Tree property operations
    fn find_min(&self) -> Option<i32> {
        self.find_min_recursive(&self.root)
    }

    fn find_min_recursive(&self, node: &Option<Box<TreeNode>>) -> Option<i32> {
        match node {
            None => None,
            Some(current) => {
                if current.left.is_none() {
                    Some(current.value)
                } else {
                    self.find_min_recursive(&current.left)
                }
            }
        }
    }

    fn find_max(&self) -> Option<i32> {
        self.find_max_recursive(&self.root)
    }

    fn find_max_recursive(&self, node: &Option<Box<TreeNode>>) -> Option<i32> {
        match node {
            None => None,
            Some(current) => {
                if current.right.is_none() {
                    Some(current.value)
                } else {
                    self.find_max_recursive(&current.right)
                }
            }
        }
    }

    fn height(&self) -> i32 {
        self.height_recursive(&self.root)
    }

    fn height_recursive(&self, node: &Option<Box<TreeNode>>) -> i32 {
        match node {
            None => -1,
            Some(current) => {
                let left_height = self.height_recursive(&current.left);
                let right_height = self.height_recursive(&current.right);
                1 + left_height.max(right_height)
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    fn len(&self) -> usize {
        self.size
    }

    // Traversal operations
    fn inorder(&self) -> Vec<i32> {
        let mut result = Vec::new();
        self.inorder_recursive(&self.root, &mut result);
        result
    }

    fn inorder_recursive(&self, node: &Option<Box<TreeNode>>, result: &mut Vec<i32>) {
        if let Some(current) = node {
            self.inorder_recursive(&current.left, result);
            result.push(current.value);
            self.inorder_recursive(&current.right, result);
        }
    }

    fn preorder(&self) -> Vec<i32> {
        let mut result = Vec::new();
        self.preorder_recursive(&self.root, &mut result);
        result
    }

    fn preorder_recursive(&self, node: &Option<Box<TreeNode>>, result: &mut Vec<i32>) {
        if let Some(current) = node {
            result.push(current.value);
            self.preorder_recursive(&current.left, result);
            self.preorder_recursive(&current.right, result);
        }
    }

    fn postorder(&self) -> Vec<i32> {
        let mut result = Vec::new();
        self.postorder_recursive(&self.root, &mut result);
        result
    }

    fn postorder_recursive(&self, node: &Option<Box<TreeNode>>, result: &mut Vec<i32>) {
        if let Some(current) = node {
            self.postorder_recursive(&current.left, result);
            self.postorder_recursive(&current.right, result);
            result.push(current.value);
        }
    }

    fn level_order(&self) -> Vec<i32> {
        let mut result = Vec::new();
        if self.root.is_none() {
            return result;
        }

        let mut queue = VecDeque::new();
        queue.push_back(self.root.as_ref().unwrap());

        while let Some(node) = queue.pop_front() {
            result.push(node.value);

            if let Some(left) = &node.left {
                queue.push_back(left);
            }
            if let Some(right) = &node.right {
                queue.push_back(right);
            }
        }

        result
    }

    // Validation
    fn is_valid_bst(&self) -> bool {
        self.is_valid_bst_recursive(&self.root, i32::MIN, i32::MAX)
    }

    fn is_valid_bst_recursive(&self, node: &Option<Box<TreeNode>>, min: i32, max: i32) -> bool {
        match node {
            None => true,
            Some(current) => {
                if current.value <= min || current.value >= max {
                    false
                } else {
                    self.is_valid_bst_recursive(&current.left, min, current.value)
                        && self.is_valid_bst_recursive(&current.right, current.value, max)
                }
            }
        }
    }

    // Visualization
    fn display_tree(&self) -> String {
        if self.root.is_none() {
            return "Empty tree".to_string();
        }
        let mut lines = Vec::new();
        self.display_recursive(&self.root, "", true, &mut lines);
        lines.join("\n")
    }

    fn display_recursive(
        &self,
        node: &Option<Box<TreeNode>>,
        prefix: &str,
        is_last: bool,
        lines: &mut Vec<String>,
    ) {
        if let Some(current) = node {
            let connector = if is_last { "└── " } else { "├── " };
            lines.push(format!("{}{}{}", prefix, connector, current.value));

            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });

            let has_left = current.left.is_some();
            let has_right = current.right.is_some();

            if has_left || has_right {
                if has_left {
                    self.display_recursive(&current.left, &new_prefix, !has_right, lines);
                }
                if has_right {
                    self.display_recursive(&current.right, &new_prefix, true, lines);
                }
            }
        }
    }
}

// Performance testing structure
#[derive(Debug)]
struct PerformanceMetrics {
    operation: String,
    time_ns: u64,
    tree_size: usize,
    tree_height: i32,
    success: bool,
}

impl PerformanceMetrics {
    fn new(operation: String, time_ns: u64, bst: &BinarySearchTree, success: bool) -> Self {
        Self {
            operation,
            time_ns,
            tree_size: bst.len(),
            tree_height: bst.height(),
            success,
        }
    }
}

// Test suite functions
fn test_basic_operations() {
    println!("Test Case: Basic Operations");
    println!("{}", "=".repeat(60));

    let mut bst = BinarySearchTree::new();

    // Test empty tree
    assert!(bst.is_empty());
    assert_eq!(bst.len(), 0);
    assert_eq!(bst.height(), -1);
    assert!(!bst.search(42));
    assert_eq!(bst.find_min(), None);
    assert_eq!(bst.find_max(), None);

    // Test single element
    assert!(bst.insert(50));
    assert!(!bst.is_empty());
    assert_eq!(bst.len(), 1);
    assert_eq!(bst.height(), 0);
    assert!(bst.search(50));
    assert_eq!(bst.find_min(), Some(50));
    assert_eq!(bst.find_max(), Some(50));

    // Test multiple insertions
    let values = vec![30, 70, 20, 40, 60, 80];
    for value in values {
        assert!(bst.insert(value));
    }

    assert_eq!(bst.len(), 7);
    assert_eq!(bst.height(), 2);
    assert_eq!(bst.find_min(), Some(20));
    assert_eq!(bst.find_max(), Some(80));

    // Test duplicate insertion (should fail)
    assert!(!bst.insert(50));
    assert_eq!(bst.len(), 7);

    println!("✅ Basic operations test passed");
    println!("Tree structure:\n{}", bst.display_tree());
    println!("Size: {}, Height: {}", bst.len(), bst.height());
}

fn test_traversals() {
    println!("\nTest Case: Tree Traversals");
    println!("{}", "=".repeat(60));

    let mut bst = BinarySearchTree::new();
    let values = vec![50, 30, 70, 20, 40, 60, 80, 10, 25, 35, 45];

    for value in values {
        bst.insert(value);
    }

    println!("Tree structure:\n{}", bst.display_tree());

    let inorder = bst.inorder();
    let preorder = bst.preorder();
    let postorder = bst.postorder();
    let level_order = bst.level_order();

    println!("Inorder:     {:?}", inorder);
    println!("Preorder:    {:?}", preorder);
    println!("Postorder:   {:?}", postorder);
    println!("Level-order: {:?}", level_order);

    // Verify inorder gives sorted sequence
    let mut sorted_inorder = inorder.clone();
    sorted_inorder.sort();
    assert_eq!(inorder, sorted_inorder);

    println!("✅ Traversal test passed (inorder gives sorted sequence)");
}

fn test_deletion() {
    println!("\nTest Case: Deletion Operations");
    println!("{}", "=".repeat(60));

    let mut bst = BinarySearchTree::new();
    let values = vec![50, 30, 70, 20, 40, 60, 80, 10, 25, 35, 45];

    for value in values {
        bst.insert(value);
    }

    println!("Original tree:\n{}", bst.display_tree());
    println!("Original inorder: {:?}", bst.inorder());

    // Delete leaf node
    assert!(bst.delete(10));
    println!("\nAfter deleting 10 (leaf):\n{}", bst.display_tree());
    println!("Inorder: {:?}", bst.inorder());

    // Delete node with one child
    assert!(bst.delete(25));
    println!("\nAfter deleting 25 (one child):\n{}", bst.display_tree());
    println!("Inorder: {:?}", bst.inorder());

    // Delete node with two children
    assert!(bst.delete(30));
    println!("\nAfter deleting 30 (two children):\n{}", bst.display_tree());
    println!("Inorder: {:?}", bst.inorder());

    // Try to delete non-existent node
    assert!(!bst.delete(100));

    assert!(bst.is_valid_bst());
    println!("✅ Deletion test passed (BST property maintained)");
}

fn test_performance() {
    println!("\nTest Case: Performance Analysis");
    println!("{}", "=".repeat(60));

    let mut metrics = Vec::new();

    // Test balanced tree performance
    let mut balanced_bst = BinarySearchTree::new();
    let balanced_values = vec![50, 25, 75, 12, 37, 62, 87, 6, 18, 31, 43, 56, 68, 81, 93];

    for &value in &balanced_values {
        let start = Instant::now();
        let success = balanced_bst.insert(value);
        let elapsed = start.elapsed().as_nanos() as u64;
        metrics.push(PerformanceMetrics::new(
            format!("Insert {} (balanced)", value),
            elapsed,
            &balanced_bst,
            success,
        ));
    }

    // Test degenerate tree performance (worst case)
    let mut degenerate_bst = BinarySearchTree::new();
    for i in 1..=15 {
        let start = Instant::now();
        let success = degenerate_bst.insert(i);
        let elapsed = start.elapsed().as_nanos() as u64;
        if i % 5 == 0 {
            // Sample every 5th insertion
            metrics.push(PerformanceMetrics::new(
                format!("Insert {} (degenerate)", i),
                elapsed,
                &degenerate_bst,
                success,
            ));
        }
    }

    // Test search performance
    for &value in &[25, 50, 75, 100] {
        let start = Instant::now();
        let found = balanced_bst.search(value);
        let elapsed = start.elapsed().as_nanos() as u64;
        metrics.push(PerformanceMetrics::new(
            format!("Search {} (balanced)", value),
            elapsed,
            &balanced_bst,
            found,
        ));
    }

    // Display results
    println!("{:<25} | {:>8} | {:>6} | {:>6} | {:>8}",
             "Operation", "Time(ns)", "Size", "Height", "Success");
    println!("{}", "-".repeat(70));

    for metric in &metrics {
        println!("{:<25} | {:>8} | {:>6} | {:>6} | {:>8}",
                 metric.operation,
                 metric.time_ns,
                 metric.tree_size,
                 metric.tree_height,
                 metric.success);
    }

    println!("\nTree Comparison:");
    println!("Balanced tree height: {} (nodes: {})", balanced_bst.height(), balanced_bst.len());
    println!("Degenerate tree height: {} (nodes: {})", degenerate_bst.height(), degenerate_bst.len());
    println!("Height difference demonstrates O(log n) vs O(n) behavior");
}

fn test_edge_cases() {
    println!("\nTest Case: Edge Cases");
    println!("{}", "=".repeat(60));

    // Test with duplicates allowed
    let mut bst_with_dups = BinarySearchTree::new_with_duplicates(true);
    assert!(bst_with_dups.insert(50));
    assert!(bst_with_dups.insert(50)); // Should succeed
    assert_eq!(bst_with_dups.len(), 2);

    // Test large tree
    let mut large_bst = BinarySearchTree::new();
    let mut values: Vec<i32> = (1..=1000).collect();
    
    // Shuffle for better balance (simple shuffle)
    for i in 0..values.len() {
        let j = (i + 7) % values.len();
        values.swap(i, j);
    }

    for value in values {
        large_bst.insert(value);
    }

    assert_eq!(large_bst.len(), 1000);
    assert!(large_bst.search(1));
    assert!(large_bst.search(500));
    assert!(large_bst.search(1000));
    assert!(!large_bst.search(1001));
    assert!(large_bst.is_valid_bst());

    println!("Large tree: {} nodes, height: {}", large_bst.len(), large_bst.height());
    println!("✅ Edge cases test passed");
}

fn main() {
    println!("Binary Search Tree - Comprehensive Implementation");
    println!("{}", "=".repeat(70));

    test_basic_operations();
    test_traversals();
    test_deletion();
    test_performance();
    test_edge_cases();

    println!("\n\nAlgorithm Summary:");
    println!("{}", "=".repeat(70));
    println!("Search:     O(log n) avg, O(n) worst | Binary search property");
    println!("Insert:     O(log n) avg, O(n) worst | Maintains BST invariant");
    println!("Delete:     O(log n) avg, O(n) worst | Three cases handled");
    println!("Traversal:  O(n) time, O(h) space   | All standard orders");
    println!("Space:      O(n) total storage       | One node per element");

    println!("\nKey Features:");
    println!("- Recursive and iterative implementations");
    println!("- Comprehensive deletion with successor replacement");
    println!("- All standard traversal algorithms");
    println!("- BST property validation");
    println!("- Performance analysis and visualization");
    println!("- Edge case handling and large tree testing");

    println!("\nApplications:");
    println!("- Database indexing and sorted data storage");
    println!("- Expression parsing and syntax trees");
    println!("- Priority queues and ordered collections");
    println!("- File system hierarchies and decision trees");
}