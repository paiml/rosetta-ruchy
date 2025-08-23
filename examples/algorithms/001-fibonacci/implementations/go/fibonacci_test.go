package main

import (
	"math/big"
	"testing"
)

// Test cases
var testCases = []struct {
	n        int
	expected uint64
}{
	{0, 0},
	{1, 1},
	{2, 1},
	{3, 2},
	{4, 3},
	{5, 5},
	{6, 8},
	{7, 13},
	{8, 21},
	{9, 34},
	{10, 55},
	{20, 6765},
	{30, 832040},
	{40, 102334155},
}

func TestFibRecursive(t *testing.T) {
	for _, tc := range testCases {
		if tc.n > 30 { // Skip slow recursive tests
			continue
		}
		result := FibRecursive(tc.n)
		if result != tc.expected {
			t.Errorf("FibRecursive(%d) = %d; want %d", tc.n, result, tc.expected)
		}
	}
}

func TestFibIterative(t *testing.T) {
	for _, tc := range testCases {
		result := FibIterative(tc.n)
		if result != tc.expected {
			t.Errorf("FibIterative(%d) = %d; want %d", tc.n, result, tc.expected)
		}
	}
}

func TestFibMemoized(t *testing.T) {
	for _, tc := range testCases {
		result := FibMemoized(tc.n)
		if result != tc.expected {
			t.Errorf("FibMemoized(%d) = %d; want %d", tc.n, result, tc.expected)
		}
	}
}

func TestFibMatrix(t *testing.T) {
	for _, tc := range testCases {
		result := FibMatrix(tc.n)
		if result != tc.expected {
			t.Errorf("FibMatrix(%d) = %d; want %d", tc.n, result, tc.expected)
		}
	}
}

func TestFibChannel(t *testing.T) {
	for _, tc := range testCases {
		result := FibChannel(tc.n)
		if result != tc.expected {
			t.Errorf("FibChannel(%d) = %d; want %d", tc.n, result, tc.expected)
		}
	}
}

func TestFibConcurrent(t *testing.T) {
	nums := []int{10, 20, 30}
	expected := []uint64{55, 6765, 832040}
	
	results := FibConcurrent(nums)
	
	for i, result := range results {
		if result != expected[i] {
			t.Errorf("FibConcurrent(%d) = %d; want %d", nums[i], result, expected[i])
		}
	}
}

func TestFibTailRecursive(t *testing.T) {
	for _, tc := range testCases {
		result := FibTailRecursive(tc.n)
		if result != tc.expected {
			t.Errorf("FibTailRecursive(%d) = %d; want %d", tc.n, result, tc.expected)
		}
	}
}

func TestFibBigInt(t *testing.T) {
	// Test small numbers
	for _, tc := range testCases {
		result := FibBigInt(tc.n)
		expected := big.NewInt(int64(tc.expected))
		if result.Cmp(expected) != 0 {
			t.Errorf("FibBigInt(%d) = %s; want %s", tc.n, result.String(), expected.String())
		}
	}
	
	// Test large number
	result100 := FibBigInt(100)
	expected100, _ := new(big.Int).SetString("354224848179261915075", 10)
	if result100.Cmp(expected100) != 0 {
		t.Errorf("FibBigInt(100) = %s; want %s", result100.String(), expected100.String())
	}
}

// Benchmarks
func BenchmarkFibRecursive30(b *testing.B) {
	for i := 0; i < b.N; i++ {
		FibRecursive(30)
	}
}

func BenchmarkFibIterative40(b *testing.B) {
	for i := 0; i < b.N; i++ {
		FibIterative(40)
	}
}

func BenchmarkFibIterative100(b *testing.B) {
	for i := 0; i < b.N; i++ {
		FibIterative(100)
	}
}

func BenchmarkFibMemoized40(b *testing.B) {
	for i := 0; i < b.N; i++ {
		FibMemoized(40)
	}
}

func BenchmarkFibMatrix40(b *testing.B) {
	for i := 0; i < b.N; i++ {
		FibMatrix(40)
	}
}

func BenchmarkFibMatrix100(b *testing.B) {
	for i := 0; i < b.N; i++ {
		FibMatrix(100)
	}
}

func BenchmarkFibChannel40(b *testing.B) {
	for i := 0; i < b.N; i++ {
		FibChannel(40)
	}
}

func BenchmarkFibTailRecursive40(b *testing.B) {
	for i := 0; i < b.N; i++ {
		FibTailRecursive(40)
	}
}

func BenchmarkFibBigInt100(b *testing.B) {
	for i := 0; i < b.N; i++ {
		FibBigInt(100)
	}
}

func BenchmarkFibBigInt1000(b *testing.B) {
	for i := 0; i < b.N; i++ {
		FibBigInt(1000)
	}
}