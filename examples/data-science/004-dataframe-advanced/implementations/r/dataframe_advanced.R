#!/usr/bin/env Rscript

# DataFrame Advanced Operations - Sprint 26
# Group-by operations, joins, and aggregations
# R implementation using dplyr and tidyverse

library(dplyr)
library(tidyr)
library(lubridate)
library(purrr)

# Set random seed for reproducibility
set.seed(42)

# Generate sample sales data
create_sales_dataframe <- function(n = 1000) {
  regions <- c("North", "South", "East", "West")
  products <- c("A", "B", "C", "D", "E")
  
  base_date <- as.Date("2024-01-01")
  
  data.frame(
    order_id = 1:n,
    date = base_date + sample(0:364, n, replace = TRUE),
    region = sample(regions, n, replace = TRUE),
    product = sample(products, n, replace = TRUE),
    customer_id = sample(1:200, n, replace = TRUE),
    quantity = sample(1:100, n, replace = TRUE),
    revenue = round(runif(n, 100, 10000) + rnorm(n) * 100, 2),
    cost = round(runif(n, 50, 5000) + rnorm(n) * 50, 2)
  )
}

# Generate sample customer data
create_customers_dataframe <- function(n = 200) {
  segments <- c("Premium", "Standard", "Basic")
  
  base_date <- as.Date("2020-01-01")
  
  data.frame(
    id = 1:n,
    name = paste0("Customer_", 1:n),
    segment = sample(segments, n, replace = TRUE),
    join_date = base_date + sample(0:1460, n, replace = TRUE),
    credit_limit = sample(1000:50000, n, replace = TRUE)
  )
}

# 1. Group-by operations with multiple aggregations
demonstrate_groupby <- function(df) {
  cat("\n1. GROUP-BY OPERATIONS:\n")
  
  # Single column grouping with multiple aggregations
  agg_single <- df %>%
    group_by(region) %>%
    summarise(
      total_revenue = sum(revenue),
      avg_revenue = mean(revenue),
      std_revenue = sd(revenue),
      total_quantity = sum(quantity),
      max_quantity = max(quantity),
      count = n(),
      .groups = 'drop'
    )
  
  cat("   Single column grouping (by region):\n")
  cat(sprintf("   Groups: %d\n", nrow(agg_single)))
  print(agg_single)
  
  # Multi-column grouping
  agg_multi <- df %>%
    group_by(region, product) %>%
    summarise(
      total_revenue = sum(revenue),
      avg_revenue = mean(revenue),
      total_quantity = sum(quantity),
      avg_cost = mean(cost),
      count = n(),
      .groups = 'drop'
    )
  
  cat(sprintf("\n   Multi-column grouping (by region, product):\n"))
  cat(sprintf("   Groups: %d\n", nrow(agg_multi)))
  
  # Custom aggregation function
  revenue_efficiency <- function(revenue, cost) {
    profit <- sum(revenue) - sum(cost)
    if (sum(cost) > 0) {
      return(profit / sum(cost) * 100)
    } else {
      return(0)
    }
  }
  
  agg_custom <- df %>%
    group_by(region, product) %>%
    summarise(
      efficiency_pct = revenue_efficiency(revenue, cost),
      .groups = 'drop'
    )
  
  cat("   Custom aggregation (efficiency):\n")
  cat(sprintf("   Average efficiency: %.2f%%\n", mean(agg_custom$efficiency_pct)))
  
  # Multiple summary statistics at once
  agg_advanced <- df %>%
    group_by(region) %>%
    summarise(
      across(c(revenue, quantity), list(
        sum = sum,
        mean = mean,
        median = median,
        q75 = ~quantile(., 0.75)
      )),
      .groups = 'drop'
    )
  
  return(agg_multi)
}

# 2. Join operations
demonstrate_joins <- function(sales, customers) {
  cat("\n2. JOIN OPERATIONS:\n")
  
  # Inner join
  inner_result <- inner_join(sales, customers, by = c("customer_id" = "id"))
  cat("   Inner join:\n")
  cat(sprintf("   Original sales rows: %d\n", nrow(sales)))
  cat(sprintf("   Matched rows: %d\n", nrow(inner_result)))
  
  # Left join
  left_result <- left_join(sales, customers, by = c("customer_id" = "id"))
  cat("\n   Left join:\n")
  cat(sprintf("   Result rows: %d\n", nrow(left_result)))
  cat(sprintf("   Unmatched sales: %d\n", sum(is.na(left_result$segment))))
  
  # Right join
  right_result <- right_join(sales, customers, by = c("customer_id" = "id"))
  cat("\n   Right join:\n")
  cat(sprintf("   Result rows: %d\n", nrow(right_result)))
  customers_with_no_sales <- length(unique(customers$id)) - 
    length(unique(na.omit(right_result$customer_id)))
  cat(sprintf("   Customers with no sales: %d\n", customers_with_no_sales))
  
  # Full outer join
  full_result <- full_join(sales, customers, by = c("customer_id" = "id"))
  cat("\n   Full outer join:\n")
  cat(sprintf("   Result rows: %d\n", nrow(full_result)))
  
  # Semi-join (customers who made purchases)
  semi_result <- semi_join(customers, sales, by = c("id" = "customer_id"))
  cat("\n   Semi-join (customers with sales):\n")
  cat(sprintf("   Active customers: %d\n", nrow(semi_result)))
  
  # Anti-join (customers who never purchased)
  anti_result <- anti_join(customers, sales, by = c("id" = "customer_id"))
  cat("\n   Anti-join (customers without sales):\n")
  cat(sprintf("   Inactive customers: %d\n", nrow(anti_result)))
  
  return(inner_result)
}

# 3. Pivot operations
demonstrate_pivot <- function(df) {
  cat("\n3. PIVOT OPERATIONS:\n")
  
  # Prepare data for pivot
  pivot_data <- df %>%
    group_by(date, product) %>%
    summarise(revenue = sum(revenue), .groups = 'drop')
  
  # Create pivot table (spread/pivot_wider)
  pivot_table <- pivot_data %>%
    pivot_wider(
      names_from = product,
      values_from = revenue,
      values_fill = 0
    )
  
  cat(sprintf("   Pivot table shape: %d x %d\n", nrow(pivot_table), ncol(pivot_table)))
  cat(sprintf("   Date range: %s to %s\n", min(pivot_table$date), max(pivot_table$date)))
  
  # Melt operation (gather/pivot_longer)
  melted <- pivot_table %>%
    pivot_longer(
      cols = -date,
      names_to = "product",
      values_to = "revenue"
    )
  cat(sprintf("   Melted shape: %d x %d\n", nrow(melted), ncol(melted)))
  
  return(pivot_table)
}

# 4. Window functions
demonstrate_window_functions <- function(df) {
  cat("\n4. WINDOW FUNCTIONS:\n")
  
  # Sort by date for time-based windows
  df_sorted <- df %>% arrange(date)
  
  # Rolling statistics (7-day window)
  window_size <- 7
  
  # Using slider package for rolling windows (if available)
  if (requireNamespace("slider", quietly = TRUE)) {
    library(slider)
    df_sorted <- df_sorted %>%
      mutate(
        rolling_mean = slide_dbl(revenue, mean, .before = window_size - 1, .complete = FALSE),
        rolling_std = slide_dbl(revenue, sd, .before = window_size - 1, .complete = FALSE),
        rolling_sum = slide_dbl(revenue, sum, .before = window_size - 1, .complete = FALSE)
      )
  } else {
    # Manual rolling window implementation
    n <- nrow(df_sorted)
    rolling_mean <- numeric(n)
    rolling_std <- numeric(n)
    
    for (i in 1:n) {
      window_start <- max(1, i - window_size + 1)
      window_data <- df_sorted$revenue[window_start:i]
      rolling_mean[i] <- mean(window_data)
      rolling_std[i] <- ifelse(length(window_data) > 1, sd(window_data), NA)
    }
    
    df_sorted$rolling_mean <- rolling_mean
    df_sorted$rolling_std <- rolling_std
  }
  
  cat(sprintf("   Rolling window size: %d\n", window_size))
  cat(sprintf("   Rolling mean range: %.2f to %.2f\n", 
              min(df_sorted$rolling_mean, na.rm = TRUE), 
              max(df_sorted$rolling_mean, na.rm = TRUE)))
  
  # Cumulative operations
  df_sorted <- df_sorted %>%
    mutate(
      cumsum_revenue = cumsum(revenue),
      cumprod_quantity_norm = cumprod(quantity / 100),
      cummax_revenue = cummax(revenue),
      cummin_revenue = cummin(revenue)
    )
  
  cat(sprintf("   Final cumulative revenue: %.2f\n", tail(df_sorted$cumsum_revenue, 1)))
  
  # Lag and lead operations
  df_sorted <- df_sorted %>%
    mutate(
      revenue_lag1 = lag(revenue, 1),
      revenue_lead1 = lead(revenue, 1),
      revenue_change = revenue - lag(revenue, 1),
      revenue_pct_change = (revenue - lag(revenue, 1)) / lag(revenue, 1) * 100
    )
  
  cat(sprintf("   Revenue changes computed: %d\n", sum(!is.na(df_sorted$revenue_change))))
  
  return(df_sorted)
}

# 5. Ranking operations
demonstrate_ranking <- function(df) {
  cat("\n5. RANKING OPERATIONS:\n")
  
  # Various ranking methods
  df <- df %>%
    mutate(
      revenue_rank = dense_rank(desc(revenue)),
      revenue_rank_min = min_rank(desc(revenue)),
      revenue_rank_row = row_number(desc(revenue)),
      revenue_pct_rank = percent_rank(revenue)
    )
  
  # Rank within groups
  df <- df %>%
    group_by(region) %>%
    mutate(revenue_rank_in_region = dense_rank(desc(revenue))) %>%
    ungroup()
  
  cat(sprintf("   Total ranks computed: %d\n", nrow(df)))
  cat(sprintf("   Top revenue rank count: %d\n", sum(df$revenue_rank == 1)))
  
  # Quantile ranks
  df$revenue_quartile <- cut(
    df$revenue,
    breaks = quantile(df$revenue, probs = seq(0, 1, 0.25)),
    labels = c("Q1", "Q2", "Q3", "Q4"),
    include.lowest = TRUE
  )
  
  cat("   Quartile distribution:\n")
  for (q in c("Q1", "Q2", "Q3", "Q4")) {
    count <- sum(df$revenue_quartile == q)
    cat(sprintf("     %s: %d records\n", q, count))
  }
  
  # N-tile function
  df$revenue_ntile <- ntile(df$revenue, 5)
  
  return(df)
}

# 6. Advanced aggregations
demonstrate_advanced_aggregations <- function(df) {
  cat("\n6. ADVANCED AGGREGATIONS:\n")
  
  # Weighted average
  weighted_avg_result <- df %>%
    group_by(region) %>%
    summarise(
      weighted_avg_revenue = sum(revenue * quantity) / sum(quantity),
      .groups = 'drop'
    )
  
  cat("   Weighted average revenue by region:\n")
  print(weighted_avg_result)
  
  # Mode calculation
  get_mode <- function(x) {
    uniq_x <- unique(x)
    uniq_x[which.max(tabulate(match(x, uniq_x)))]
  }
  
  mode_result <- df %>%
    group_by(region) %>%
    summarise(
      mode_product = get_mode(product),
      mode_count = sum(product == get_mode(product)),
      .groups = 'drop'
    )
  
  cat("\n   Most popular product by region:\n")
  print(mode_result)
  
  # Complex aggregations with multiple functions
  complex_agg <- df %>%
    group_by(region) %>%
    summarise(
      revenue_sum = sum(revenue),
      revenue_mean = mean(revenue),
      revenue_median = median(revenue),
      revenue_q75 = quantile(revenue, 0.75),
      quantity_sum = sum(quantity),
      quantity_std = sd(quantity),
      cost_min = min(cost),
      cost_max = max(cost),
      .groups = 'drop'
    )
  
  return(weighted_avg_result)
}

# Performance benchmark function
benchmark_operations <- function(n = 10000) {
  cat("\n7. PERFORMANCE BENCHMARK:\n")
  
  # Generate larger dataset
  large_df <- create_sales_dataframe(n)
  customers <- create_customers_dataframe(200)
  
  # Benchmark group-by
  start_time <- Sys.time()
  grouped <- large_df %>%
    group_by(region, product) %>%
    summarise(
      total = sum(revenue),
      avg_qty = mean(quantity),
      .groups = 'drop'
    )
  groupby_time <- as.numeric(Sys.time() - start_time)
  cat(sprintf("   Group-by time (%d rows): %.2fms\n", n, groupby_time * 1000))
  
  # Benchmark join
  start_time <- Sys.time()
  joined <- inner_join(large_df, customers, by = c("customer_id" = "id"))
  join_time <- as.numeric(Sys.time() - start_time)
  cat(sprintf("   Inner join time (%d rows): %.2fms\n", n, join_time * 1000))
  
  # Benchmark pivot
  start_time <- Sys.time()
  pivot_data <- large_df %>%
    group_by(date, product) %>%
    summarise(revenue = sum(revenue), .groups = 'drop') %>%
    pivot_wider(names_from = product, values_from = revenue, values_fill = 0)
  pivot_time <- as.numeric(Sys.time() - start_time)
  cat(sprintf("   Pivot time (%d rows): %.2fms\n", n, pivot_time * 1000))
  
  return(list(
    groupby = groupby_time,
    join = join_time,
    pivot = pivot_time
  ))
}

# Main demonstration function
main <- function() {
  cat("=== DataFrame Advanced Operations in R (dplyr/tidyverse) ===\n")
  
  # Create sample datasets
  sales_df <- create_sales_dataframe(1000)
  customers_df <- create_customers_dataframe(200)
  
  cat("\nDataset shapes:\n")
  cat(sprintf("  Sales: %d x %d\n", nrow(sales_df), ncol(sales_df)))
  cat(sprintf("  Customers: %d x %d\n", nrow(customers_df), ncol(customers_df)))
  
  # Demonstrate all operations
  grouped_result <- demonstrate_groupby(sales_df)
  join_result <- demonstrate_joins(sales_df, customers_df)
  pivot_result <- demonstrate_pivot(sales_df)
  window_result <- demonstrate_window_functions(sales_df)
  rank_result <- demonstrate_ranking(sales_df)
  advanced_result <- demonstrate_advanced_aggregations(sales_df)
  
  # Run performance benchmark
  benchmark_results <- benchmark_operations(10000)
  
  cat("\n=== All DataFrame operations completed successfully ===\n")
  
  return(list(
    grouped = grouped_result,
    joined = join_result,
    pivot = pivot_result,
    windowed = window_result,
    ranked = rank_result,
    advanced = advanced_result,
    benchmark = benchmark_results
  ))
}

# Run if executed directly
if (!interactive()) {
  results <- main()
}