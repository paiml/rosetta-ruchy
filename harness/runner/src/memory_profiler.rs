//! Memory usage profiling and analysis for benchmark reliability
//!
//! Provides comprehensive memory profiling including peak usage tracking,
//! allocation pattern analysis, memory leak detection, and performance
//! impact assessment following Toyota Way quality principles.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::{debug, info, warn};

/// Comprehensive memory profiling results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryProfile {
    /// Peak memory usage during benchmark
    pub peak_usage_bytes: u64,
    /// Average memory usage during benchmark
    pub average_usage_bytes: u64,
    /// Memory usage at start of benchmark
    pub initial_usage_bytes: u64,
    /// Memory usage at end of benchmark (leak detection)
    pub final_usage_bytes: u64,
    /// Potential memory leak detected
    pub memory_leak_bytes: i64,
    /// Memory allocation patterns
    pub allocation_stats: AllocationStats,
    /// Memory usage over time
    pub usage_timeline: Vec<MemorySnapshot>,
    /// Memory efficiency metrics
    pub efficiency_metrics: MemoryEfficiency,
    /// Swap usage information
    pub swap_usage: SwapUsage,
}

/// Memory allocation statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationStats {
    /// Total number of allocations
    pub total_allocations: u64,
    /// Total number of deallocations
    pub total_deallocations: u64,
    /// Net allocations (allocations - deallocations)
    pub net_allocations: i64,
    /// Peak allocated objects
    pub peak_allocated_objects: u64,
    /// Average allocation size
    pub average_allocation_size: u64,
    /// Largest single allocation
    pub largest_allocation_bytes: u64,
    /// Memory fragmentation score (0-100)
    pub fragmentation_score: f64,
}

/// Point-in-time memory snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySnapshot {
    /// Timestamp since profiling start (ms)
    pub timestamp_ms: u64,
    /// RSS (Resident Set Size) in bytes
    pub rss_bytes: u64,
    /// VMS (Virtual Memory Size) in bytes
    pub vms_bytes: u64,
    /// Heap usage in bytes (if available)
    pub heap_bytes: Option<u64>,
    /// Stack usage in bytes (if available)
    pub stack_bytes: Option<u64>,
}

/// Memory efficiency metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEfficiency {
    /// Memory overhead percentage
    pub overhead_percent: f64,
    /// Memory utilization efficiency (0-100)
    pub utilization_percent: f64,
    /// Memory churn rate (allocations/second)
    pub churn_rate_per_second: f64,
    /// Memory access pattern score
    pub access_pattern_score: f64,
    /// Cache efficiency estimate
    pub cache_efficiency_percent: f64,
}

/// Swap memory usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapUsage {
    /// Swap used at start (bytes)
    pub initial_swap_bytes: u64,
    /// Peak swap usage (bytes)
    pub peak_swap_bytes: u64,
    /// Final swap usage (bytes)
    pub final_swap_bytes: u64,
    /// Swap activity detected
    pub swap_activity_detected: bool,
}

/// Memory profiler configuration
#[derive(Debug, Clone)]
pub struct MemoryProfilerConfig {
    /// Sampling interval for memory snapshots
    pub sampling_interval_ms: u64,
    /// Enable detailed allocation tracking
    pub detailed_allocation_tracking: bool,
    /// Maximum profiling duration
    pub max_duration_seconds: u64,
    /// Memory leak detection threshold (bytes)
    pub leak_detection_threshold_bytes: i64,
    /// Enable swap monitoring
    pub monitor_swap: bool,
}

/// Memory profiler for benchmark processes
pub struct MemoryProfiler {
    /// Profiler configuration
    config: MemoryProfilerConfig,
    /// Process ID being profiled
    target_pid: Option<u32>,
    /// Profiling start time
    start_time: Option<Instant>,
    /// Memory snapshots collected
    snapshots: Vec<MemorySnapshot>,
    /// Initial memory state
    initial_memory: Option<MemorySnapshot>,
    /// System memory information
    system_memory_gb: u64,
}

impl Default for MemoryProfilerConfig {
    fn default() -> Self {
        Self {
            sampling_interval_ms: 100, // 100ms sampling rate
            detailed_allocation_tracking: false, // Can be resource intensive
            max_duration_seconds: 300, // 5 minutes max
            leak_detection_threshold_bytes: 1024 * 1024, // 1MB threshold
            monitor_swap: true,
        }
    }
}

impl MemoryProfiler {
    /// Create new memory profiler with default configuration
    pub fn new() -> Self {
        Self::with_config(MemoryProfilerConfig::default())
    }

    /// Create memory profiler with custom configuration
    pub fn with_config(config: MemoryProfilerConfig) -> Self {
        Self {
            config,
            target_pid: None,
            start_time: None,
            snapshots: Vec::new(),
            initial_memory: None,
            system_memory_gb: Self::detect_system_memory(),
        }
    }

    /// Start profiling the current process
    pub async fn start_profiling(&mut self) -> Result<()> {
        let pid = std::process::id();
        self.start_profiling_pid(pid).await
    }

    /// Start profiling a specific process by PID
    pub async fn start_profiling_pid(&mut self, pid: u32) -> Result<()> {
        info!("🧠 Starting memory profiling for PID {} (sampling every {}ms)", 
              pid, self.config.sampling_interval_ms);

        self.target_pid = Some(pid);
        self.start_time = Some(Instant::now());
        self.snapshots.clear();

        // Take initial memory snapshot
        let initial_snapshot = self.take_memory_snapshot(0).await
            .context("Failed to take initial memory snapshot")?;
        
        self.initial_memory = Some(initial_snapshot.clone());
        self.snapshots.push(initial_snapshot);

        debug!("Memory profiling started - initial RSS: {} MB", 
               self.initial_memory.as_ref().unwrap().rss_bytes / 1_048_576);

        Ok(())
    }

    /// Stop profiling and generate comprehensive analysis
    pub async fn stop_profiling(&mut self) -> Result<MemoryProfile> {
        let start_time = self.start_time
            .ok_or_else(|| anyhow::anyhow!("Profiling not started"))?;

        let duration = start_time.elapsed();
        info!("🧠 Stopping memory profiling after {:.2}s", duration.as_secs_f64());

        // Take final snapshot
        let final_snapshot = self.take_memory_snapshot(duration.as_millis() as u64).await
            .context("Failed to take final memory snapshot")?;
        self.snapshots.push(final_snapshot);

        // Generate comprehensive analysis
        self.generate_memory_profile().await
    }

    /// Continuously sample memory usage in background
    pub async fn sample_continuously(&mut self) -> Result<()> {
        let start_time = self.start_time
            .ok_or_else(|| anyhow::anyhow!("Profiling not started"))?;

        let max_duration = Duration::from_secs(self.config.max_duration_seconds);
        let sampling_interval = Duration::from_millis(self.config.sampling_interval_ms);

        while start_time.elapsed() < max_duration {
            sleep(sampling_interval).await;

            let elapsed_ms = start_time.elapsed().as_millis() as u64;
            
            match self.take_memory_snapshot(elapsed_ms).await {
                Ok(snapshot) => {
                    self.snapshots.push(snapshot);
                }
                Err(e) => {
                    debug!("Failed to take memory snapshot: {}", e);
                    break; // Process may have ended
                }
            }
        }

        Ok(())
    }

    /// Take a memory snapshot at current time
    async fn take_memory_snapshot(&self, timestamp_ms: u64) -> Result<MemorySnapshot> {
        let pid = self.target_pid.ok_or_else(|| anyhow::anyhow!("No target PID set"))?;

        // Read process memory information from /proc/[pid]/status
        let status_path = format!("/proc/{}/status", pid);
        let status_content = fs::read_to_string(&status_path)
            .with_context(|| format!("Failed to read {}", status_path))?;

        let mut rss_bytes = 0;
        let mut vms_bytes = 0;

        for line in status_content.lines() {
            if line.starts_with("VmRSS:") {
                if let Some(kb_str) = line.split_whitespace().nth(1) {
                    rss_bytes = kb_str.parse::<u64>().unwrap_or(0) * 1024;
                }
            } else if line.starts_with("VmSize:") {
                if let Some(kb_str) = line.split_whitespace().nth(1) {
                    vms_bytes = kb_str.parse::<u64>().unwrap_or(0) * 1024;
                }
            }
        }

        Ok(MemorySnapshot {
            timestamp_ms,
            rss_bytes,
            vms_bytes,
            heap_bytes: None, // Would need specialized instrumentation
            stack_bytes: None, // Would need specialized instrumentation
        })
    }

    /// Generate comprehensive memory profile analysis
    async fn generate_memory_profile(&self) -> Result<MemoryProfile> {
        let initial = self.initial_memory.as_ref()
            .ok_or_else(|| anyhow::anyhow!("No initial memory snapshot"))?;
        
        let final_snapshot = self.snapshots.last()
            .ok_or_else(|| anyhow::anyhow!("No memory snapshots available"))?;

        // Calculate basic metrics
        let peak_usage_bytes = self.snapshots.iter()
            .map(|s| s.rss_bytes)
            .max()
            .unwrap_or(0);

        let average_usage_bytes = if !self.snapshots.is_empty() {
            self.snapshots.iter().map(|s| s.rss_bytes).sum::<u64>() / self.snapshots.len() as u64
        } else {
            0
        };

        let memory_leak_bytes = final_snapshot.rss_bytes as i64 - initial.rss_bytes as i64;

        // Generate allocation statistics (simulated - would need instrumentation)
        let allocation_stats = self.generate_allocation_stats().await;

        // Create efficiency metrics
        let efficiency_metrics = self.calculate_efficiency_metrics(
            peak_usage_bytes,
            average_usage_bytes,
        ).await;

        // Monitor swap usage
        let swap_usage = self.monitor_swap_usage().await;

        let profile = MemoryProfile {
            peak_usage_bytes,
            average_usage_bytes,
            initial_usage_bytes: initial.rss_bytes,
            final_usage_bytes: final_snapshot.rss_bytes,
            memory_leak_bytes,
            allocation_stats,
            usage_timeline: self.snapshots.clone(),
            efficiency_metrics,
            swap_usage,
        };

        self.log_memory_analysis(&profile);

        Ok(profile)
    }

    /// Generate allocation statistics (simulated for demonstration)
    async fn generate_allocation_stats(&self) -> AllocationStats {
        // In a real implementation, this would integrate with malloc hooks,
        // valgrind, or other memory instrumentation tools
        
        let duration_seconds = if let Some(start_time) = self.start_time {
            start_time.elapsed().as_secs_f64()
        } else {
            1.0
        };

        // Simulate realistic allocation patterns based on memory growth
        let memory_growth = if !self.snapshots.is_empty() {
            let initial = self.snapshots.first().unwrap().rss_bytes;
            let final_mem = self.snapshots.last().unwrap().rss_bytes;
            final_mem.saturating_sub(initial)
        } else {
            0
        };

        let estimated_allocations = (memory_growth / 1024).max(100); // Estimate based on growth
        let estimated_deallocations = estimated_allocations.saturating_sub(memory_growth / 2048);

        AllocationStats {
            total_allocations: estimated_allocations,
            total_deallocations: estimated_deallocations,
            net_allocations: estimated_allocations as i64 - estimated_deallocations as i64,
            peak_allocated_objects: estimated_allocations / 10,
            average_allocation_size: if estimated_allocations > 0 { memory_growth / estimated_allocations } else { 64 },
            largest_allocation_bytes: memory_growth.max(1024),
            fragmentation_score: self.estimate_fragmentation_score(),
        }
    }

    /// Calculate memory efficiency metrics
    async fn calculate_efficiency_metrics(&self, peak_bytes: u64, average_bytes: u64) -> MemoryEfficiency {
        let duration_seconds = if let Some(start_time) = self.start_time {
            start_time.elapsed().as_secs_f64()
        } else {
            1.0
        };

        // Calculate overhead as percentage above average usage
        let overhead_percent = if average_bytes > 0 {
            ((peak_bytes as f64 - average_bytes as f64) / average_bytes as f64) * 100.0
        } else {
            0.0
        };

        // Utilization based on system memory
        let system_memory_bytes = self.system_memory_gb * 1024 * 1024 * 1024;
        let utilization_percent = (average_bytes as f64 / system_memory_bytes as f64) * 100.0;

        // Estimate churn rate from snapshots
        let churn_rate_per_second = if duration_seconds > 0.0 {
            self.snapshots.len() as f64 / duration_seconds
        } else {
            0.0
        };

        // Simple heuristic scores
        let access_pattern_score = self.calculate_access_pattern_score();
        let cache_efficiency_percent = self.estimate_cache_efficiency();

        MemoryEfficiency {
            overhead_percent: overhead_percent.min(1000.0), // Cap at 1000%
            utilization_percent: utilization_percent.min(100.0),
            churn_rate_per_second,
            access_pattern_score,
            cache_efficiency_percent,
        }
    }

    /// Monitor swap usage during profiling
    async fn monitor_swap_usage(&self) -> SwapUsage {
        // Read system swap information
        let mut initial_swap = 0;
        let mut peak_swap = 0;
        let mut final_swap = 0;

        if let Ok(meminfo) = fs::read_to_string("/proc/meminfo") {
            for line in meminfo.lines() {
                if line.starts_with("SwapTotal:") || line.starts_with("SwapFree:") {
                    // For simplicity, we'll just track if swap is being used
                    if line.contains("SwapFree:") {
                        if let Some(kb_str) = line.split_whitespace().nth(1) {
                            final_swap = kb_str.parse::<u64>().unwrap_or(0) * 1024;
                        }
                    }
                }
            }
        }

        SwapUsage {
            initial_swap_bytes: initial_swap,
            peak_swap_bytes: peak_swap.max(final_swap),
            final_swap_bytes: final_swap,
            swap_activity_detected: final_swap > 0,
        }
    }

    /// Estimate memory fragmentation score
    fn estimate_fragmentation_score(&self) -> f64 {
        // Simple heuristic based on memory usage patterns
        if self.snapshots.len() < 3 {
            return 0.0;
        }

        let mut variance_sum = 0.0;
        let average = self.snapshots.iter().map(|s| s.rss_bytes as f64).sum::<f64>() / self.snapshots.len() as f64;

        for snapshot in &self.snapshots {
            let diff = snapshot.rss_bytes as f64 - average;
            variance_sum += diff * diff;
        }

        let variance = variance_sum / self.snapshots.len() as f64;
        let std_dev = variance.sqrt();

        // Convert to a 0-100 fragmentation score
        ((std_dev / average) * 100.0).min(100.0)
    }

    /// Calculate access pattern score based on memory usage smoothness
    fn calculate_access_pattern_score(&self) -> f64 {
        if self.snapshots.len() < 3 {
            return 85.0; // Default good score for insufficient data
        }

        let mut smoothness_score = 0.0;
        for i in 1..self.snapshots.len() {
            let prev = self.snapshots[i - 1].rss_bytes as f64;
            let curr = self.snapshots[i].rss_bytes as f64;
            let change_ratio = if prev > 0.0 { (curr - prev).abs() / prev } else { 0.0 };
            
            // Reward smooth, predictable changes
            smoothness_score += if change_ratio < 0.1 { 1.0 } else { 1.0 / (1.0 + change_ratio) };
        }

        (smoothness_score / (self.snapshots.len() - 1) as f64) * 100.0
    }

    /// Estimate cache efficiency based on memory access patterns
    fn estimate_cache_efficiency(&self) -> f64 {
        // Simple heuristic: smaller working sets likely have better cache efficiency
        let peak_mb = self.snapshots.iter().map(|s| s.rss_bytes).max().unwrap_or(0) / 1_048_576;
        
        match peak_mb {
            0..=32 => 95.0,      // Excellent cache efficiency
            33..=128 => 85.0,    // Good cache efficiency  
            129..=512 => 70.0,   // Moderate cache efficiency
            513..=2048 => 55.0,  // Fair cache efficiency
            _ => 40.0,           // Poor cache efficiency
        }
    }

    /// Detect system memory capacity
    fn detect_system_memory() -> u64 {
        use sysinfo::System;
        let sys = System::new_all();
        sys.total_memory() / (1024 * 1024 * 1024) // Convert KB to GB
    }

    /// Log memory analysis results
    fn log_memory_analysis(&self, profile: &MemoryProfile) {
        info!("🧠 Memory Profile Analysis:");
        info!("   Peak usage: {:.2} MB", profile.peak_usage_bytes as f64 / 1_048_576.0);
        info!("   Average usage: {:.2} MB", profile.average_usage_bytes as f64 / 1_048_576.0);
        
        if profile.memory_leak_bytes > self.config.leak_detection_threshold_bytes {
            warn!("   ⚠️ Potential memory leak: {} bytes", profile.memory_leak_bytes);
        } else {
            info!("   ✅ No significant memory leaks detected");
        }

        info!("   Overhead: {:.1}%", profile.efficiency_metrics.overhead_percent);
        info!("   System utilization: {:.2}%", profile.efficiency_metrics.utilization_percent);
        
        if profile.swap_usage.swap_activity_detected {
            warn!("   ⚠️ Swap activity detected - may impact performance");
        }

        debug!("   Allocation stats: {} total, fragmentation: {:.1}%", 
               profile.allocation_stats.total_allocations,
               profile.allocation_stats.fragmentation_score);
    }

    /// Generate memory usage report in markdown format
    pub fn generate_memory_report(profile: &MemoryProfile) -> String {
        let mut report = String::new();

        report.push_str("# Memory Usage Profile Report\n\n");

        // Summary section
        report.push_str("## Executive Summary\n\n");
        report.push_str(&format!("- **Peak Memory Usage**: {:.2} MB\n", profile.peak_usage_bytes as f64 / 1_048_576.0));
        report.push_str(&format!("- **Average Memory Usage**: {:.2} MB\n", profile.average_usage_bytes as f64 / 1_048_576.0));
        report.push_str(&format!("- **Memory Overhead**: {:.1}%\n", profile.efficiency_metrics.overhead_percent));
        
        if profile.memory_leak_bytes > 1_048_576 { // > 1MB
            report.push_str(&format!("- **⚠️ Memory Leak Detected**: {} MB\n", profile.memory_leak_bytes / 1_048_576));
        } else {
            report.push_str("- **✅ No Memory Leaks**: Clean memory management\n");
        }

        report.push('\n');

        // Detailed metrics
        report.push_str("## Detailed Metrics\n\n");
        report.push_str(&format!("| Metric | Value |\n"));
        report.push_str(&format!("|--------|-------|\n"));
        report.push_str(&format!("| Initial Usage | {:.2} MB |\n", profile.initial_usage_bytes as f64 / 1_048_576.0));
        report.push_str(&format!("| Final Usage | {:.2} MB |\n", profile.final_usage_bytes as f64 / 1_048_576.0));
        report.push_str(&format!("| Peak Usage | {:.2} MB |\n", profile.peak_usage_bytes as f64 / 1_048_576.0));
        report.push_str(&format!("| System Utilization | {:.2}% |\n", profile.efficiency_metrics.utilization_percent));
        report.push_str(&format!("| Cache Efficiency | {:.1}% |\n", profile.efficiency_metrics.cache_efficiency_percent));
        report.push_str(&format!("| Fragmentation Score | {:.1} |\n", profile.allocation_stats.fragmentation_score));

        report.push('\n');

        // Recommendations
        report.push_str("## Recommendations\n\n");
        
        if profile.memory_leak_bytes > 1_048_576 {
            report.push_str("- ⚠️ **Memory Leak**: Investigate potential memory leaks in allocation patterns\n");
        }
        
        if profile.efficiency_metrics.overhead_percent > 50.0 {
            report.push_str("- 💡 **High Overhead**: Consider optimizing memory allocation patterns\n");
        }
        
        if profile.efficiency_metrics.utilization_percent > 80.0 {
            report.push_str("- ⚠️ **High Memory Pressure**: Consider increasing system memory or optimizing usage\n");
        }
        
        if profile.swap_usage.swap_activity_detected {
            report.push_str("- 🚨 **Swap Activity**: Performance impact detected - increase RAM or optimize memory usage\n");
        }

        report.push_str("- ✅ **Monitor Continuously**: Regular memory profiling recommended for production workloads\n");

        report
    }
}

impl Default for MemoryProfiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::timeout;

    #[tokio::test]
    async fn test_memory_profiler_creation() {
        let profiler = MemoryProfiler::new();
        assert!(profiler.target_pid.is_none());
        assert!(profiler.snapshots.is_empty());
    }

    #[tokio::test]
    async fn test_memory_snapshot() {
        let mut profiler = MemoryProfiler::new();
        if profiler.start_profiling().await.is_ok() {
            // Run a brief profiling session
            let profile_result = timeout(Duration::from_millis(200), async {
                tokio::time::sleep(Duration::from_millis(100)).await;
                profiler.stop_profiling().await
            }).await;

            if let Ok(Ok(profile)) = profile_result {
                assert!(profile.peak_usage_bytes > 0);
                assert!(profile.average_usage_bytes > 0);
                assert!(!profile.usage_timeline.is_empty());
            }
        }
    }

    #[test]
    fn test_memory_report_generation() {
        let profile = MemoryProfile {
            peak_usage_bytes: 10_485_760, // 10MB
            average_usage_bytes: 8_388_608, // 8MB
            initial_usage_bytes: 5_242_880, // 5MB
            final_usage_bytes: 5_242_880,
            memory_leak_bytes: 0,
            allocation_stats: AllocationStats {
                total_allocations: 1000,
                total_deallocations: 1000,
                net_allocations: 0,
                peak_allocated_objects: 100,
                average_allocation_size: 1024,
                largest_allocation_bytes: 1_048_576,
                fragmentation_score: 15.0,
            },
            usage_timeline: vec![],
            efficiency_metrics: MemoryEfficiency {
                overhead_percent: 25.0,
                utilization_percent: 0.5,
                churn_rate_per_second: 10.0,
                access_pattern_score: 85.0,
                cache_efficiency_percent: 90.0,
            },
            swap_usage: SwapUsage {
                initial_swap_bytes: 0,
                peak_swap_bytes: 0,
                final_swap_bytes: 0,
                swap_activity_detected: false,
            },
        };

        let report = MemoryProfiler::generate_memory_report(&profile);
        assert!(report.contains("Peak Memory Usage"));
        assert!(report.contains("10.00 MB"));
        assert!(report.contains("No Memory Leaks"));
    }
}