// Topological Sort - Rust Implementation
// Multiple algorithm approaches for DAG vertex ordering

use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

// Graph representation for topological sorting
#[derive(Debug, Clone)]
struct DirectedGraph {
    vertices: HashSet<i32>,
    edges: Vec<(i32, i32)>,
    adjacency_list: HashMap<i32, Vec<i32>>,
    reverse_adjacency_list: HashMap<i32, Vec<i32>>,
}

impl DirectedGraph {
    fn new() -> Self {
        Self {
            vertices: HashSet::new(),
            edges: Vec::new(),
            adjacency_list: HashMap::new(),
            reverse_adjacency_list: HashMap::new(),
        }
    }

    fn add_vertex(&mut self, vertex: i32) {
        self.vertices.insert(vertex);
        self.adjacency_list.entry(vertex).or_insert_with(Vec::new);
        self.reverse_adjacency_list
            .entry(vertex)
            .or_insert_with(Vec::new);
    }

    fn add_edge(&mut self, from: i32, to: i32) {
        self.vertices.insert(from);
        self.vertices.insert(to);
        self.edges.push((from, to));

        self.adjacency_list
            .entry(from)
            .or_insert_with(Vec::new)
            .push(to);
        self.reverse_adjacency_list
            .entry(to)
            .or_insert_with(Vec::new)
            .push(from);

        // Ensure both vertices have entries
        self.adjacency_list.entry(to).or_insert_with(Vec::new);
        self.reverse_adjacency_list
            .entry(from)
            .or_insert_with(Vec::new);
    }

    fn get_vertices(&self) -> Vec<i32> {
        let mut vertices: Vec<i32> = self.vertices.iter().cloned().collect();
        vertices.sort();
        vertices
    }

    fn get_neighbors(&self, vertex: i32) -> Vec<i32> {
        self.adjacency_list
            .get(&vertex)
            .cloned()
            .unwrap_or_default()
    }

    fn get_in_degree(&self, vertex: i32) -> usize {
        self.reverse_adjacency_list
            .get(&vertex)
            .map_or(0, |v| v.len())
    }

    fn has_cycle(&self) -> bool {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        for &vertex in &self.vertices {
            if !visited.contains(&vertex) {
                if self.has_cycle_util(vertex, &mut visited, &mut rec_stack) {
                    return true;
                }
            }
        }
        false
    }

    fn has_cycle_util(
        &self,
        vertex: i32,
        visited: &mut HashSet<i32>,
        rec_stack: &mut HashSet<i32>,
    ) -> bool {
        visited.insert(vertex);
        rec_stack.insert(vertex);

        for neighbor in self.get_neighbors(vertex) {
            if !visited.contains(&neighbor) {
                if self.has_cycle_util(neighbor, visited, rec_stack) {
                    return true;
                }
            } else if rec_stack.contains(&neighbor) {
                return true;
            }
        }

        rec_stack.remove(&vertex);
        false
    }
}

// Result structure for topological sort
#[derive(Debug, Clone)]
struct TopologicalSortResult {
    ordering: Vec<i32>,
    algorithm: String,
    time_ms: f64,
    has_cycle: bool,
    is_valid: bool,
}

impl TopologicalSortResult {
    fn new(algorithm: String) -> Self {
        Self {
            ordering: Vec::new(),
            algorithm,
            time_ms: 0.0,
            has_cycle: false,
            is_valid: false,
        }
    }
}

// 1. DFS-based Topological Sort (Recursive)
fn topological_sort_dfs_recursive(graph: &DirectedGraph) -> TopologicalSortResult {
    let start = Instant::now();
    let mut result = TopologicalSortResult::new("DFS Recursive".to_string());

    if graph.has_cycle() {
        result.has_cycle = true;
        result.time_ms = start.elapsed().as_secs_f64() * 1000.0;
        return result;
    }

    let mut visited = HashSet::new();
    let mut stack = Vec::new();

    for &vertex in &graph.vertices {
        if !visited.contains(&vertex) {
            dfs_recursive_util(graph, vertex, &mut visited, &mut stack);
        }
    }

    stack.reverse();
    result.ordering = stack;
    result.is_valid = true;
    result.time_ms = start.elapsed().as_secs_f64() * 1000.0;
    result
}

fn dfs_recursive_util(
    graph: &DirectedGraph,
    vertex: i32,
    visited: &mut HashSet<i32>,
    stack: &mut Vec<i32>,
) {
    visited.insert(vertex);

    for neighbor in graph.get_neighbors(vertex) {
        if !visited.contains(&neighbor) {
            dfs_recursive_util(graph, neighbor, visited, stack);
        }
    }

    stack.push(vertex);
}

// 2. DFS-based Topological Sort (Iterative)
fn topological_sort_dfs_iterative(graph: &DirectedGraph) -> TopologicalSortResult {
    let start = Instant::now();
    let mut result = TopologicalSortResult::new("DFS Iterative".to_string());

    if graph.has_cycle() {
        result.has_cycle = true;
        result.time_ms = start.elapsed().as_secs_f64() * 1000.0;
        return result;
    }

    let mut visited = HashSet::new();
    let mut finished = Vec::new();

    for &start_vertex in &graph.vertices {
        if !visited.contains(&start_vertex) {
            let mut stack = vec![(start_vertex, false)];

            while let Some((vertex, processed)) = stack.pop() {
                if processed {
                    finished.push(vertex);
                } else if !visited.contains(&vertex) {
                    visited.insert(vertex);
                    stack.push((vertex, true));

                    for neighbor in graph.get_neighbors(vertex) {
                        if !visited.contains(&neighbor) {
                            stack.push((neighbor, false));
                        }
                    }
                }
            }
        }
    }

    finished.reverse();
    result.ordering = finished;
    result.is_valid = true;
    result.time_ms = start.elapsed().as_secs_f64() * 1000.0;
    result
}

// 3. Kahn's Algorithm (BFS-based)
fn topological_sort_kahn(graph: &DirectedGraph) -> TopologicalSortResult {
    let start = Instant::now();
    let mut result = TopologicalSortResult::new("Kahn's Algorithm (BFS)".to_string());

    // Calculate in-degrees
    let mut in_degree: HashMap<i32, usize> = HashMap::new();
    for &vertex in &graph.vertices {
        in_degree.insert(vertex, graph.get_in_degree(vertex));
    }

    // Find vertices with in-degree 0
    let mut queue = VecDeque::new();
    for (&vertex, &degree) in &in_degree {
        if degree == 0 {
            queue.push_back(vertex);
        }
    }

    let mut ordering = Vec::new();

    while let Some(vertex) = queue.pop_front() {
        ordering.push(vertex);

        // Reduce in-degree of neighbors
        for neighbor in graph.get_neighbors(vertex) {
            if let Some(degree) = in_degree.get_mut(&neighbor) {
                *degree -= 1;
                if *degree == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    // Check if all vertices are processed (no cycle)
    if ordering.len() != graph.vertices.len() {
        result.has_cycle = true;
    } else {
        result.ordering = ordering;
        result.is_valid = true;
    }

    result.time_ms = start.elapsed().as_secs_f64() * 1000.0;
    result
}

// Validation helper
fn validate_topological_order(graph: &DirectedGraph, ordering: &[i32]) -> bool {
    let mut position = HashMap::new();

    // Record position of each vertex in the ordering
    for (i, &vertex) in ordering.iter().enumerate() {
        position.insert(vertex, i);
    }

    // Check all edges respect the ordering
    for &(from, to) in &graph.edges {
        if let (Some(&pos_from), Some(&pos_to)) = (position.get(&from), position.get(&to)) {
            if pos_from >= pos_to {
                return false; // Edge violates topological order
            }
        }
    }

    true
}

// Visualization helper
fn visualize_graph(graph: &DirectedGraph) {
    println!("Graph Structure:");
    println!("{}", "-".repeat(40));

    let vertices = graph.get_vertices();
    for vertex in vertices {
        let neighbors = graph.get_neighbors(vertex);
        if neighbors.is_empty() {
            println!("  {} → (no outgoing edges)", vertex);
        } else {
            println!("  {} → {:?}", vertex, neighbors);
        }
    }

    println!("  Total vertices: {}", graph.vertices.len());
    println!("  Total edges: {}", graph.edges.len());
}

// Test case runner
fn run_test_case(name: &str, graph: DirectedGraph) {
    println!("\nTest Case: {}", name);
    println!("{}", "=".repeat(60));

    visualize_graph(&graph);

    // Run all algorithms
    let results = vec![
        topological_sort_dfs_recursive(&graph),
        topological_sort_dfs_iterative(&graph),
        topological_sort_kahn(&graph),
    ];

    println!("\nAlgorithm Results:");
    println!("{}", "-".repeat(70));
    println!(
        "{:<25} | {:>8} | {:>10} | {:>8} | {}",
        "Algorithm", "Valid", "Time (ms)", "Cycle", "Ordering"
    );
    println!("{}", "-".repeat(70));

    for result in &results {
        let cycle_str = if result.has_cycle { "Yes" } else { "No" };
        let valid_str = if result.is_valid { "Yes" } else { "No" };

        let ordering_str = if result.ordering.is_empty() {
            "N/A (cycle detected)".to_string()
        } else if result.ordering.len() > 8 {
            format!("{:?}...", &result.ordering[..6])
        } else {
            format!("{:?}", result.ordering)
        };

        println!(
            "{:<25} | {:>8} | {:>10.3} | {:>8} | {}",
            result.algorithm, valid_str, result.time_ms, cycle_str, ordering_str
        );
    }

    // Validate orderings
    if let Some(valid_result) = results.iter().find(|r| r.is_valid) {
        let is_valid = validate_topological_order(&graph, &valid_result.ordering);
        println!(
            "\nValidation: {}",
            if is_valid { "✅ PASS" } else { "❌ FAIL" }
        );

        if !valid_result.ordering.is_empty() {
            println!("Sample valid ordering: {:?}", valid_result.ordering);
        }
    }

    // Check algorithm consistency
    let valid_results: Vec<_> = results.iter().filter(|r| r.is_valid).collect();
    if valid_results.len() > 1 {
        println!("Multiple valid solutions found - this is expected for DAGs");
    }
}

fn main() {
    println!("Topological Sort - Multiple Algorithm Implementation");
    println!("{}", "=".repeat(70));

    // Test Case 1: Simple Linear Chain
    let mut linear_chain = DirectedGraph::new();
    linear_chain.add_edge(1, 2);
    linear_chain.add_edge(2, 3);
    linear_chain.add_edge(3, 4);
    run_test_case("Linear Chain (1→2→3→4)", linear_chain);

    // Test Case 2: Diamond DAG
    let mut diamond = DirectedGraph::new();
    diamond.add_edge(1, 2);
    diamond.add_edge(1, 3);
    diamond.add_edge(2, 4);
    diamond.add_edge(3, 4);
    run_test_case("Diamond DAG", diamond);

    // Test Case 3: Complex DAG (Course Prerequisites)
    let mut courses = DirectedGraph::new();
    // Prerequisites: Math101 → {Stats, Calculus}, Stats → DataScience, Calculus → Physics
    courses.add_edge(101, 201); // Math101 → Stats
    courses.add_edge(101, 202); // Math101 → Calculus
    courses.add_edge(201, 301); // Stats → DataScience
    courses.add_edge(202, 302); // Calculus → Physics
    courses.add_edge(202, 303); // Calculus → Engineering
    courses.add_edge(301, 401); // DataScience → ML
    courses.add_edge(302, 401); // Physics → ML (requires both paths)
    run_test_case("Course Prerequisites DAG", courses);

    // Test Case 4: Single Vertex
    let mut single = DirectedGraph::new();
    single.add_vertex(42);
    run_test_case("Single Vertex", single);

    // Test Case 5: Disconnected Components
    let mut disconnected = DirectedGraph::new();
    disconnected.add_edge(1, 2);
    disconnected.add_edge(3, 4);
    disconnected.add_edge(5, 6);
    disconnected.add_vertex(7); // Isolated vertex
    run_test_case("Disconnected Components", disconnected);

    // Test Case 6: Graph with Cycle
    let mut cyclic = DirectedGraph::new();
    cyclic.add_edge(1, 2);
    cyclic.add_edge(2, 3);
    cyclic.add_edge(3, 1); // Creates cycle
    cyclic.add_edge(2, 4);
    run_test_case("Graph with Cycle (Invalid for TopSort)", cyclic);

    // Test Case 7: Complex Real-World Example (Build Dependencies)
    let mut build_deps = DirectedGraph::new();
    // Simulating: utils → {parser, logger}, parser → compiler, logger → compiler, compiler → linker
    build_deps.add_edge(1, 2); // utils → parser
    build_deps.add_edge(1, 3); // utils → logger
    build_deps.add_edge(2, 4); // parser → compiler
    build_deps.add_edge(3, 4); // logger → compiler
    build_deps.add_edge(4, 5); // compiler → linker
    build_deps.add_edge(1, 6); // utils → optimizer (independent path)
    build_deps.add_edge(6, 5); // optimizer → linker
    run_test_case("Build Dependencies DAG", build_deps);

    // Performance test with larger graph
    let mut large_dag = DirectedGraph::new();
    for i in 0..100 {
        for j in (i + 1)..std::cmp::min(i + 5, 100) {
            large_dag.add_edge(i, j);
        }
    }
    run_test_case("Large DAG (100 vertices)", large_dag);

    println!("\n\nAlgorithm Summary:");
    println!("{}", "=".repeat(70));
    println!("DFS Recursive:     O(V + E) time, O(V) space, natural implementation");
    println!("DFS Iterative:     O(V + E) time, O(V) space, avoids recursion");
    println!("Kahn's Algorithm:  O(V + E) time, O(V) space, detects cycles early");
    println!("\nAll algorithms are optimal with linear time complexity!");
    println!("\nApplications:");
    println!("- Build systems and compilation order");
    println!("- Package dependency resolution");
    println!("- Task scheduling with prerequisites");
    println!("- Course prerequisite planning");
    println!("- Spreadsheet formula evaluation order");
}
