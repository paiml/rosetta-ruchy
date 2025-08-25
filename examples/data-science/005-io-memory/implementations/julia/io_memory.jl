#!/usr/bin/env julia

# I/O Operations and Memory Optimization - Sprint 27
# Efficient data I/O and memory management
# Julia implementation using CSV.jl, Arrow.jl, and memory optimization techniques

using CSV
using DataFrames
using Arrow
using JSON3
using Mmap
using SparseArrays
using Random
using BenchmarkTools

# Memory limits for testing
const MAX_MEMORY_MB = 1024
const CHUNK_SIZE = 10000

# Generate sample data for testing
function generate_sample_csv(filename::String, n_rows::Int, n_cols::Int)
    df = DataFrame()
    for i in 1:n_cols
        df[!, Symbol("col_$i")] = rand(n_rows)
    end
    CSV.write(filename, df)
    return df
end

# 1. Chunked CSV reading for large files
function read_csv_chunked(filepath::String, chunk_size::Int=CHUNK_SIZE)
    println("\n1. CHUNKED CSV READING:")
    
    chunks_read = 0
    total_rows = 0
    peak_memory = 0
    
    # Read file in chunks
    for chunk in CSV.Chunks(filepath, chunksize=chunk_size)
        df_chunk = DataFrame(chunk)
        chunks_read += 1
        total_rows += nrow(df_chunk)
        
        # Estimate memory usage
        memory_usage = Base.summarysize(df_chunk)
        peak_memory = max(peak_memory, memory_usage)
        
        # Process chunk (example: calculate sum)
        chunk_sum = sum(sum(col) for col in eachcol(df_chunk) if eltype(col) <: Number)
        
        # Clear chunk from memory
        df_chunk = nothing
        GC.gc(false)  # Suggest garbage collection
    end
    
    println("   Chunks processed: $chunks_read")
    println("   Total rows: $total_rows")
    println("   Peak memory: $(round(peak_memory / 1024 / 1024, digits=2)) MB")
    
    return chunks_read, total_rows, peak_memory
end

# 2. Memory-efficient CSV writing
function write_csv_buffered(filepath::String, df::DataFrame, buffer_size::Int=8192)
    println("\n2. BUFFERED CSV WRITING:")
    
    start_time = time()
    
    # Use CSV.write with custom buffer size
    open(filepath, "w") do io
        buffer = IOBuffer(sizehint=buffer_size)
        CSV.write(buffer, df)
        write(io, take!(buffer))
    end
    
    write_time = time() - start_time
    file_size = filesize(filepath)
    
    println("   Rows written: $(nrow(df))")
    println("   File size: $(round(file_size / 1024 / 1024, digits=2)) MB")
    println("   Write time: $(round(write_time * 1000, digits=2)) ms")
    
    return nrow(df), file_size, write_time
end

# 3. Arrow format for efficient I/O
function arrow_io_operations(df::DataFrame)
    println("\n3. ARROW FORMAT I/O:")
    
    # Write to Arrow format
    arrow_file = "data.arrow"
    start_time = time()
    Arrow.write(arrow_file, df)
    write_time = time() - start_time
    
    # Read from Arrow format
    start_time = time()
    df_arrow = DataFrame(Arrow.Table(arrow_file))
    read_time = time() - start_time
    
    arrow_size = filesize(arrow_file)
    
    println("   Arrow write time: $(round(write_time * 1000, digits=2)) ms")
    println("   Arrow read time: $(round(read_time * 1000, digits=2)) ms")
    println("   Arrow file size: $(round(arrow_size / 1024 / 1024, digits=2)) MB")
    
    # Memory-mapped Arrow reading
    start_time = time()
    arrow_mmap = Arrow.Table(arrow_file; mmap=true)
    mmap_time = time() - start_time
    
    println("   Memory-mapped read: $(round(mmap_time * 1000, digits=2)) ms")
    
    # Clean up
    rm(arrow_file, force=true)
    
    return write_time, read_time, mmap_time
end

# 4. Type optimization for memory efficiency
function optimize_column_types(df::DataFrame)
    println("\n4. COLUMN TYPE OPTIMIZATION:")
    
    original_size = Base.summarysize(df)
    df_optimized = DataFrame()
    
    for col_name in names(df)
        col = df[!, col_name]
        
        if eltype(col) <: AbstractFloat
            # Check if can be converted to integer
            if all(x -> x == floor(x), skipmissing(col))
                col_int = Int.(col)
                
                # Check integer size optimization
                min_val, max_val = extrema(skipmissing(col_int))
                
                if min_val >= typemin(Int8) && max_val <= typemax(Int8)
                    df_optimized[!, col_name] = Int8.(col_int)
                elseif min_val >= typemin(Int16) && max_val <= typemax(Int16)
                    df_optimized[!, col_name] = Int16.(col_int)
                elseif min_val >= typemin(Int32) && max_val <= typemax(Int32)
                    df_optimized[!, col_name] = Int32.(col_int)
                else
                    df_optimized[!, col_name] = col_int
                end
            else
                # Check if Float32 is sufficient
                if maximum(abs.(skipmissing(col))) < 1e6
                    df_optimized[!, col_name] = Float32.(col)
                else
                    df_optimized[!, col_name] = col
                end
            end
        elseif eltype(col) <: AbstractString
            # Convert to categorical for repeated values
            unique_ratio = length(unique(col)) / length(col)
            if unique_ratio < 0.5
                df_optimized[!, col_name] = categorical(col)
            else
                df_optimized[!, col_name] = col
            end
        else
            df_optimized[!, col_name] = col
        end
    end
    
    optimized_size = Base.summarysize(df_optimized)
    
    println("   Original size: $(round(original_size / 1024 / 1024, digits=2)) MB")
    println("   Optimized size: $(round(optimized_size / 1024 / 1024, digits=2)) MB")
    println("   Reduction: $(round((1 - optimized_size/original_size) * 100, digits=1))%")
    
    return df_optimized
end

# 5. Streaming aggregation with constant memory
function streaming_aggregation(filepath::String, chunk_size::Int=CHUNK_SIZE)
    println("\n5. STREAMING AGGREGATION:")
    
    sum_total = 0.0
    count_total = 0
    mean_accumulator = 0.0
    chunks_processed = 0
    
    for chunk in CSV.Chunks(filepath, chunksize=chunk_size)
        df_chunk = DataFrame(chunk)
        
        # Compute aggregations on chunk
        for col in eachcol(df_chunk)
            if eltype(col) <: Number
                chunk_sum = sum(col)
                chunk_count = length(col)
                
                # Update running totals
                sum_total += chunk_sum
                count_total += chunk_count
                
                # Welford's online algorithm for mean
                chunks_processed += 1
                delta = chunk_sum/chunk_count - mean_accumulator
                mean_accumulator += delta / chunks_processed
            end
        end
    end
    
    final_mean = sum_total / count_total
    
    println("   Total sum: $sum_total")
    println("   Total count: $count_total")
    println("   Mean: $final_mean")
    println("   Chunks processed: $chunks_processed")
    println("   Memory usage: O(1) - constant")
    
    return sum_total, count_total, final_mean
end

# 6. Memory pool implementation
mutable struct MemoryPool
    buffers::Vector{Vector{Float64}}
    available::BitVector
    buffer_size::Int
end

function MemoryPool(pool_size::Int, buffer_size::Int)
    buffers = [Vector{Float64}(undef, buffer_size) for _ in 1:pool_size]
    available = trues(pool_size)
    MemoryPool(buffers, available, buffer_size)
end

function acquire!(pool::MemoryPool)
    idx = findfirst(pool.available)
    if idx !== nothing
        pool.available[idx] = false
        return idx, pool.buffers[idx]
    end
    return nothing, nothing
end

function release!(pool::MemoryPool, idx::Int)
    pool.available[idx] = true
end

function demonstrate_memory_pool()
    println("\n6. MEMORY POOL:")
    
    pool = MemoryPool(10, 1000)
    
    # Acquire buffers
    acquired = []
    for i in 1:5
        idx, buffer = acquire!(pool)
        if idx !== nothing
            push!(acquired, idx)
            fill!(buffer, i)  # Use buffer
        end
    end
    
    println("   Buffers acquired: $(length(acquired))")
    println("   Buffers available: $(sum(pool.available))")
    
    # Release buffers
    for idx in acquired
        release!(pool, idx)
    end
    
    println("   After release: $(sum(pool.available)) available")
    
    total_memory = length(pool.buffers) * pool.buffer_size * sizeof(Float64)
    println("   Total pool memory: $(round(total_memory / 1024 / 1024, digits=2)) MB")
    
    return pool
end

# 7. Sparse matrix for efficient storage
function demonstrate_sparse_storage(density::Float64=0.01)
    println("\n7. SPARSE MATRIX STORAGE:")
    
    n_rows, n_cols = 10000, 10000
    n_nonzero = Int(floor(n_rows * n_cols * density))
    
    # Create sparse matrix
    I = rand(1:n_rows, n_nonzero)
    J = rand(1:n_cols, n_nonzero)
    V = rand(n_nonzero)
    
    sparse_mat = sparse(I, J, V, n_rows, n_cols)
    
    # Memory comparison
    dense_size = n_rows * n_cols * sizeof(Float64)
    sparse_size = (length(sparse_mat.nzval) * sizeof(Float64) + 
                   length(sparse_mat.rowval) * sizeof(Int) +
                   length(sparse_mat.colptr) * sizeof(Int))
    
    println("   Matrix dimensions: $n_rows × $n_cols")
    println("   Non-zero elements: $(nnz(sparse_mat))")
    println("   Density: $(round(density * 100, digits=2))%")
    println("   Dense size: $(round(dense_size / 1024 / 1024, digits=2)) MB")
    println("   Sparse size: $(round(sparse_size / 1024 / 1024, digits=2)) MB")
    println("   Compression ratio: $(round(dense_size / sparse_size, digits=1)):1")
    
    return sparse_mat
end

# 8. JSON Lines format for streaming
function jsonl_streaming(filepath::String, n_records::Int=1000)
    println("\n8. JSON LINES STREAMING:")
    
    # Write JSONL
    start_time = time()
    open(filepath, "w") do io
        for i in 1:n_records
            record = Dict(
                "id" => i,
                "value" => rand(),
                "category" => rand(["A", "B", "C"]),
                "timestamp" => time()
            )
            println(io, JSON3.write(record))
        end
    end
    write_time = time() - start_time
    
    # Stream read JSONL
    start_time = time()
    count = 0
    sum_values = 0.0
    
    open(filepath, "r") do io
        for line in eachline(io)
            record = JSON3.read(line, Dict)
            count += 1
            sum_values += record["value"]
        end
    end
    read_time = time() - start_time
    
    file_size = filesize(filepath)
    
    println("   Records written: $n_records")
    println("   File size: $(round(file_size / 1024, digits=2)) KB")
    println("   Write time: $(round(write_time * 1000, digits=2)) ms")
    println("   Stream read time: $(round(read_time * 1000, digits=2)) ms")
    println("   Average value: $(sum_values / count)")
    
    # Clean up
    rm(filepath, force=true)
    
    return write_time, read_time
end

# Memory profiling function
function profile_memory_usage()
    println("\n9. MEMORY PROFILING:")
    
    # Get current memory usage
    gc_stats = Base.gc_num()
    mem_before = Sys.free_memory()
    
    # Create large dataset
    df = DataFrame(
        a = rand(100000),
        b = rand(Int, 100000),
        c = rand(["A", "B", "C", "D"], 100000)
    )
    
    # Force garbage collection
    GC.gc()
    
    mem_after = Sys.free_memory()
    memory_used = mem_before - mem_after
    
    println("   Dataset rows: $(nrow(df))")
    println("   Memory used: $(round(memory_used / 1024 / 1024, digits=2)) MB")
    println("   GC collections: $(Base.gc_num().collect - gc_stats.collect)")
    
    return memory_used
end

# Main demonstration function
function main()
    println("=== I/O Operations and Memory Optimization in Julia ===")
    
    # Create test data
    test_csv = "test_data.csv"
    df = generate_sample_csv(test_csv, 50000, 10)
    
    # Demonstrate all operations
    try
        chunks, rows, memory = read_csv_chunked(test_csv)
        
        write_rows, file_size, write_time = write_csv_buffered("output.csv", df[1:1000, :])
        
        arrow_write, arrow_read, arrow_mmap = arrow_io_operations(df[1:10000, :])
        
        df_optimized = optimize_column_types(df[1:1000, :])
        
        sum_total, count, mean = streaming_aggregation(test_csv)
        
        pool = demonstrate_memory_pool()
        
        sparse_mat = demonstrate_sparse_storage(0.01)
        
        jsonl_write, jsonl_read = jsonl_streaming("data.jsonl", 1000)
        
        memory_used = profile_memory_usage()
        
        println("\n=== All I/O operations completed successfully ===")
        
    finally
        # Clean up test files
        rm(test_csv, force=true)
        rm("output.csv", force=true)
    end
end

# Run if executed directly
if abspath(PROGRAM_FILE) == @__FILE__
    main()
end