//! System environment isolation for benchmark reliability
//!
//! Provides CPU affinity control, performance governor management, and
//! system noise reduction to ensure consistent benchmark results.
//!
//! # Toyota Way Principles
//! - **Genchi Genbutsu**: Control the actual environment, don't guess
//! - **Jidoka**: Stop benchmarking if environment cannot be controlled
//! - **Kaizen**: Continuously improve measurement accuracy

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tracing::{info, warn};

/// Environment isolation configuration and management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentController {
    /// CPU cores to isolate for benchmarking
    pub isolated_cores: Vec<usize>,
    /// Target CPU governor (performance, powersave, etc.)
    pub target_governor: String,
    /// Whether to disable CPU frequency scaling
    pub disable_freq_scaling: bool,
    /// Whether to disable interrupt balancing
    pub disable_irq_balance: bool,
    /// Current environment state
    pub current_state: EnvironmentState,
}

/// Current system environment state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentState {
    /// Available CPU cores
    pub available_cores: Vec<usize>,
    /// Current CPU governor per core
    pub cpu_governors: Vec<String>,
    /// Current CPU frequencies (in MHz)
    pub cpu_frequencies: Vec<u32>,
    /// System load average
    pub load_average: (f64, f64, f64),
    /// Memory usage info
    pub memory_info: MemoryInfo,
    /// IRQ balance status
    pub irq_balance_active: bool,
}

/// Memory usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    /// Total system memory in bytes
    pub total_bytes: u64,
    /// Available memory in bytes  
    pub available_bytes: u64,
    /// Memory usage percentage
    pub usage_percent: f64,
    /// Swap usage in bytes
    pub swap_used_bytes: u64,
}

/// CPU isolation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IsolationResult {
    /// Whether isolation was successful
    pub success: bool,
    /// Cores successfully isolated
    pub isolated_cores: Vec<usize>,
    /// Applied CPU governor
    pub applied_governor: Option<String>,
    /// Warnings encountered
    pub warnings: Vec<String>,
    /// Errors encountered
    pub errors: Vec<String>,
}

impl EnvironmentController {
    /// Create new environment controller with default settings
    pub fn new() -> Self {
        Self {
            isolated_cores: vec![0], // Default to core 0
            target_governor: "performance".to_string(),
            disable_freq_scaling: true,
            disable_irq_balance: false, // Conservative default
            current_state: EnvironmentState::default(),
        }
    }

    /// Configure CPU cores to isolate
    pub fn with_isolated_cores(mut self, cores: Vec<usize>) -> Self {
        self.isolated_cores = cores;
        self
    }

    /// Configure target CPU governor
    pub fn with_governor(mut self, governor: &str) -> Self {
        self.target_governor = governor.to_string();
        self
    }

    /// Enable/disable frequency scaling control
    pub fn with_freq_scaling_control(mut self, disable: bool) -> Self {
        self.disable_freq_scaling = disable;
        self
    }

    /// Detect current system environment
    pub async fn detect_environment(&mut self) -> Result<()> {
        info!("🔍 Detecting system environment for benchmark isolation");

        self.current_state = self
            .gather_system_state()
            .await
            .context("Failed to gather system state")?;

        info!(
            "📊 Environment detected: {} cores, governor={:?}, load={:.2}",
            self.current_state.available_cores.len(),
            self.current_state
                .cpu_governors
                .first()
                .unwrap_or(&"unknown".to_string()),
            self.current_state.load_average.0
        );

        Ok(())
    }

    /// Apply environment isolation for benchmarking
    pub async fn apply_isolation(&mut self) -> Result<IsolationResult> {
        info!("🔒 Applying environment isolation for Toyota Way quality benchmarks");

        let mut result = IsolationResult {
            success: true,
            isolated_cores: Vec::new(),
            applied_governor: None,
            warnings: Vec::new(),
            errors: Vec::new(),
        };

        // Step 1: Validate requested cores are available
        self.validate_core_availability(&mut result)?;

        // Step 2: Set CPU affinity for current process
        if let Err(e) = self.set_cpu_affinity(&mut result).await {
            result.errors.push(format!("CPU affinity failed: {}", e));
            result.success = false;
        }

        // Step 3: Configure CPU governor
        if let Err(e) = self.configure_cpu_governor(&mut result).await {
            result
                .errors
                .push(format!("CPU governor configuration failed: {}", e));
            // This is often a permission issue, so we continue with a warning
            result
                .warnings
                .push("CPU governor control requires root privileges".to_string());
        }

        // Step 4: Control frequency scaling
        if self.disable_freq_scaling {
            if let Err(e) = self.control_frequency_scaling(&mut result).await {
                result
                    .warnings
                    .push(format!("Frequency scaling control failed: {}", e));
            }
        }

        // Step 5: Check system noise level
        self.assess_system_noise(&mut result).await;

        if result.success {
            info!("✅ Environment isolation applied successfully");
        } else {
            warn!("⚠️ Environment isolation partially failed - benchmark quality may be affected");
        }

        Ok(result)
    }

    /// Validate that requested cores are available
    fn validate_core_availability(&self, result: &mut IsolationResult) -> Result<()> {
        for &core in &self.isolated_cores {
            if !self.current_state.available_cores.contains(&core) {
                result.errors.push(format!("Core {} not available", core));
                result.success = false;
            }
        }

        if !self.isolated_cores.is_empty() && result.success {
            info!("🎯 Targeting cores: {:?}", self.isolated_cores);
        }

        Ok(())
    }

    /// Set CPU affinity for the current process
    async fn set_cpu_affinity(&self, result: &mut IsolationResult) -> Result<()> {
        use nix::sched::{sched_setaffinity, CpuSet};
        use nix::unistd::Pid;

        let mut cpu_set = CpuSet::new();

        for &core in &self.isolated_cores {
            cpu_set
                .set(core)
                .with_context(|| format!("Failed to set core {} in CPU set", core))?;
        }

        sched_setaffinity(Pid::from_raw(0), &cpu_set).context("Failed to set CPU affinity")?;

        result.isolated_cores = self.isolated_cores.clone();
        info!("📌 CPU affinity set to cores: {:?}", self.isolated_cores);

        Ok(())
    }

    /// Configure CPU governor for performance
    async fn configure_cpu_governor(&self, result: &mut IsolationResult) -> Result<()> {
        let mut governors_set = Vec::new();

        for &core in &self.isolated_cores {
            let governor_path = format!(
                "/sys/devices/system/cpu/cpu{}/cpufreq/scaling_governor",
                core
            );

            if Path::new(&governor_path).exists() {
                match self
                    .try_set_governor(&governor_path, &self.target_governor)
                    .await
                {
                    Ok(()) => {
                        governors_set.push(core);
                        info!("⚡ Core {} governor set to {}", core, self.target_governor);
                    }
                    Err(e) => {
                        warn!("Failed to set governor for core {}: {}", core, e);
                    }
                }
            } else {
                result
                    .warnings
                    .push(format!("Governor control not available for core {}", core));
            }
        }

        if !governors_set.is_empty() {
            result.applied_governor = Some(self.target_governor.clone());
        }

        Ok(())
    }

    /// Attempt to set CPU governor (requires root privileges)
    async fn try_set_governor(&self, path: &str, governor: &str) -> Result<()> {
        // Check if governor is available
        let available_path = path.replace("scaling_governor", "scaling_available_governors");
        if Path::new(&available_path).exists() {
            let available = fs::read_to_string(&available_path)
                .context("Failed to read available governors")?;

            if !available.contains(governor) {
                anyhow::bail!(
                    "Governor '{}' not available. Available: {}",
                    governor,
                    available.trim()
                );
            }
        }

        // Try to set the governor (may fail due to permissions)
        fs::write(path, governor)
            .with_context(|| format!("Failed to write '{}' to {}", governor, path))?;

        Ok(())
    }

    /// Control CPU frequency scaling
    async fn control_frequency_scaling(&self, result: &mut IsolationResult) -> Result<()> {
        for &core in &self.isolated_cores {
            let min_freq_path = format!(
                "/sys/devices/system/cpu/cpu{}/cpufreq/scaling_min_freq",
                core
            );
            let max_freq_path = format!(
                "/sys/devices/system/cpu/cpu{}/cpufreq/scaling_max_freq",
                core
            );

            if Path::new(&min_freq_path).exists() && Path::new(&max_freq_path).exists() {
                match self
                    .try_lock_frequency(core, &min_freq_path, &max_freq_path)
                    .await
                {
                    Ok(freq) => {
                        info!("🔒 Core {} frequency locked at {} MHz", core, freq / 1000);
                    }
                    Err(e) => {
                        result
                            .warnings
                            .push(format!("Frequency lock failed for core {}: {}", core, e));
                    }
                }
            }
        }

        Ok(())
    }

    /// Try to lock CPU frequency to maximum
    async fn try_lock_frequency(
        &self,
        _core: usize,
        min_path: &str,
        max_path: &str,
    ) -> Result<u32> {
        let max_freq_str = fs::read_to_string(max_path).context("Failed to read max frequency")?;

        let max_freq: u32 = max_freq_str
            .trim()
            .parse()
            .context("Failed to parse max frequency")?;

        // Set min freq to max freq (effectively locking frequency)
        fs::write(min_path, max_freq.to_string()).context("Failed to lock frequency")?;

        Ok(max_freq)
    }

    /// Assess system noise level
    async fn assess_system_noise(&self, result: &mut IsolationResult) {
        let load = self.current_state.load_average.0;
        let memory_usage = self.current_state.memory_info.usage_percent;

        if load > 0.5 {
            result
                .warnings
                .push(format!("High system load: {:.2}", load));
        }

        if memory_usage > 80.0 {
            result
                .warnings
                .push(format!("High memory usage: {:.1}%", memory_usage));
        }

        if self.current_state.irq_balance_active {
            result
                .warnings
                .push("IRQ balancing is active - may affect benchmark consistency".to_string());
        }

        info!(
            "📈 System noise assessment: load={:.2}, memory={:.1}%, irq_balance={}",
            load, memory_usage, self.current_state.irq_balance_active
        );
    }

    /// Gather current system state
    async fn gather_system_state(&self) -> Result<EnvironmentState> {
        let available_cores = self.detect_available_cores()?;
        let cpu_governors = self.detect_cpu_governors(&available_cores).await;
        let cpu_frequencies = self.detect_cpu_frequencies(&available_cores).await;
        let load_average = self.read_load_average()?;
        let memory_info = self.gather_memory_info()?;
        let irq_balance_active = self.check_irq_balance_status();

        Ok(EnvironmentState {
            available_cores,
            cpu_governors,
            cpu_frequencies,
            load_average,
            memory_info,
            irq_balance_active,
        })
    }

    /// Detect available CPU cores
    fn detect_available_cores(&self) -> Result<Vec<usize>> {
        let mut cores = Vec::new();
        let cpu_dir = Path::new("/sys/devices/system/cpu");

        if cpu_dir.exists() {
            for entry in fs::read_dir(cpu_dir)? {
                let entry = entry?;
                let name = entry.file_name();
                let name_str = name.to_string_lossy();

                if name_str.starts_with("cpu") && name_str.len() > 3 {
                    if let Ok(core_num) = name_str[3..].parse::<usize>() {
                        cores.push(core_num);
                    }
                }
            }
        }

        cores.sort_unstable();
        Ok(cores)
    }

    /// Detect CPU governors for each core
    async fn detect_cpu_governors(&self, cores: &[usize]) -> Vec<String> {
        let mut governors = Vec::new();

        for &core in cores {
            let governor_path = format!(
                "/sys/devices/system/cpu/cpu{}/cpufreq/scaling_governor",
                core
            );
            let governor = fs::read_to_string(&governor_path)
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|_| "unknown".to_string());
            governors.push(governor);
        }

        governors
    }

    /// Detect CPU frequencies for each core
    async fn detect_cpu_frequencies(&self, cores: &[usize]) -> Vec<u32> {
        let mut frequencies = Vec::new();

        for &core in cores {
            let freq_path = format!(
                "/sys/devices/system/cpu/cpu{}/cpufreq/scaling_cur_freq",
                core
            );
            let freq = fs::read_to_string(&freq_path)
                .and_then(|s| {
                    s.trim()
                        .parse::<u32>()
                        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
                })
                .map(|f| f / 1000) // Convert to MHz
                .unwrap_or(0);
            frequencies.push(freq);
        }

        frequencies
    }

    /// Read system load average
    fn read_load_average(&self) -> Result<(f64, f64, f64)> {
        let loadavg =
            fs::read_to_string("/proc/loadavg").context("Failed to read /proc/loadavg")?;

        let parts: Vec<&str> = loadavg.split_whitespace().collect();
        if parts.len() >= 3 {
            let load1: f64 = parts[0].parse().unwrap_or(0.0);
            let load5: f64 = parts[1].parse().unwrap_or(0.0);
            let load15: f64 = parts[2].parse().unwrap_or(0.0);
            Ok((load1, load5, load15))
        } else {
            Ok((0.0, 0.0, 0.0))
        }
    }

    /// Gather memory usage information
    fn gather_memory_info(&self) -> Result<MemoryInfo> {
        use sysinfo::System;

        let sys = System::new_all();
        let total_bytes = sys.total_memory() * 1024; // sysinfo returns KB
        let used_bytes = sys.used_memory() * 1024;
        let available_bytes = total_bytes - used_bytes;
        let usage_percent = (used_bytes as f64 / total_bytes as f64) * 100.0;
        let swap_used_bytes = sys.used_swap() * 1024;

        Ok(MemoryInfo {
            total_bytes,
            available_bytes,
            usage_percent,
            swap_used_bytes,
        })
    }

    /// Check if IRQ balance is active
    fn check_irq_balance_status(&self) -> bool {
        // Check if irqbalance service is running
        std::process::Command::new("pgrep")
            .arg("irqbalance")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    /// Restore original environment settings
    pub async fn restore_environment(&self) -> Result<()> {
        info!("🔄 Restoring original environment settings");

        // For now, we don't restore settings as it could interfere with other processes
        // In a production environment, we might save original state and restore it

        warn!("Environment restoration not implemented - manual cleanup may be required");
        Ok(())
    }
}

impl Default for EnvironmentState {
    fn default() -> Self {
        Self {
            available_cores: Vec::new(),
            cpu_governors: Vec::new(),
            cpu_frequencies: Vec::new(),
            load_average: (0.0, 0.0, 0.0),
            memory_info: MemoryInfo {
                total_bytes: 0,
                available_bytes: 0,
                usage_percent: 0.0,
                swap_used_bytes: 0,
            },
            irq_balance_active: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_environment_detection() {
        let mut controller = EnvironmentController::new();

        // This test may fail in some environments (like containers)
        // so we make it lenient
        if controller.detect_environment().await.is_ok() {
            assert!(!controller.current_state.available_cores.is_empty());
        }
    }

    #[test]
    fn test_controller_configuration() {
        let controller = EnvironmentController::new()
            .with_isolated_cores(vec![0, 1])
            .with_governor("powersave")
            .with_freq_scaling_control(false);

        assert_eq!(controller.isolated_cores, vec![0, 1]);
        assert_eq!(controller.target_governor, "powersave");
        assert!(!controller.disable_freq_scaling);
    }
}
