# Topological Sort

## Problem Statement

Topological sorting is a linear ordering of vertices in a directed acyclic graph (DAG) such that for every directed edge (u, v), vertex u comes before vertex v in the ordering. It's fundamental for dependency resolution, task scheduling, and determining compilation order.

## Complexity Analysis

- **DFS-based (Recursive)**: O(V + E) time, O(V) space
- **DFS-based (Iterative)**: O(V + E) time, O(V) space  
- **Kahn's Algorithm (BFS)**: O(V + E) time, O(V) space
- **All algorithms are optimal**: Linear in the size of the graph

## Algorithm Variants

### 1. DFS-based (Recursive)
- Perform DFS traversal
- Add vertices to stack when finishing processing
- Pop from stack to get topological order
- Natural and intuitive implementation

### 2. DFS-based (Iterative)
- Use explicit stack instead of recursion
- Same logic as recursive version
- Avoids stack overflow for deep graphs

### 3. Kahn's Algorithm (BFS-based)
- Start with vertices having in-degree 0
- Remove vertices and update in-degrees
- Add newly zero in-degree vertices to queue
- Better for detecting cycles during processing

## Applications

1. **Build Systems**: Determining compilation order
2. **Package Managers**: Resolving dependencies
3. **Task Scheduling**: Ordering tasks with prerequisites
4. **Course Prerequisites**: Academic course planning
5. **Spreadsheet Formulas**: Cell calculation order
6. **Unix Make**: Target dependency resolution

## Test Cases

1. **Simple Linear Chain**
   - A → B → C → D
   - Unique topological ordering

2. **Diamond DAG**
   - Multiple valid orderings
   - Tests algorithm consistency

3. **Complex DAG**
   - Multiple sources and sinks
   - Real-world dependency structure

4. **Single Vertex**
   - Edge case handling

5. **Disconnected Components**
   - Multiple independent DAGs

6. **Cycle Detection**
   - Should detect and report cycles
   - Not a valid DAG for topological sort

## Properties Tested

- **Correctness**: All dependencies respected
- **Completeness**: All vertices included
- **Cycle Detection**: Identifies invalid graphs
- **Multiple Solutions**: Different valid orderings
- **Performance**: Linear time complexity verified

## Expected Outputs

- Valid topological ordering(s)
- Cycle detection for invalid graphs
- Performance metrics
- Visualization of dependencies
- Comparison between algorithms