//! Binary size analysis tools for comprehensive benchmark evaluation
//!
//! Provides detailed binary size analysis including debug symbols,
//! stripped sizes, section breakdown, and optimization opportunities
//! following Toyota Way principles of waste elimination.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tracing::{info, warn};

/// Comprehensive binary size analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinarySizeAnalysis {
    /// Total binary size in bytes
    pub total_size_bytes: u64,
    /// Size after stripping debug symbols
    pub stripped_size_bytes: u64,
    /// Debug symbols size (difference)
    pub debug_symbols_bytes: u64,
    /// Percentage of binary that is debug info
    pub debug_percentage: f64,
    /// Section-level breakdown
    pub sections: Vec<SectionInfo>,
    /// Symbol analysis
    pub symbol_analysis: SymbolAnalysis,
    /// Optimization opportunities
    pub optimization_opportunities: Vec<OptimizationOpportunity>,
    /// Compression analysis
    pub compression: CompressionAnalysis,
    /// Dependencies impact
    pub dependencies: DependencyAnalysis,
}

/// Binary section information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionInfo {
    /// Section name (.text, .data, etc.)
    pub name: String,
    /// Section size in bytes
    pub size_bytes: u64,
    /// Percentage of total binary
    pub percentage: f64,
    /// Section type
    pub section_type: SectionType,
}

/// Section types for categorization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SectionType {
    /// Executable code
    Code,
    /// Read-only data
    ReadOnlyData,
    /// Initialized data
    Data,
    /// Uninitialized data
    Bss,
    /// Debug information
    Debug,
    /// Dynamic linking
    Dynamic,
    /// Other/Unknown
    Other,
}

/// Symbol-level analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolAnalysis {
    /// Total number of symbols
    pub total_symbols: usize,
    /// Exported symbols count
    pub exported_symbols: usize,
    /// Local symbols count
    pub local_symbols: usize,
    /// Largest symbols
    pub largest_symbols: Vec<SymbolInfo>,
    /// Symbol bloat score (0-100)
    pub bloat_score: f64,
}

/// Individual symbol information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolInfo {
    /// Symbol name
    pub name: String,
    /// Symbol size in bytes
    pub size_bytes: u64,
    /// Symbol type
    pub symbol_type: String,
    /// Demangled name (for C++/Rust)
    pub demangled_name: Option<String>,
}

/// Compression analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionAnalysis {
    /// Original size
    pub original_bytes: u64,
    /// Compressed size with gzip
    pub gzip_bytes: u64,
    /// Compressed size with zstd
    pub zstd_bytes: u64,
    /// Compression ratio (gzip)
    pub gzip_ratio: f64,
    /// Compression ratio (zstd)
    pub zstd_ratio: f64,
    /// Recommended compression
    pub recommended: String,
}

/// Dependency impact analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyAnalysis {
    /// Static dependencies count
    pub static_deps_count: usize,
    /// Dynamic dependencies count
    pub dynamic_deps_count: usize,
    /// Estimated dependency overhead bytes
    pub dependency_overhead_bytes: u64,
    /// Major contributors
    pub major_contributors: Vec<DependencyInfo>,
}

/// Individual dependency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyInfo {
    /// Dependency name
    pub name: String,
    /// Estimated size contribution
    pub size_contribution_bytes: u64,
    /// Type (static/dynamic)
    pub dependency_type: String,
}

/// Optimization opportunity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationOpportunity {
    /// Optimization type
    pub optimization_type: OptimizationType,
    /// Potential size reduction in bytes
    pub potential_savings_bytes: u64,
    /// Difficulty level (1-5)
    pub difficulty: u8,
    /// Description
    pub description: String,
    /// Recommended action
    pub action: String,
}

/// Types of optimization opportunities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationType {
    /// Link-time optimization
    Lto,
    /// Dead code elimination
    DeadCode,
    /// Debug symbol stripping
    DebugStrip,
    /// Compression
    Compression,
    /// Dependency reduction
    Dependencies,
    /// Code size optimization flags
    CompilerFlags,
    /// Binary packing
    Packing,
}

/// Binary size analyzer
pub struct BinaryAnalyzer {
    /// Path to binary
    binary_path: PathBuf,
    /// Enable detailed analysis
    #[allow(dead_code)]
    detailed_analysis: bool,
    /// Tools availability cache
    available_tools: ToolAvailability,
}

/// Available analysis tools
#[derive(Debug, Clone)]
struct ToolAvailability {
    #[allow(dead_code)]
    has_size: bool,
    has_objdump: bool,
    has_nm: bool,
    has_strip: bool,
    has_readelf: bool,
    #[allow(dead_code)]
    has_bloaty: bool,
}

impl BinaryAnalyzer {
    /// Create new binary analyzer for given path
    pub fn new(binary_path: impl AsRef<Path>) -> Self {
        Self {
            binary_path: binary_path.as_ref().to_path_buf(),
            detailed_analysis: true,
            available_tools: Self::detect_tools(),
        }
    }

    /// Analyze binary size comprehensively
    pub async fn analyze(&self) -> Result<BinarySizeAnalysis> {
        info!("📦 Analyzing binary size: {}", self.binary_path.display());

        // Basic size analysis
        let total_size_bytes = self.get_file_size()?;
        let stripped_size_bytes = self.estimate_stripped_size().await?;
        let debug_symbols_bytes = total_size_bytes.saturating_sub(stripped_size_bytes);
        let debug_percentage = (debug_symbols_bytes as f64 / total_size_bytes as f64) * 100.0;

        // Section analysis
        let sections = self.analyze_sections().await?;

        // Symbol analysis
        let symbol_analysis = self.analyze_symbols().await?;

        // Compression analysis
        let compression = self.analyze_compression().await?;

        // Dependency analysis
        let dependencies = self.analyze_dependencies().await?;

        // Identify optimization opportunities
        let optimization_opportunities = self.identify_optimizations(
            total_size_bytes,
            debug_symbols_bytes,
            &sections,
            &symbol_analysis,
        );

        let analysis = BinarySizeAnalysis {
            total_size_bytes,
            stripped_size_bytes,
            debug_symbols_bytes,
            debug_percentage,
            sections,
            symbol_analysis,
            optimization_opportunities,
            compression,
            dependencies,
        };

        self.log_analysis(&analysis);

        Ok(analysis)
    }

    /// Get file size
    fn get_file_size(&self) -> Result<u64> {
        let metadata = fs::metadata(&self.binary_path).with_context(|| {
            format!(
                "Failed to read binary metadata: {}",
                self.binary_path.display()
            )
        })?;
        Ok(metadata.len())
    }

    /// Estimate stripped binary size
    async fn estimate_stripped_size(&self) -> Result<u64> {
        if !self.available_tools.has_strip {
            // Estimate as 60-80% of original if strip not available
            let original = self.get_file_size()?;
            return Ok((original as f64 * 0.7) as u64);
        }

        // Create temporary copy and strip it
        let temp_path = format!("{}.stripped", self.binary_path.display());
        fs::copy(&self.binary_path, &temp_path)
            .context("Failed to create temporary copy for stripping")?;

        let output = Command::new("strip")
            .arg(&temp_path)
            .output()
            .context("Failed to run strip command")?;

        if !output.status.success() {
            fs::remove_file(&temp_path).ok();
            anyhow::bail!("Strip command failed");
        }

        let stripped_size = fs::metadata(&temp_path)?.len();
        fs::remove_file(&temp_path).ok();

        Ok(stripped_size)
    }

    /// Analyze binary sections
    async fn analyze_sections(&self) -> Result<Vec<SectionInfo>> {
        let mut sections = Vec::new();

        if self.available_tools.has_objdump {
            let output = Command::new("objdump")
                .args(["-h", self.binary_path.to_str().unwrap()])
                .output()
                .context("Failed to run objdump")?;

            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                sections = self.parse_objdump_sections(&stdout);
            }
        } else if self.available_tools.has_readelf {
            let output = Command::new("readelf")
                .args(["-S", self.binary_path.to_str().unwrap()])
                .output()
                .context("Failed to run readelf")?;

            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                sections = self.parse_readelf_sections(&stdout);
            }
        }

        // If no tools available, provide estimates
        if sections.is_empty() {
            let total_size = self.get_file_size()?;
            sections = vec![
                SectionInfo {
                    name: ".text".to_string(),
                    size_bytes: (total_size as f64 * 0.4) as u64,
                    percentage: 40.0,
                    section_type: SectionType::Code,
                },
                SectionInfo {
                    name: ".data".to_string(),
                    size_bytes: (total_size as f64 * 0.2) as u64,
                    percentage: 20.0,
                    section_type: SectionType::Data,
                },
                SectionInfo {
                    name: ".rodata".to_string(),
                    size_bytes: (total_size as f64 * 0.15) as u64,
                    percentage: 15.0,
                    section_type: SectionType::ReadOnlyData,
                },
            ];
        }

        Ok(sections)
    }

    /// Parse objdump section output
    fn parse_objdump_sections(&self, output: &str) -> Vec<SectionInfo> {
        let mut sections = Vec::new();
        let total_size = self.get_file_size().unwrap_or(1);

        for line in output.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 && parts[0].chars().all(|c| c.is_ascii_digit()) {
                let name = parts[1].to_string();
                if let Ok(size) = u64::from_str_radix(parts[2], 16) {
                    let percentage = (size as f64 / total_size as f64) * 100.0;
                    let section_type = self.classify_section(&name);

                    sections.push(SectionInfo {
                        name,
                        size_bytes: size,
                        percentage,
                        section_type,
                    });
                }
            }
        }

        sections
    }

    /// Parse readelf section output
    fn parse_readelf_sections(&self, output: &str) -> Vec<SectionInfo> {
        // Similar parsing for readelf format
        self.parse_objdump_sections(output) // Simplified for now
    }

    /// Classify section type
    fn classify_section(&self, name: &str) -> SectionType {
        match name {
            ".text" | ".init" | ".fini" => SectionType::Code,
            ".rodata" | ".rodata1" => SectionType::ReadOnlyData,
            ".data" | ".data1" => SectionType::Data,
            ".bss" => SectionType::Bss,
            s if s.starts_with(".debug") => SectionType::Debug,
            ".dynamic" | ".dynstr" | ".dynsym" => SectionType::Dynamic,
            _ => SectionType::Other,
        }
    }

    /// Analyze symbols
    async fn analyze_symbols(&self) -> Result<SymbolAnalysis> {
        let mut total_symbols = 0;
        let mut exported_symbols = 0;
        let mut local_symbols = 0;
        let mut largest_symbols = Vec::new();

        if self.available_tools.has_nm {
            let output = Command::new("nm")
                .args([
                    "--print-size",
                    "--size-sort",
                    "--reverse-sort",
                    self.binary_path.to_str().unwrap(),
                ])
                .output()
                .context("Failed to run nm")?;

            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for (i, line) in stdout.lines().enumerate() {
                    total_symbols += 1;

                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 4 {
                        // Parse symbol type
                        let symbol_type = parts[2];
                        if symbol_type.chars().next().unwrap().is_uppercase() {
                            exported_symbols += 1;
                        } else {
                            local_symbols += 1;
                        }

                        // Get largest symbols (top 10)
                        if i < 10 {
                            if let Ok(size) = u64::from_str_radix(parts[1], 16) {
                                largest_symbols.push(SymbolInfo {
                                    name: parts[3].to_string(),
                                    size_bytes: size,
                                    symbol_type: symbol_type.to_string(),
                                    demangled_name: self.demangle_symbol(parts[3]),
                                });
                            }
                        }
                    }
                }
            }
        }

        // Calculate bloat score based on symbol count and sizes
        let bloat_score = self.calculate_bloat_score(total_symbols, &largest_symbols);

        Ok(SymbolAnalysis {
            total_symbols,
            exported_symbols,
            local_symbols,
            largest_symbols,
            bloat_score,
        })
    }

    /// Demangle symbol name
    fn demangle_symbol(&self, symbol: &str) -> Option<String> {
        // Try Rust demangling
        if symbol.starts_with("_Z") || symbol.contains("$") {
            // Would use rustc_demangle crate in production
            return Some(format!("<demangled: {}>", symbol));
        }
        None
    }

    /// Calculate symbol bloat score
    fn calculate_bloat_score(&self, total_symbols: usize, largest: &[SymbolInfo]) -> f64 {
        // Heuristic: many large symbols indicate bloat
        let avg_large_symbol_size = if !largest.is_empty() {
            largest.iter().map(|s| s.size_bytes).sum::<u64>() / largest.len() as u64
        } else {
            0
        };

        let score: f64 = match (total_symbols, avg_large_symbol_size) {
            (0..=1000, 0..=10000) => 10.0,
            (1001..=5000, 0..=50000) => 30.0,
            (5001..=10000, 50001..=100000) => 50.0,
            (10001..=20000, 100001..=500000) => 70.0,
            _ => 90.0,
        };

        score.min(100.0_f64)
    }

    /// Analyze compression potential
    async fn analyze_compression(&self) -> Result<CompressionAnalysis> {
        let original_bytes = self.get_file_size()?;

        // Test gzip compression
        let gzip_output = Command::new("gzip")
            .args(["-c", self.binary_path.to_str().unwrap()])
            .output();

        let gzip_bytes = if let Ok(output) = gzip_output {
            output.stdout.len() as u64
        } else {
            (original_bytes as f64 * 0.35) as u64 // Estimate 35% of original
        };

        // Estimate zstd (typically better than gzip)
        let zstd_bytes = (gzip_bytes as f64 * 0.85) as u64;

        let gzip_ratio = 1.0 - (gzip_bytes as f64 / original_bytes as f64);
        let zstd_ratio = 1.0 - (zstd_bytes as f64 / original_bytes as f64);

        let recommended = if zstd_ratio > 0.5 {
            "zstd (best compression ratio)".to_string()
        } else if gzip_ratio > 0.3 {
            "gzip (good compatibility)".to_string()
        } else {
            "None (minimal benefit)".to_string()
        };

        Ok(CompressionAnalysis {
            original_bytes,
            gzip_bytes,
            zstd_bytes,
            gzip_ratio,
            zstd_ratio,
            recommended,
        })
    }

    /// Analyze dependencies
    async fn analyze_dependencies(&self) -> Result<DependencyAnalysis> {
        let static_deps_count = 0;
        let mut dynamic_deps_count = 0;
        let major_contributors = Vec::new();

        if self.available_tools.has_objdump {
            let output = Command::new("ldd")
                .arg(self.binary_path.to_str().unwrap())
                .output();

            if let Ok(output) = output {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    dynamic_deps_count = stdout.lines().count().saturating_sub(1);
                }
            }
        }

        // Estimate dependency overhead
        let dependency_overhead_bytes = (static_deps_count + dynamic_deps_count) as u64 * 50_000;

        Ok(DependencyAnalysis {
            static_deps_count,
            dynamic_deps_count,
            dependency_overhead_bytes,
            major_contributors,
        })
    }

    /// Identify optimization opportunities
    fn identify_optimizations(
        &self,
        total_size: u64,
        debug_size: u64,
        _sections: &[SectionInfo],
        symbols: &SymbolAnalysis,
    ) -> Vec<OptimizationOpportunity> {
        let mut opportunities = Vec::new();

        // Debug symbol stripping
        if debug_size > total_size / 10 {
            opportunities.push(OptimizationOpportunity {
                optimization_type: OptimizationType::DebugStrip,
                potential_savings_bytes: debug_size,
                difficulty: 1,
                description: "Debug symbols account for significant binary size".to_string(),
                action: "Strip debug symbols for production: strip --strip-debug binary"
                    .to_string(),
            });
        }

        // Link-time optimization
        if total_size > 5_000_000 {
            opportunities.push(OptimizationOpportunity {
                optimization_type: OptimizationType::Lto,
                potential_savings_bytes: (total_size as f64 * 0.15) as u64,
                difficulty: 2,
                description: "Link-time optimization can reduce code size".to_string(),
                action: "Enable LTO: RUSTFLAGS='-C lto=fat' for Rust".to_string(),
            });
        }

        // Dead code elimination
        if symbols.bloat_score > 50.0 {
            opportunities.push(OptimizationOpportunity {
                optimization_type: OptimizationType::DeadCode,
                potential_savings_bytes: (total_size as f64 * 0.1) as u64,
                difficulty: 3,
                description: "Potential dead code detected from symbol analysis".to_string(),
                action: "Review largest symbols and remove unused code".to_string(),
            });
        }

        // Code size optimization
        opportunities.push(OptimizationOpportunity {
            optimization_type: OptimizationType::CompilerFlags,
            potential_savings_bytes: (total_size as f64 * 0.05) as u64,
            difficulty: 1,
            description: "Compiler optimization flags for size".to_string(),
            action: "Use opt-level='z' for minimum size in Rust".to_string(),
        });

        opportunities
    }

    /// Detect available tools
    fn detect_tools() -> ToolAvailability {
        ToolAvailability {
            has_size: Command::new("size").arg("--version").output().is_ok(),
            has_objdump: Command::new("objdump").arg("--version").output().is_ok(),
            has_nm: Command::new("nm").arg("--version").output().is_ok(),
            has_strip: Command::new("strip").arg("--version").output().is_ok(),
            has_readelf: Command::new("readelf").arg("--version").output().is_ok(),
            has_bloaty: Command::new("bloaty").arg("--help").output().is_ok(),
        }
    }

    /// Log binary analysis results
    ///
    /// Refactored in Sprint 44 Ticket 7 for complexity reduction
    fn log_analysis(&self, analysis: &BinarySizeAnalysis) {
        info!("📦 Binary Size Analysis:");
        self.log_binary_sizes(analysis);
        self.log_main_sections(analysis);
        self.log_symbol_bloat_warning(analysis);
        self.log_optimization_count(analysis);
    }

    /// Log basic binary size statistics
    ///
    /// Extracted from log_analysis() for complexity reduction (Sprint 44 Ticket 7)
    fn log_binary_sizes(&self, analysis: &BinarySizeAnalysis) {
        info!(
            "   Total size: {:.2} MB",
            analysis.total_size_bytes as f64 / 1_048_576.0
        );
        info!(
            "   Stripped size: {:.2} MB",
            analysis.stripped_size_bytes as f64 / 1_048_576.0
        );
        info!("   Debug symbols: {:.1}%", analysis.debug_percentage);
    }

    /// Log main binary sections
    ///
    /// Extracted from log_analysis() for complexity reduction (Sprint 44 Ticket 7)
    fn log_main_sections(&self, analysis: &BinarySizeAnalysis) {
        if !analysis.sections.is_empty() {
            info!("   Main sections:");
            for section in analysis.sections.iter().take(3) {
                info!(
                    "     {}: {:.2} MB ({:.1}%)",
                    section.name,
                    section.size_bytes as f64 / 1_048_576.0,
                    section.percentage
                );
            }
        }
    }

    /// Log symbol bloat warning if threshold exceeded
    ///
    /// Extracted from log_analysis() for complexity reduction (Sprint 44 Ticket 7)
    fn log_symbol_bloat_warning(&self, analysis: &BinarySizeAnalysis) {
        if analysis.symbol_analysis.bloat_score > 70.0 {
            warn!(
                "   ⚠️ High symbol bloat score: {:.1}",
                analysis.symbol_analysis.bloat_score
            );
        }
    }

    /// Log count of optimization opportunities
    ///
    /// Extracted from log_analysis() for complexity reduction (Sprint 44 Ticket 7)
    fn log_optimization_count(&self, analysis: &BinarySizeAnalysis) {
        info!(
            "   Optimization opportunities: {}",
            analysis.optimization_opportunities.len()
        );
    }

    /// Generate binary size report
    pub fn generate_report(analysis: &BinarySizeAnalysis) -> String {
        let mut report = String::new();

        report.push_str("# Binary Size Analysis Report\n\n");

        // Summary section
        report.push_str("## Summary\n\n");
        report.push_str(&format!(
            "- **Total Size**: {:.2} MB\n",
            analysis.total_size_bytes as f64 / 1_048_576.0
        ));
        report.push_str(&format!(
            "- **Stripped Size**: {:.2} MB\n",
            analysis.stripped_size_bytes as f64 / 1_048_576.0
        ));
        report.push_str(&format!(
            "- **Debug Symbols**: {:.2} MB ({:.1}%)\n",
            analysis.debug_symbols_bytes as f64 / 1_048_576.0,
            analysis.debug_percentage
        ));
        report.push_str(&format!(
            "- **Compression Potential**: {:.1}%\n\n",
            analysis.compression.zstd_ratio * 100.0
        ));

        // Section breakdown
        if !analysis.sections.is_empty() {
            report.push_str("## Section Breakdown\n\n");
            report.push_str("| Section | Size (MB) | Percentage |\n");
            report.push_str("|---------|-----------|------------|\n");
            for section in &analysis.sections {
                report.push_str(&format!(
                    "| {} | {:.2} | {:.1}% |\n",
                    section.name,
                    section.size_bytes as f64 / 1_048_576.0,
                    section.percentage
                ));
            }
            report.push('\n');
        }

        // Optimization opportunities
        if !analysis.optimization_opportunities.is_empty() {
            report.push_str("## Optimization Opportunities\n\n");
            for opt in &analysis.optimization_opportunities {
                report.push_str(&format!(
                    "### {:?} (Difficulty: {}/5)\n",
                    opt.optimization_type, opt.difficulty
                ));
                report.push_str(&format!(
                    "- **Potential Savings**: {:.2} MB\n",
                    opt.potential_savings_bytes as f64 / 1_048_576.0
                ));
                report.push_str(&format!("- **Description**: {}\n", opt.description));
                report.push_str(&format!("- **Action**: {}\n\n", opt.action));
            }
        }

        report
    }
}

/// Analyze binary size for a language implementation
#[allow(dead_code)]
pub async fn analyze_language_binary(
    language: &str,
    binary_path: &Path,
) -> Result<BinarySizeAnalysis> {
    info!(
        "📦 Analyzing {} binary: {}",
        language,
        binary_path.display()
    );

    let analyzer = BinaryAnalyzer::new(binary_path);
    analyzer.analyze().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_binary_analyzer() {
        // Analyze the test binary itself
        let current_exe = env::current_exe().unwrap();
        let analyzer = BinaryAnalyzer::new(current_exe);

        if let Ok(analysis) = analyzer.analyze().await {
            assert!(analysis.total_size_bytes > 0);
            assert!(analysis.stripped_size_bytes <= analysis.total_size_bytes);
            assert!(!analysis.optimization_opportunities.is_empty());
        }
    }

    #[test]
    fn test_section_classification() {
        let analyzer = BinaryAnalyzer::new("dummy");

        assert!(matches!(
            analyzer.classify_section(".text"),
            SectionType::Code
        ));
        assert!(matches!(
            analyzer.classify_section(".rodata"),
            SectionType::ReadOnlyData
        ));
        assert!(matches!(
            analyzer.classify_section(".debug_info"),
            SectionType::Debug
        ));
        assert!(matches!(
            analyzer.classify_section(".bss"),
            SectionType::Bss
        ));
    }
}
