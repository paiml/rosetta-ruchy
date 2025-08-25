#!/usr/bin/env python3

"""
DataFrame Advanced Operations - Sprint 26
Group-by operations, joins, and aggregations
Python implementation using pandas
"""

import pandas as pd
import numpy as np
from datetime import datetime, timedelta
import time
from typing import Dict, List, Tuple, Any

# Set random seed for reproducibility
np.random.seed(42)

def create_sales_dataframe(n: int = 1000) -> pd.DataFrame:
    """Generate sample sales data"""
    regions = ['North', 'South', 'East', 'West']
    products = ['A', 'B', 'C', 'D', 'E']
    
    base_date = datetime(2024, 1, 1)
    
    df = pd.DataFrame({
        'order_id': range(1, n + 1),
        'date': [base_date + timedelta(days=np.random.randint(0, 365)) for _ in range(n)],
        'region': np.random.choice(regions, n),
        'product': np.random.choice(products, n),
        'customer_id': np.random.randint(1, 201, n),
        'quantity': np.random.randint(1, 101, n),
        'revenue': np.round(np.random.uniform(100, 10000, n) + np.random.randn(n) * 100, 2),
        'cost': np.round(np.random.uniform(50, 5000, n) + np.random.randn(n) * 50, 2)
    })
    
    return df

def create_customers_dataframe(n: int = 200) -> pd.DataFrame:
    """Generate sample customer data"""
    segments = ['Premium', 'Standard', 'Basic']
    
    base_date = datetime(2020, 1, 1)
    
    df = pd.DataFrame({
        'id': range(1, n + 1),
        'name': [f'Customer_{i}' for i in range(1, n + 1)],
        'segment': np.random.choice(segments, n),
        'join_date': [base_date + timedelta(days=np.random.randint(0, 1461)) for _ in range(n)],
        'credit_limit': np.random.randint(1000, 50001, n)
    })
    
    return df

def demonstrate_groupby(df: pd.DataFrame) -> pd.DataFrame:
    """Demonstrate group-by operations with multiple aggregations"""
    print("\n1. GROUP-BY OPERATIONS:")
    
    # Single column grouping with multiple aggregations
    agg_single = df.groupby('region').agg({
        'revenue': ['sum', 'mean', 'std'],
        'quantity': ['sum', 'max'],
        'order_id': 'count'
    })
    agg_single.columns = ['_'.join(col).strip() for col in agg_single.columns.values]
    agg_single = agg_single.rename(columns={'order_id_count': 'count'})
    
    print("   Single column grouping (by region):")
    print(f"   Groups: {len(agg_single)}")
    print(agg_single.head())
    
    # Multi-column grouping
    agg_multi = df.groupby(['region', 'product']).agg({
        'revenue': ['sum', 'mean'],
        'quantity': 'sum',
        'cost': 'mean',
        'order_id': 'count'
    })
    agg_multi.columns = ['_'.join(col).strip() for col in agg_multi.columns.values]
    agg_multi = agg_multi.rename(columns={'order_id_count': 'count'})
    agg_multi = agg_multi.reset_index()
    
    print(f"\n   Multi-column grouping (by region, product):")
    print(f"   Groups: {len(agg_multi)}")
    
    # Custom aggregation function
    def revenue_efficiency(group):
        profit = group['revenue'].sum() - group['cost'].sum()
        return profit / group['cost'].sum() * 100 if group['cost'].sum() > 0 else 0
    
    agg_custom = df.groupby(['region', 'product']).apply(revenue_efficiency)
    agg_custom = agg_custom.reset_index(name='efficiency_pct')
    
    print(f"   Custom aggregation (efficiency):")
    print(f"   Average efficiency: {agg_custom['efficiency_pct'].mean():.2f}%")
    
    # Named aggregations (pandas >= 0.25)
    agg_named = df.groupby('region').agg(
        total_revenue=('revenue', 'sum'),
        avg_revenue=('revenue', 'mean'),
        total_quantity=('quantity', 'sum'),
        max_quantity=('quantity', 'max'),
        order_count=('order_id', 'count')
    )
    
    return agg_multi

def demonstrate_joins(sales: pd.DataFrame, customers: pd.DataFrame) -> pd.DataFrame:
    """Demonstrate various join operations"""
    print("\n2. JOIN OPERATIONS:")
    
    # Inner join
    inner_result = pd.merge(sales, customers, left_on='customer_id', right_on='id', how='inner')
    print(f"   Inner join:")
    print(f"   Original sales rows: {len(sales)}")
    print(f"   Matched rows: {len(inner_result)}")
    
    # Left join
    left_result = pd.merge(sales, customers, left_on='customer_id', right_on='id', how='left')
    print(f"\n   Left join:")
    print(f"   Result rows: {len(left_result)}")
    print(f"   Unmatched sales: {left_result['segment'].isna().sum()}")
    
    # Right join
    right_result = pd.merge(sales, customers, left_on='customer_id', right_on='id', how='right')
    print(f"\n   Right join:")
    print(f"   Result rows: {len(right_result)}")
    customers_with_no_sales = len(customers['id'].unique()) - len(right_result['customer_id'].dropna().unique())
    print(f"   Customers with no sales: {customers_with_no_sales}")
    
    # Full outer join
    full_result = pd.merge(sales, customers, left_on='customer_id', right_on='id', how='outer')
    print(f"\n   Full outer join:")
    print(f"   Result rows: {len(full_result)}")
    
    # Semi-join (customers who made purchases) - using isin
    active_customer_ids = sales['customer_id'].unique()
    semi_result = customers[customers['id'].isin(active_customer_ids)]
    print(f"\n   Semi-join (customers with sales):")
    print(f"   Active customers: {len(semi_result)}")
    
    # Anti-join (customers who never purchased) - using ~isin
    anti_result = customers[~customers['id'].isin(active_customer_ids)]
    print(f"\n   Anti-join (customers without sales):")
    print(f"   Inactive customers: {len(anti_result)}")
    
    # Cross join (cartesian product) - for small example
    small_sales = sales.head(5)
    small_customers = customers.head(3)
    small_sales['key'] = 1
    small_customers['key'] = 1
    cross_result = pd.merge(small_sales, small_customers, on='key').drop('key', axis=1)
    print(f"\n   Cross join (5x3 example):")
    print(f"   Result rows: {len(cross_result)}")
    
    return inner_result

def demonstrate_pivot(df: pd.DataFrame) -> pd.DataFrame:
    """Demonstrate pivot operations"""
    print("\n3. PIVOT OPERATIONS:")
    
    # Prepare data for pivot
    pivot_data = df.groupby(['date', 'product'])['revenue'].sum().reset_index()
    
    # Create pivot table
    pivot_table = pivot_data.pivot(index='date', columns='product', values='revenue')
    pivot_table = pivot_table.fillna(0)
    
    print(f"   Pivot table shape: {pivot_table.shape}")
    print(f"   Date range: {pivot_table.index.min()} to {pivot_table.index.max()}")
    
    # Alternative: pivot_table function with aggregation
    pivot_table_agg = pd.pivot_table(
        df,
        values='revenue',
        index='date',
        columns='product',
        aggfunc='sum',
        fill_value=0
    )
    
    # Melt operation (reverse of pivot)
    melted = pivot_table.reset_index().melt(
        id_vars='date',
        var_name='product',
        value_name='revenue'
    )
    print(f"   Melted shape: {melted.shape}")
    
    # Multi-level pivot
    multi_pivot = pd.pivot_table(
        df,
        values=['revenue', 'quantity'],
        index='region',
        columns='product',
        aggfunc={'revenue': 'sum', 'quantity': 'mean'},
        fill_value=0
    )
    print(f"   Multi-level pivot shape: {multi_pivot.shape}")
    
    return pivot_table

def demonstrate_window_functions(df: pd.DataFrame) -> pd.DataFrame:
    """Demonstrate window functions and rolling operations"""
    print("\n4. WINDOW FUNCTIONS:")
    
    # Sort by date for time-based windows
    df_sorted = df.sort_values('date').copy()
    
    # Rolling statistics (7-day window)
    window_size = 7
    df_sorted['rolling_mean'] = df_sorted['revenue'].rolling(window=window_size, min_periods=1).mean()
    df_sorted['rolling_std'] = df_sorted['revenue'].rolling(window=window_size, min_periods=1).std()
    df_sorted['rolling_sum'] = df_sorted['revenue'].rolling(window=window_size, min_periods=1).sum()
    
    print(f"   Rolling window size: {window_size}")
    print(f"   Rolling mean range: {df_sorted['rolling_mean'].min():.2f} to {df_sorted['rolling_mean'].max():.2f}")
    
    # Expanding window (cumulative from start)
    df_sorted['expanding_mean'] = df_sorted['revenue'].expanding().mean()
    df_sorted['expanding_max'] = df_sorted['revenue'].expanding().max()
    
    # Cumulative operations
    df_sorted['cumsum_revenue'] = df_sorted['revenue'].cumsum()
    df_sorted['cumprod_quantity_norm'] = (df_sorted['quantity'] / 100).cumprod()
    df_sorted['cummax_revenue'] = df_sorted['revenue'].cummax()
    
    print(f"   Final cumulative revenue: {df_sorted['cumsum_revenue'].iloc[-1]:.2f}")
    
    # Lag and lead operations
    df_sorted['revenue_lag1'] = df_sorted['revenue'].shift(1)
    df_sorted['revenue_lead1'] = df_sorted['revenue'].shift(-1)
    df_sorted['revenue_change'] = df_sorted['revenue'].diff()
    df_sorted['revenue_pct_change'] = df_sorted['revenue'].pct_change()
    
    print(f"   Revenue changes computed: {df_sorted['revenue_change'].notna().sum()}")
    
    # Exponentially weighted moving average
    df_sorted['ewma_revenue'] = df_sorted['revenue'].ewm(span=7, adjust=False).mean()
    
    return df_sorted

def demonstrate_ranking(df: pd.DataFrame) -> pd.DataFrame:
    """Demonstrate ranking operations"""
    print("\n5. RANKING OPERATIONS:")
    
    df = df.copy()
    
    # Various ranking methods
    df['revenue_rank'] = df['revenue'].rank(method='dense', ascending=False)
    df['revenue_rank_min'] = df['revenue'].rank(method='min', ascending=False)
    df['revenue_rank_avg'] = df['revenue'].rank(method='average', ascending=False)
    
    # Percent rank
    df['revenue_pct_rank'] = df['revenue'].rank(pct=True)
    
    # Rank within groups
    df['revenue_rank_in_region'] = df.groupby('region')['revenue'].rank(method='dense', ascending=False)
    
    print(f"   Total ranks computed: {len(df)}")
    print(f"   Top revenue rank count: {(df['revenue_rank'] == 1).sum()}")
    
    # Quantile ranks
    df['revenue_quartile'] = pd.qcut(df['revenue'], q=4, labels=['Q1', 'Q2', 'Q3', 'Q4'])
    
    # Decile ranks
    df['revenue_decile'] = pd.qcut(df['revenue'], q=10, labels=False) + 1
    
    print("   Quartile distribution:")
    for q in ['Q1', 'Q2', 'Q3', 'Q4']:
        count = (df['revenue_quartile'] == q).sum()
        print(f"     {q}: {count} records")
    
    # N-tile with custom bins
    df['revenue_ntile'] = pd.cut(df['revenue'], bins=5, labels=['Very Low', 'Low', 'Medium', 'High', 'Very High'])
    
    return df

def demonstrate_advanced_aggregations(df: pd.DataFrame) -> pd.DataFrame:
    """Demonstrate advanced aggregation techniques"""
    print("\n6. ADVANCED AGGREGATIONS:")
    
    # Weighted average
    def weighted_avg(group):
        return (group['revenue'] * group['quantity']).sum() / group['quantity'].sum()
    
    weighted_avg_result = df.groupby('region').apply(weighted_avg).reset_index(name='weighted_avg_revenue')
    
    print("   Weighted average revenue by region:")
    print(weighted_avg_result)
    
    # Mode calculation
    mode_result = df.groupby('region')['product'].agg(lambda x: x.mode()[0] if len(x.mode()) > 0 else None)
    mode_result = mode_result.reset_index(name='mode_product')
    
    # Count of mode occurrences
    for region in mode_result['region']:
        product = mode_result[mode_result['region'] == region]['mode_product'].values[0]
        count = len(df[(df['region'] == region) & (df['product'] == product)])
        mode_result.loc[mode_result['region'] == region, 'mode_count'] = count
    
    print("\n   Most popular product by region:")
    print(mode_result)
    
    # Multiple aggregations with different functions
    complex_agg = df.groupby('region').agg({
        'revenue': ['sum', 'mean', 'median', lambda x: x.quantile(0.75)],
        'quantity': ['sum', 'std'],
        'cost': ['min', 'max']
    })
    
    # Flatten column names
    complex_agg.columns = ['_'.join(col).strip() for col in complex_agg.columns.values]
    
    return weighted_avg_result

def benchmark_operations(n: int = 10000) -> Dict[str, float]:
    """Benchmark performance of various operations"""
    print("\n7. PERFORMANCE BENCHMARK:")
    
    # Generate larger dataset
    large_df = create_sales_dataframe(n)
    customers = create_customers_dataframe(200)
    
    results = {}
    
    # Benchmark group-by
    start_time = time.time()
    grouped = large_df.groupby(['region', 'product']).agg({
        'revenue': 'sum',
        'quantity': 'mean'
    })
    results['groupby'] = time.time() - start_time
    print(f"   Group-by time ({n} rows): {results['groupby'] * 1000:.2f}ms")
    
    # Benchmark join
    start_time = time.time()
    joined = pd.merge(large_df, customers, left_on='customer_id', right_on='id', how='inner')
    results['join'] = time.time() - start_time
    print(f"   Inner join time ({n} rows): {results['join'] * 1000:.2f}ms")
    
    # Benchmark pivot
    start_time = time.time()
    pivot = pd.pivot_table(
        large_df,
        values='revenue',
        index='date',
        columns='product',
        aggfunc='sum',
        fill_value=0
    )
    results['pivot'] = time.time() - start_time
    print(f"   Pivot time ({n} rows): {results['pivot'] * 1000:.2f}ms")
    
    # Benchmark rolling window
    start_time = time.time()
    large_df_sorted = large_df.sort_values('date')
    rolling = large_df_sorted['revenue'].rolling(window=7, min_periods=1).mean()
    results['rolling'] = time.time() - start_time
    print(f"   Rolling window time ({n} rows): {results['rolling'] * 1000:.2f}ms")
    
    return results

def main():
    """Main demonstration function"""
    print("=== DataFrame Advanced Operations in Python (pandas) ===")
    
    # Create sample datasets
    sales_df = create_sales_dataframe(1000)
    customers_df = create_customers_dataframe(200)
    
    print(f"\nDataset shapes:")
    print(f"  Sales: {sales_df.shape}")
    print(f"  Customers: {customers_df.shape}")
    
    # Demonstrate all operations
    grouped_result = demonstrate_groupby(sales_df)
    join_result = demonstrate_joins(sales_df, customers_df)
    pivot_result = demonstrate_pivot(sales_df)
    window_result = demonstrate_window_functions(sales_df)
    rank_result = demonstrate_ranking(sales_df)
    advanced_result = demonstrate_advanced_aggregations(sales_df)
    
    # Run performance benchmark
    benchmark_results = benchmark_operations(10000)
    
    print("\n=== All DataFrame operations completed successfully ===")
    
    return {
        'grouped': grouped_result,
        'joined': join_result,
        'pivot': pivot_result,
        'windowed': window_result,
        'ranked': rank_result,
        'advanced': advanced_result,
        'benchmark': benchmark_results
    }

if __name__ == "__main__":
    results = main()