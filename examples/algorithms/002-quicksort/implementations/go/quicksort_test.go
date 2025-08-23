package main

import (
	"math/rand"
	"sort"
	"testing"
	"time"
)

func TestQuicksortEmpty(t *testing.T) {
	arr := []int{}
	QuicksortInplace(arr)
	if len(arr) != 0 {
		t.Errorf("Expected empty array, got %v", arr)
	}
}

func TestQuicksortSingle(t *testing.T) {
	arr := []int{42}
	QuicksortInplace(arr)
	expected := []int{42}
	if !equal(arr, expected) {
		t.Errorf("Expected %v, got %v", expected, arr)
	}
}

func TestQuicksortSorted(t *testing.T) {
	arr := []int{1, 2, 3, 4, 5}
	QuicksortInplace(arr)
	expected := []int{1, 2, 3, 4, 5}
	if !equal(arr, expected) {
		t.Errorf("Expected %v, got %v", expected, arr)
	}
}

func TestQuicksortReverse(t *testing.T) {
	arr := []int{5, 4, 3, 2, 1}
	QuicksortInplace(arr)
	expected := []int{1, 2, 3, 4, 5}
	if !equal(arr, expected) {
		t.Errorf("Expected %v, got %v", expected, arr)
	}
}

func TestQuicksortRandom(t *testing.T) {
	arr := []int{3, 1, 4, 1, 5, 9, 2, 6}
	QuicksortInplace(arr)
	expected := []int{1, 1, 2, 3, 4, 5, 6, 9}
	if !equal(arr, expected) {
		t.Errorf("Expected %v, got %v", expected, arr)
	}
}

func TestQuicksortDuplicates(t *testing.T) {
	arr := []int{5, 5, 5, 5, 5}
	QuicksortInplace(arr)
	expected := []int{5, 5, 5, 5, 5}
	if !equal(arr, expected) {
		t.Errorf("Expected %v, got %v", expected, arr)
	}
}

func TestQuicksortNegative(t *testing.T) {
	arr := []int{3, -1, 4, -1, 5, 9, -2, 6}
	QuicksortInplace(arr)
	expected := []int{-2, -1, -1, 3, 4, 5, 6, 9}
	if !equal(arr, expected) {
		t.Errorf("Expected %v, got %v", expected, arr)
	}
}

func TestQuicksortFunctional(t *testing.T) {
	arr := []int{3, 1, 4, 1, 5, 9, 2, 6}
	original := make([]int, len(arr))
	copy(original, arr)
	
	sorted := QuicksortFunctional(arr)
	expected := []int{1, 1, 2, 3, 4, 5, 6, 9}
	
	if !equal(sorted, expected) {
		t.Errorf("Expected %v, got %v", expected, sorted)
	}
	
	// Original should be unchanged
	if !equal(arr, original) {
		t.Errorf("Original array was modified: expected %v, got %v", original, arr)
	}
}

func TestQuicksortThreeWay(t *testing.T) {
	arr := []int{3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5}
	QuicksortThreeWay(arr)
	expected := []int{1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9}
	if !equal(arr, expected) {
		t.Errorf("Expected %v, got %v", expected, arr)
	}
}

func TestQuicksortParallel(t *testing.T) {
	arr := []int{3, 1, 4, 1, 5, 9, 2, 6}
	sorted := QuicksortParallel(arr, 1000)
	expected := []int{1, 1, 2, 3, 4, 5, 6, 9}
	if !equal(sorted, expected) {
		t.Errorf("Expected %v, got %v", expected, sorted)
	}
}

func TestSortableInterface(t *testing.T) {
	sortable := IntSlice{3, 1, 4, 1, 5, 9, 2, 6}
	if sortable.IsSorted() {
		t.Error("Array should not be sorted initially")
	}
	
	sortable.Sort()
	if !sortable.IsSorted() {
		t.Error("Array should be sorted after Sort()")
	}
	
	expected := IntSlice{1, 1, 2, 3, 4, 5, 6, 9}
	if !equal(sortable, expected) {
		t.Errorf("Expected %v, got %v", expected, sortable)
	}
}

// Property-based tests
func TestPropertySortedHasSameElements(t *testing.T) {
	rand.Seed(42)
	for i := 0; i < 1000; i++ {
		size := rand.Intn(50)
		arr := make([]int, size)
		for j := range arr {
			arr[j] = rand.Intn(200) - 100
		}
		
		sorted := make([]int, len(arr))
		copy(sorted, arr)
		QuicksortInplace(sorted)
		
		origSorted := make([]int, len(arr))
		copy(origSorted, arr)
		sort.Ints(origSorted)
		
		if !equal(sorted, origSorted) {
			t.Errorf("Iteration %d: quicksort result differs from sort.Ints", i)
			t.Errorf("Original: %v", arr)
			t.Errorf("Quicksort: %v", sorted)
			t.Errorf("sort.Ints: %v", origSorted)
		}
	}
}

func TestPropertySortedIsOrdered(t *testing.T) {
	rand.Seed(42)
	for i := 0; i < 1000; i++ {
		size := rand.Intn(50)
		arr := make([]int, size)
		for j := range arr {
			arr[j] = rand.Intn(200) - 100
		}
		
		QuicksortInplace(arr)
		
		for j := 0; j < len(arr)-1; j++ {
			if arr[j] > arr[j+1] {
				t.Errorf("Iteration %d: array not sorted at indices %d, %d: %d > %d", 
					i, j, j+1, arr[j], arr[j+1])
				t.Errorf("Full array: %v", arr)
				break
			}
		}
	}
}

func TestImplementationConsistency(t *testing.T) {
	rand.Seed(42)
	for i := 0; i < 100; i++ {
		size := rand.Intn(30)
		arr := make([]int, size)
		for j := range arr {
			arr[j] = rand.Intn(100) - 50
		}
		
		arr1 := make([]int, len(arr))
		copy(arr1, arr)
		QuicksortInplace(arr1)
		
		arr2 := QuicksortFunctional(arr)
		
		arr3 := make([]int, len(arr))
		copy(arr3, arr)
		QuicksortThreeWay(arr3)
		
		arr4 := QuicksortParallel(arr, 1000)
		
		if !equal(arr1, arr2) {
			t.Errorf("Iteration %d: in-place vs functional mismatch", i)
		}
		if !equal(arr2, arr3) {
			t.Errorf("Iteration %d: functional vs three-way mismatch", i)
		}
		if !equal(arr3, arr4) {
			t.Errorf("Iteration %d: three-way vs parallel mismatch", i)
		}
	}
}

// Benchmarks
func BenchmarkQuicksortInplace1K(b *testing.B) {
	arr := generateRandomArray(1000)
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		arrCopy := make([]int, len(arr))
		copy(arrCopy, arr)
		QuicksortInplace(arrCopy)
	}
}

func BenchmarkQuicksortInplace10K(b *testing.B) {
	arr := generateRandomArray(10000)
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		arrCopy := make([]int, len(arr))
		copy(arrCopy, arr)
		QuicksortInplace(arrCopy)
	}
}

func BenchmarkQuicksortFunctional1K(b *testing.B) {
	arr := generateRandomArray(1000)
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		QuicksortFunctional(arr)
	}
}

func BenchmarkQuicksortParallel10K(b *testing.B) {
	arr := generateRandomArray(10000)
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		QuicksortParallel(arr, 1000)
	}
}

func BenchmarkStandardLibrary10K(b *testing.B) {
	arr := generateRandomArray(10000)
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		arrCopy := make([]int, len(arr))
		copy(arrCopy, arr)
		sort.Ints(arrCopy)
	}
}

// Helper functions
func equal(a, b []int) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if a[i] != b[i] {
			return false
		}
	}
	return true
}

func generateRandomArray(size int) []int {
	rand.Seed(time.Now().UnixNano())
	arr := make([]int, size)
	for i := range arr {
		arr[i] = rand.Intn(2000) - 1000
	}
	return arr
}