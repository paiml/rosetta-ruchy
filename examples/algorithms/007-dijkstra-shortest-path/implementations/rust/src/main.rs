// Dijkstra's Shortest Path - Rust Baseline Implementation
// Single-source shortest paths with priority queue optimization

use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Ordering;
use std::fmt::{self, Display, Debug};
use std::time::Instant;
use std::f64;

// Node representation for the graph
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    id: String,
}

impl Node {
    fn new(id: impl Into<String>) -> Self {
        Node { id: id.into() }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

// Edge with weight
#[derive(Debug, Clone)]
struct Edge {
    from: Node,
    to: Node,
    weight: f64,
}

impl Edge {
    fn new(from: Node, to: Node, weight: f64) -> Self {
        if weight < 0.0 {
            panic!("Negative edge weights not supported by Dijkstra's algorithm");
        }
        Edge { from, to, weight }
    }
}

// Graph representation using adjacency list
#[derive(Debug, Clone)]
struct Graph {
    nodes: HashSet<Node>,
    adjacency_list: HashMap<Node, Vec<(Node, f64)>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: HashSet::new(),
            adjacency_list: HashMap::new(),
        }
    }
    
    fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.clone());
        self.adjacency_list.entry(node).or_insert_with(Vec::new);
    }
    
    fn add_edge(&mut self, from: Node, to: Node, weight: f64) {
        if weight < 0.0 {
            panic!("Negative edge weights not supported");
        }
        
        self.add_node(from.clone());
        self.add_node(to.clone());
        
        self.adjacency_list
            .entry(from)
            .or_insert_with(Vec::new)
            .push((to, weight));
    }
    
    fn add_undirected_edge(&mut self, a: Node, b: Node, weight: f64) {
        self.add_edge(a.clone(), b.clone(), weight);
        self.add_edge(b, a, weight);
    }
    
    fn neighbors(&self, node: &Node) -> Option<&Vec<(Node, f64)>> {
        self.adjacency_list.get(node)
    }
    
    fn node_count(&self) -> usize {
        self.nodes.len()
    }
    
    fn edge_count(&self) -> usize {
        self.adjacency_list
            .values()
            .map(|neighbors| neighbors.len())
            .sum()
    }
    
    fn display(&self) {
        println!("Graph with {} nodes and {} edges:", 
                 self.node_count(), self.edge_count());
        
        for node in &self.nodes {
            if let Some(neighbors) = self.neighbors(node) {
                if !neighbors.is_empty() {
                    print!("  {} -> ", node);
                    for (i, (neighbor, weight)) in neighbors.iter().enumerate() {
                        if i > 0 {
                            print!(", ");
                        }
                        print!("{}({})", neighbor, weight);
                    }
                    println!();
                }
            }
        }
    }
}

// State for priority queue in Dijkstra's algorithm
#[derive(Debug, Clone)]
struct State {
    node: Node,
    distance: f64,
}

impl State {
    fn new(node: Node, distance: f64) -> Self {
        State { node, distance }
    }
}

// Implement ordering for priority queue (min-heap based on distance)
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap
        other.distance.partial_cmp(&self.distance)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Result of Dijkstra's algorithm
#[derive(Debug, Clone)]
struct DijkstraResult {
    distances: HashMap<Node, f64>,
    predecessors: HashMap<Node, Node>,
    visited_count: usize,
}

impl DijkstraResult {
    fn get_distance(&self, node: &Node) -> Option<f64> {
        self.distances.get(node).copied()
    }
    
    fn get_path(&self, target: &Node) -> Option<Vec<Node>> {
        if !self.distances.contains_key(target) {
            return None;
        }
        
        if self.distances[target] == f64::INFINITY {
            return None;
        }
        
        let mut path = Vec::new();
        let mut current = target.clone();
        
        path.push(current.clone());
        
        while let Some(predecessor) = self.predecessors.get(&current) {
            current = predecessor.clone();
            path.push(current.clone());
        }
        
        path.reverse();
        Some(path)
    }
    
    fn display_paths(&self, source: &Node) {
        println!("\nShortest paths from {}:", source);
        
        let mut nodes: Vec<_> = self.distances.keys().collect();
        nodes.sort_by_key(|n| n.id.as_str());
        
        for node in nodes {
            let distance = self.distances[node];
            
            if distance == f64::INFINITY {
                println!("  {} → {}: ∞ (unreachable)", source, node);
            } else {
                if let Some(path) = self.get_path(node) {
                    let path_str = path.iter()
                        .map(|n| n.id.as_str())
                        .collect::<Vec<_>>()
                        .join(" → ");
                    println!("  {} → {}: {} [{}]", source, node, distance, path_str);
                }
            }
        }
    }
}

// Dijkstra's algorithm implementation
fn dijkstra(graph: &Graph, source: &Node) -> DijkstraResult {
    let mut distances: HashMap<Node, f64> = HashMap::new();
    let mut predecessors: HashMap<Node, Node> = HashMap::new();
    let mut visited: HashSet<Node> = HashSet::new();
    let mut heap = BinaryHeap::new();
    let mut visited_count = 0;
    
    // Initialize distances
    for node in &graph.nodes {
        distances.insert(node.clone(), f64::INFINITY);
    }
    distances.insert(source.clone(), 0.0);
    
    // Add source to heap
    heap.push(State::new(source.clone(), 0.0));
    
    while let Some(State { node, distance }) = heap.pop() {
        // Skip if already visited
        if visited.contains(&node) {
            continue;
        }
        
        visited.insert(node.clone());
        visited_count += 1;
        
        // Skip if we found a longer path
        if distance > distances[&node] {
            continue;
        }
        
        // Explore neighbors
        if let Some(neighbors) = graph.neighbors(&node) {
            for (neighbor, weight) in neighbors {
                let alt_distance = distance + weight;
                
                if alt_distance < distances[neighbor] {
                    distances.insert(neighbor.clone(), alt_distance);
                    predecessors.insert(neighbor.clone(), node.clone());
                    heap.push(State::new(neighbor.clone(), alt_distance));
                }
            }
        }
    }
    
    DijkstraResult {
        distances,
        predecessors,
        visited_count,
    }
}

// Concurrent multi-source Dijkstra
fn parallel_dijkstra(graph: &Graph, sources: Vec<Node>) -> Vec<(Node, DijkstraResult)> {
    use std::sync::Arc;
    use std::thread;
    
    let graph = Arc::new(graph.clone());
    let mut handles = vec![];
    
    for source in sources {
        let graph_clone = Arc::clone(&graph);
        let handle = thread::spawn(move || {
            let result = dijkstra(&graph_clone, &source);
            (source, result)
        });
        handles.push(handle);
    }
    
    handles.into_iter()
        .map(|h| h.join().unwrap())
        .collect()
}

// Generate different types of graphs for testing
fn generate_grid_graph(width: usize, height: usize) -> Graph {
    let mut graph = Graph::new();
    
    for y in 0..height {
        for x in 0..width {
            let node = Node::new(format!("({},{})", x, y));
            
            // Add edges to adjacent cells
            if x > 0 {
                let left = Node::new(format!("({},{})", x - 1, y));
                graph.add_edge(node.clone(), left, 1.0);
            }
            if x < width - 1 {
                let right = Node::new(format!("({},{})", x + 1, y));
                graph.add_edge(node.clone(), right, 1.0);
            }
            if y > 0 {
                let up = Node::new(format!("({},{})", x, y - 1));
                graph.add_edge(node.clone(), up, 1.0);
            }
            if y < height - 1 {
                let down = Node::new(format!("({},{})", x, y + 1));
                graph.add_edge(node.clone(), down, 1.0);
            }
        }
    }
    
    graph
}

fn generate_complete_graph(n: usize) -> Graph {
    let mut graph = Graph::new();
    let nodes: Vec<Node> = (0..n).map(|i| Node::new(format!("N{}", i))).collect();
    
    for i in 0..n {
        for j in i + 1..n {
            let weight = ((i + j) % 10 + 1) as f64; // Pseudo-random weight
            graph.add_undirected_edge(nodes[i].clone(), nodes[j].clone(), weight);
        }
    }
    
    graph
}

// Benchmark function
fn benchmark_dijkstra(graph: &Graph, source: &Node, name: &str) {
    let start = Instant::now();
    let result = dijkstra(graph, source);
    let duration = start.elapsed();
    
    println!("\n📊 Benchmark: {}", name);
    println!("   Graph size: {} nodes, {} edges", 
             graph.node_count(), graph.edge_count());
    println!("   Visited nodes: {}", result.visited_count);
    println!("   Time: {:?}", duration);
    println!("   Throughput: {:.0} nodes/sec", 
             result.visited_count as f64 / duration.as_secs_f64());
}

fn main() {
    println!("🗺️  Dijkstra's Shortest Path - Rust Baseline Implementation");
    println!("==========================================================");
    
    // Example 1: Simple graph from README
    println!("\n📍 Example 1: Simple Graph");
    let mut graph = Graph::new();
    
    // Build the example graph
    graph.add_edge(Node::new("A"), Node::new("B"), 4.0);
    graph.add_edge(Node::new("A"), Node::new("C"), 2.0);
    graph.add_edge(Node::new("B"), Node::new("D"), 5.0);
    graph.add_edge(Node::new("C"), Node::new("D"), 8.0);
    graph.add_edge(Node::new("C"), Node::new("E"), 10.0);
    graph.add_edge(Node::new("D"), Node::new("E"), 2.0);
    graph.add_edge(Node::new("D"), Node::new("F"), 6.0);
    graph.add_edge(Node::new("E"), Node::new("F"), 3.0);
    
    graph.display();
    
    // Run Dijkstra from source A
    let source = Node::new("A");
    let result = dijkstra(&graph, &source);
    result.display_paths(&source);
    
    // Show specific path
    let target = Node::new("F");
    if let Some(path) = result.get_path(&target) {
        println!("\n✅ Shortest path from {} to {}: {:?} (distance: {})",
                 source, target, path, result.get_distance(&target).unwrap());
    }
    
    // Example 2: Unreachable nodes
    println!("\n📍 Example 2: Graph with Unreachable Nodes");
    let mut disconnected = Graph::new();
    disconnected.add_edge(Node::new("A"), Node::new("B"), 1.0);
    disconnected.add_edge(Node::new("C"), Node::new("D"), 1.0);
    
    disconnected.display();
    
    let result = dijkstra(&disconnected, &Node::new("A"));
    result.display_paths(&Node::new("A"));
    
    // Example 3: Grid graph
    println!("\n📍 Example 3: Grid Graph (5x5)");
    let grid = generate_grid_graph(5, 5);
    
    let source = Node::new("(0,0)");
    let target = Node::new("(4,4)");
    
    let start = Instant::now();
    let result = dijkstra(&grid, &source);
    let duration = start.elapsed();
    
    if let Some(path) = result.get_path(&target) {
        println!("Path from {} to {}: {} nodes (distance: {})",
                 source, target, path.len(), result.get_distance(&target).unwrap());
        println!("Time: {:?}", duration);
    }
    
    // Benchmarks
    println!("\n⚡ Performance Benchmarks");
    
    // Small complete graph
    let complete_10 = generate_complete_graph(10);
    benchmark_dijkstra(&complete_10, &Node::new("N0"), "Complete Graph (10 nodes)");
    
    // Larger complete graph
    let complete_100 = generate_complete_graph(100);
    benchmark_dijkstra(&complete_100, &Node::new("N0"), "Complete Graph (100 nodes)");
    
    // Large grid graph
    let grid_100 = generate_grid_graph(100, 100);
    benchmark_dijkstra(&grid_100, &Node::new("(0,0)"), "Grid Graph (100x100)");
    
    // Test parallel Dijkstra
    println!("\n🚀 Parallel Multi-Source Dijkstra");
    let sources = vec![
        Node::new("N0"),
        Node::new("N10"),
        Node::new("N20"),
        Node::new("N30"),
    ];
    
    let start = Instant::now();
    let results = parallel_dijkstra(&complete_100, sources.clone());
    let duration = start.elapsed();
    
    println!("Computed shortest paths from {} sources in {:?}", 
             sources.len(), duration);
    
    for (source, result) in &results {
        let avg_distance: f64 = result.distances
            .values()
            .filter(|&&d| d != f64::INFINITY)
            .sum::<f64>() / result.distances.len() as f64;
        println!("  From {}: avg distance = {:.2}", source, avg_distance);
    }
    
    println!("\n✅ Dijkstra's algorithm baseline established");
    println!("🎯 Ready for performance comparison with other implementations");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_single_node() {
        let mut graph = Graph::new();
        graph.add_node(Node::new("A"));
        
        let result = dijkstra(&graph, &Node::new("A"));
        assert_eq!(result.get_distance(&Node::new("A")), Some(0.0));
    }
    
    #[test]
    fn test_simple_path() {
        let mut graph = Graph::new();
        graph.add_edge(Node::new("A"), Node::new("B"), 5.0);
        
        let result = dijkstra(&graph, &Node::new("A"));
        assert_eq!(result.get_distance(&Node::new("B")), Some(5.0));
        
        let path = result.get_path(&Node::new("B")).unwrap();
        assert_eq!(path.len(), 2);
        assert_eq!(path[0].id, "A");
        assert_eq!(path[1].id, "B");
    }
    
    #[test]
    fn test_shortest_path_selection() {
        let mut graph = Graph::new();
        graph.add_edge(Node::new("A"), Node::new("B"), 4.0);
        graph.add_edge(Node::new("B"), Node::new("C"), 3.0);
        graph.add_edge(Node::new("A"), Node::new("C"), 10.0);
        
        let result = dijkstra(&graph, &Node::new("A"));
        assert_eq!(result.get_distance(&Node::new("C")), Some(7.0));
        
        let path = result.get_path(&Node::new("C")).unwrap();
        assert_eq!(path.len(), 3);
        assert_eq!(path[1].id, "B"); // Should go through B
    }
    
    #[test]
    fn test_unreachable_node() {
        let mut graph = Graph::new();
        graph.add_edge(Node::new("A"), Node::new("B"), 1.0);
        graph.add_node(Node::new("C"));
        
        let result = dijkstra(&graph, &Node::new("A"));
        assert_eq!(result.get_distance(&Node::new("C")), Some(f64::INFINITY));
        assert_eq!(result.get_path(&Node::new("C")), None);
    }
    
    #[test]
    #[should_panic(expected = "Negative edge weights not supported")]
    fn test_negative_edge_rejection() {
        let mut graph = Graph::new();
        graph.add_edge(Node::new("A"), Node::new("B"), -1.0);
    }
}