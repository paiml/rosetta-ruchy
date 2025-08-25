#!/usr/bin/env julia

# DataFrame Advanced Operations - Sprint 26
# Group-by operations, joins, and aggregations
# Julia implementation using DataFrames.jl

using DataFrames
using Statistics
using Random
using Dates

# Set random seed for reproducibility
Random.seed!(42)

# Generate sample sales data
function create_sales_dataframe(n::Int=1000)
    regions = ["North", "South", "East", "West"]
    products = ["A", "B", "C", "D", "E"]
    
    DataFrame(
        order_id = 1:n,
        date = Date(2024, 1, 1) .+ Day.(rand(0:364, n)),
        region = rand(regions, n),
        product = rand(products, n),
        customer_id = rand(1:200, n),
        quantity = rand(1:100, n),
        revenue = round.(rand(100:10000, n) .+ randn(n) .* 100, digits=2),
        cost = round.(rand(50:5000, n) .+ randn(n) .* 50, digits=2)
    )
end

# Generate sample customer data
function create_customers_dataframe(n::Int=200)
    segments = ["Premium", "Standard", "Basic"]
    
    DataFrame(
        id = 1:n,
        name = ["Customer_$i" for i in 1:n],
        segment = rand(segments, n),
        join_date = Date(2020, 1, 1) .+ Day.(rand(0:1460, n)),
        credit_limit = rand(1000:50000, n)
    )
end

# 1. Group-by operations with multiple aggregations
function demonstrate_groupby(df::DataFrame)
    println("\n1. GROUP-BY OPERATIONS:")
    
    # Single column grouping with multiple aggregations
    grouped_single = groupby(df, :region)
    agg_single = combine(grouped_single,
        :revenue => sum => :total_revenue,
        :revenue => mean => :avg_revenue,
        :revenue => std => :std_revenue,
        :quantity => sum => :total_quantity,
        :quantity => maximum => :max_quantity,
        nrow => :count
    )
    
    println("   Single column grouping (by region):")
    println("   Groups: $(nrow(agg_single))")
    show(agg_single, allrows=false, allcols=true)
    
    # Multi-column grouping
    grouped_multi = groupby(df, [:region, :product])
    agg_multi = combine(grouped_multi,
        :revenue => sum => :total_revenue,
        :revenue => mean => :avg_revenue,
        :quantity => sum => :total_quantity,
        :cost => mean => :avg_cost,
        nrow => :count
    )
    
    println("\n   Multi-column grouping (by region, product):")
    println("   Groups: $(nrow(agg_multi))")
    
    # Custom aggregation function
    function revenue_efficiency(revenue, cost)
        profit = revenue .- cost
        return sum(profit) / sum(cost) * 100
    end
    
    agg_custom = combine(grouped_multi,
        [:revenue, :cost] => revenue_efficiency => :efficiency_pct
    )
    
    println("   Custom aggregation (efficiency):")
    println("   Average efficiency: $(round(mean(agg_custom.efficiency_pct), digits=2))%")
    
    return agg_multi
end

# 2. Join operations
function demonstrate_joins(sales::DataFrame, customers::DataFrame)
    println("\n2. JOIN OPERATIONS:")
    
    # Inner join
    inner_result = innerjoin(sales, customers, on = :customer_id => :id)
    println("   Inner join:")
    println("   Original sales rows: $(nrow(sales))")
    println("   Matched rows: $(nrow(inner_result))")
    
    # Left join
    left_result = leftjoin(sales, customers, on = :customer_id => :id)
    println("\n   Left join:")
    println("   Result rows: $(nrow(left_result))")
    println("   Unmatched sales: $(sum(ismissing.(left_result.segment)))")
    
    # Right join
    right_result = rightjoin(sales, customers, on = :customer_id => :id)
    println("\n   Right join:")
    println("   Result rows: $(nrow(right_result))")
    println("   Customers with no sales: $(length(unique(customers.id)) - length(unique(skipmissing(right_result.customer_id))))")
    
    # Full outer join
    full_result = outerjoin(sales, customers, on = :customer_id => :id)
    println("\n   Full outer join:")
    println("   Result rows: $(nrow(full_result))")
    
    # Semi-join (customers who made purchases)
    semi_result = semijoin(customers, sales, on = :id => :customer_id)
    println("\n   Semi-join (customers with sales):")
    println("   Active customers: $(nrow(semi_result))")
    
    # Anti-join (customers who never purchased)
    anti_result = antijoin(customers, sales, on = :id => :customer_id)
    println("\n   Anti-join (customers without sales):")
    println("   Inactive customers: $(nrow(anti_result))")
    
    return inner_result
end

# 3. Pivot operations
function demonstrate_pivot(df::DataFrame)
    println("\n3. PIVOT OPERATIONS:")
    
    # Prepare data for pivot
    pivot_data = combine(groupby(df, [:date, :product]),
        :revenue => sum => :revenue
    )
    
    # Create pivot table (unstack)
    pivot_table = unstack(pivot_data, :date, :product, :revenue, fill=0.0)
    
    println("   Pivot table shape: $(size(pivot_table))")
    println("   Date range: $(minimum(pivot_table.date)) to $(maximum(pivot_table.date))")
    
    # Melt operation (reverse of pivot)
    melted = stack(pivot_table, Not(:date), variable_name=:product, value_name=:revenue)
    println("   Melted shape: $(size(melted))")
    
    return pivot_table
end

# 4. Window functions
function demonstrate_window_functions(df::DataFrame)
    println("\n4. WINDOW FUNCTIONS:")
    
    # Sort by date for time-based windows
    sorted_df = sort(df, :date)
    
    # Rolling statistics (7-day window)
    window_size = 7
    rolling_mean = Float64[]
    rolling_std = Float64[]
    
    for i in 1:nrow(sorted_df)
        window_start = max(1, i - window_size + 1)
        window_data = sorted_df.revenue[window_start:i]
        push!(rolling_mean, mean(window_data))
        push!(rolling_std, std(window_data))
    end
    
    sorted_df.rolling_mean = rolling_mean
    sorted_df.rolling_std = rolling_std
    
    println("   Rolling window size: $window_size")
    println("   Rolling mean range: $(round(minimum(rolling_mean), digits=2)) to $(round(maximum(rolling_mean), digits=2))")
    
    # Cumulative operations
    sorted_df.cumsum_revenue = cumsum(sorted_df.revenue)
    sorted_df.cumprod_quantity = cumprod(sorted_df.quantity ./ 100)  # Normalized
    
    println("   Final cumulative revenue: $(round(sorted_df.cumsum_revenue[end], digits=2))")
    
    # Lag and lead operations
    sorted_df.revenue_lag1 = lag(sorted_df.revenue, 1)
    sorted_df.revenue_lead1 = lead(sorted_df.revenue, 1)
    sorted_df.revenue_change = sorted_df.revenue .- lag(sorted_df.revenue, 1, default=0)
    
    println("   Revenue changes computed: $(sum(.!ismissing.(sorted_df.revenue_change)))")
    
    return sorted_df
end

# 5. Ranking operations
function demonstrate_ranking(df::DataFrame)
    println("\n5. RANKING OPERATIONS:")
    
    # Dense rank
    df.revenue_rank = denserank(df.revenue, rev=true)
    
    # Percent rank
    df.revenue_pct_rank = percent_rank(df.revenue)
    
    # Rank within groups
    grouped = groupby(df, :region)
    ranked = combine(grouped) do gdf
        gdf.revenue_rank_in_region = denserank(gdf.revenue, rev=true)
        gdf
    end
    
    println("   Total ranks computed: $(nrow(df))")
    println("   Top revenue rank count: $(sum(df.revenue_rank .== 1))")
    
    # Quantile ranks
    df.revenue_quartile = cut(df.revenue, 4, labels=["Q1", "Q2", "Q3", "Q4"])
    
    println("   Quartile distribution:")
    for q in ["Q1", "Q2", "Q3", "Q4"]
        count = sum(df.revenue_quartile .== q)
        println("     $q: $count records")
    end
    
    return df
end

# Helper function for percent rank
function percent_rank(x::Vector)
    n = length(x)
    ranks = ordinalrank(x)
    return (ranks .- 1) ./ (n - 1)
end

# Helper function for ordinal rank
function ordinalrank(x::Vector)
    sorted_indices = sortperm(x)
    ranks = similar(x, Int)
    for (i, idx) in enumerate(sorted_indices)
        ranks[idx] = i
    end
    return ranks
end

# 6. Advanced aggregations
function demonstrate_advanced_aggregations(df::DataFrame)
    println("\n6. ADVANCED AGGREGATIONS:")
    
    # Weighted average
    grouped = groupby(df, :region)
    weighted_avg = combine(grouped) do gdf
        total_weight = sum(gdf.quantity)
        weighted_sum = sum(gdf.revenue .* gdf.quantity)
        DataFrame(weighted_avg_revenue = weighted_sum / total_weight)
    end
    
    println("   Weighted average revenue by region:")
    show(weighted_avg, allrows=true)
    
    # Mode calculation
    mode_product = combine(groupby(df, :region)) do gdf
        product_counts = countmap(gdf.product)
        mode_prod = argmax(product_counts)
        DataFrame(mode_product = mode_prod, mode_count = product_counts[mode_prod])
    end
    
    println("\n   Most popular product by region:")
    show(mode_product, allrows=true)
    
    return weighted_avg
end

# Helper function for counting
function countmap(x)
    counts = Dict{eltype(x), Int}()
    for val in x
        counts[val] = get(counts, val, 0) + 1
    end
    return counts
end

# Performance benchmark function
function benchmark_operations(n::Int=10000)
    println("\n7. PERFORMANCE BENCHMARK:")
    
    # Generate larger dataset
    large_df = create_sales_dataframe(n)
    
    # Benchmark group-by
    start_time = time()
    grouped = groupby(large_df, [:region, :product])
    agg = combine(grouped,
        :revenue => sum => :total,
        :quantity => mean => :avg_qty
    )
    groupby_time = time() - start_time
    
    println("   Group-by time ($n rows): $(round(groupby_time * 1000, digits=2))ms")
    
    # Benchmark join
    customers = create_customers_dataframe(200)
    start_time = time()
    joined = innerjoin(large_df, customers, on = :customer_id => :id)
    join_time = time() - start_time
    
    println("   Inner join time ($n rows): $(round(join_time * 1000, digits=2))ms")
    
    # Benchmark pivot
    start_time = time()
    pivot_data = combine(groupby(large_df, [:date, :product]), :revenue => sum => :revenue)
    pivot = unstack(pivot_data, :date, :product, :revenue, fill=0.0)
    pivot_time = time() - start_time
    
    println("   Pivot time ($n rows): $(round(pivot_time * 1000, digits=2))ms")
    
    return Dict(
        "groupby" => groupby_time,
        "join" => join_time,
        "pivot" => pivot_time
    )
end

# Main demonstration function
function main()
    println("=== DataFrame Advanced Operations in Julia ===")
    
    # Create sample datasets
    sales_df = create_sales_dataframe(1000)
    customers_df = create_customers_dataframe(200)
    
    println("\nDataset shapes:")
    println("  Sales: $(size(sales_df))")
    println("  Customers: $(size(customers_df))")
    
    # Demonstrate all operations
    grouped_result = demonstrate_groupby(sales_df)
    join_result = demonstrate_joins(sales_df, customers_df)
    pivot_result = demonstrate_pivot(sales_df)
    window_result = demonstrate_window_functions(sales_df)
    rank_result = demonstrate_ranking(sales_df)
    advanced_result = demonstrate_advanced_aggregations(sales_df)
    
    # Run performance benchmark
    benchmark_results = benchmark_operations(10000)
    
    println("\n=== All DataFrame operations completed successfully ===")
    
    return Dict(
        "grouped" => grouped_result,
        "joined" => join_result,
        "pivot" => pivot_result,
        "windowed" => window_result,
        "ranked" => rank_result,
        "advanced" => advanced_result,
        "benchmark" => benchmark_results
    )
end

# Run if executed directly
if abspath(PROGRAM_FILE) == @__FILE__
    results = main()
end