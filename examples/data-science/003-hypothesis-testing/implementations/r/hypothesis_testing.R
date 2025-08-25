#!/usr/bin/env Rscript

# Hypothesis Testing Implementation in R
# Sprint 25: Advanced Statistical Methods

set.seed(42)

# Test result structure
create_test_result <- function(statistic, p_value, df=NULL, reject_null, alpha=0.05) {
  list(
    statistic = statistic,
    p_value = p_value,
    df = df,
    reject_null = reject_null,
    alpha = alpha
  )
}

# Normality result structure
create_normality_result <- function(statistic, p_value, is_normal) {
  list(
    statistic = statistic,
    p_value = p_value,
    is_normal = is_normal
  )
}

# One-sample t-test
one_sample_t_test <- function(data, mu0, alpha=0.05) {
  stopifnot(length(data) >= 2)
  
  result <- t.test(data, mu = mu0, conf.level = 1 - alpha)
  
  create_test_result(
    statistic = as.numeric(result$statistic),
    p_value = result$p.value,
    df = as.numeric(result$parameter),
    reject_null = result$p.value < alpha,
    alpha = alpha
  )
}

# Two-sample Welch's t-test
welch_t_test <- function(sample1, sample2, alpha=0.05) {
  stopifnot(length(sample1) >= 2 && length(sample2) >= 2)
  
  result <- t.test(sample1, sample2, var.equal = FALSE, conf.level = 1 - alpha)
  
  create_test_result(
    statistic = as.numeric(result$statistic),
    p_value = result$p.value,
    df = as.numeric(result$parameter),
    reject_null = result$p.value < alpha,
    alpha = alpha
  )
}

# Paired t-test
paired_t_test <- function(sample1, sample2, alpha=0.05) {
  stopifnot(length(sample1) == length(sample2))
  stopifnot(length(sample1) >= 2)
  
  result <- t.test(sample1, sample2, paired = TRUE, conf.level = 1 - alpha)
  
  create_test_result(
    statistic = as.numeric(result$statistic),
    p_value = result$p.value,
    df = as.numeric(result$parameter),
    reject_null = result$p.value < alpha,
    alpha = alpha
  )
}

# Mann-Whitney U test (Wilcoxon rank-sum test)
mann_whitney_u_test <- function(sample1, sample2, alpha=0.05) {
  stopifnot(length(sample1) >= 1 && length(sample2) >= 1)
  
  result <- wilcox.test(sample1, sample2, exact = FALSE)
  
  create_test_result(
    statistic = as.numeric(result$statistic),
    p_value = result$p.value,
    df = NULL,
    reject_null = result$p.value < alpha,
    alpha = alpha
  )
}

# Wilcoxon signed-rank test
wilcoxon_signed_rank_test <- function(sample1, sample2, alpha=0.05) {
  stopifnot(length(sample1) == length(sample2))
  stopifnot(length(sample1) >= 2)
  
  result <- wilcox.test(sample1, sample2, paired = TRUE, exact = FALSE)
  
  create_test_result(
    statistic = as.numeric(result$statistic),
    p_value = result$p.value,
    df = NULL,
    reject_null = result$p.value < alpha,
    alpha = alpha
  )
}

# Chi-square test of independence
chi_square_test <- function(observed, alpha=0.05) {
  stopifnot(nrow(observed) >= 2 && ncol(observed) >= 2)
  
  result <- chisq.test(observed)
  
  create_test_result(
    statistic = as.numeric(result$statistic),
    p_value = result$p.value,
    df = as.numeric(result$parameter),
    reject_null = result$p.value < alpha,
    alpha = alpha
  )
}

# Shapiro-Wilk test for normality
shapiro_wilk_test <- function(data, alpha=0.05) {
  stopifnot(length(data) >= 3 && length(data) <= 5000)
  
  result <- shapiro.test(data)
  
  create_normality_result(
    statistic = as.numeric(result$statistic),
    p_value = result$p.value,
    is_normal = result$p.value > alpha
  )
}

# Anderson-Darling test (using nortest package if available)
anderson_darling_test <- function(data, alpha=0.05) {
  stopifnot(length(data) >= 2)
  
  # Check if nortest package is available
  if (requireNamespace("nortest", quietly = TRUE)) {
    result <- nortest::ad.test(data)
    statistic <- as.numeric(result$statistic)
    p_value <- result$p.value
  } else {
    # Simplified implementation without nortest
    n <- length(data)
    x <- sort(data)
    mu <- mean(x)
    sigma <- sd(x)
    z <- (x - mu) / sigma
    
    # Calculate A² statistic
    S <- 0
    for (i in 1:n) {
      S <- S + (2*i - 1) * (log(pnorm(z[i])) + log(1 - pnorm(z[n+1-i])))
    }
    A2 <- -n - S/n
    
    # Adjusted statistic
    A2_star <- A2 * (1 + 0.75/n + 2.25/n^2)
    
    # Approximate p-value
    if (A2_star < 0.2) {
      p_value <- 1 - exp(-13.436 + 101.14*A2_star - 223.73*A2_star^2)
    } else if (A2_star < 0.34) {
      p_value <- 1 - exp(-8.318 + 42.796*A2_star - 59.938*A2_star^2)
    } else if (A2_star < 0.6) {
      p_value <- exp(0.9177 - 4.279*A2_star - 1.38*A2_star^2)
    } else {
      p_value <- exp(1.2937 - 5.709*A2_star + 0.0186*A2_star^2)
    }
    
    statistic <- A2_star
  }
  
  create_normality_result(
    statistic = statistic,
    p_value = p_value,
    is_normal = p_value > alpha
  )
}

# Kolmogorov-Smirnov test
kolmogorov_smirnov_test <- function(data, alpha=0.05) {
  stopifnot(length(data) >= 2)
  
  result <- ks.test(data, "pnorm", mean = mean(data), sd = sd(data))
  
  create_normality_result(
    statistic = as.numeric(result$statistic),
    p_value = result$p.value,
    is_normal = result$p.value > alpha
  )
}

# Jarque-Bera test for normality
jarque_bera_test <- function(data, alpha=0.05) {
  n <- length(data)
  stopifnot(n >= 4)
  
  # Calculate moments
  mu <- mean(data)
  m2 <- mean((data - mu)^2)
  m3 <- mean((data - mu)^3)
  m4 <- mean((data - mu)^4)
  
  # Skewness and kurtosis
  skewness <- m3 / m2^(3/2)
  kurtosis <- m4 / m2^2 - 3
  
  # JB statistic
  JB <- n/6 * (skewness^2 + kurtosis^2/4)
  
  # P-value from chi-square distribution with 2 df
  p_value <- 1 - pchisq(JB, df = 2)
  
  create_normality_result(
    statistic = JB,
    p_value = p_value,
    is_normal = p_value > alpha
  )
}

# One-way ANOVA
one_way_anova <- function(groups, alpha=0.05) {
  stopifnot(length(groups) >= 2)
  
  # Combine groups into a single vector and create factor
  data_vec <- unlist(groups)
  group_factor <- factor(rep(1:length(groups), sapply(groups, length)))
  
  result <- aov(data_vec ~ group_factor)
  summary_result <- summary(result)[[1]]
  
  F_stat <- summary_result$`F value`[1]
  p_value <- summary_result$`Pr(>F)`[1]
  df_between <- summary_result$Df[1]
  
  create_test_result(
    statistic = F_stat,
    p_value = p_value,
    df = df_between,
    reject_null = p_value < alpha,
    alpha = alpha
  )
}

# Kruskal-Wallis test
kruskal_wallis_test <- function(groups, alpha=0.05) {
  stopifnot(length(groups) >= 2)
  
  # Combine groups into a single vector and create factor
  data_vec <- unlist(groups)
  group_factor <- factor(rep(1:length(groups), sapply(groups, length)))
  
  result <- kruskal.test(data_vec, group_factor)
  
  create_test_result(
    statistic = as.numeric(result$statistic),
    p_value = result$p.value,
    df = as.numeric(result$parameter),
    reject_null = result$p.value < alpha,
    alpha = alpha
  )
}

# Friedman test (for repeated measures)
friedman_test <- function(data_matrix, alpha=0.05) {
  stopifnot(nrow(data_matrix) >= 2 && ncol(data_matrix) >= 2)
  
  result <- friedman.test(data_matrix)
  
  create_test_result(
    statistic = as.numeric(result$statistic),
    p_value = result$p.value,
    df = as.numeric(result$parameter),
    reject_null = result$p.value < alpha,
    alpha = alpha
  )
}

# Effect size calculations
cohens_d <- function(sample1, sample2) {
  n1 <- length(sample1)
  n2 <- length(sample2)
  x1_bar <- mean(sample1)
  x2_bar <- mean(sample2)
  s1 <- sd(sample1)
  s2 <- sd(sample2)
  
  # Pooled standard deviation
  s_pooled <- sqrt(((n1-1)*s1^2 + (n2-1)*s2^2) / (n1+n2-2))
  
  (x1_bar - x2_bar) / s_pooled
}

# Eta squared for ANOVA
eta_squared <- function(groups) {
  data_vec <- unlist(groups)
  group_factor <- factor(rep(1:length(groups), sapply(groups, length)))
  
  result <- aov(data_vec ~ group_factor)
  summary_result <- summary(result)[[1]]
  
  SS_between <- summary_result$`Sum Sq`[1]
  SS_total <- sum(summary_result$`Sum Sq`)
  
  SS_between / SS_total
}

# Power analysis (simplified)
power_t_test <- function(n, effect_size, alpha=0.05, type="two.sample") {
  # Simplified power calculation for t-test
  if (type == "two.sample") {
    df <- 2*n - 2
    ncp <- effect_size * sqrt(n/2)
  } else {
    df <- n - 1
    ncp <- effect_size * sqrt(n)
  }
  
  critical_t <- qt(1 - alpha/2, df)
  power <- 1 - pt(critical_t, df, ncp) + pt(-critical_t, df, ncp)
  
  power
}

# Main demonstration function
main <- function() {
  cat("=== Hypothesis Testing in R ===\n\n")
  
  # Generate test data
  normal_data1 <- rnorm(100, mean = 5, sd = 2)
  normal_data2 <- rnorm(100, mean = 5.5, sd = 2.5)
  paired_data1 <- rnorm(50, mean = 10, sd = 1.5)
  paired_data2 <- paired_data1 + rnorm(50, mean = 0.3, sd = 0.5)
  
  # Parametric tests
  cat("1. One-Sample T-Test\n")
  result <- one_sample_t_test(normal_data1, 5.0)
  cat(sprintf("   t-statistic: %.4f\n", result$statistic))
  cat(sprintf("   p-value: %.4f\n", result$p_value))
  cat(sprintf("   Reject null: %s\n\n", result$reject_null))
  
  cat("2. Welch's T-Test\n")
  result <- welch_t_test(normal_data1, normal_data2)
  cat(sprintf("   t-statistic: %.4f\n", result$statistic))
  cat(sprintf("   p-value: %.4f\n", result$p_value))
  cat(sprintf("   df: %.2f\n", result$df))
  cat(sprintf("   Cohen's d: %.4f\n\n", cohens_d(normal_data1, normal_data2)))
  
  cat("3. Paired T-Test\n")
  result <- paired_t_test(paired_data1, paired_data2)
  cat(sprintf("   t-statistic: %.4f\n", result$statistic))
  cat(sprintf("   p-value: %.4f\n\n", result$p_value))
  
  # Non-parametric tests
  cat("4. Mann-Whitney U Test\n")
  result <- mann_whitney_u_test(normal_data1, normal_data2)
  cat(sprintf("   U-statistic: %.4f\n", result$statistic))
  cat(sprintf("   p-value: %.4f\n\n", result$p_value))
  
  cat("5. Wilcoxon Signed-Rank Test\n")
  result <- wilcoxon_signed_rank_test(paired_data1, paired_data2)
  cat(sprintf("   W-statistic: %.4f\n", result$statistic))
  cat(sprintf("   p-value: %.4f\n\n", result$p_value))
  
  # Chi-square test
  cat("6. Chi-Square Test\n")
  contingency <- matrix(c(10, 15, 8, 12, 18, 6, 8, 10, 14), nrow = 3, byrow = TRUE)
  result <- chi_square_test(contingency)
  cat(sprintf("   χ²-statistic: %.4f\n", result$statistic))
  cat(sprintf("   p-value: %.4f\n", result$p_value))
  cat(sprintf("   df: %d\n\n", as.integer(result$df)))
  
  # Normality tests
  cat("7. Shapiro-Wilk Test\n")
  norm_result <- shapiro_wilk_test(normal_data1)
  cat(sprintf("   W-statistic: %.4f\n", norm_result$statistic))
  cat(sprintf("   p-value: %.4f\n", norm_result$p_value))
  cat(sprintf("   Is normal: %s\n\n", norm_result$is_normal))
  
  cat("8. Anderson-Darling Test\n")
  norm_result <- anderson_darling_test(normal_data1)
  cat(sprintf("   A²-statistic: %.4f\n", norm_result$statistic))
  cat(sprintf("   p-value: %.4f\n", norm_result$p_value))
  cat(sprintf("   Is normal: %s\n\n", norm_result$is_normal))
  
  cat("9. Kolmogorov-Smirnov Test\n")
  norm_result <- kolmogorov_smirnov_test(normal_data1)
  cat(sprintf("   D-statistic: %.4f\n", norm_result$statistic))
  cat(sprintf("   p-value: %.4f\n", norm_result$p_value))
  cat(sprintf("   Is normal: %s\n\n", norm_result$is_normal))
  
  cat("10. Jarque-Bera Test\n")
  norm_result <- jarque_bera_test(normal_data1)
  cat(sprintf("   JB-statistic: %.4f\n", norm_result$statistic))
  cat(sprintf("   p-value: %.4f\n", norm_result$p_value))
  cat(sprintf("   Is normal: %s\n\n", norm_result$is_normal))
  
  # ANOVA
  cat("11. One-Way ANOVA\n")
  groups <- list(
    rnorm(30, mean = 1),
    rnorm(30, mean = 2),
    rnorm(30, mean = 3)
  )
  result <- one_way_anova(groups)
  cat(sprintf("   F-statistic: %.4f\n", result$statistic))
  cat(sprintf("   p-value: %.4f\n", result$p_value))
  cat(sprintf("   df: %d\n", as.integer(result$df)))
  cat(sprintf("   Eta squared: %.4f\n\n", eta_squared(groups)))
  
  cat("12. Kruskal-Wallis Test\n")
  result <- kruskal_wallis_test(groups)
  cat(sprintf("   H-statistic: %.4f\n", result$statistic))
  cat(sprintf("   p-value: %.4f\n", result$p_value))
  cat(sprintf("   df: %d\n\n", as.integer(result$df)))
  
  # Friedman test
  cat("13. Friedman Test\n")
  repeated_data <- matrix(rnorm(30*3), nrow = 30, ncol = 3)
  result <- friedman_test(repeated_data)
  cat(sprintf("   χ²-statistic: %.4f\n", result$statistic))
  cat(sprintf("   p-value: %.4f\n", result$p_value))
  cat(sprintf("   df: %d\n\n", as.integer(result$df)))
  
  # Power analysis
  cat("14. Power Analysis\n")
  power <- power_t_test(n = 50, effect_size = 0.5, alpha = 0.05, type = "two.sample")
  cat(sprintf("   Power for n=50, d=0.5: %.4f\n", power))
  power <- power_t_test(n = 100, effect_size = 0.5, alpha = 0.05, type = "two.sample")
  cat(sprintf("   Power for n=100, d=0.5: %.4f\n\n", power))
  
  cat("=== All hypothesis tests completed successfully ===\n")
}

# Run if executed directly
if (!interactive()) {
  main()
}