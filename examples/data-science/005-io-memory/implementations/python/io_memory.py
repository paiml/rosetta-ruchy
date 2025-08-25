#!/usr/bin/env python3

"""
I/O Operations and Memory Optimization - Sprint 27
Efficient data I/O and memory management
Python implementation using pandas, pyarrow, and memory optimization
"""

import pandas as pd
import numpy as np
import pyarrow as pa
import pyarrow.parquet as pq
import json
import time
import os
import gc
from io import StringIO
from typing import Iterator, Tuple, Optional
import psutil

# Memory limits
MAX_MEMORY_MB = 1024
CHUNK_SIZE = 10000

def get_memory_usage():
    """Get current memory usage in MB"""
    process = psutil.Process(os.getpid())
    return process.memory_info().rss / 1024 / 1024

def read_csv_chunked(filepath: str, chunksize: int = CHUNK_SIZE) -> Tuple[int, int, float]:
    """Read CSV in chunks for memory efficiency"""
    print("\n1. CHUNKED CSV READING:")
    
    chunks_read = 0
    total_rows = 0
    peak_memory = 0
    
    for chunk in pd.read_csv(filepath, chunksize=chunksize):
        chunks_read += 1
        total_rows += len(chunk)
        
        # Process chunk
        chunk_sum = chunk.select_dtypes(include=[np.number]).sum().sum()
        
        # Track memory
        current_memory = get_memory_usage()
        peak_memory = max(peak_memory, current_memory)
        
        # Clear chunk
        del chunk
        gc.collect()
    
    print(f"   Chunks processed: {chunks_read}")
    print(f"   Total rows: {total_rows}")
    print(f"   Peak memory: {peak_memory:.2f} MB")
    
    return chunks_read, total_rows, peak_memory

def optimize_dtypes(df: pd.DataFrame) -> pd.DataFrame:
    """Optimize DataFrame dtypes for memory efficiency"""
    print("\n2. DTYPE OPTIMIZATION:")
    
    original_memory = df.memory_usage(deep=True).sum() / 1024 / 1024
    
    for col in df.columns:
        col_type = df[col].dtype
        
        if col_type != 'object':
            c_min = df[col].min()
            c_max = df[col].max()
            
            if str(col_type)[:3] == 'int':
                if c_min > np.iinfo(np.int8).min and c_max < np.iinfo(np.int8).max:
                    df[col] = df[col].astype(np.int8)
                elif c_min > np.iinfo(np.int16).min and c_max < np.iinfo(np.int16).max:
                    df[col] = df[col].astype(np.int16)
                elif c_min > np.iinfo(np.int32).min and c_max < np.iinfo(np.int32).max:
                    df[col] = df[col].astype(np.int32)
            else:
                if c_min > np.finfo(np.float32).min and c_max < np.finfo(np.float32).max:
                    df[col] = df[col].astype(np.float32)
        else:
            # Convert string to category if less than 50% unique
            num_unique = len(df[col].unique())
            if num_unique / len(df[col]) < 0.5:
                df[col] = df[col].astype('category')
    
    optimized_memory = df.memory_usage(deep=True).sum() / 1024 / 1024
    
    print(f"   Original memory: {original_memory:.2f} MB")
    print(f"   Optimized memory: {optimized_memory:.2f} MB")
    print(f"   Reduction: {(1 - optimized_memory/original_memory)*100:.1f}%")
    
    return df

def parquet_operations(df: pd.DataFrame) -> Tuple[float, float, int]:
    """Demonstrate Parquet format I/O"""
    print("\n3. PARQUET FORMAT:")
    
    # Write Parquet
    start = time.time()
    df.to_parquet('data.parquet', compression='snappy')
    write_time = time.time() - start
    
    # Read Parquet
    start = time.time()
    df_parquet = pd.read_parquet('data.parquet')
    read_time = time.time() - start
    
    file_size = os.path.getsize('data.parquet')
    
    print(f"   Write time: {write_time*1000:.2f} ms")
    print(f"   Read time: {read_time*1000:.2f} ms")
    print(f"   File size: {file_size/1024/1024:.2f} MB")
    
    # Clean up
    os.remove('data.parquet')
    
    return write_time, read_time, file_size

def streaming_aggregation(filepath: str, chunksize: int = CHUNK_SIZE):
    """Perform aggregation with constant memory"""
    print("\n4. STREAMING AGGREGATION:")
    
    sum_total = 0
    count_total = 0
    mean_accumulator = 0
    chunks = 0
    
    for chunk in pd.read_csv(filepath, chunksize=chunksize):
        numeric_cols = chunk.select_dtypes(include=[np.number])
        
        chunk_sum = numeric_cols.sum().sum()
        chunk_count = numeric_cols.count().sum()
        
        sum_total += chunk_sum
        count_total += chunk_count
        
        chunks += 1
        chunk_mean = chunk_sum / chunk_count if chunk_count > 0 else 0
        delta = chunk_mean - mean_accumulator
        mean_accumulator += delta / chunks
    
    final_mean = sum_total / count_total if count_total > 0 else 0
    
    print(f"   Total sum: {sum_total}")
    print(f"   Total count: {count_total}")
    print(f"   Mean: {final_mean:.4f}")
    print(f"   Memory: O(1) constant")

class MemoryPool:
    """Memory pool for buffer reuse"""
    def __init__(self, pool_size: int, buffer_size: int):
        self.buffers = [np.zeros(buffer_size) for _ in range(pool_size)]
        self.available = [True] * pool_size
        self.buffer_size = buffer_size
    
    def acquire(self) -> Optional[Tuple[int, np.ndarray]]:
        for i, avail in enumerate(self.available):
            if avail:
                self.available[i] = False
                return i, self.buffers[i]
        return None
    
    def release(self, idx: int):
        self.available[idx] = True

def demonstrate_memory_pool():
    """Demonstrate memory pool usage"""
    print("\n5. MEMORY POOL:")
    
    pool = MemoryPool(10, 1000)
    
    # Acquire buffers
    acquired = []
    for i in range(5):
        result = pool.acquire()
        if result:
            idx, buffer = result
            acquired.append(idx)
            buffer.fill(i)
    
    print(f"   Buffers acquired: {len(acquired)}")
    print(f"   Available: {sum(pool.available)}")
    
    # Release buffers
    for idx in acquired:
        pool.release(idx)
    
    print(f"   After release: {sum(pool.available)} available")
    
    total_memory = len(pool.buffers) * pool.buffer_size * 8 / 1024 / 1024
    print(f"   Pool memory: {total_memory:.2f} MB")

def jsonl_streaming(filepath: str, n_records: int = 1000):
    """JSON Lines format for streaming"""
    print("\n6. JSON LINES STREAMING:")
    
    # Write JSONL
    start = time.time()
    with open(filepath, 'w') as f:
        for i in range(n_records):
            record = {
                'id': i,
                'value': np.random.random(),
                'category': np.random.choice(['A', 'B', 'C'])
            }
            f.write(json.dumps(record) + '\n')
    write_time = time.time() - start
    
    # Stream read JSONL
    start = time.time()
    count = 0
    sum_values = 0
    
    with open(filepath, 'r') as f:
        for line in f:
            record = json.loads(line)
            count += 1
            sum_values += record['value']
    
    read_time = time.time() - start
    
    print(f"   Records: {n_records}")
    print(f"   Write time: {write_time*1000:.2f} ms")
    print(f"   Read time: {read_time*1000:.2f} ms")
    print(f"   Avg value: {sum_values/count:.4f}")
    
    # Clean up
    os.remove(filepath)

def main():
    """Main demonstration"""
    print("=== I/O Operations and Memory Optimization in Python ===")
    
    # Generate test data
    df = pd.DataFrame({
        'a': np.random.randn(50000),
        'b': np.random.randint(0, 100, 50000),
        'c': np.random.choice(['A', 'B', 'C', 'D'], 50000),
        'd': np.random.randn(50000) * 100
    })
    
    # Save test CSV
    df.to_csv('test_data.csv', index=False)
    
    try:
        # Demonstrate operations
        chunks, rows, memory = read_csv_chunked('test_data.csv')
        
        df_sample = df.head(10000).copy()
        df_optimized = optimize_dtypes(df_sample)
        
        write_time, read_time, file_size = parquet_operations(df_sample)
        
        streaming_aggregation('test_data.csv')
        
        demonstrate_memory_pool()
        
        jsonl_streaming('data.jsonl')
        
        print("\n=== All operations completed successfully ===")
        
    finally:
        # Clean up
        if os.path.exists('test_data.csv'):
            os.remove('test_data.csv')

if __name__ == "__main__":
    main()