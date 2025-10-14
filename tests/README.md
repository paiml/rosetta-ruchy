# Rosetta Ruchy Test Suite

**Test-Driven Development (TDD) Infrastructure**

This directory contains comprehensive tests for all Ruchy implementations across the project, following the ruchy-book pattern of test-first development.

## Directory Structure

```
tests/
├── algorithms/          # Algorithm-specific tests
│   ├── fibonacci/
│   ├── quicksort/
│   ├── dijkstra/
│   └── ...
├── data-science/        # Data science algorithm tests
│   ├── dataframe-ops/
│   ├── statistical-analysis/
│   └── ...
├── advanced-ai/         # Advanced AI tests
│   ├── deep-learning/
│   └── ...
├── integration/         # End-to-end integration tests
├── property-based/      # Property-based tests
└── test-results/        # Test execution results

## Testing Philosophy

### EXTREME TDD Approach

1. **Test FIRST** - Write tests before implementation
2. **Verify ALWAYS** - Every example has comprehensive tests
3. **Automate EVERYTHING** - CI/CD runs tests on every commit
4. **Measure CONTINUOUSLY** - Track coverage, success rates, quality scores

### Test Categories

#### 1. Unit Tests (per algorithm)
```ruchy
// tests/algorithms/fibonacci/test_fibonacci.ruchy
fun test_fibonacci_base_cases() {
    assert_eq!(fibonacci(0), 0);
    assert_eq!(fibonacci(1), 1);
}

fun test_fibonacci_iterative() {
    assert_eq!(fibonacci_iterative(10), 55);
    assert_eq!(fibonacci_iterative(20), 6765);
}
```

#### 2. Property-Based Tests
```ruchy
// tests/property-based/test_sorting_properties.ruchy
property test_sort_idempotent(arr: Vec<i32>) {
    let sorted_once = quicksort(arr.clone());
    let sorted_twice = quicksort(sorted_once.clone());
    assert_eq!(sorted_once, sorted_twice);
}

property test_sort_preserves_length(arr: Vec<i32>) {
    let sorted = quicksort(arr.clone());
    assert_eq!(arr.len(), sorted.len());
}
```

#### 3. Integration Tests
```ruchy
// tests/integration/test_full_workflow.ruchy
fun test_algorithm_pipeline() {
    // Test complete workflow: input → algorithm → verification
    let input = generate_test_data();
    let result = run_algorithm(input);
    verify_correctness(result);
    verify_performance(result);
}
```

#### 4. Formal Verification Tests
```ruchy
// Automatically generated from ruchy provability
fun test_formal_verification() {
    // Verify provability score ≥ 90/100
    // Verify quality score ≥ 0.90/1.0
    // Verify complexity analysis
}
```

## Running Tests

### All Tests
```bash
make test-all           # Run complete test suite
make test-coverage      # Run with coverage analysis
make test-parallel      # Run tests in parallel
```

### By Category
```bash
make test-algorithms    # Algorithm tests only
make test-data-science  # Data science tests only
make test-advanced-ai   # Advanced AI tests only
```

### Individual Tests
```bash
ruchy test tests/algorithms/fibonacci/test_fibonacci.ruchy
make test-file FILE=tests/algorithms/quicksort/test_quicksort.ruchy
```

## Coverage Requirements

**Minimum Coverage**: 85% for all passing examples

```bash
make test-coverage
# Output:
# algorithms: 89% coverage (85/124 passing)
# data-science: 92% coverage (35/40 passing)
# advanced-ai: 67% coverage (4/6 passing)
# Overall: 87% coverage (124/170 passing)
```

## Quality Gates

All tests must pass these gates:

1. ✅ **Syntax Validation** - `ruchy check` passes
2. ✅ **Formal Verification** - Provability ≥ 90/100
3. ✅ **Quality Score** - Quality ≥ 0.90/1.0 (A-)
4. ✅ **Complexity** - All functions ≤ 20 complexity
5. ✅ **Coverage** - ≥ 85% test coverage

## CI/CD Integration

Tests run automatically on:
- Every commit (quality-gates.yml)
- Every PR (regression-check.yml)
- Nightly (nightly-tests.yml)

See `.github/workflows/` for details.

## Test Results

Results are tracked in:
- `test-results.json` - Machine-readable test results
- `INTEGRATION.md` - Human-readable status report
- `reports/dashboard.html` - Visual dashboard

## Writing New Tests

### Template for Algorithm Tests

```ruchy
// tests/algorithms/ALGORITHM_NAME/test_ALGORITHM_NAME.ruchy

// Test 1: Base cases
fun test_base_cases() {
    // TODO: Test edge cases
}

// Test 2: Known inputs/outputs
fun test_known_values() {
    // TODO: Test against known correct outputs
}

// Test 3: Properties (idempotence, commutativity, etc.)
fun test_properties() {
    // TODO: Test mathematical properties
}

// Test 4: Performance bounds
fun test_performance() {
    // TODO: Verify BigO complexity matches theoretical
}

// Test 5: Formal verification
fun test_verification() {
    // TODO: Run ruchy provability and score
}

fun main() {
    println!("Running ALGORITHM_NAME tests...");
    test_base_cases();
    test_known_values();
    test_properties();
    test_performance();
    test_verification();
    println!("✅ All tests passed!");
}
```

## Best Practices

1. **One test file per algorithm** - Keep tests focused
2. **Descriptive test names** - `test_fibonacci_negative_input_fails()`
3. **Use assertions** - `assert_eq!`, `assert!(condition)`
4. **Test edge cases first** - Empty inputs, boundary values, negative cases
5. **Property-based where possible** - Generative testing for comprehensive coverage
6. **Formal verification mandatory** - Every algorithm must have provability tests

## Contributing

All new algorithms MUST include comprehensive tests:

```bash
# 1. Write tests first
vim tests/algorithms/new-algorithm/test_new_algorithm.ruchy

# 2. Run tests (they should fail - RED)
ruchy test tests/algorithms/new-algorithm/test_new_algorithm.ruchy

# 3. Implement algorithm
vim examples/algorithms/new-algorithm/implementations/ruchy/new_algorithm.ruchy

# 4. Run tests again (they should pass - GREEN)
ruchy test tests/algorithms/new-algorithm/test_new_algorithm.ruchy

# 5. Refactor and optimize
# 6. Verify formal properties
ruchy provability new_algorithm.ruchy
ruchy score new_algorithm.ruchy

# 7. Update test results
make test-all-examples
make update-integration
```

## Test Statistics

**Current Status** (Auto-updated from test-results.json):
- Total Tests: 170 examples
- Passing: 124 (72.9%)
- Coverage: 87% (estimated)
- Quality Score: A- average (0.90+)

---

**Philosophy**: *"Test first, verify always, automate everything"*

**Inspired by**: [ruchy-book](https://github.com/paiml/ruchy-book) test-driven documentation approach
