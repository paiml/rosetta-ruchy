#!/usr/bin/env Rscript

# I/O Operations and Memory Optimization - Sprint 27
# R implementation using data.table and arrow

library(data.table)
library(arrow)
library(jsonlite)

# Memory limits
MAX_MEMORY_MB <- 1024
CHUNK_SIZE <- 10000

# Chunked CSV reading
read_csv_chunked <- function(filepath, chunk_size = CHUNK_SIZE) {
  cat("\n1. CHUNKED CSV READING:\n")
  
  chunks_read <- 0
  total_rows <- 0
  
  # Read in chunks using data.table
  con <- file(filepath, "r")
  header <- readLines(con, n = 1)
  
  while(TRUE) {
    chunk <- readLines(con, n = chunk_size)
    if(length(chunk) == 0) break
    
    # Process chunk
    chunk_data <- fread(text = c(header, chunk))
    chunks_read <- chunks_read + 1
    total_rows <- total_rows + nrow(chunk_data)
    
    # Clear chunk
    rm(chunk_data)
    gc(verbose = FALSE)
  }
  
  close(con)
  
  cat(sprintf("   Chunks processed: %d\n", chunks_read))
  cat(sprintf("   Total rows: %d\n", total_rows))
  
  return(list(chunks = chunks_read, rows = total_rows))
}

# Memory-efficient data.table operations
optimize_dtypes <- function(dt) {
  cat("\n2. DTYPE OPTIMIZATION:\n")
  
  original_size <- object.size(dt)
  
  for(col in names(dt)) {
    if(is.numeric(dt[[col]])) {
      col_min <- min(dt[[col]], na.rm = TRUE)
      col_max <- max(dt[[col]], na.rm = TRUE)
      
      if(all(dt[[col]] == floor(dt[[col]]), na.rm = TRUE)) {
        # Integer optimization
        if(col_min >= -128 && col_max <= 127) {
          set(dt, j = col, value = as.integer(dt[[col]]))
        }
      }
    } else if(is.character(dt[[col]])) {
      # Convert to factor if many duplicates
      unique_ratio <- length(unique(dt[[col]])) / nrow(dt)
      if(unique_ratio < 0.5) {
        set(dt, j = col, value = as.factor(dt[[col]]))
      }
    }
  }
  
  optimized_size <- object.size(dt)
  
  cat(sprintf("   Original: %.2f MB\n", as.numeric(original_size) / 1024 / 1024))
  cat(sprintf("   Optimized: %.2f MB\n", as.numeric(optimized_size) / 1024 / 1024))
  cat(sprintf("   Reduction: %.1f%%\n", 
              (1 - as.numeric(optimized_size)/as.numeric(original_size)) * 100))
  
  return(dt)
}

# Arrow format operations
arrow_operations <- function(dt) {
  cat("\n3. ARROW FORMAT:\n")
  
  # Write to Arrow
  start_time <- Sys.time()
  write_parquet(dt, "data.parquet")
  write_time <- as.numeric(Sys.time() - start_time)
  
  # Read from Arrow
  start_time <- Sys.time()
  dt_arrow <- read_parquet("data.parquet")
  read_time <- as.numeric(Sys.time() - start_time)
  
  file_size <- file.info("data.parquet")$size
  
  cat(sprintf("   Write time: %.2f ms\n", write_time * 1000))
  cat(sprintf("   Read time: %.2f ms\n", read_time * 1000))
  cat(sprintf("   File size: %.2f MB\n", file_size / 1024 / 1024))
  
  # Clean up
  unlink("data.parquet")
  
  return(list(write = write_time, read = read_time))
}

# Streaming aggregation
streaming_aggregation <- function(filepath, chunk_size = CHUNK_SIZE) {
  cat("\n4. STREAMING AGGREGATION:\n")
  
  sum_total <- 0
  count_total <- 0
  chunks <- 0
  
  con <- file(filepath, "r")
  header <- readLines(con, n = 1)
  
  while(TRUE) {
    chunk <- readLines(con, n = chunk_size)
    if(length(chunk) == 0) break
    
    chunk_dt <- fread(text = c(header, chunk))
    numeric_cols <- names(chunk_dt)[sapply(chunk_dt, is.numeric)]
    
    for(col in numeric_cols) {
      sum_total <- sum_total + sum(chunk_dt[[col]], na.rm = TRUE)
      count_total <- count_total + sum(!is.na(chunk_dt[[col]]))
    }
    
    chunks <- chunks + 1
    rm(chunk_dt)
    gc(verbose = FALSE)
  }
  
  close(con)
  
  cat(sprintf("   Total sum: %.2f\n", sum_total))
  cat(sprintf("   Total count: %d\n", count_total))
  cat(sprintf("   Mean: %.4f\n", sum_total / count_total))
  cat("   Memory: O(1) constant\n")
}

# Memory pool implementation
MemoryPool <- setRefClass("MemoryPool",
  fields = list(
    buffers = "list",
    available = "logical",
    buffer_size = "numeric"
  ),
  methods = list(
    initialize = function(pool_size, buf_size) {
      buffers <<- lapply(1:pool_size, function(x) numeric(buf_size))
      available <<- rep(TRUE, pool_size)
      buffer_size <<- buf_size
    },
    acquire = function() {
      idx <- which(available)[1]
      if(!is.na(idx)) {
        available[idx] <<- FALSE
        return(list(idx = idx, buffer = buffers[[idx]]))
      }
      return(NULL)
    },
    release = function(idx) {
      available[idx] <<- TRUE
    }
  )
)

demonstrate_memory_pool <- function() {
  cat("\n5. MEMORY POOL:\n")
  
  pool <- MemoryPool$new(10, 1000)
  
  # Acquire buffers
  acquired <- list()
  for(i in 1:5) {
    result <- pool$acquire()
    if(!is.null(result)) {
      acquired[[length(acquired) + 1]] <- result$idx
    }
  }
  
  cat(sprintf("   Buffers acquired: %d\n", length(acquired)))
  cat(sprintf("   Available: %d\n", sum(pool$available)))
  
  # Release buffers
  for(idx in acquired) {
    pool$release(idx)
  }
  
  cat(sprintf("   After release: %d available\n", sum(pool$available)))
  
  total_memory <- length(pool$buffers) * pool$buffer_size * 8 / 1024 / 1024
  cat(sprintf("   Pool memory: %.2f MB\n", total_memory))
}

# Main function
main <- function() {
  cat("=== I/O Operations and Memory Optimization in R ===\n")
  
  # Generate test data
  set.seed(42)
  dt <- data.table(
    a = rnorm(50000),
    b = sample(1:100, 50000, replace = TRUE),
    c = sample(c("A", "B", "C", "D"), 50000, replace = TRUE),
    d = rnorm(50000) * 100
  )
  
  # Save test CSV
  fwrite(dt, "test_data.csv")
  
  tryCatch({
    # Demonstrate operations
    result <- read_csv_chunked("test_data.csv")
    
    dt_sample <- dt[1:10000]
    dt_optimized <- optimize_dtypes(copy(dt_sample))
    
    arrow_times <- arrow_operations(dt_sample)
    
    streaming_aggregation("test_data.csv")
    
    demonstrate_memory_pool()
    
    cat("\n=== All operations completed successfully ===\n")
  }, finally = {
    # Clean up
    if(file.exists("test_data.csv")) {
      unlink("test_data.csv")
    }
  })
}

# Run if executed directly
if (!interactive()) {
  main()
}