#!/usr/bin/env python3
"""
Benchmark script for Python Fibonacci implementations
Outputs results in JSON format compatible with Rosetta Ruchy
"""

import json
import time
import statistics
from fibonacci import FibonacciBenchmark


def run_benchmark_suite():
    """Run comprehensive benchmark suite"""
    benchmark = FibonacciBenchmark()
    
    # Test configurations from spec.toml
    test_configs = [
        {'name': 'tiny', 'n': 5, 'expected': 5, 'variants': ['all']},
        {'name': 'small', 'n': 10, 'expected': 55, 'variants': ['all']},
        {'name': 'medium_recursive', 'n': 30, 'expected': 832040, 
         'variants': ['recursive', 'memoized', 'lru_cached']},
        {'name': 'medium_iterative', 'n': 40, 'expected': 102334155,
         'variants': ['iterative', 'memoized', 'lru_cached', 'matrix', 'tail_recursive']},
        {'name': 'large', 'n': 100, 'expected': 354224848179261915075,
         'variants': ['iterative', 'memoized', 'lru_cached', 'matrix']},
    ]
    
    results = {
        'language': 'python',
        'version': '3.x',
        'timestamp': time.time(),
        'variants': {}
    }
    
    for config in test_configs:
        test_name = config['name']
        n = config['n']
        expected = config['expected']
        
        # Determine which variants to test
        if config['variants'] == ['all']:
            variants_to_test = list(benchmark.implementations.keys())
            # Skip slow recursive for large n
            if n > 35:
                variants_to_test.remove('recursive')
        else:
            variants_to_test = config['variants']
        
        for variant in variants_to_test:
            if variant not in results['variants']:
                results['variants'][variant] = {'test_cases': {}}
            
            # Run multiple iterations for statistics
            times = []
            iterations = 10 if n < 30 else 5
            
            for _ in range(iterations):
                try:
                    result, duration_ns = benchmark.run(n, variant)
                    times.append(duration_ns)
                    correct = (result == expected)
                except (RecursionError, Exception) as e:
                    result = None
                    correct = False
                    times = [float('inf')]
                    break
            
            # Calculate statistics
            if times and times[0] != float('inf'):
                stats = {
                    'mean': statistics.mean(times),
                    'median': statistics.median(times),
                    'std_dev': statistics.stdev(times) if len(times) > 1 else 0,
                    'min': min(times),
                    'max': max(times),
                }
            else:
                stats = None
            
            results['variants'][variant]['test_cases'][test_name] = {
                'input': n,
                'output': result,
                'expected': expected,
                'correct': correct,
                'time_ns': int(statistics.median(times)) if times else None,
                'iterations': len(times),
                'statistics': stats
            }
    
    return results


def main():
    """Main benchmark runner"""
    print("Running Python Fibonacci benchmarks...")
    results = run_benchmark_suite()
    
    # Output JSON results
    print(json.dumps(results, indent=2))
    
    # Save to file
    with open('results/python_benchmark.json', 'w') as f:
        json.dump(results, f, indent=2)
    
    print("\nBenchmark complete. Results saved to results/python_benchmark.json")


if __name__ == '__main__':
    # Create results directory if it doesn't exist
    import os
    os.makedirs('results', exist_ok=True)
    
    main()