#!/usr/bin/env python3
"""
Fibonacci Sequence - Python Implementation
Pure recursive for fair comparison
"""

def fibonacci(n: int) -> int:
    """
    Pure recursive Fibonacci
    Time: O(2^n), Space: O(n)
    """
    if n <= 1:
        return n
    else:
        return fibonacci(n - 1) + fibonacci(n - 2)


def test_fibonacci():
    """Test Fibonacci with known values"""
    assert fibonacci(0) == 0
    assert fibonacci(1) == 1
    assert fibonacci(2) == 1
    assert fibonacci(3) == 2
    assert fibonacci(5) == 5
    assert fibonacci(10) == 55
    assert fibonacci(20) == 6765
    print("All tests passed!")


def main():
    """Demonstrate Fibonacci sequence"""
    test_fibonacci()
    
    print("Fibonacci Sequence (Python):")
    for i in range(10):
        print(f"F({i}) = {fibonacci(i)}")


if __name__ == "__main__":
    main()