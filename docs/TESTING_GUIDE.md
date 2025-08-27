# Ruchy Testing Framework Guide

**Sprint 40**: Pure Ruchy Testing Infrastructure  
**Date**: 2025-08-27  
**Version**: v1.22.0

## Overview

This guide provides comprehensive testing patterns for Ruchy programs using the pure Ruchy testing framework. The framework eliminates external dependencies and showcases Ruchy's capabilities for meta-programming and tooling development.

## Testing Framework Architecture

### Core Components

| Component | File | Purpose | Status |
|-----------|------|---------|--------|
| Test Runner | `test_runner.ruchy` | Test execution engine | ⚠️ Syntax issues |
| Assertions | `assertions.ruchy` | Assertion library | ⚠️ Match syntax |
| Property Testing | `property_testing.ruchy` | Property-based testing | ✅ Implemented |
| Coverage Analysis | `coverage.ruchy` | Code coverage tracking | ✅ Implemented |
| Mutation Testing | `mutation.ruchy` | Test quality analysis | ✅ Implemented |

### Framework Features

#### 1. Pure Ruchy Implementation
- **No External Dependencies**: Uses only Ruchy standard library
- **Self-Contained**: All testing logic implemented in Ruchy
- **Demonstrative**: Showcases Ruchy's meta-programming capabilities

#### 2. Comprehensive Testing Types
- **Unit Testing**: Function-level testing with assertions
- **Property Testing**: QuickCheck-style property verification
- **Integration Testing**: Component interaction testing
- **Mutation Testing**: Test suite quality analysis
- **Coverage Analysis**: Line and branch coverage tracking

#### 3. Multiple Output Formats
- **Text Reports**: Human-readable console output
- **JSON Reports**: Machine-readable for CI/CD
- **XML Reports**: Compatible with test frameworks
- **HTML Reports**: Visual coverage reports

## Testing Patterns

### 1. Basic Unit Testing Pattern

```rust
// test_calculator.ruchy
fun test_addition() -> bool {
    let result = add(2, 3);
    Assert::assert_eq(result, 5)
}

fun test_subtraction() -> bool {
    let result = subtract(5, 3);
    Assert::assert_eq(result, 2)
}

fun run_calculator_tests() {
    let mut runner = TestRunner::new(TestConfig {
        parallel: false,
        verbose: true,
        coverage: true,
        mutation: false,
        output_format: vec![116, 101, 120, 116], // "text"
        max_workers: 1
    });
    
    runner.add_test("test_addition", test_addition);
    runner.add_test("test_subtraction", test_subtraction);
    
    let report = runner.run_all_tests();
    runner.generate_report(&report);
}
```

### 2. Property-Based Testing Pattern

```rust
// test_properties.ruchy
fun test_addition_commutative() -> bool {
    let mut tester = PropertyTester::new(PropertyConfig {
        max_iterations: 100,
        max_shrink_iterations: 10,
        seed: 42,
        verbose: false
    });
    
    // Property: a + b = b + a
    let property = |x: i32| -> bool {
        let y = (x * 7 + 13) % 100;
        add(x, y) == add(y, x)
    };
    
    let result = tester.test_property(property, Generator::IntRange(-100, 100));
    result.success
}

fun test_sorting_idempotent() -> bool {
    let mut tester = PropertyTester::new(PropertyConfig {
        max_iterations: 50,
        max_shrink_iterations: 5,
        seed: 123,
        verbose: false
    });
    
    // Property: sort(sort(x)) = sort(x)
    let property = |v: Vec<i32>| -> bool {
        let sorted_once = sort(v.clone());
        let sorted_twice = sort(sorted_once.clone());
        vectors_equal(&sorted_once, &sorted_twice)
    };
    
    let result = tester.test_vector_property(property, Generator::VectorOfInts(10, 0, 100));
    result.success
}
```

### 3. Coverage-Driven Testing Pattern

```rust
// test_with_coverage.ruchy
fun run_comprehensive_tests() {
    let test_files = vec![
        vec![109, 97, 105, 110, 46, 114, 117, 99, 104, 121],    // "main.ruchy"
        vec![108, 105, 98, 46, 114, 117, 99, 104, 121],       // "lib.ruchy"
        vec![117, 116, 105, 108, 115, 46, 114, 117, 99, 104, 121] // "utils.ruchy"
    ];
    
    // Run tests with coverage analysis
    let report = TestCoverageIntegration::run_tests_with_coverage(test_files);
    
    // Check coverage thresholds
    let threshold_met = TestCoverageIntegration::check_coverage_thresholds(&report, 8000); // 80%
    
    if !threshold_met {
        println("⚠️ Coverage below threshold - add more tests");
        // Identify uncovered lines and add tests
    }
}
```

### 4. Mutation Testing Pattern

```rust
// test_quality_analysis.ruchy
fun analyze_test_quality() {
    let mut tester = MutationTester::new();
    
    // Add source files for mutation testing
    tester.add_source_file(vec![109, 97, 105, 110, 46, 114, 117, 99, 104, 121]);
    tester.add_source_file(vec![108, 105, 98, 46, 114, 117, 99, 104, 121]);
    
    // Run mutation testing
    let report = tester.run_mutation_testing();
    
    // Generate detailed report
    tester.generate_report(&report);
    
    // Check quality threshold
    if report.mutation_score < 8000 {  // 80%
        println("❌ Test suite quality needs improvement");
        // Show survived mutants for targeted test improvements
    }
}
```

## Assertion Patterns

### 1. Basic Assertions

```rust
// Equality and inequality
Assert::assert_eq(actual, expected);
Assert::assert_ne(actual, not_expected);

// Comparisons
Assert::assert_gt(larger, smaller);
Assert::assert_lt(smaller, larger);
Assert::assert_ge(value, minimum);
Assert::assert_le(value, maximum);

// Boolean checks
Assert::assert_true(condition);
Assert::assert_false(!condition);

// Range checks
Assert::assert_in_range(value, min, max);
```

### 2. Vector Assertions

```rust
// Vector equality
let expected = vec![1, 2, 3];
let actual = process_data();
Assert::assert_vec_eq(actual, expected);

// Vector contains
let haystack = vec![1, 2, 3, 4, 5];
Assert::assert_vec_contains(haystack, 3);

// Vector empty
let empty_result = vec![];
Assert::assert_vec_empty(empty_result);
```

### 3. Approximation Assertions (Fixed-Point)

```rust
// Approximate equality for scaled integers
let expected = 1000;  // Represents 1.000
let actual = 1005;    // Represents 1.005
let tolerance = 10;   // 0.01 tolerance
Assert::assert_approx_eq(actual, expected, tolerance);
```

### 4. Custom Assertions

```rust
fun assert_sorted(v: &Vec<i32>) -> AssertResult {
    for i in 0..(v.len() - 1) {
        if let (Some(a), Some(b)) = (v.get(i), v.get(i + 1)) {
            if *a > *b {
                let message = vec![86, 101, 99, 116, 111, 114, 32, 110, 111, 116, 32, 115, 111, 114, 116, 101, 100]; // "Vector not sorted"
                return AssertResult::Failure(message);
            }
        }
    }
    AssertResult::Success
}
```

## Test Organization

### 1. File Naming Conventions

```
project/
├── src/
│   ├── main.ruchy           # Main application code
│   ├── lib.ruchy            # Library functions
│   └── utils.ruchy          # Utility functions
└── tests/
    ├── test_main.ruchy      # Tests for main.ruchy
    ├── test_lib.ruchy       # Tests for lib.ruchy
    ├── test_utils.ruchy     # Tests for utils.ruchy
    ├── test_integration.ruchy # Integration tests
    └── test_properties.ruchy  # Property-based tests
```

### 2. Test Function Naming

```rust
// Unit tests
fun test_addition() -> bool { }
fun test_subtraction() -> bool { }
fun test_multiplication() -> bool { }

// Property tests
fun property_addition_commutative() -> bool { }
fun property_multiplication_associative() -> bool { }

// Integration tests
fun integration_workflow_complete() -> bool { }
fun integration_error_handling() -> bool { }
```

### 3. Test Setup and Teardown

```rust
struct TestEnvironment {
    setup_data: Vec<i32>,
    temp_files: Vec<Vec<i32>>
}

impl TestEnvironment {
    fun setup() -> TestEnvironment {
        println("🔧 Setting up test environment");
        // Initialize test data
        TestEnvironment {
            setup_data: vec![1, 2, 3, 4, 5],
            temp_files: vec![]
        }
    }
    
    fun teardown(&mut self) {
        println("🧹 Cleaning up test environment");
        // Clean up resources
        self.temp_files.clear();
    }
}
```

## Integration with Ruchy Tools

### 1. Using ruchy test Command

```bash
# Basic test execution
ruchy test test_main.ruchy

# Verbose output
ruchy test test_main.ruchy --verbose

# Coverage analysis
ruchy test test_main.ruchy --coverage

# Property testing with more iterations
ruchy test test_properties.ruchy --property-iterations 1000
```

### 2. Quality Gates Integration

```makefile
# Makefile integration
test-quality:
    @echo "=== TESTING QUALITY PIPELINE ==="
    ruchy test tests/*.ruchy --coverage --min-coverage 80
    ruchy test tests/*.ruchy --mutation --min-mutation-score 75
    ruchy score tests/*.ruchy --min 0.85

# CI/CD integration
test-ci:
    ruchy test tests/*.ruchy --format json > test-results.json
    ruchy test tests/*.ruchy --coverage --format xml > coverage.xml
```

### 3. Continuous Testing

```rust
// test_monitor.ruchy - Continuous test runner
fun monitor_continuous_testing() {
    let mut last_run_time = 0;
    
    loop {
        // Check for file changes
        if source_files_changed(last_run_time) {
            println("🔄 Source files changed, running tests...");
            
            // Run full test suite
            run_all_tests();
            
            last_run_time = get_current_time();
        }
        
        // Wait before next check
        sleep_ms(1000);
    }
}
```

## Test Debugging Patterns

### 1. Verbose Test Output

```rust
let config = TestConfig {
    parallel: false,
    verbose: true,     // Enable detailed output
    coverage: true,
    mutation: false,
    output_format: vec![116, 101, 120, 116],
    max_workers: 1
};
```

### 2. Isolated Test Execution

```rust
// Run single test in isolation
fun debug_specific_test() {
    let test_name = vec![116, 101, 115, 116, 95, 97, 100, 100]; // "test_add"
    
    println("🔍 Debugging specific test:");
    print_vec(&test_name);
    println("");
    
    let result = run_single_test_isolated(test_name);
    if !result.success {
        print_debug_info(&result);
    }
}
```

### 3. Test Data Generation Debug

```rust
// Debug property test data generation
fun debug_property_generation() {
    let mut tester = PropertyTester::new(PropertyConfig {
        max_iterations: 10,
        max_shrink_iterations: 5,
        seed: 12345,
        verbose: true      // Show generated inputs
    });
    
    // This will show all generated test inputs
    let property = |x: i32| -> bool { true };
    tester.test_property(property, Generator::IntRange(-100, 100));
}
```

## Performance Optimization

### 1. Parallel Test Execution

```rust
let config = TestConfig {
    parallel: true,      // Enable parallel execution
    verbose: false,      // Reduce output for performance
    coverage: false,     // Disable coverage for speed
    mutation: false,
    output_format: vec![106, 115, 111, 110], // "json" for faster parsing
    max_workers: 4       // Use 4 worker threads
};
```

### 2. Test Selection and Filtering

```rust
// Only run failing tests
fun run_failing_tests_only() {
    let previous_results = load_previous_test_results();
    let failing_tests = get_failing_tests(&previous_results);
    
    for test_name in failing_tests.iter() {
        run_single_test(test_name);
    }
}

// Run tests affected by changes
fun run_affected_tests(changed_files: Vec<Vec<i32>>) {
    let affected_tests = analyze_test_dependencies(changed_files);
    
    for test in affected_tests.iter() {
        run_single_test(test);
    }
}
```

### 3. Caching and Incremental Testing

```rust
// Cache test results for incremental runs
struct TestCache {
    results: Vec<CachedResult>,
    last_run_time: i32
}

impl TestCache {
    fun should_run_test(&self, test_name: &Vec<i32>, source_file: &Vec<i32>) -> bool {
        // Check if source file was modified since last test run
        let source_modified_time = get_file_modified_time(source_file);
        source_modified_time > self.last_run_time
    }
}
```

## Best Practices

### 1. Test Design Principles

#### FIRST Principles
- **Fast**: Tests should run quickly
- **Independent**: Tests should not depend on each other
- **Repeatable**: Tests should produce same results consistently
- **Self-Validating**: Tests should have clear pass/fail status
- **Timely**: Tests should be written just before production code

#### AAA Pattern (Arrange-Act-Assert)
```rust
fun test_calculator_addition() -> bool {
    // Arrange
    let calculator = Calculator::new();
    let a = 5;
    let b = 3;
    
    // Act
    let result = calculator.add(a, b);
    
    // Assert
    match Assert::assert_eq(result, 8) {
        AssertResult::Success => true,
        AssertResult::Failure(_) => false
    }
}
```

### 2. Test Coverage Guidelines

#### Minimum Coverage Targets
- **Line Coverage**: 80% minimum, 90%+ for critical code
- **Branch Coverage**: 75% minimum, 85%+ for critical code
- **Function Coverage**: 95% minimum for public APIs
- **Mutation Score**: 70% minimum, 80%+ for critical algorithms

#### Coverage Exclusions
```rust
// Mark code that shouldn't be covered
#[test_coverage(exclude)]
fun debug_only_function() {
    // Debug code that doesn't need testing
}

#[test_coverage(exclude = "unreachable")]
fun unreachable_error_handler() {
    // Error handling that's theoretically unreachable
}
```

### 3. Property Testing Guidelines

#### Good Property Examples
```rust
// Mathematical properties
property_addition_commutative: |a, b| a + b == b + a
property_reverse_involution: |v| reverse(reverse(v)) == v
property_sort_idempotent: |v| sort(sort(v)) == sort(v)

// Invariant properties
property_list_length: |v, x| { let mut v2 = v.clone(); v2.push(x); v2.len() == v.len() + 1 }
property_map_preserves_length: |v, f| map(v, f).len() == v.len()
```

#### Property Testing Strategies
1. **Start with obvious properties**: Identity, commutativity, associativity
2. **Test invariants**: Properties that should always hold
3. **Test relationships**: Input-output relationships that must be preserved
4. **Use shrinking**: Let the framework find minimal failing cases

### 4. Error Handling in Tests

```rust
// Handle expected errors gracefully
fun test_division_by_zero() -> bool {
    let result = divide(10, 0);
    match result {
        Ok(_) => false,        // Should not succeed
        Err(error) => {
            // Verify it's the right type of error
            Assert::assert_vec_contains(error, vec![68, 105, 118, 105, 115, 105, 111, 110]) // Contains "Division"
        }
    }
}

// Test error messages
fun test_error_messages_helpful() -> bool {
    let error = validate_input("").unwrap_err();
    let error_string = error_to_string(error);
    
    // Verify error message is helpful
    Assert::assert_vec_contains(error_string, vec![105, 110, 112, 117, 116]) && // Contains "input"
    Assert::assert_vec_contains(error_string, vec![101, 109, 112, 116, 121])    // Contains "empty"
}
```

## Troubleshooting

### Common Issues and Solutions

#### 1. Enum Match Syntax Errors
**Problem**: `Expected Colon, found Comma` in match expressions
**Solution**: Use let binding before match
```rust
// ❌ Problematic
match MyEnum::Variant {
    MyEnum::A => do_something(),
    MyEnum::B => do_something_else()
}

// ✅ Working
let value = MyEnum::Variant;
match value {
    MyEnum::A => do_something(),
    MyEnum::B => do_something_else()
}
```

#### 2. Vector String Representation
**Problem**: Need to display vector contents as strings
**Solution**: Create utility functions
```rust
fun print_vec_as_string(v: &Vec<i32>) {
    for byte in v.iter() {
        if *byte >= 32 && *byte <= 126 {
            // Convert to character representation
            print_char(*byte as char);
        }
    }
}
```

#### 3. Test Timeout Issues
**Problem**: Property tests taking too long
**Solution**: Reduce iterations or add timeout
```rust
let config = PropertyConfig {
    max_iterations: 50,     // Reduce from 100
    max_shrink_iterations: 5, // Reduce shrinking
    seed: 42,
    verbose: false
};
```

#### 4. Coverage Integration Issues
**Problem**: Coverage analysis not working with test runner
**Solution**: Use separate coverage runs
```bash
# Run tests first
ruchy test test_suite.ruchy

# Then run coverage analysis separately
ruchy coverage test_suite.ruchy --source src/
```

### Debugging Test Failures

#### 1. Systematic Debug Approach
```rust
fun debug_failing_test(test_name: Vec<i32>) {
    println("🔍 Debugging failing test:");
    print_vec(&test_name);
    println("");
    
    // Step 1: Run test in isolation
    let result = run_test_isolated(test_name.clone());
    
    // Step 2: Add verbose logging
    let verbose_result = run_test_with_logging(test_name.clone());
    
    // Step 3: Check test data
    validate_test_inputs(test_name.clone());
    
    // Step 4: Verify assertions
    check_assertion_logic(test_name);
}
```

#### 2. Test Data Validation
```rust
fun validate_test_inputs(test_name: Vec<i32>) {
    println("📊 Validating test inputs:");
    
    // Check if inputs are in expected ranges
    // Verify test data setup is correct
    // Ensure no corrupted test data
}
```

## Conclusion

The Ruchy testing framework provides comprehensive testing capabilities while showcasing the language's advanced features. Key benefits include:

- **Pure Ruchy Implementation**: No external dependencies
- **Comprehensive Coverage**: Unit, property, integration, and mutation testing
- **Advanced Analysis**: Coverage and mutation testing for quality assurance
- **Multiple Output Formats**: JSON, XML, HTML, and text reports
- **CI/CD Integration**: Compatible with modern development workflows

The framework demonstrates Ruchy's capabilities in meta-programming and tooling development while providing production-ready testing infrastructure.

**Key Takeaways**:
- Start with basic unit tests and assertions
- Add property-based testing for mathematical properties
- Use coverage analysis to identify gaps
- Apply mutation testing to validate test quality
- Integrate with quality gates and CI/CD pipelines

---

**Resources**:
- Testing framework source: `framework/ruchy-test/src/`
- Example test suites: `examples/*/test_*.ruchy`
- Integration examples: Makefile patterns in this guide
- Quality gates: Pre-commit hooks and CI configurations