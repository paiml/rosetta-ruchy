#!/usr/bin/env julia

using Statistics
using Distributions
using Random

# Test result structure
struct TestResult
    statistic::Float64
    p_value::Float64
    df::Union{Float64, Nothing}
    reject_null::Bool
    alpha::Float64
end

# Normality test result
struct NormalityResult
    statistic::Float64
    p_value::Float64
    is_normal::Bool
end

# One-sample t-test
function one_sample_t_test(data::Vector{Float64}, μ₀::Float64; alpha::Float64=0.05)
    n = length(data)
    @assert n >= 2 "Sample size must be at least 2"
    
    x̄ = mean(data)
    s = std(data, corrected=true)
    se = s / sqrt(n)
    
    t_stat = (x̄ - μ₀) / se
    df = n - 1
    
    # Two-tailed p-value
    p_value = 2 * cdf(TDist(df), -abs(t_stat))
    
    TestResult(t_stat, p_value, df, p_value < alpha, alpha)
end

# Two-sample Welch's t-test (unequal variances)
function welch_t_test(sample1::Vector{Float64}, sample2::Vector{Float64}; alpha::Float64=0.05)
    n1, n2 = length(sample1), length(sample2)
    @assert n1 >= 2 && n2 >= 2 "Both samples must have at least 2 observations"
    
    x̄1, x̄2 = mean(sample1), mean(sample2)
    s1², s2² = var(sample1, corrected=true), var(sample2, corrected=true)
    
    se = sqrt(s1²/n1 + s2²/n2)
    t_stat = (x̄1 - x̄2) / se
    
    # Welch-Satterthwaite degrees of freedom
    df = (s1²/n1 + s2²/n2)^2 / ((s1²/n1)^2/(n1-1) + (s2²/n2)^2/(n2-1))
    
    # Two-tailed p-value
    p_value = 2 * cdf(TDist(df), -abs(t_stat))
    
    TestResult(t_stat, p_value, df, p_value < alpha, alpha)
end

# Paired t-test
function paired_t_test(sample1::Vector{Float64}, sample2::Vector{Float64}; alpha::Float64=0.05)
    n = length(sample1)
    @assert n == length(sample2) "Samples must have equal length for paired test"
    @assert n >= 2 "Sample size must be at least 2"
    
    differences = sample1 .- sample2
    d̄ = mean(differences)
    sd = std(differences, corrected=true)
    se = sd / sqrt(n)
    
    t_stat = d̄ / se
    df = n - 1
    
    # Two-tailed p-value
    p_value = 2 * cdf(TDist(df), -abs(t_stat))
    
    TestResult(t_stat, p_value, df, p_value < alpha, alpha)
end

# Mann-Whitney U test (Wilcoxon rank-sum test)
function mann_whitney_u_test(sample1::Vector{Float64}, sample2::Vector{Float64}; alpha::Float64=0.05)
    n1, n2 = length(sample1), length(sample2)
    @assert n1 >= 1 && n2 >= 1 "Both samples must have at least 1 observation"
    
    # Combine and rank
    combined = vcat(sample1, sample2)
    ranks = ordinalrank(combined)
    
    # Sum of ranks for each group
    R1 = sum(ranks[1:n1])
    R2 = sum(ranks[n1+1:end])
    
    # U statistics
    U1 = R1 - n1*(n1+1)/2
    U2 = R2 - n2*(n2+1)/2
    U = min(U1, U2)
    
    # Normal approximation for large samples
    μU = n1*n2/2
    σU = sqrt(n1*n2*(n1+n2+1)/12)
    
    # Continuity correction
    z = (U - μU + 0.5) / σU
    p_value = 2 * cdf(Normal(), -abs(z))
    
    TestResult(U, p_value, nothing, p_value < alpha, alpha)
end

# Wilcoxon signed-rank test
function wilcoxon_signed_rank_test(sample1::Vector{Float64}, sample2::Vector{Float64}; alpha::Float64=0.05)
    n = length(sample1)
    @assert n == length(sample2) "Samples must have equal length"
    @assert n >= 2 "Sample size must be at least 2"
    
    differences = sample1 .- sample2
    # Remove zeros
    differences = differences[differences .!= 0]
    n_eff = length(differences)
    
    if n_eff == 0
        return TestResult(0.0, 1.0, nothing, false, alpha)
    end
    
    # Rank absolute values
    abs_diffs = abs.(differences)
    ranks = ordinalrank(abs_diffs)
    
    # Sum of positive and negative ranks
    W_plus = sum(ranks[differences .> 0])
    W_minus = sum(ranks[differences .< 0])
    W = min(W_plus, W_minus)
    
    # Normal approximation
    μW = n_eff*(n_eff+1)/4
    σW = sqrt(n_eff*(n_eff+1)*(2*n_eff+1)/24)
    
    z = (W - μW + 0.5) / σW
    p_value = 2 * cdf(Normal(), -abs(z))
    
    TestResult(W, p_value, nothing, p_value < alpha, alpha)
end

# Chi-square test of independence
function chi_square_test(observed::Matrix{Float64}; alpha::Float64=0.05)
    rows, cols = size(observed)
    @assert rows >= 2 && cols >= 2 "Contingency table must be at least 2x2"
    
    total = sum(observed)
    row_sums = sum(observed, dims=2)
    col_sums = sum(observed, dims=1)
    
    # Expected frequencies
    expected = (row_sums * col_sums) / total
    
    # Chi-square statistic
    χ² = sum((observed .- expected).^2 ./ expected)
    
    # Degrees of freedom
    df = (rows - 1) * (cols - 1)
    
    # P-value
    p_value = 1 - cdf(Chisq(df), χ²)
    
    TestResult(χ², p_value, Float64(df), p_value < alpha, alpha)
end

# Shapiro-Wilk test for normality (simplified version)
function shapiro_wilk_test(data::Vector{Float64}; alpha::Float64=0.05)
    n = length(data)
    @assert n >= 3 "Sample size must be at least 3"
    
    # Sort data
    x = sort(data)
    x̄ = mean(x)
    
    # Calculate S²
    S² = sum((x .- x̄).^2)
    
    # Calculate W statistic (simplified)
    # For a full implementation, we'd need the Shapiro-Wilk coefficients
    # This is an approximation using normal probability plot correlation
    
    # Calculate expected normal order statistics
    m = n ÷ 2
    a = zeros(m)
    for i in 1:m
        p = (i - 0.375) / (n + 0.25)
        a[i] = quantile(Normal(), p)
    end
    
    # Calculate b
    b = 0.0
    for i in 1:m
        b += a[i] * (x[n+1-i] - x[i])
    end
    
    W = b^2 / S²
    
    # Approximate p-value (would need proper Shapiro-Wilk distribution)
    # Using a rough approximation based on sample size
    if n <= 50
        # Log-normal approximation for small samples
        μ = -1.2725 + 1.0521 * log(n)
        σ = 1.0308 - 0.2604 * log(n)
        z = (log(1 - W) - μ) / σ
        p_value = cdf(Normal(), z)
    else
        # Normal approximation for larger samples
        z = (W - 0.9) * sqrt(n) / 0.15
        p_value = cdf(Normal(), -abs(z))
    end
    
    NormalityResult(W, p_value, p_value > alpha)
end

# Anderson-Darling test for normality
function anderson_darling_test(data::Vector{Float64}; alpha::Float64=0.05)
    n = length(data)
    @assert n >= 2 "Sample size must be at least 2"
    
    # Standardize data
    x = sort(data)
    μ = mean(x)
    σ = std(x, corrected=true)
    z = (x .- μ) / σ
    
    # Calculate A² statistic
    S = 0.0
    for i in 1:n
        S += (2i - 1) * (log(cdf(Normal(), z[i])) + log(1 - cdf(Normal(), z[n+1-i])))
    end
    A² = -n - S/n
    
    # Adjusted statistic
    A²_star = A² * (1 + 0.75/n + 2.25/n^2)
    
    # Approximate p-value (simplified)
    if A²_star < 0.2
        p_value = 1 - exp(-13.436 + 101.14*A²_star - 223.73*A²_star^2)
    elseif A²_star < 0.34
        p_value = 1 - exp(-8.318 + 42.796*A²_star - 59.938*A²_star^2)
    elseif A²_star < 0.6
        p_value = exp(0.9177 - 4.279*A²_star - 1.38*A²_star^2)
    else
        p_value = exp(1.2937 - 5.709*A²_star + 0.0186*A²_star^2)
    end
    
    NormalityResult(A²_star, p_value, p_value > alpha)
end

# ANOVA (one-way)
function one_way_anova(groups::Vector{Vector{Float64}}; alpha::Float64=0.05)
    k = length(groups)
    @assert k >= 2 "Must have at least 2 groups"
    
    # Calculate group statistics
    n_groups = [length(g) for g in groups]
    n_total = sum(n_groups)
    
    group_means = [mean(g) for g in groups]
    grand_mean = sum(g -> sum(g), groups) / n_total
    
    # Sum of squares
    SS_between = sum(n_groups[i] * (group_means[i] - grand_mean)^2 for i in 1:k)
    SS_within = sum(sum((x - group_means[i])^2 for x in groups[i]) for i in 1:k)
    
    # Degrees of freedom
    df_between = k - 1
    df_within = n_total - k
    
    # Mean squares
    MS_between = SS_between / df_between
    MS_within = SS_within / df_within
    
    # F-statistic
    F_stat = MS_between / MS_within
    
    # P-value
    p_value = 1 - cdf(FDist(df_between, df_within), F_stat)
    
    TestResult(F_stat, p_value, Float64(df_between), p_value < alpha, alpha)
end

# Helper function for ranking
function ordinalrank(x::Vector{Float64})
    n = length(x)
    sorted_indices = sortperm(x)
    ranks = zeros(n)
    
    i = 1
    while i <= n
        j = i
        while j < n && x[sorted_indices[j+1]] == x[sorted_indices[i]]
            j += 1
        end
        
        # Average rank for ties
        avg_rank = (i + j) / 2
        for k in i:j
            ranks[sorted_indices[k]] = avg_rank
        end
        
        i = j + 1
    end
    
    return ranks
end

# Effect size calculations
function cohens_d(sample1::Vector{Float64}, sample2::Vector{Float64})
    n1, n2 = length(sample1), length(sample2)
    x̄1, x̄2 = mean(sample1), mean(sample2)
    s1, s2 = std(sample1, corrected=true), std(sample2, corrected=true)
    
    # Pooled standard deviation
    s_pooled = sqrt(((n1-1)*s1^2 + (n2-1)*s2^2) / (n1+n2-2))
    
    (x̄1 - x̄2) / s_pooled
end

# Main demonstration function
function main()
    Random.seed!(42)
    
    println("=== Hypothesis Testing in Julia ===\n")
    
    # Generate test data
    normal_data1 = randn(100) .* 2 .+ 5
    normal_data2 = randn(100) .* 2.5 .+ 5.5
    paired_data1 = randn(50) .* 1.5 .+ 10
    paired_data2 = paired_data1 .+ randn(50) .* 0.5 .+ 0.3
    
    # Parametric tests
    println("1. One-Sample T-Test")
    result = one_sample_t_test(normal_data1, 5.0)
    println("   t-statistic: $(round(result.statistic, digits=4))")
    println("   p-value: $(round(result.p_value, digits=4))")
    println("   Reject null: $(result.reject_null)\n")
    
    println("2. Welch's T-Test")
    result = welch_t_test(normal_data1, normal_data2)
    println("   t-statistic: $(round(result.statistic, digits=4))")
    println("   p-value: $(round(result.p_value, digits=4))")
    println("   df: $(round(result.df, digits=2))")
    println("   Cohen's d: $(round(cohens_d(normal_data1, normal_data2), digits=4))\n")
    
    println("3. Paired T-Test")
    result = paired_t_test(paired_data1, paired_data2)
    println("   t-statistic: $(round(result.statistic, digits=4))")
    println("   p-value: $(round(result.p_value, digits=4))\n")
    
    # Non-parametric tests
    println("4. Mann-Whitney U Test")
    result = mann_whitney_u_test(normal_data1, normal_data2)
    println("   U-statistic: $(round(result.statistic, digits=4))")
    println("   p-value: $(round(result.p_value, digits=4))\n")
    
    println("5. Wilcoxon Signed-Rank Test")
    result = wilcoxon_signed_rank_test(paired_data1, paired_data2)
    println("   W-statistic: $(round(result.statistic, digits=4))")
    println("   p-value: $(round(result.p_value, digits=4))\n")
    
    # Chi-square test
    println("6. Chi-Square Test")
    contingency = [10.0 15.0 8.0; 12.0 18.0 6.0; 8.0 10.0 14.0]
    result = chi_square_test(contingency)
    println("   χ²-statistic: $(round(result.statistic, digits=4))")
    println("   p-value: $(round(result.p_value, digits=4))")
    println("   df: $(Int(result.df))\n")
    
    # Normality tests
    println("7. Shapiro-Wilk Test")
    norm_result = shapiro_wilk_test(normal_data1)
    println("   W-statistic: $(round(norm_result.statistic, digits=4))")
    println("   p-value: $(round(norm_result.p_value, digits=4))")
    println("   Is normal: $(norm_result.is_normal)\n")
    
    println("8. Anderson-Darling Test")
    norm_result = anderson_darling_test(normal_data1)
    println("   A²-statistic: $(round(norm_result.statistic, digits=4))")
    println("   p-value: $(round(norm_result.p_value, digits=4))")
    println("   Is normal: $(norm_result.is_normal)\n")
    
    # ANOVA
    println("9. One-Way ANOVA")
    groups = [randn(30) .+ i for i in 1:3]
    result = one_way_anova(groups)
    println("   F-statistic: $(round(result.statistic, digits=4))")
    println("   p-value: $(round(result.p_value, digits=4))")
    println("   df: $(Int(result.df))\n")
    
    println("=== All hypothesis tests completed successfully ===")
end

# Run if executed directly
if abspath(PROGRAM_FILE) == @__FILE__
    main()
end