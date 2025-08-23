package main

import (
	"fmt"
	"math/big"
	"os"
	"strconv"
	"sync"
	"time"
)

// FibRecursive computes Fibonacci recursively (exponential complexity)
func FibRecursive(n int) uint64 {
	if n <= 1 {
		return uint64(n)
	}
	return FibRecursive(n-1) + FibRecursive(n-2)
}

// FibIterative computes Fibonacci iteratively (linear complexity)
func FibIterative(n int) uint64 {
	if n <= 1 {
		return uint64(n)
	}
	
	var prev, curr uint64 = 0, 1
	for i := 2; i <= n; i++ {
		prev, curr = curr, prev+curr
	}
	
	return curr
}

// FibMemoized computes Fibonacci with memoization
func FibMemoized(n int) uint64 {
	cache := make(map[int]uint64)
	return fibMemoHelper(n, cache)
}

func fibMemoHelper(n int, cache map[int]uint64) uint64 {
	if val, ok := cache[n]; ok {
		return val
	}
	
	if n <= 1 {
		cache[n] = uint64(n)
		return uint64(n)
	}
	
	result := fibMemoHelper(n-1, cache) + fibMemoHelper(n-2, cache)
	cache[n] = result
	return result
}

// FibMatrix computes Fibonacci using matrix multiplication (logarithmic complexity)
func FibMatrix(n int) uint64 {
	if n == 0 {
		return 0
	}
	if n == 1 {
		return 1
	}
	
	type matrix [2][2]uint64
	
	mult := func(a, b matrix) matrix {
		return matrix{
			{a[0][0]*b[0][0] + a[0][1]*b[1][0], a[0][0]*b[0][1] + a[0][1]*b[1][1]},
			{a[1][0]*b[0][0] + a[1][1]*b[1][0], a[1][0]*b[0][1] + a[1][1]*b[1][1]},
		}
	}
	
	var pow func(mat matrix, p int) matrix
	pow = func(mat matrix, p int) matrix {
		if p == 1 {
			return mat
		}
		if p%2 == 0 {
			half := pow(mat, p/2)
			return mult(half, half)
		}
		return mult(mat, pow(mat, p-1))
	}
	
	base := matrix{{1, 1}, {1, 0}}
	result := pow(base, n)
	return result[0][1]
}

// FibChannel computes Fibonacci using Go channels (concurrency demonstration)
func FibChannel(n int) uint64 {
	if n <= 1 {
		return uint64(n)
	}
	
	ch := make(chan uint64, 2)
	
	go func() {
		ch <- FibIterative(n)
	}()
	
	return <-ch
}

// FibConcurrent computes multiple Fibonacci numbers concurrently
func FibConcurrent(nums []int) []uint64 {
	results := make([]uint64, len(nums))
	var wg sync.WaitGroup
	
	for i, n := range nums {
		wg.Add(1)
		go func(idx, num int) {
			defer wg.Done()
			results[idx] = FibIterative(num)
		}(i, n)
	}
	
	wg.Wait()
	return results
}

// FibWorkerPool computes Fibonacci using a worker pool pattern
func FibWorkerPool(nums []int, workers int) []uint64 {
	jobs := make(chan int, len(nums))
	results := make(chan struct {
		index int
		value uint64
	}, len(nums))
	
	// Start workers
	var wg sync.WaitGroup
	for w := 0; w < workers; w++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for n := range jobs {
				results <- struct {
					index int
					value uint64
				}{n, FibIterative(n)}
			}
		}()
	}
	
	// Send jobs
	for i, n := range nums {
		jobs <- n
		_ = i // index handling simplified for this example
	}
	close(jobs)
	
	// Wait for workers
	wg.Wait()
	close(results)
	
	// Collect results
	output := make([]uint64, len(nums))
	for r := range results {
		output[r.index] = r.value
	}
	
	return output
}

// FibBigInt computes Fibonacci for large numbers using big.Int
func FibBigInt(n int) *big.Int {
	if n <= 1 {
		return big.NewInt(int64(n))
	}
	
	prev := big.NewInt(0)
	curr := big.NewInt(1)
	
	for i := 2; i <= n; i++ {
		next := new(big.Int).Add(prev, curr)
		prev = curr
		curr = next
	}
	
	return curr
}

// FibTailRecursive computes Fibonacci tail-recursively
func FibTailRecursive(n int) uint64 {
	return fibTailHelper(n, 0, 1)
}

func fibTailHelper(n int, prev, curr uint64) uint64 {
	if n == 0 {
		return prev
	}
	return fibTailHelper(n-1, curr, prev+curr)
}

// Benchmark runs a Fibonacci function and returns execution time
func Benchmark(name string, n int, fn func(int) uint64) {
	start := time.Now()
	result := fn(n)
	duration := time.Since(start)
	
	fmt.Printf("%s: fib(%d) = %d, time = %v\n", name, n, result, duration)
}

// BenchmarkBig runs a big.Int Fibonacci function and returns execution time
func BenchmarkBig(name string, n int, fn func(int) *big.Int) {
	start := time.Now()
	result := fn(n)
	duration := time.Since(start)
	
	fmt.Printf("%s: fib(%d) = %s, time = %v\n", name, n, result.String(), duration)
}

func main() {
	if len(os.Args) < 2 {
		fmt.Printf("Usage: %s <n> [variant]\n", os.Args[0])
		fmt.Println("Variants: recursive, iterative, memoized, matrix, channel, concurrent, tail, bigint")
		os.Exit(1)
	}
	
	n, err := strconv.Atoi(os.Args[1])
	if err != nil {
		fmt.Fprintf(os.Stderr, "Invalid number: %s\n", os.Args[1])
		os.Exit(1)
	}
	
	variant := "iterative"
	if len(os.Args) > 2 {
		variant = os.Args[2]
	}
	
	switch variant {
	case "recursive":
		if n > 40 {
			fmt.Println("Warning: recursive is slow for n > 40")
		}
		Benchmark("Recursive", n, FibRecursive)
	case "iterative":
		Benchmark("Iterative", n, FibIterative)
	case "memoized":
		Benchmark("Memoized", n, FibMemoized)
	case "matrix":
		Benchmark("Matrix", n, FibMatrix)
	case "channel":
		Benchmark("Channel", n, FibChannel)
	case "concurrent":
		nums := make([]int, n+1)
		for i := range nums {
			nums[i] = i
		}
		start := time.Now()
		results := FibConcurrent(nums)
		duration := time.Since(start)
		fmt.Printf("Concurrent: fib(%d) = %d, time = %v\n", n, results[n], duration)
	case "tail":
		Benchmark("TailRecursive", n, FibTailRecursive)
	case "bigint":
		BenchmarkBig("BigInt", n, FibBigInt)
	default:
		fmt.Fprintf(os.Stderr, "Unknown variant: %s\n", variant)
		os.Exit(1)
	}
}