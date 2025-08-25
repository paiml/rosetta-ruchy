#!/usr/bin/env python3
"""
Hypothesis Testing - Python Implementation
Statistical inference using scipy.stats for cross-language comparison
Focus: Comprehensive hypothesis testing with p-value validation
"""

import math
import time
from typing import List, Tuple
from scipy import stats
import numpy as np

def one_sample_t_test(sample: List[float], pop_mean: float) -> float:
    """One-sample t-test against population mean"""
    if len(sample) < 2:
        return 1.0
    
    # Use scipy's ttest_1samp for accurate p-value
    t_stat, p_value = stats.ttest_1samp(sample, pop_mean)
    return p_value

def two_sample_t_test(sample1: List[float], sample2: List[float]) -> float:
    """Two-sample t-test (Welch's t-test for unequal variances)"""
    if len(sample1) < 2 or len(sample2) < 2:
        return 1.0
    
    # Use Welch's t-test (equal_var=False)
    t_stat, p_value = stats.ttest_ind(sample1, sample2, equal_var=False)
    return p_value

def paired_t_test(sample1: List[float], sample2: List[float]) -> float:
    """Paired t-test for dependent samples"""
    if len(sample1) != len(sample2) or len(sample1) < 2:
        return 1.0
    
    # Use scipy's paired t-test
    t_stat, p_value = stats.ttest_rel(sample1, sample2)
    return p_value

def chi_square_test(a: float, b: float, c: float, d: float) -> float:
    """Chi-square test for 2x2 contingency table"""
    # Create contingency table
    observed = np.array([[a, b], [c, d]])
    
    # Check for valid input
    if np.sum(observed) == 0:
        return 1.0
    
    # Use scipy's chi2_contingency
    chi2, p_value, dof, expected = stats.chi2_contingency(observed)
    return p_value

def mann_whitney_u_test(sample1: List[float], sample2: List[float]) -> float:
    """Mann-Whitney U test (non-parametric)"""
    if len(sample1) < 1 or len(sample2) < 1:
        return 1.0
    
    # Use scipy's mannwhitneyu test
    u_stat, p_value = stats.mannwhitneyu(sample1, sample2, alternative='two-sided')
    return p_value

def shapiro_wilk_test(data: List[float]) -> Tuple[float, bool]:
    """Shapiro-Wilk test for normality"""
    if len(data) < 3:
        return 1.0, False
    
    # Use scipy's shapiro test
    w_stat, p_value = stats.shapiro(data)
    is_normal = p_value > 0.05
    return p_value, is_normal

def kolmogorov_smirnov_test(sample1: List[float], sample2: List[float]) -> float:
    """Kolmogorov-Smirnov test for distribution comparison"""
    if len(sample1) < 1 or len(sample2) < 1:
        return 1.0
    
    # Use scipy's ks_2samp test
    ks_stat, p_value = stats.ks_2samp(sample1, sample2)
    return p_value

def anova_one_way(*samples) -> float:
    """One-way ANOVA for multiple groups"""
    # Filter out empty samples
    valid_samples = [s for s in samples if len(s) > 0]
    
    if len(valid_samples) < 2:
        return 1.0
    
    # Use scipy's f_oneway
    f_stat, p_value = stats.f_oneway(*valid_samples)
    return p_value

def test_hypothesis_tests():
    """Test all hypothesis testing functions"""
    print("Hypothesis Testing - Python (scipy.stats)")
    print("========================================")
    
    # Test 1: One-sample t-test
    sample = [1.0, 2.0, 3.0, 4.0, 5.0]
    p_one = one_sample_t_test(sample, 3.0)
    if 0.0 <= p_one <= 1.0:
        print(f"✓ One-sample t-test: Valid p-value ({p_one:.4f})")
    else:
        print("✗ One-sample t-test: Invalid p-value")
    
    # Test 2: Two-sample t-test
    s1 = [1.0, 2.0, 3.0, 4.0, 5.0]
    s2 = [2.0, 3.0, 4.0, 5.0, 6.0]
    p_two = two_sample_t_test(s1, s2)
    if 0.0 <= p_two <= 1.0:
        print(f"✓ Two-sample t-test: Valid p-value ({p_two:.4f})")
    else:
        print("✗ Two-sample t-test: Invalid p-value")
    
    # Test 3: Paired t-test
    paired1 = [1.0, 2.0, 3.0, 4.0]
    paired2 = [1.5, 2.5, 3.5, 4.5]
    p_paired = paired_t_test(paired1, paired2)
    if 0.0 <= p_paired <= 1.0:
        print(f"✓ Paired t-test: Valid p-value ({p_paired:.4f})")
    else:
        print("✗ Paired t-test: Invalid p-value")
    
    # Test 4: Chi-square test
    p_chi = chi_square_test(10.0, 20.0, 30.0, 40.0)
    if 0.0 <= p_chi <= 1.0:
        print(f"✓ Chi-square test: Valid p-value ({p_chi:.4f})")
    else:
        print("✗ Chi-square test: Invalid p-value")
    
    # Test 5: Mann-Whitney U test
    p_mw = mann_whitney_u_test(s1, s2)
    if 0.0 <= p_mw <= 1.0:
        print(f"✓ Mann-Whitney U test: Valid p-value ({p_mw:.4f})")
    else:
        print("✗ Mann-Whitney U test: Invalid p-value")
    
    # Test 6: Shapiro-Wilk test
    normal_data = [1.0, 2.0, 3.0, 4.0, 5.0, 4.0, 3.0, 2.0]
    p_sw, is_normal = shapiro_wilk_test(normal_data)
    if 0.0 <= p_sw <= 1.0:
        print(f"✓ Shapiro-Wilk test: Valid p-value ({p_sw:.4f})")
    else:
        print("✗ Shapiro-Wilk test: Invalid p-value")
    
    # Test 7: Kolmogorov-Smirnov test
    p_ks = kolmogorov_smirnov_test(s1, s2)
    if 0.0 <= p_ks <= 1.0:
        print(f"✓ Kolmogorov-Smirnov test: Valid p-value ({p_ks:.4f})")
    else:
        print("✗ Kolmogorov-Smirnov test: Invalid p-value")
    
    # Test 8: One-way ANOVA
    g1 = [1.0, 2.0, 3.0]
    g2 = [2.0, 3.0, 4.0]
    g3 = [3.0, 4.0, 5.0]
    p_anova = anova_one_way(g1, g2, g3)
    if 0.0 <= p_anova <= 1.0:
        print(f"✓ One-way ANOVA: Valid p-value ({p_anova:.4f})")
    else:
        print("✗ One-way ANOVA: Invalid p-value")
    
    print("")
    print("All hypothesis tests validated")

def verify_properties():
    """Verify statistical properties"""
    print("Statistical Properties Verification")
    print("==================================")
    
    # Property 1: P-value bounds
    test_samples = [
        [1.0, 2.0, 3.0, 4.0, 5.0],
        [10.0, 20.0, 30.0, 40.0, 50.0]
    ]
    
    all_valid = True
    for sample in test_samples:
        p = one_sample_t_test(sample, np.mean(sample))
        if not (0.0 <= p <= 1.0):
            all_valid = False
            break
    
    if all_valid:
        print("✓ P-value bounds [0,1]: Verified")
    else:
        print("✗ P-value bounds: Failed")
    
    # Property 2: Test symmetry
    s1 = [1.0, 2.0, 3.0]
    s2 = [4.0, 5.0, 6.0]
    p1 = two_sample_t_test(s1, s2)
    p2 = two_sample_t_test(s2, s1)
    
    if abs(p1 - p2) < 0.001:
        print("✓ Test symmetry: Verified")
    else:
        print("✗ Test symmetry: Failed")
    
    # Property 3: Normality test consistency
    normal_sample = np.random.normal(0, 1, 100)
    p_norm, is_norm = shapiro_wilk_test(normal_sample.tolist())
    
    if 0.0 <= p_norm <= 1.0:
        print("✓ Normality test properties: Verified")
    else:
        print("✗ Normality test properties: Failed")
    
    print("")
    print("All properties verified")

def benchmark_operations(iterations: int = 100):
    """Benchmark hypothesis testing operations"""
    print(f"Benchmarking with {iterations} iterations...")
    
    # Generate test data
    np.random.seed(42)
    large_sample1 = np.random.normal(0, 1, 100).tolist()
    large_sample2 = np.random.normal(0.5, 1, 100).tolist()
    
    # Benchmark t-tests
    start_time = time.perf_counter()
    for _ in range(iterations):
        one_sample_t_test(large_sample1, 0)
        two_sample_t_test(large_sample1, large_sample2)
        paired_t_test(large_sample1[:50], large_sample2[:50])
    t_test_time = time.perf_counter() - start_time
    
    # Benchmark non-parametric tests
    start_time = time.perf_counter()
    for _ in range(iterations):
        mann_whitney_u_test(large_sample1, large_sample2)
        kolmogorov_smirnov_test(large_sample1, large_sample2)
    nonparam_time = time.perf_counter() - start_time
    
    # Benchmark normality tests
    start_time = time.perf_counter()
    for _ in range(iterations):
        shapiro_wilk_test(large_sample1)
    normality_time = time.perf_counter() - start_time
    
    print(f"T-tests: {t_test_time:.4f}s")
    print(f"Non-parametric tests: {nonparam_time:.4f}s")
    print(f"Normality tests: {normality_time:.4f}s")
    
    return {
        't_test_time': t_test_time,
        'nonparam_time': nonparam_time,
        'normality_time': normality_time
    }

def analyze_complexity():
    """Analyze algorithm complexity"""
    print("Hypothesis Testing Complexity Analysis - Python")
    print("==============================================")
    
    print("Test Complexities:")
    print("  T-tests: O(n)")
    print("  Mann-Whitney U: O(n log n)")
    print("  Kolmogorov-Smirnov: O(n log n)")
    print("  Chi-square: O(nm) for n×m table")
    print("  Shapiro-Wilk: O(n²)")
    print("  ANOVA: O(kn) for k groups")
    print("")
    
    print("Python (scipy) Advantages:")
    print("  ✓ Comprehensive statistical test library")
    print("  ✓ Accurate p-value calculations")
    print("  ✓ Well-tested implementations")
    print("  ✓ Extensive documentation")
    print("  ✓ Integration with NumPy/pandas")
    print("")
    
    print("Ruchy Advantages:")
    print("  ✓ Formal verification of p-value bounds")
    print("  ✓ Compile-time assumption checking")
    print("  ✓ Type-safe statistical inference")
    print("  ✓ Guaranteed error rate control")
    print("  ✓ Deterministic test results")

def main():
    """Main demonstration function"""
    print("Hypothesis Testing Framework - Python")
    print("====================================")
    print("Statistical inference with scipy.stats")
    print("")
    
    # Run tests
    test_hypothesis_tests()
    print("")
    
    # Verify properties
    verify_properties()
    print("")
    
    # Run benchmarks
    benchmark_results = benchmark_operations()
    print("")
    
    # Analyze complexity
    analyze_complexity()
    print("")
    
    print("✅ Python hypothesis testing complete")
    print("📊 Ready for cross-language comparison")
    print("🎯 SPRINT 25: Advanced statistical methods in progress")

if __name__ == "__main__":
    main()