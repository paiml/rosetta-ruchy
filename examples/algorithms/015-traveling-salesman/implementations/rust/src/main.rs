// Traveling Salesman Problem - Rust Implementation
// Multiple algorithm approaches for the classic NP-hard optimization problem

use std::time::Instant;

// Graph representation for TSP
#[derive(Debug, Clone)]
struct Graph {
    n: usize,
    distances: Vec<Vec<f64>>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Self {
            n,
            distances: vec![vec![0.0; n]; n],
        }
    }

    fn from_points(points: &[(f64, f64)]) -> Self {
        let n = points.len();
        let mut graph = Self::new(n);

        for i in 0..n {
            for j in 0..n {
                if i != j {
                    let dx = points[i].0 - points[j].0;
                    let dy = points[i].1 - points[j].1;
                    graph.distances[i][j] = (dx * dx + dy * dy).sqrt();
                }
            }
        }

        graph
    }

    fn add_edge(&mut self, from: usize, to: usize, distance: f64) {
        self.distances[from][to] = distance;
    }

    fn get_distance(&self, from: usize, to: usize) -> f64 {
        self.distances[from][to]
    }

    fn tour_distance(&self, tour: &[usize]) -> f64 {
        let mut total = 0.0;
        for i in 0..tour.len() {
            let from = tour[i];
            let to = tour[(i + 1) % tour.len()];
            total += self.get_distance(from, to);
        }
        total
    }
}

// Solution structure
#[derive(Debug, Clone)]
struct TSPSolution {
    tour: Vec<usize>,
    distance: f64,
    algorithm: String,
    time_ms: f64,
}

// 1. Brute Force Algorithm (optimal but exponential)
fn tsp_brute_force(graph: &Graph) -> TSPSolution {
    let start = Instant::now();
    let n = graph.n;

    if n > 10 {
        // Too large for brute force
        return TSPSolution {
            tour: vec![],
            distance: f64::INFINITY,
            algorithm: "Brute Force (skipped - too large)".to_string(),
            time_ms: 0.0,
        };
    }

    let mut cities: Vec<usize> = (1..n).collect();
    let mut best_tour = vec![0];
    best_tour.extend_from_slice(&cities);
    let mut best_distance = graph.tour_distance(&best_tour);

    // Generate all permutations
    permute(&mut cities, 0, graph, &mut best_tour, &mut best_distance);

    TSPSolution {
        tour: best_tour,
        distance: best_distance,
        algorithm: "Brute Force".to_string(),
        time_ms: start.elapsed().as_secs_f64() * 1000.0,
    }
}

fn permute(
    cities: &mut Vec<usize>,
    k: usize,
    graph: &Graph,
    best_tour: &mut Vec<usize>,
    best_distance: &mut f64,
) {
    if k == cities.len() {
        let mut tour = vec![0];
        tour.extend_from_slice(cities);
        let distance = graph.tour_distance(&tour);
        if distance < *best_distance {
            *best_distance = distance;
            *best_tour = tour;
        }
        return;
    }

    for i in k..cities.len() {
        cities.swap(k, i);
        permute(cities, k + 1, graph, best_tour, best_distance);
        cities.swap(k, i);
    }
}

// 2. Dynamic Programming (Held-Karp Algorithm)
fn tsp_dynamic_programming(graph: &Graph) -> TSPSolution {
    let start = Instant::now();
    let n = graph.n;

    if n > 20 {
        // Too large for DP
        return TSPSolution {
            tour: vec![],
            distance: f64::INFINITY,
            algorithm: "Dynamic Programming (skipped - too large)".to_string(),
            time_ms: 0.0,
        };
    }

    let num_subsets = 1 << n;
    let mut dp = vec![vec![f64::INFINITY; n]; num_subsets];
    let mut parent = vec![vec![None; n]; num_subsets];

    // Base case: starting from city 0
    dp[1][0] = 0.0;

    // Fill DP table
    for mask in 1..num_subsets {
        for last in 0..n {
            if (mask & (1 << last)) == 0 {
                continue;
            }

            let prev_mask = mask ^ (1 << last);
            if prev_mask == 0 {
                continue;
            }

            for prev in 0..n {
                if (prev_mask & (1 << prev)) == 0 {
                    continue;
                }

                let cost = dp[prev_mask][prev] + graph.get_distance(prev, last);
                if cost < dp[mask][last] {
                    dp[mask][last] = cost;
                    parent[mask][last] = Some(prev);
                }
            }
        }
    }

    // Find minimum cost to visit all cities and return to start
    let all_visited = num_subsets - 1;
    let mut min_cost = f64::INFINITY;
    let mut last_city = 0;

    for last in 1..n {
        let cost = dp[all_visited][last] + graph.get_distance(last, 0);
        if cost < min_cost {
            min_cost = cost;
            last_city = last;
        }
    }

    // Reconstruct tour
    let mut tour = vec![];
    let mut mask = all_visited;
    let mut current = last_city;

    while let Some(prev) = parent[mask][current] {
        tour.push(current);
        mask ^= 1 << current;
        current = prev;
    }
    tour.push(0);
    tour.reverse();

    TSPSolution {
        tour,
        distance: min_cost,
        algorithm: "Dynamic Programming (Held-Karp)".to_string(),
        time_ms: start.elapsed().as_secs_f64() * 1000.0,
    }
}

// 3. Nearest Neighbor Heuristic
fn tsp_nearest_neighbor(graph: &Graph) -> TSPSolution {
    let start = Instant::now();
    let n = graph.n;
    let mut visited = vec![false; n];
    let mut tour = vec![0];
    visited[0] = true;
    let mut current = 0;
    let mut total_distance = 0.0;

    for _ in 1..n {
        let mut nearest = None;
        let mut min_dist = f64::INFINITY;

        for next in 0..n {
            if !visited[next] {
                let dist = graph.get_distance(current, next);
                if dist < min_dist {
                    min_dist = dist;
                    nearest = Some(next);
                }
            }
        }

        if let Some(next) = nearest {
            tour.push(next);
            visited[next] = true;
            total_distance += min_dist;
            current = next;
        }
    }

    // Return to start
    total_distance += graph.get_distance(current, 0);

    TSPSolution {
        tour,
        distance: total_distance,
        algorithm: "Nearest Neighbor".to_string(),
        time_ms: start.elapsed().as_secs_f64() * 1000.0,
    }
}

// 4. 2-opt Local Search Improvement
fn tsp_two_opt(graph: &Graph, initial_tour: &[usize]) -> TSPSolution {
    let start = Instant::now();
    let n = graph.n;
    let mut tour = initial_tour.to_vec();
    let mut improved = true;
    let mut total_distance = graph.tour_distance(&tour);

    while improved {
        improved = false;

        for i in 0..n - 1 {
            for j in i + 2..n {
                // Don't reverse the entire tour
                if i == 0 && j == n - 1 {
                    continue;
                }

                // Calculate change in distance
                let current_dist = graph.get_distance(tour[i], tour[i + 1])
                    + graph.get_distance(tour[j], tour[(j + 1) % n]);
                let new_dist = graph.get_distance(tour[i], tour[j])
                    + graph.get_distance(tour[i + 1], tour[(j + 1) % n]);

                if new_dist < current_dist {
                    // Reverse the segment between i+1 and j
                    tour[i + 1..=j].reverse();
                    total_distance = total_distance - current_dist + new_dist;
                    improved = true;
                }
            }
        }
    }

    TSPSolution {
        tour,
        distance: total_distance,
        algorithm: "2-opt".to_string(),
        time_ms: start.elapsed().as_secs_f64() * 1000.0,
    }
}

// 5. Simulated Annealing
fn tsp_simulated_annealing(graph: &Graph, initial_tour: &[usize]) -> TSPSolution {
    let start = Instant::now();
    let n = graph.n;
    let mut current_tour = initial_tour.to_vec();
    let mut current_distance = graph.tour_distance(&current_tour);
    let mut best_tour = current_tour.clone();
    let mut best_distance = current_distance;

    let mut temperature = 100.0;
    let cooling_rate = 0.995;
    let min_temperature = 0.001;

    let mut rng = SimpleRng::new(42);

    while temperature > min_temperature {
        // Generate neighbor by swapping two random cities
        let i = 1 + (rng.next() % (n - 1) as u32) as usize;
        let j = 1 + (rng.next() % (n - 1) as u32) as usize;

        if i != j {
            current_tour.swap(i, j);
            let new_distance = graph.tour_distance(&current_tour);
            let delta = new_distance - current_distance;

            // Accept or reject the new solution
            if delta < 0.0 || rng.next_float() < (-delta / temperature).exp() {
                current_distance = new_distance;
                if current_distance < best_distance {
                    best_tour = current_tour.clone();
                    best_distance = current_distance;
                }
            } else {
                // Revert the swap
                current_tour.swap(i, j);
            }
        }

        temperature *= cooling_rate;
    }

    TSPSolution {
        tour: best_tour,
        distance: best_distance,
        algorithm: "Simulated Annealing".to_string(),
        time_ms: start.elapsed().as_secs_f64() * 1000.0,
    }
}

// Simple RNG for reproducible results
struct SimpleRng {
    seed: u32,
}

impl SimpleRng {
    fn new(seed: u32) -> Self {
        Self { seed }
    }

    fn next(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
        (self.seed / 65536) % 32768
    }

    fn next_float(&mut self) -> f64 {
        self.next() as f64 / 32768.0
    }
}

// Visualization helper
fn visualize_tour(tour: &[usize], points: &[(f64, f64)]) {
    println!("\nTour Visualization:");
    println!("{}", "=".repeat(50));

    // Simple ASCII art visualization for small instances
    if points.len() <= 10 {
        let scale = 20.0;
        let mut grid = vec![vec![' '; 40]; 20];

        // Plot cities
        for (i, &(x, y)) in points.iter().enumerate() {
            let gx = (x * scale) as usize;
            let gy = (y * scale) as usize;
            if gx < 40 && gy < 20 {
                grid[gy][gx] = (b'A' + i as u8) as char;
            }
        }

        // Print grid
        for row in grid {
            println!("{}", row.iter().collect::<String>());
        }
    }

    // Print tour
    print!("Tour: ");
    for (i, &city) in tour.iter().enumerate() {
        print!("{}", (b'A' + city as u8) as char);
        if i < tour.len() - 1 {
            print!(" → ");
        }
    }
    println!(" → {}", (b'A' + tour[0] as u8) as char);
}

fn run_test_case(name: &str, graph: &Graph, points: Option<&[(f64, f64)]>) {
    println!("\nTest Case: {}", name);
    println!("{}", "=".repeat(60));
    println!("Number of cities: {}", graph.n);

    // Run algorithms based on problem size
    let solutions = if graph.n <= 10 {
        vec![
            tsp_brute_force(graph),
            tsp_dynamic_programming(graph),
            tsp_nearest_neighbor(graph),
            {
                let nn_solution = tsp_nearest_neighbor(graph);
                tsp_two_opt(graph, &nn_solution.tour)
            },
            {
                let nn_solution = tsp_nearest_neighbor(graph);
                tsp_simulated_annealing(graph, &nn_solution.tour)
            },
        ]
    } else if graph.n <= 20 {
        vec![
            tsp_dynamic_programming(graph),
            tsp_nearest_neighbor(graph),
            {
                let nn_solution = tsp_nearest_neighbor(graph);
                tsp_two_opt(graph, &nn_solution.tour)
            },
            {
                let nn_solution = tsp_nearest_neighbor(graph);
                tsp_simulated_annealing(graph, &nn_solution.tour)
            },
        ]
    } else {
        vec![
            tsp_nearest_neighbor(graph),
            {
                let nn_solution = tsp_nearest_neighbor(graph);
                tsp_two_opt(graph, &nn_solution.tour)
            },
            {
                let nn_solution = tsp_nearest_neighbor(graph);
                tsp_simulated_annealing(graph, &nn_solution.tour)
            },
        ]
    };

    // Find optimal solution (if available)
    let optimal = solutions
        .iter()
        .filter(|s| s.algorithm.contains("Brute") || s.algorithm.contains("Dynamic"))
        .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    println!("\nAlgorithm Performance:");
    println!("{}", "-".repeat(70));
    println!(
        "{:<30} | {:>10} | {:>10} | {:>10}",
        "Algorithm", "Distance", "Time (ms)", "Quality"
    );
    println!("{}", "-".repeat(70));

    for solution in &solutions {
        let quality = if let Some(opt) = optimal {
            format!("{:.2}x", solution.distance / opt.distance)
        } else {
            "N/A".to_string()
        };

        println!(
            "{:<30} | {:>10.2} | {:>10.3} | {:>10}",
            solution.algorithm, solution.distance, solution.time_ms, quality
        );
    }

    // Show best tour
    if let Some(best) = solutions
        .iter()
        .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
    {
        println!(
            "\nBest Solution: {} (distance: {:.2})",
            best.algorithm, best.distance
        );
        if let Some(pts) = points {
            visualize_tour(&best.tour, pts);
        }
    }
}

fn main() {
    println!("Traveling Salesman Problem - Multiple Algorithm Implementation");
    println!("{}", "=".repeat(70));

    // Test Case 1: Small triangle
    let triangle_points = vec![(0.0, 0.0), (1.0, 0.0), (0.5, 0.866)];
    let triangle = Graph::from_points(&triangle_points);
    run_test_case("Triangle (3 cities)", &triangle, Some(&triangle_points));

    // Test Case 2: Square
    let square_points = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
    let square = Graph::from_points(&square_points);
    run_test_case("Square (4 cities)", &square, Some(&square_points));

    // Test Case 3: Pentagon
    let pentagon_points: Vec<(f64, f64)> = (0..5)
        .map(|i| {
            let angle = 2.0 * std::f64::consts::PI * i as f64 / 5.0;
            (angle.cos(), angle.sin())
        })
        .collect();
    let pentagon = Graph::from_points(&pentagon_points);
    run_test_case(
        "Regular Pentagon (5 cities)",
        &pentagon,
        Some(&pentagon_points),
    );

    // Test Case 4: Random 10 cities
    let mut rng = SimpleRng::new(12345);
    let random10_points: Vec<(f64, f64)> = (0..10)
        .map(|_| (rng.next_float(), rng.next_float()))
        .collect();
    let random10 = Graph::from_points(&random10_points);
    run_test_case("Random 10 cities", &random10, None);

    // Test Case 5: Random 20 cities (DP limit)
    let random20_points: Vec<(f64, f64)> = (0..20)
        .map(|_| (rng.next_float(), rng.next_float()))
        .collect();
    let random20 = Graph::from_points(&random20_points);
    run_test_case("Random 20 cities", &random20, None);

    // Test Case 6: Large random (heuristics only)
    let random50_points: Vec<(f64, f64)> = (0..50)
        .map(|_| (rng.next_float(), rng.next_float()))
        .collect();
    let random50 = Graph::from_points(&random50_points);
    run_test_case("Random 50 cities (heuristics only)", &random50, None);

    // Algorithm complexity summary
    println!("\n\nAlgorithm Complexity Summary:");
    println!("{}", "=".repeat(70));
    println!("Brute Force:          O(n!) time, O(n) space");
    println!("Dynamic Programming:  O(n²·2ⁿ) time, O(n·2ⁿ) space");
    println!("Nearest Neighbor:     O(n²) time, O(n) space");
    println!("2-opt:               O(n²) per iteration, O(n) space");
    println!("Simulated Annealing: O(iterations) time, O(n) space");
    println!("\nPractical Limits:");
    println!("- Brute Force: ≤10 cities");
    println!("- Dynamic Programming: ≤20 cities");
    println!("- Heuristics: Thousands of cities");
}
