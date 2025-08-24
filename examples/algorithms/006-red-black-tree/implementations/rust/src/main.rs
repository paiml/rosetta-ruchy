// Red-Black Tree - Rust Baseline Implementation
// Self-balancing binary search tree with guaranteed O(log n) operations

use std::cmp::Ordering;
use std::fmt::{self, Debug, Display};
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Red,
    Black,
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Red => write!(f, "R"),
            Color::Black => write!(f, "B"),
        }
    }
}

#[derive(Debug)]
struct RBNode<T> {
    value: T,
    color: Color,
    left: Option<Box<RBNode<T>>>,
    right: Option<Box<RBNode<T>>>,
}

impl<T: Ord + Debug + Display> RBNode<T> {
    fn new(value: T) -> Self {
        RBNode {
            value,
            color: Color::Red, // New nodes are always red
            left: None,
            right: None,
        }
    }

    fn is_red(&self) -> bool {
        self.color == Color::Red
    }

    fn is_black(&self) -> bool {
        self.color == Color::Black
    }

    fn flip_color(&mut self) {
        self.color = match self.color {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        };
    }

    fn height(&self) -> usize {
        1 + std::cmp::max(
            self.left.as_ref().map_or(0, |n| n.height()),
            self.right.as_ref().map_or(0, |n| n.height()),
        )
    }

    fn black_height(&self) -> usize {
        let current = if self.is_black() { 1 } else { 0 };
        current + self.left.as_ref().map_or(0, |n| n.black_height())
    }

    fn verify_invariants(&self) -> Result<usize, String> {
        // Check no consecutive red nodes
        if self.is_red() {
            if let Some(ref left) = self.left {
                if left.is_red() {
                    return Err(format!("Red node {} has red left child", self.value));
                }
            }
            if let Some(ref right) = self.right {
                if right.is_red() {
                    return Err(format!("Red node {} has red right child", self.value));
                }
            }
        }

        // Recursively verify and get black heights
        let left_black_height = if let Some(ref left) = self.left {
            left.verify_invariants()?
        } else {
            0
        };

        let right_black_height = if let Some(ref right) = self.right {
            right.verify_invariants()?
        } else {
            0
        };

        // Check equal black heights
        if left_black_height != right_black_height {
            return Err(format!(
                "Node {} has unequal black heights: left={}, right={}",
                self.value, left_black_height, right_black_height
            ));
        }

        // Return black height including this node
        Ok(left_black_height + if self.is_black() { 1 } else { 0 })
    }
}

pub struct RedBlackTree<T> {
    root: Option<Box<RBNode<T>>>,
    size: usize,
    rotation_count: usize,
    recolor_count: usize,
}

impl<T: Ord + Debug + Clone + Display> RedBlackTree<T> {
    pub fn new() -> Self {
        RedBlackTree {
            root: None,
            size: 0,
            rotation_count: 0,
            recolor_count: 0,
        }
    }

    pub fn insert(&mut self, value: T) {
        let root = self.root.take();
        self.root = self.insert_recursive(root, value);

        // Root must always be black
        if let Some(ref mut root) = self.root {
            if root.is_red() {
                root.color = Color::Black;
                self.recolor_count += 1;
            }
        }

        self.size += 1;
    }

    fn insert_recursive(
        &mut self,
        node: Option<Box<RBNode<T>>>,
        value: T,
    ) -> Option<Box<RBNode<T>>> {
        let mut node = match node {
            None => return Some(Box::new(RBNode::new(value))),
            Some(node) => node,
        };

        match value.cmp(&node.value) {
            Ordering::Less => {
                node.left = self.insert_recursive(node.left.take(), value);
            }
            Ordering::Greater => {
                node.right = self.insert_recursive(node.right.take(), value);
            }
            Ordering::Equal => {
                // Update value (or ignore duplicates)
                node.value = value;
                return Some(node);
            }
        }

        // Fix any violations
        Some(self.fix_up(node))
    }

    fn fix_up(&mut self, mut node: Box<RBNode<T>>) -> Box<RBNode<T>> {
        // Check for red-red violations and fix them

        // Case 1: Right child is red and left is not
        if self.is_red_node(&node.right) && !self.is_red_node(&node.left) {
            node = self.rotate_left(node);
        }

        // Case 2: Left child and left-left grandchild are both red
        if self.is_red_node(&node.left) {
            if let Some(ref left) = node.left {
                if self.is_red_node(&left.left) {
                    node = self.rotate_right(node);
                }
            }
        }

        // Case 3: Both children are red
        if self.is_red_node(&node.left) && self.is_red_node(&node.right) {
            self.flip_colors(&mut node);
        }

        node
    }

    fn is_red_node(&self, node: &Option<Box<RBNode<T>>>) -> bool {
        node.as_ref().map_or(false, |n| n.is_red())
    }

    fn rotate_left(&mut self, mut node: Box<RBNode<T>>) -> Box<RBNode<T>> {
        let mut right = node.right.take().expect("rotate_left requires right child");
        node.right = right.left.take();
        right.left = Some(node);

        // Swap colors
        let right_color = right.color;
        right.color = right.left.as_ref().unwrap().color;
        right.left.as_mut().unwrap().color = right_color;

        self.rotation_count += 1;
        right
    }

    fn rotate_right(&mut self, mut node: Box<RBNode<T>>) -> Box<RBNode<T>> {
        let mut left = node.left.take().expect("rotate_right requires left child");
        node.left = left.right.take();
        left.right = Some(node);

        // Swap colors
        let left_color = left.color;
        left.color = left.right.as_ref().unwrap().color;
        left.right.as_mut().unwrap().color = left_color;

        self.rotation_count += 1;
        left
    }

    fn flip_colors(&mut self, node: &mut Box<RBNode<T>>) {
        node.flip_color();
        if let Some(ref mut left) = node.left {
            left.flip_color();
        }
        if let Some(ref mut right) = node.right {
            right.flip_color();
        }
        self.recolor_count += 3;
    }

    pub fn search(&self, value: &T) -> bool {
        self.search_node(&self.root, value)
    }

    fn search_node(&self, node: &Option<Box<RBNode<T>>>, value: &T) -> bool {
        match node {
            None => false,
            Some(n) => match value.cmp(&n.value) {
                Ordering::Equal => true,
                Ordering::Less => self.search_node(&n.left, value),
                Ordering::Greater => self.search_node(&n.right, value),
            },
        }
    }

    pub fn min(&self) -> Option<&T> {
        self.root.as_ref().map(|root| {
            let mut current = root;
            while let Some(ref left) = current.left {
                current = left;
            }
            &current.value
        })
    }

    pub fn max(&self) -> Option<&T> {
        self.root.as_ref().map(|root| {
            let mut current = root;
            while let Some(ref right) = current.right {
                current = right;
            }
            &current.value
        })
    }

    pub fn height(&self) -> usize {
        self.root.as_ref().map_or(0, |n| n.height())
    }

    pub fn black_height(&self) -> usize {
        self.root.as_ref().map_or(0, |n| n.black_height())
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn verify_invariants(&self) -> Result<(), String> {
        // Root must be black
        if let Some(ref root) = self.root {
            if root.is_red() {
                return Err("Root is not black".to_string());
            }
            root.verify_invariants()?;
        }
        Ok(())
    }

    pub fn inorder_traversal(&self) -> Vec<T> {
        let mut result = Vec::new();
        self.inorder_recursive(&self.root, &mut result);
        result
    }

    fn inorder_recursive(&self, node: &Option<Box<RBNode<T>>>, result: &mut Vec<T>) {
        if let Some(ref n) = node {
            self.inorder_recursive(&n.left, result);
            result.push(n.value.clone());
            self.inorder_recursive(&n.right, result);
        }
    }

    pub fn display_tree(&self) {
        println!("Red-Black Tree (size: {}):", self.size);
        if let Some(ref root) = self.root {
            self.display_node(root, "", "", true);
        } else {
            println!("  (empty)");
        }
    }

    fn display_node(&self, node: &Box<RBNode<T>>, prefix: &str, child_prefix: &str, is_last: bool) {
        let connector = if is_last { "└── " } else { "├── " };
        println!("{}{}{:?}({})", prefix, connector, node.value, node.color);

        let new_prefix = format!("{}{}", child_prefix, if is_last { "    " } else { "│   " });

        let children: Vec<_> = vec![&node.left, &node.right]
            .into_iter()
            .filter_map(|child| child.as_ref())
            .collect();

        for (i, child) in children.iter().enumerate() {
            let is_last_child = i == children.len() - 1;
            self.display_node(
                child,
                &format!("{}", child_prefix),
                &new_prefix,
                is_last_child,
            );
        }
    }

    pub fn statistics(&self) -> TreeStatistics {
        TreeStatistics {
            size: self.size,
            height: self.height(),
            black_height: self.black_height(),
            rotation_count: self.rotation_count,
            recolor_count: self.recolor_count,
            theoretical_max_height: if self.size > 0 {
                2.0 * (self.size as f64 + 1.0).log2()
            } else {
                0.0
            },
        }
    }
}

#[derive(Debug)]
pub struct TreeStatistics {
    pub size: usize,
    pub height: usize,
    pub black_height: usize,
    pub rotation_count: usize,
    pub recolor_count: usize,
    pub theoretical_max_height: f64,
}

impl Display for TreeStatistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "📊 Red-Black Tree Statistics:")?;
        writeln!(f, "   Size: {}", self.size)?;
        writeln!(f, "   Height: {}", self.height)?;
        writeln!(f, "   Black height: {}", self.black_height)?;
        writeln!(
            f,
            "   Theoretical max height: {:.1}",
            self.theoretical_max_height
        )?;
        writeln!(
            f,
            "   Height efficiency: {:.1}%",
            if self.theoretical_max_height > 0.0 {
                self.height as f64 / self.theoretical_max_height * 100.0
            } else {
                0.0
            }
        )?;
        writeln!(f, "   Total rotations: {}", self.rotation_count)?;
        writeln!(f, "   Total recolors: {}", self.recolor_count)?;
        writeln!(
            f,
            "   Operations per insert: {:.2}",
            if self.size > 0 {
                (self.rotation_count + self.recolor_count) as f64 / self.size as f64
            } else {
                0.0
            }
        )
    }
}

fn benchmark_operations(size: usize) {
    println!("\n⚡ Benchmarking with {} elements:", size);

    let mut tree = RedBlackTree::new();
    let data: Vec<i32> = (0..size as i32).map(|i| i * 7 % (size as i32)).collect();

    // Benchmark insertions
    let start = Instant::now();
    for &value in &data {
        tree.insert(value);
    }
    let insert_time = start.elapsed();

    // Verify invariants
    tree.verify_invariants().expect("Invariants violated!");

    // Benchmark searches
    let start = Instant::now();
    let mut found = 0;
    for &value in &data {
        if tree.search(&value) {
            found += 1;
        }
    }
    let search_time = start.elapsed();

    println!(
        "   Insert time: {:?} ({:.0} ops/sec)",
        insert_time,
        size as f64 / insert_time.as_secs_f64()
    );
    println!(
        "   Search time: {:?} ({:.0} ops/sec)",
        search_time,
        size as f64 / search_time.as_secs_f64()
    );
    println!("   Found: {}/{}", found, size);

    let stats = tree.statistics();
    println!("{}", stats);
}

fn demonstrate_rotations() {
    println!("\n🔄 Demonstrating Rotations and Recoloring:");

    let mut tree = RedBlackTree::new();

    // Create a pattern that will cause rotations
    let values = vec![7, 3, 11, 1, 5, 9, 13, 0, 2, 4, 6, 8, 10, 12, 14];

    for value in values {
        println!("\nInserting {}:", value);
        tree.insert(value);
        tree.display_tree();

        if let Err(e) = tree.verify_invariants() {
            panic!("Invariant violation after inserting {}: {}", value, e);
        }
    }

    let stats = tree.statistics();
    println!("\n{}", stats);
}

fn stress_test_sequential() {
    println!("\n🔥 Stress Test - Sequential Insertion:");

    let mut tree = RedBlackTree::new();
    let size = 1000;

    // Worst case: sequential insertion
    let start = Instant::now();
    for i in 0..size {
        tree.insert(i);
    }
    let time = start.elapsed();

    tree.verify_invariants().expect("Invariants violated!");

    println!("   Sequential insertion of {} elements: {:?}", size, time);
    println!(
        "   Height: {} (theoretical max: {:.1})",
        tree.height(),
        2.0 * (size as f64 + 1.0).log2()
    );
    println!(
        "   Rotations: {}, Recolors: {}",
        tree.rotation_count, tree.recolor_count
    );
}

fn main() {
    println!("🌳 Red-Black Tree - Rust Baseline Implementation");
    println!("================================================");

    // Basic demonstration
    demonstrate_rotations();

    // Stress test with sequential insertion
    stress_test_sequential();

    // Benchmark different sizes
    for size in [100, 1000, 10000, 100000] {
        benchmark_operations(size);
    }

    // Test worst-case patterns
    println!("\n🔍 Testing Pathological Patterns:");

    let mut ascending_tree = RedBlackTree::new();
    let mut descending_tree = RedBlackTree::new();

    for i in 0..100 {
        ascending_tree.insert(i);
        descending_tree.insert(100 - i);
    }

    println!(
        "Ascending pattern - Height: {}, Black height: {}",
        ascending_tree.height(),
        ascending_tree.black_height()
    );
    println!(
        "Descending pattern - Height: {}, Black height: {}",
        descending_tree.height(),
        descending_tree.black_height()
    );

    ascending_tree
        .verify_invariants()
        .expect("Ascending tree invariants violated!");
    descending_tree
        .verify_invariants()
        .expect("Descending tree invariants violated!");

    println!("\n✅ All Red-Black tree invariants maintained!");
    println!("🎯 Baseline performance established for comparison");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let tree: RedBlackTree<i32> = RedBlackTree::new();
        assert_eq!(tree.size(), 0);
        assert_eq!(tree.height(), 0);
        assert!(tree.is_empty());
        assert!(tree.verify_invariants().is_ok());
    }

    #[test]
    fn test_single_element() {
        let mut tree = RedBlackTree::new();
        tree.insert(42);

        assert_eq!(tree.size(), 1);
        assert!(tree.search(&42));
        assert!(!tree.search(&41));
        assert_eq!(tree.min(), Some(&42));
        assert_eq!(tree.max(), Some(&42));
        assert!(tree.verify_invariants().is_ok());
    }

    #[test]
    fn test_sequential_insertion() {
        let mut tree = RedBlackTree::new();

        for i in 0..100 {
            tree.insert(i);
            assert!(
                tree.verify_invariants().is_ok(),
                "Invariants violated after inserting {}",
                i
            );
        }

        assert_eq!(tree.size(), 100);
        assert!(tree.height() <= (2.0 * 101_f64.log2()) as usize);

        for i in 0..100 {
            assert!(tree.search(&i), "Value {} not found", i);
        }
    }

    #[test]
    fn test_random_operations() {
        use std::collections::HashSet;

        let mut tree = RedBlackTree::new();
        let mut expected = HashSet::new();

        // Random-ish pattern
        let values = vec![50, 25, 75, 12, 37, 62, 87, 6, 18, 31, 43, 56, 68, 81, 93];

        for value in values {
            tree.insert(value);
            expected.insert(value);
            assert!(tree.verify_invariants().is_ok());
        }

        for &value in &expected {
            assert!(tree.search(&value));
        }

        let inorder = tree.inorder_traversal();
        let mut sorted = expected.into_iter().collect::<Vec<_>>();
        sorted.sort();

        assert_eq!(inorder, sorted);
    }
}
