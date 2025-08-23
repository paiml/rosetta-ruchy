//! Criterion benchmarks for Fibonacci implementations

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use fibonacci_rust::*;

fn benchmark_recursive(c: &mut Criterion) {
    let mut group = c.benchmark_group("recursive");
    
    for n in [10, 20, 30].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(n), n, |b, &n| {
            b.iter(|| fib_recursive(black_box(n)));
        });
    }
    
    group.finish();
}

fn benchmark_iterative(c: &mut Criterion) {
    let mut group = c.benchmark_group("iterative");
    
    for n in [10, 40, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(n), n, |b, &n| {
            if *n <= 92 {
                b.iter(|| fib_iterative(black_box(*n)));
            } else {
                b.iter(|| fib_iterative_big(black_box(*n)));
            }
        });
    }
    
    group.finish();
}

fn benchmark_memoized(c: &mut Criterion) {
    let mut group = c.benchmark_group("memoized");
    
    for n in [10, 30, 40, 50].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(n), n, |b, &n| {
            b.iter(|| fib_memoized(black_box(*n)));
        });
    }
    
    group.finish();
}

fn benchmark_matrix(c: &mut Criterion) {
    let mut group = c.benchmark_group("matrix");
    
    for n in [10, 40, 100, 1000].iter() {
        if *n <= 92 {
            group.bench_with_input(BenchmarkId::from_parameter(n), n, |b, &n| {
                b.iter(|| fib_matrix(black_box(*n)));
            });
        }
    }
    
    group.finish();
}

fn benchmark_tail_recursive(c: &mut Criterion) {
    let mut group = c.benchmark_group("tail_recursive");
    
    for n in [10, 40, 100].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(n), n, |b, &n| {
            b.iter(|| fib_tail_recursive(black_box(*n)));
        });
    }
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_recursive,
    benchmark_iterative,
    benchmark_memoized,
    benchmark_matrix,
    benchmark_tail_recursive
);
criterion_main!(benches);