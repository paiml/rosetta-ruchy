package main

import (
	"fmt"
	"math/rand"
	"runtime"
	"sort"
	"sync"
	"time"
)

// Sortable interface for generic sorting
type Sortable interface {
	Sort()
	IsSorted() bool
	Len() int
	Less(i, j int) bool
	Swap(i, j int)
}

// IntSlice implements Sortable for []int
type IntSlice []int

func (s IntSlice) Sort() {
	QuicksortInplace(s)
}

func (s IntSlice) IsSorted() bool {
	for i := 1; i < len(s); i++ {
		if s[i-1] > s[i] {
			return false
		}
	}
	return true
}

func (s IntSlice) Len() int           { return len(s) }
func (s IntSlice) Less(i, j int) bool { return s[i] < s[j] }
func (s IntSlice) Swap(i, j int)      { s[i], s[j] = s[j], s[i] }

// QuicksortInplace sorts the slice in-place
func QuicksortInplace(arr []int) {
	if len(arr) > 1 {
		quicksortRange(arr, 0, len(arr)-1)
	}
}

func quicksortRange(arr []int, low, high int) {
	if low < high {
		pivotIndex := partition(arr, low, high)
		
		if pivotIndex > 0 {
			quicksortRange(arr, low, pivotIndex-1)
		}
		if pivotIndex+1 <= high {
			quicksortRange(arr, pivotIndex+1, high)
		}
	}
}

func partition(arr []int, low, high int) int {
	pivot := arr[high]
	i := low
	
	for j := low; j < high; j++ {
		if arr[j] <= pivot {
			arr[i], arr[j] = arr[j], arr[i]
			i++
		}
	}
	
	arr[i], arr[high] = arr[high], arr[i]
	return i
}

// QuicksortFunctional returns a new sorted slice
func QuicksortFunctional(arr []int) []int {
	if len(arr) <= 1 {
		result := make([]int, len(arr))
		copy(result, arr)
		return result
	}
	
	pivot := arr[len(arr)/2]
	var less, equal, greater []int
	
	for _, x := range arr {
		if x < pivot {
			less = append(less, x)
		} else if x == pivot {
			equal = append(equal, x)
		} else {
			greater = append(greater, x)
		}
	}
	
	sortedLess := QuicksortFunctional(less)
	sortedGreater := QuicksortFunctional(greater)
	
	result := make([]int, 0, len(arr))
	result = append(result, sortedLess...)
	result = append(result, equal...)
	result = append(result, sortedGreater...)
	
	return result
}

// QuicksortThreeWay sorts using three-way partitioning
func QuicksortThreeWay(arr []int) {
	if len(arr) <= 1 {
		return
	}
	threeWayPartitionSort(arr, 0, len(arr)-1)
}

func threeWayPartitionSort(arr []int, low, high int) {
	if low >= high {
		return
	}
	
	pivot := arr[low]
	lt := low
	gt := high
	i := low + 1
	
	for i <= gt {
		if arr[i] < pivot {
			arr[i], arr[lt] = arr[lt], arr[i]
			lt++
			i++
		} else if arr[i] > pivot {
			arr[i], arr[gt] = arr[gt], arr[i]
			gt--
		} else {
			i++
		}
	}
	
	if lt > 0 {
		threeWayPartitionSort(arr, low, lt-1)
	}
	threeWayPartitionSort(arr, gt+1, high)
}

// QuicksortParallel sorts using goroutines
func QuicksortParallel(arr []int, threshold int) []int {
	if len(arr) <= threshold {
		return QuicksortFunctional(arr)
	}
	
	if len(arr) <= 1 {
		result := make([]int, len(arr))
		copy(result, arr)
		return result
	}
	
	pivot := arr[len(arr)/2]
	var less, equal, greater []int
	
	for _, x := range arr {
		if x < pivot {
			less = append(less, x)
		} else if x == pivot {
			equal = append(equal, x)
		} else {
			greater = append(greater, x)
		}
	}
	
	var wg sync.WaitGroup
	var sortedLess, sortedGreater []int
	
	wg.Add(2)
	
	go func() {
		defer wg.Done()
		sortedLess = QuicksortParallel(less, threshold)
	}()
	
	go func() {
		defer wg.Done()
		sortedGreater = QuicksortParallel(greater, threshold)
	}()
	
	wg.Wait()
	
	result := make([]int, 0, len(arr))
	result = append(result, sortedLess...)
	result = append(result, equal...)
	result = append(result, sortedGreater...)
	
	return result
}

// Benchmark function
func benchmark(name string, fn func()) {
	start := time.Now()
	fn()
	duration := time.Since(start)
	fmt.Printf("  %s time: %v\n", name, duration)
}

func main() {
	testArrays := [][]int{
		{},
		{42},
		{3, 1, 4, 1, 5, 9, 2, 6},
		{5, 4, 3, 2, 1},
		{1, 2, 3, 4, 5},
		{5, 5, 5, 5, 5},
	}
	
	fmt.Println("Quicksort Implementations Demo\n")
	
	for i, arr := range testArrays {
		fmt.Printf("Test case %d: %v\n", i+1, arr)
		
		arr1 := make([]int, len(arr))
		copy(arr1, arr)
		QuicksortInplace(arr1)
		fmt.Printf("  In-place:    %v\n", arr1)
		
		arr2 := QuicksortFunctional(arr)
		fmt.Printf("  Functional:  %v\n", arr2)
		
		arr3 := make([]int, len(arr))
		copy(arr3, arr)
		QuicksortThreeWay(arr3)
		fmt.Printf("  Three-way:   %v\n", arr3)
		
		if len(arr) > 0 {
			arr4 := QuicksortParallel(arr, 1000)
			fmt.Printf("  Parallel:    %v\n", arr4)
		}
		
		fmt.Println()
	}
	
	// Performance demonstration
	fmt.Println("Performance demonstration with large array:")
	largeArray := make([]int, 10000)
	for i := range largeArray {
		largeArray[i] = (i*37 + 11) % 1000
	}
	
	fmt.Printf("  Array size: %d\n", len(largeArray))
	fmt.Printf("  Number of CPUs: %d\n", runtime.NumCPU())
	
	benchmark("In-place", func() {
		arrCopy := make([]int, len(largeArray))
		copy(arrCopy, largeArray)
		QuicksortInplace(arrCopy)
	})
	
	benchmark("Functional", func() {
		QuicksortFunctional(largeArray)
	})
	
	benchmark("Parallel", func() {
		QuicksortParallel(largeArray, 1000)
	})
	
	// Test Sortable interface
	fmt.Println("\nTesting Sortable interface:")
	sortableArray := IntSlice{3, 1, 4, 1, 5, 9, 2, 6}
	fmt.Printf("Before sorting: %v, IsSorted: %t\n", sortableArray, sortableArray.IsSorted())
	sortableArray.Sort()
	fmt.Printf("After sorting:  %v, IsSorted: %t\n", sortableArray, sortableArray.IsSorted())
}