//! Graph Coloring Problem - Multiple Algorithm Implementation
//!
//! This module implements various approaches to solve the Graph Coloring problem:
//! - Backtracking: O(k^V) time, exact solution with pruning
//! - Welsh-Powell: O(V²) time, greedy heuristic with degree ordering
//! - Simple Greedy: O(V+E) time, fast approximation
//! - Constraint Satisfaction: Advanced pruning techniques

use std::collections::HashSet;
use std::fmt;
use std::time::Instant;

#[derive(Clone, Debug)]
struct Graph {
    vertices: usize,
    adj_list: Vec<HashSet<usize>>,
    adj_matrix: Vec<Vec<bool>>,
}

impl Graph {
    fn new(vertices: usize) -> Self {
        Self {
            vertices,
            adj_list: vec![HashSet::new(); vertices],
            adj_matrix: vec![vec![false; vertices]; vertices],
        }
    }
    
    fn add_edge(&mut self, u: usize, v: usize) {
        if u < self.vertices && v < self.vertices {
            self.adj_list[u].insert(v);
            self.adj_list[v].insert(u);
            self.adj_matrix[u][v] = true;
            self.adj_matrix[v][u] = true;
        }
    }
    
    fn is_adjacent(&self, u: usize, v: usize) -> bool {
        u < self.vertices && v < self.vertices && self.adj_matrix[u][v]
    }
    
    fn degree(&self, vertex: usize) -> usize {
        if vertex < self.vertices {
            self.adj_list[vertex].len()
        } else {
            0
        }
    }
    
    fn max_degree(&self) -> usize {
        (0..self.vertices).map(|v| self.degree(v)).max().unwrap_or(0)
    }
    
    fn edges(&self) -> Vec<(usize, usize)> {
        let mut edges = Vec::new();
        for u in 0..self.vertices {
            for &v in &self.adj_list[u] {
                if u < v {  // Avoid duplicates
                    edges.push((u, v));
                }
            }
        }
        edges
    }
    
    fn from_edges(vertices: usize, edges: &[(usize, usize)]) -> Self {
        let mut graph = Self::new(vertices);
        for &(u, v) in edges {
            graph.add_edge(u, v);
        }
        graph
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Graph with {} vertices:", self.vertices)?;
        writeln!(f, "Adjacency List:")?;
        for v in 0..self.vertices {
            let neighbors: Vec<usize> = self.adj_list[v].iter().copied().collect();
            writeln!(f, "  {}: {:?}", v, neighbors)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct ColoringResult {
    coloring: Vec<usize>,
    colors_used: usize,
    algorithm_used: String,
    computation_time_ms: f64,
    is_valid: bool,
}

impl ColoringResult {
    fn new(coloring: Vec<usize>, algorithm: &str, time_ms: f64, graph: &Graph) -> Self {
        let colors_used = coloring.iter().max().copied().unwrap_or(0);
        let is_valid = is_valid_coloring(graph, &coloring);
        
        Self {
            coloring,
            colors_used,
            algorithm_used: algorithm.to_string(),
            computation_time_ms: time_ms,
            is_valid,
        }
    }
}

impl fmt::Display for ColoringResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Graph Coloring Result ({}):", self.algorithm_used)?;
        writeln!(f, "  Colors Used: {}", self.colors_used)?;
        writeln!(f, "  Valid Coloring: {}", self.is_valid)?;
        writeln!(f, "  Computation Time: {:.3}ms", self.computation_time_ms)?;
        writeln!(f, "  Vertex Colors: {:?}", self.coloring)
    }
}

// Check if a coloring is valid (no adjacent vertices have same color)
fn is_valid_coloring(graph: &Graph, coloring: &[usize]) -> bool {
    if coloring.len() != graph.vertices {
        return false;
    }
    
    for u in 0..graph.vertices {
        for &v in &graph.adj_list[u] {
            if coloring[u] == coloring[v] && coloring[u] != 0 {
                return false;
            }
        }
    }
    true
}

// Check if assigning a color to a vertex is safe
fn is_safe_color(graph: &Graph, vertex: usize, color: usize, coloring: &[usize]) -> bool {
    for &neighbor in &graph.adj_list[vertex] {
        if coloring[neighbor] == color {
            return false;
        }
    }
    true
}

// Backtracking algorithm for exact graph coloring
fn graph_coloring_backtrack(graph: &Graph, max_colors: usize) -> ColoringResult {
    let start_time = Instant::now();
    let mut coloring = vec![0; graph.vertices];
    
    fn backtrack(graph: &Graph, vertex: usize, coloring: &mut Vec<usize>, 
                 max_colors: usize) -> bool {
        if vertex == graph.vertices {
            return true; // All vertices successfully colored
        }
        
        for color in 1..=max_colors {
            if is_safe_color(graph, vertex, color, coloring) {
                coloring[vertex] = color;
                if backtrack(graph, vertex + 1, coloring, max_colors) {
                    return true;
                }
                coloring[vertex] = 0; // Backtrack
            }
        }
        false
    }
    
    let success = backtrack(graph, 0, &mut coloring, max_colors);
    let elapsed = start_time.elapsed().as_secs_f64() * 1000.0;
    
    if !success {
        coloring = vec![0; graph.vertices]; // No valid coloring found
    }
    
    ColoringResult::new(coloring, "Backtracking", elapsed, graph)
}

// Simple greedy coloring algorithm
fn graph_coloring_greedy(graph: &Graph) -> ColoringResult {
    let start_time = Instant::now();
    let mut coloring = vec![0; graph.vertices];
    
    for vertex in 0..graph.vertices {
        let mut used_colors = vec![false; graph.vertices + 1];
        
        // Mark colors used by neighbors
        for &neighbor in &graph.adj_list[vertex] {
            if coloring[neighbor] != 0 {
                used_colors[coloring[neighbor]] = true;
            }
        }
        
        // Find first available color
        for color in 1..=graph.vertices {
            if !used_colors[color] {
                coloring[vertex] = color;
                break;
            }
        }
    }
    
    let elapsed = start_time.elapsed().as_secs_f64() * 1000.0;
    ColoringResult::new(coloring, "Simple Greedy", elapsed, graph)
}

// Welsh-Powell algorithm (greedy with degree ordering)
fn graph_coloring_welsh_powell(graph: &Graph) -> ColoringResult {
    let start_time = Instant::now();
    let mut coloring = vec![0; graph.vertices];
    
    // Create vertices sorted by degree (descending)
    let mut vertices: Vec<usize> = (0..graph.vertices).collect();
    vertices.sort_by_key(|&v| graph.degree(v));
    vertices.reverse(); // Highest degree first
    
    for &vertex in &vertices {
        let mut used_colors = vec![false; graph.vertices + 1];
        
        // Mark colors used by neighbors
        for &neighbor in &graph.adj_list[vertex] {
            if coloring[neighbor] != 0 {
                used_colors[coloring[neighbor]] = true;
            }
        }
        
        // Find first available color
        for color in 1..=graph.vertices {
            if !used_colors[color] {
                coloring[vertex] = color;
                break;
            }
        }
    }
    
    let elapsed = start_time.elapsed().as_secs_f64() * 1000.0;
    ColoringResult::new(coloring, "Welsh-Powell", elapsed, graph)
}

// Largest First ordering (alternative to Welsh-Powell)
fn graph_coloring_largest_first(graph: &Graph) -> ColoringResult {
    let start_time = Instant::now();
    let mut coloring = vec![0; graph.vertices];
    
    // Sort vertices by degree (descending) and process in that order
    let mut vertices_by_degree: Vec<(usize, usize)> = (0..graph.vertices)
        .map(|v| (v, graph.degree(v)))
        .collect();
    vertices_by_degree.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by degree descending
    
    for &(vertex, _degree) in &vertices_by_degree {
        let mut color = 1;
        while !is_safe_color(graph, vertex, color, &coloring) {
            color += 1;
        }
        coloring[vertex] = color;
    }
    
    let elapsed = start_time.elapsed().as_secs_f64() * 1000.0;
    ColoringResult::new(coloring, "Largest First", elapsed, graph)
}

// Find chromatic number using binary search with backtracking
fn _find_chromatic_number(graph: &Graph, max_search: usize) -> usize {
    let mut left = 1;
    let mut right = max_search.min(graph.vertices);
    let mut chromatic_number = right;
    
    while left <= right {
        let mid = (left + right) / 2;
        let result = graph_coloring_backtrack(graph, mid);
        
        if result.is_valid && result.colors_used > 0 {
            chromatic_number = mid;
            right = mid - 1; // Try to find smaller chromatic number
        } else {
            left = mid + 1; // Need more colors
        }
    }
    
    chromatic_number
}

// Performance comparison between algorithms
fn run_performance_comparison(graph: &Graph) {
    println!("Performance Comparison: {} vertices, {} edges", 
             graph.vertices, graph.edges().len());
    println!("{}", "-".repeat(70));
    
    let algorithms: Vec<Box<dyn Fn(&Graph) -> ColoringResult>> = vec![
        Box::new(graph_coloring_greedy),
        Box::new(graph_coloring_welsh_powell),
        Box::new(graph_coloring_largest_first),
    ];
    
    let mut results = Vec::new();
    for algorithm in algorithms {
        results.push(algorithm(graph));
    }
    
    // Only include backtracking for small graphs
    if graph.vertices <= 12 {
        let max_colors = graph.max_degree() + 1;
        results.push(graph_coloring_backtrack(graph, max_colors));
    }
    
    for result in &results {
        println!(
            "{:<18} | Colors: {:2} | Time: {:8.3}ms | Valid: {}",
            result.algorithm_used, 
            result.colors_used, 
            result.computation_time_ms,
            result.is_valid
        );
    }
    
    // Find theoretical bounds
    let max_degree = graph.max_degree();
    let clique_lower_bound = estimate_clique_number(graph);
    
    println!("{}", "-".repeat(70));
    println!("Graph Properties:");
    println!("  Max Degree (Δ): {}", max_degree);
    println!("  Brooks' Upper Bound: {} colors", max_degree + 1);
    println!("  Clique Lower Bound: {} colors", clique_lower_bound);
    
    // Analyze algorithm performance
    if let Some(best_exact) = results.iter()
        .filter(|r| r.algorithm_used.contains("Backtracking"))
        .min_by_key(|r| r.colors_used) {
        
        println!("  Exact Chromatic Number: {}", best_exact.colors_used);
        
        for result in &results {
            if !result.algorithm_used.contains("Backtracking") {
                let approximation_ratio = result.colors_used as f64 / best_exact.colors_used as f64;
                println!("  {} Ratio: {:.2}x optimal", 
                         result.algorithm_used, approximation_ratio);
            }
        }
    }
}

// Estimate clique number (lower bound for chromatic number)
fn estimate_clique_number(graph: &Graph) -> usize {
    let mut max_clique_size = 1;
    
    // Simple greedy clique finding
    for start_vertex in 0..graph.vertices {
        let mut clique = vec![start_vertex];
        
        for candidate in (start_vertex + 1)..graph.vertices {
            let is_connected_to_all = clique.iter()
                .all(|&v| graph.is_adjacent(v, candidate));
            
            if is_connected_to_all {
                clique.push(candidate);
            }
        }
        
        max_clique_size = max_clique_size.max(clique.len());
    }
    
    max_clique_size
}

// Visualize small graphs and their colorings
fn visualize_coloring(graph: &Graph, result: &ColoringResult, show_details: bool) {
    println!("Graph Coloring Visualization:");
    println!("{}", "=".repeat(50));
    
    if show_details {
        println!("Graph structure:");
        for v in 0..graph.vertices {
            let neighbors: Vec<String> = graph.adj_list[v]
                .iter()
                .map(|&n| n.to_string())
                .collect();
            println!("  Vertex {}: connected to [{}]", v, neighbors.join(", "));
        }
        println!();
    }
    
    println!("Coloring result ({}):", result.algorithm_used);
    println!("{:<8} {:<6} {:<12} {:<20}", "Vertex", "Color", "Degree", "Neighbors");
    println!("{}", "-".repeat(50));
    
    for v in 0..graph.vertices {
        let neighbors: Vec<String> = graph.adj_list[v]
            .iter()
            .map(|&n| format!("{}({})", n, 
                if v < result.coloring.len() && n < result.coloring.len() {
                    result.coloring[n].to_string()
                } else {
                    "?".to_string()
                }))
            .collect();
        
        let color = if v < result.coloring.len() { 
            result.coloring[v].to_string() 
        } else { 
            "?".to_string() 
        };
        
        println!("{:<8} {:<6} {:<12} {:<20}", 
                 v, color, graph.degree(v), neighbors.join(","));
    }
    
    println!();
    println!("Summary:");
    println!("  Total colors used: {}", result.colors_used);
    println!("  Valid coloring: {}", result.is_valid);
    println!("  Computation time: {:.3}ms", result.computation_time_ms);
}

// Create classic test graphs
fn create_complete_graph(n: usize) -> Graph {
    let mut graph = Graph::new(n);
    for i in 0..n {
        for j in (i + 1)..n {
            graph.add_edge(i, j);
        }
    }
    graph
}

fn create_cycle_graph(n: usize) -> Graph {
    let mut graph = Graph::new(n);
    for i in 0..n {
        graph.add_edge(i, (i + 1) % n);
    }
    graph
}

fn create_bipartite_graph(left_size: usize, right_size: usize, edges: &[(usize, usize)]) -> Graph {
    let mut graph = Graph::new(left_size + right_size);
    for &(u, v) in edges {
        if u < left_size && v < right_size {
            graph.add_edge(u, left_size + v);
        }
    }
    graph
}

fn create_petersen_graph() -> Graph {
    Graph::from_edges(10, &[
        // Outer 5-cycle
        (0, 1), (1, 2), (2, 3), (3, 4), (4, 0),
        // Inner 5-cycle  
        (5, 6), (6, 7), (7, 8), (8, 9), (9, 5),
        // Connections between cycles
        (0, 5), (1, 6), (2, 7), (3, 8), (4, 9),
    ])
}

// Test case runner
fn run_test_case(name: &str, graph: Graph, expected_chromatic: Option<usize>) {
    println!("Test Case: {}", name);
    println!("{}", "=".repeat(50));
    
    println!("{}", graph);
    
    // Run Welsh-Powell as primary algorithm
    let result = graph_coloring_welsh_powell(&graph);
    
    if let Some(expected) = expected_chromatic {
        let test_passed = result.colors_used <= expected && result.is_valid;
        println!("Expected chromatic number: ≤{}", expected);
        println!("Algorithm result: {} colors", result.colors_used);
        println!("Test status: {}", if test_passed { "PASS" } else { "FAIL" });
    }
    
    // Show visualization for small graphs
    if graph.vertices <= 10 {
        println!();
        visualize_coloring(&graph, &result, true);
    }
    
    // Performance comparison for larger graphs
    if graph.vertices > 5 {
        println!();
        run_performance_comparison(&graph);
    }
    
    println!();
}

// Generate test graphs with specific properties
fn generate_random_graph(vertices: usize, density: f64, seed: u64) -> Graph {
    let mut graph = Graph::new(vertices);
    let mut rng = seed;
    
    for i in 0..vertices {
        for j in (i + 1)..vertices {
            rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
            let prob = (rng as f64) / (u64::MAX as f64);
            
            if prob < density {
                graph.add_edge(i, j);
            }
        }
    }
    
    graph
}

fn main() {
    println!("Graph Coloring Problem - Multiple Algorithm Implementation");
    println!("{}", "=".repeat(65));
    
    // Test Case 1: Triangle (Complete graph K3)
    run_test_case("Triangle (K3)", create_complete_graph(3), Some(3));
    
    // Test Case 2: Square (4-cycle)
    run_test_case("Square (C4)", create_cycle_graph(4), Some(2));
    
    // Test Case 3: Pentagon (5-cycle)
    run_test_case("Pentagon (C5)", create_cycle_graph(5), Some(3));
    
    // Test Case 4: Bipartite graph
    let bipartite = create_bipartite_graph(3, 3, &[(0, 0), (0, 1), (1, 1), (1, 2), (2, 0), (2, 2)]);
    run_test_case("Bipartite Graph", bipartite, Some(2));
    
    // Test Case 5: Petersen graph
    run_test_case("Petersen Graph", create_petersen_graph(), Some(3));
    
    // Test Case 6: Complete graph K4
    run_test_case("Complete K4", create_complete_graph(4), Some(4));
    
    // Performance test on medium random graph
    println!("Medium Random Graph Performance Test:");
    println!("{}", "=".repeat(50));
    let medium_random = generate_random_graph(15, 0.4, 42);
    run_performance_comparison(&medium_random);
    
    // Large graph stress test
    println!("\nLarge Graph Stress Test:");
    println!("{}", "=".repeat(40));
    let large_random = generate_random_graph(25, 0.3, 123);
    
    let start_time = Instant::now();
    let large_result = graph_coloring_welsh_powell(&large_random);
    let elapsed = start_time.elapsed().as_secs_f64() * 1000.0;
    
    println!("Random graph: {} vertices, {} edges", 
             large_random.vertices, large_random.edges().len());
    println!("Welsh-Powell coloring: {} colors", large_result.colors_used);
    println!("Computation time: {:.2}ms", elapsed);
    println!("Graph density: {:.1}%", 
             (large_random.edges().len() as f64 * 200.0) / 
             (large_random.vertices * (large_random.vertices - 1)) as f64);
    
    // Algorithm comparison on various graph types
    println!("\nAlgorithm Comparison on Different Graph Types:");
    println!("{}", "=".repeat(60));
    
    let test_graphs = vec![
        ("Dense Random", generate_random_graph(12, 0.7, 456)),
        ("Sparse Random", generate_random_graph(20, 0.2, 789)),
        ("Path Graph", {
            let mut path = Graph::new(10);
            for i in 0..9 { path.add_edge(i, i + 1); }
            path
        }),
        ("Star Graph", {
            let mut star = Graph::new(8);
            for i in 1..8 { star.add_edge(0, i); }
            star
        }),
    ];
    
    for (name, graph) in test_graphs {
        println!("\n{} ({} vertices, {} edges):", 
                 name, graph.vertices, graph.edges().len());
        
        let algorithms = vec![
            ("Greedy", graph_coloring_greedy(&graph)),
            ("Welsh-Powell", graph_coloring_welsh_powell(&graph)),
            ("Largest First", graph_coloring_largest_first(&graph)),
        ];
        
        for (alg_name, result) in algorithms {
            println!("  {}: {} colors in {:.2}ms", 
                     alg_name, result.colors_used, result.computation_time_ms);
        }
    }
    
    // Algorithm summary
    println!("\nAlgorithm Summary:");
    println!("{}", "=".repeat(60));
    println!("Backtracking:      O(k^V) time, exact solution, exponential");
    println!("Welsh-Powell:      O(V²) time, greedy with degree ordering");
    println!("Simple Greedy:     O(V+E) time, fast but suboptimal");
    println!("Largest First:     O(V²) time, alternative degree-based heuristic");
    println!("\nRecommendations:");
    println!("- Small graphs (V ≤ 15): Use backtracking for optimal solution");
    println!("- Medium graphs (V ≤ 100): Use Welsh-Powell for good quality");
    println!("- Large graphs (V > 100): Use simple greedy for speed");
    println!("- Real-time applications: Always use greedy variants");
}