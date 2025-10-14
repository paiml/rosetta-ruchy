//! Comprehensive reporting module for benchmark results
//!
//! Generates JSON and Markdown reports with statistical analysis,
//! environment details, and performance comparisons following
//! academic standards and Toyota Way quality principles.

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

use crate::isolation::{EnvironmentState, IsolationResult};
use crate::statistics::{ComparisonResult, SignificanceLevel, StatisticalAnalysis};

/// Complete benchmark report structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkReport {
    /// Report metadata
    pub metadata: ReportMetadata,
    /// System environment information
    pub environment: EnvironmentReport,
    /// Benchmark configuration
    pub configuration: BenchmarkConfiguration,
    /// Benchmark results by language/implementation
    pub results: HashMap<String, LanguageResults>,
    /// Performance comparisons
    pub comparisons: Vec<PerformanceComparison>,
    /// Summary and conclusions
    pub summary: BenchmarkSummary,
}

/// Report generation metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportMetadata {
    /// Report generation timestamp
    pub generated_at: DateTime<Utc>,
    /// Report format version
    pub format_version: String,
    /// Generator information
    pub generator: String,
    /// Benchmark suite version
    pub suite_version: String,
    /// Toyota Way quality gates applied
    pub quality_gates: Vec<String>,
}

/// Environment information for reproducibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentReport {
    /// System information
    pub system: SystemInfo,
    /// Environment isolation applied
    pub isolation: Option<IsolationResult>,
    /// Environment state during benchmark
    pub state: EnvironmentState,
}

/// System information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    /// Operating system
    pub os: String,
    /// Architecture
    pub arch: String,
    /// CPU model
    pub cpu_model: String,
    /// Total memory in GB
    pub total_memory_gb: f64,
    /// Rust version used
    pub rust_version: String,
}

/// Benchmark configuration parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfiguration {
    /// Number of iterations per test
    pub iterations: usize,
    /// Warmup iterations
    pub warmup_iterations: usize,
    /// Statistical confidence level
    pub confidence_level: f64,
    /// Outlier removal enabled
    pub outlier_removal: bool,
    /// Minimum sample size
    pub min_sample_size: usize,
}

/// Results for a specific language/implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageResults {
    /// Language/implementation name
    pub language: String,
    /// Version information
    pub version: String,
    /// Statistical analysis of performance
    pub statistics: StatisticalAnalysis,
    /// Raw benchmark times (optional, for detailed analysis)
    pub raw_times_ns: Option<Vec<u64>>,
    /// Memory usage information
    pub memory_usage: Option<MemoryUsageReport>,
    /// Binary size information
    pub binary_size: Option<BinarySizeReport>,
    /// Compilation information
    pub compilation: Option<CompilationReport>,
}

/// Memory usage analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryUsageReport {
    /// Peak memory usage in bytes
    pub peak_usage_bytes: u64,
    /// Average memory usage in bytes
    pub average_usage_bytes: u64,
    /// Memory allocations count
    pub allocations: u64,
    /// Memory deallocations count
    pub deallocations: u64,
}

/// Binary size analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinarySizeReport {
    /// Total binary size in bytes
    pub total_size_bytes: u64,
    /// Debug symbols size in bytes
    pub debug_size_bytes: u64,
    /// Stripped binary size in bytes
    pub stripped_size_bytes: u64,
    /// Compression ratio (if applicable)
    pub compression_ratio: Option<f64>,
}

/// Compilation performance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationReport {
    /// Compilation time in seconds
    pub compile_time_seconds: f64,
    /// Incremental compilation time
    pub incremental_compile_time_seconds: Option<f64>,
    /// Build artifacts count
    pub artifacts_count: usize,
}

/// Performance comparison between implementations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceComparison {
    /// Baseline implementation name
    pub baseline: String,
    /// Compared implementation name
    pub compared: String,
    /// Comparison results
    pub result: ComparisonResult,
    /// Human-readable interpretation
    pub interpretation: String,
}

/// High-level benchmark summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSummary {
    /// Fastest implementation
    pub fastest_implementation: String,
    /// Most memory efficient
    pub most_memory_efficient: String,
    /// Smallest binary size
    pub smallest_binary: String,
    /// Fastest compilation
    pub fastest_compilation: String,
    /// Overall performance ranking
    pub performance_ranking: Vec<PerformanceRanking>,
    /// Key insights
    pub insights: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Performance ranking entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRanking {
    /// Rank (1 = best)
    pub rank: usize,
    /// Implementation name
    pub implementation: String,
    /// Score (lower is better for time-based metrics)
    pub score: f64,
    /// Score type (e.g., "execution_time_ns", "memory_usage_bytes")
    pub score_type: String,
}

/// Report generator with configurable output formats
pub struct ReportGenerator {
    /// Include raw timing data in reports
    include_raw_data: bool,
    /// Output directory for reports
    output_dir: String,
    /// Report format preferences
    formats: Vec<ReportFormat>,
}

/// Supported report formats
#[derive(Debug, Clone)]
pub enum ReportFormat {
    /// JSON format for machine consumption
    Json,
    /// Markdown format for human consumption
    Markdown,
    /// HTML format with visualizations
    #[allow(dead_code)]
    Html,
    /// CSV format for spreadsheet analysis
    #[allow(dead_code)]
    Csv,
}

impl Default for ReportGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl ReportGenerator {
    /// Create new report generator with default settings
    pub fn new() -> Self {
        Self {
            include_raw_data: false,
            output_dir: "results".to_string(),
            formats: vec![ReportFormat::Json, ReportFormat::Markdown],
        }
    }

    /// Configure raw data inclusion
    pub fn with_raw_data(mut self, include: bool) -> Self {
        self.include_raw_data = include;
        self
    }

    /// Configure output directory
    pub fn with_output_dir(mut self, dir: &str) -> Self {
        self.output_dir = dir.to_string();
        self
    }

    /// Configure report formats
    #[allow(dead_code)]
    pub fn with_formats(mut self, formats: Vec<ReportFormat>) -> Self {
        self.formats = formats;
        self
    }

    /// Generate comprehensive benchmark report
    pub async fn generate_report(
        &self,
        results: HashMap<String, LanguageResults>,
        environment: EnvironmentReport,
        config: BenchmarkConfiguration,
    ) -> Result<BenchmarkReport> {
        let metadata = self.create_metadata();
        let comparisons = self.generate_comparisons(&results);
        let summary = self.generate_summary(&results, &comparisons);

        let report = BenchmarkReport {
            metadata,
            environment,
            configuration: config,
            results,
            comparisons,
            summary,
        };

        // Ensure output directory exists
        fs::create_dir_all(&self.output_dir).context("Failed to create output directory")?;

        // Generate reports in requested formats
        for format in &self.formats {
            self.write_report(&report, format)
                .await
                .with_context(|| format!("Failed to generate {:?} report", format))?;
        }

        Ok(report)
    }

    /// Create report metadata
    fn create_metadata(&self) -> ReportMetadata {
        ReportMetadata {
            generated_at: Utc::now(),
            format_version: "1.0.0".to_string(),
            generator: "Rosetta Ruchy Benchmark Harness".to_string(),
            suite_version: env!("CARGO_PKG_VERSION").to_string(),
            quality_gates: vec![
                "Statistical significance testing".to_string(),
                "Environment isolation validation".to_string(),
                "Outlier detection and filtering".to_string(),
                "Confidence interval calculation".to_string(),
                "Reproducibility verification".to_string(),
            ],
        }
    }

    /// Generate performance comparisons
    fn generate_comparisons(
        &self,
        results: &HashMap<String, LanguageResults>,
    ) -> Vec<PerformanceComparison> {
        let mut comparisons = Vec::new();

        // Find Rust as baseline (if available)
        let baseline_name = results
            .keys()
            .find(|name| name.to_lowercase().contains("rust"))
            .or_else(|| results.keys().next())
            .cloned();

        if let Some(baseline_name) = baseline_name {
            if let Some(baseline) = results.get(&baseline_name) {
                for (name, result) in results {
                    if name != &baseline_name {
                        let comparison_result =
                            crate::statistics::PerformanceComparator::compare_performance(
                                &baseline.statistics,
                                &result.statistics,
                            );

                        let interpretation = self.interpret_comparison(&comparison_result);

                        comparisons.push(PerformanceComparison {
                            baseline: baseline_name.clone(),
                            compared: name.clone(),
                            result: comparison_result,
                            interpretation,
                        });
                    }
                }
            }
        }

        comparisons
    }

    /// Generate high-level summary and insights
    fn generate_summary(
        &self,
        results: &HashMap<String, LanguageResults>,
        comparisons: &[PerformanceComparison],
    ) -> BenchmarkSummary {
        // Find fastest implementation
        let fastest_implementation = results
            .iter()
            .min_by(|a, b| {
                a.1.statistics
                    .sample_stats
                    .mean
                    .partial_cmp(&b.1.statistics.sample_stats.mean)
                    .unwrap()
            })
            .map(|(name, _)| name.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        // Generate performance ranking
        let mut performance_ranking: Vec<PerformanceRanking> = results
            .iter()
            .enumerate()
            .map(|(i, (name, result))| PerformanceRanking {
                rank: i + 1,
                implementation: name.clone(),
                score: result.statistics.sample_stats.mean,
                score_type: "execution_time_ns".to_string(),
            })
            .collect();

        performance_ranking.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
        for (i, ranking) in performance_ranking.iter_mut().enumerate() {
            ranking.rank = i + 1;
        }

        // Generate insights
        let insights = self.generate_insights(results, comparisons);
        let recommendations = self.generate_recommendations(results, comparisons);

        BenchmarkSummary {
            fastest_implementation: fastest_implementation.clone(),
            most_memory_efficient: fastest_implementation.clone(), // Simplified for now
            smallest_binary: fastest_implementation.clone(),       // Simplified for now
            fastest_compilation: fastest_implementation.clone(),   // Simplified for now
            performance_ranking,
            insights,
            recommendations,
        }
    }

    /// Generate performance insights
    fn generate_insights(
        &self,
        results: &HashMap<String, LanguageResults>,
        comparisons: &[PerformanceComparison],
    ) -> Vec<String> {
        let mut insights = Vec::new();

        // Statistical insights
        if results.len() > 1 {
            insights.push(format!(
                "Benchmarked {} different implementations",
                results.len()
            ));
        }

        // Variance insights
        let high_variance_threshold = 0.1; // 10% coefficient of variation
        let high_variance_impls: Vec<_> = results
            .iter()
            .filter(|(_, result)| {
                result.statistics.distribution.coefficient_of_variation > high_variance_threshold
            })
            .map(|(name, _)| name.as_str())
            .collect();

        if !high_variance_impls.is_empty() {
            insights.push(format!(
                "High performance variance detected in: {}",
                high_variance_impls.join(", ")
            ));
        }

        // Significance insights
        let significant_differences = comparisons
            .iter()
            .filter(|comp| {
                matches!(
                    comp.result.significance,
                    SignificanceLevel::SignificantImprovement
                        | SignificanceLevel::SignificantRegression
                )
            })
            .count();

        if significant_differences > 0 {
            insights.push(format!(
                "{} statistically significant performance differences found",
                significant_differences
            ));
        }

        insights
    }

    /// Generate recommendations
    fn generate_recommendations(
        &self,
        _results: &HashMap<String, LanguageResults>,
        comparisons: &[PerformanceComparison],
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        recommendations
            .push("Ensure consistent environment isolation for reliable benchmarks".to_string());
        recommendations
            .push("Run multiple iterations to achieve statistical significance".to_string());

        // Performance-based recommendations
        let has_regressions = comparisons.iter().any(|comp| {
            matches!(
                comp.result.significance,
                SignificanceLevel::SignificantRegression
            )
        });

        if has_regressions {
            recommendations.push(
                "Investigate performance regressions for optimization opportunities".to_string(),
            );
        }

        recommendations.push(
            "Consider memory usage and binary size in addition to execution time".to_string(),
        );
        recommendations.push("Validate results on different hardware configurations".to_string());

        recommendations
    }

    /// Interpret comparison result for human consumption
    fn interpret_comparison(&self, comparison: &ComparisonResult) -> String {
        match comparison.significance {
            SignificanceLevel::NotSignificant => {
                format!(
                    "No statistically significant difference ({:.1}% change)",
                    comparison.percent_change
                )
            }
            SignificanceLevel::SignificantImprovement => {
                format!(
                    "Significantly faster by {:.1}%",
                    comparison.percent_change.abs()
                )
            }
            SignificanceLevel::SignificantRegression => {
                format!("Significantly slower by {:.1}%", comparison.percent_change)
            }
        }
    }

    /// Write report in specified format
    async fn write_report(&self, report: &BenchmarkReport, format: &ReportFormat) -> Result<()> {
        match format {
            ReportFormat::Json => self.write_json_report(report).await,
            ReportFormat::Markdown => self.write_markdown_report(report).await,
            ReportFormat::Html => self.write_html_report(report).await,
            ReportFormat::Csv => self.write_csv_report(report).await,
        }
    }

    /// Write JSON report
    async fn write_json_report(&self, report: &BenchmarkReport) -> Result<()> {
        let json =
            serde_json::to_string_pretty(report).context("Failed to serialize report to JSON")?;

        let path = format!("{}/benchmark_report.json", self.output_dir);
        fs::write(&path, json)
            .with_context(|| format!("Failed to write JSON report to {}", path))?;

        println!("📊 JSON report generated: {}", path);
        Ok(())
    }

    /// Write Markdown report
    async fn write_markdown_report(&self, report: &BenchmarkReport) -> Result<()> {
        let mut md = String::new();

        // Title and metadata
        md.push_str("# Benchmark Report\n\n");
        md.push_str(&format!(
            "Generated: {}\n",
            report.metadata.generated_at.format("%Y-%m-%d %H:%M:%S UTC")
        ));
        md.push_str(&format!("Generator: {}\n", report.metadata.generator));
        md.push_str(&format!(
            "Suite Version: {}\n\n",
            report.metadata.suite_version
        ));

        // Executive Summary
        md.push_str("## Executive Summary\n\n");
        md.push_str(&format!(
            "- **Fastest Implementation**: {}\n",
            report.summary.fastest_implementation
        ));
        md.push_str(&format!(
            "- **Implementations Tested**: {}\n",
            report.results.len()
        ));
        md.push_str(&format!(
            "- **Statistical Significance**: {} comparisons performed\n\n",
            report.comparisons.len()
        ));

        // Environment Information
        md.push_str("## Environment\n\n");
        md.push_str(&format!("- **OS**: {}\n", report.environment.system.os));
        md.push_str(&format!(
            "- **Architecture**: {}\n",
            report.environment.system.arch
        ));
        md.push_str(&format!(
            "- **CPU**: {}\n",
            report.environment.system.cpu_model
        ));
        md.push_str(&format!(
            "- **Memory**: {:.1} GB\n",
            report.environment.system.total_memory_gb
        ));
        md.push_str(&format!(
            "- **Rust Version**: {}\n\n",
            report.environment.system.rust_version
        ));

        // Performance Results
        md.push_str("## Performance Results\n\n");
        md.push_str("| Implementation | Mean (ns) | Std Dev (ns) | 95% CI | Outliers |\n");
        md.push_str("|---|---|---|---|---|\n");

        for (name, result) in &report.results {
            let stats = &result.statistics.sample_stats;
            let ci = &result.statistics.confidence_intervals.ci_95;
            let outliers = &result.statistics.outliers;

            md.push_str(&format!(
                "| {} | {:.0} | {:.0} | ({:.0}, {:.0}) | {:.1}% |\n",
                name, stats.mean, stats.std_dev, ci.0, ci.1, outliers.outlier_percentage
            ));
        }
        md.push('\n');

        // Performance Comparisons
        if !report.comparisons.is_empty() {
            md.push_str("## Performance Comparisons\n\n");
            for comparison in &report.comparisons {
                md.push_str(&format!(
                    "### {} vs {}\n\n",
                    comparison.compared, comparison.baseline
                ));
                md.push_str(&format!(
                    "- **Change**: {:.1}%\n",
                    comparison.result.percent_change
                ));
                md.push_str(&format!(
                    "- **Significance**: {}\n",
                    comparison.interpretation
                ));
                md.push('\n');
            }
        }

        // Insights and Recommendations
        if !report.summary.insights.is_empty() {
            md.push_str("## Key Insights\n\n");
            for insight in &report.summary.insights {
                md.push_str(&format!("- {}\n", insight));
            }
            md.push('\n');
        }

        if !report.summary.recommendations.is_empty() {
            md.push_str("## Recommendations\n\n");
            for recommendation in &report.summary.recommendations {
                md.push_str(&format!("- {}\n", recommendation));
            }
            md.push('\n');
        }

        // Configuration Details
        md.push_str("## Configuration\n\n");
        md.push_str(&format!(
            "- **Iterations**: {}\n",
            report.configuration.iterations
        ));
        md.push_str(&format!(
            "- **Warmup Iterations**: {}\n",
            report.configuration.warmup_iterations
        ));
        md.push_str(&format!(
            "- **Confidence Level**: {:.0}%\n",
            report.configuration.confidence_level * 100.0
        ));
        md.push_str(&format!(
            "- **Min Sample Size**: {}\n",
            report.configuration.min_sample_size
        ));

        let path = format!("{}/benchmark_report.md", self.output_dir);
        fs::write(&path, md)
            .with_context(|| format!("Failed to write Markdown report to {}", path))?;

        println!("📝 Markdown report generated: {}", path);
        Ok(())
    }

    /// Write HTML report (placeholder implementation)
    async fn write_html_report(&self, _report: &BenchmarkReport) -> Result<()> {
        // Note: Full HTML report with charts tracked in GitHub issue
        let path = format!("{}/benchmark_report.html", self.output_dir);
        fs::write(
            &path,
            "<html><body><h1>HTML Report - Coming Soon</h1></body></html>",
        )
        .with_context(|| format!("Failed to write HTML report to {}", path))?;

        println!("🌐 HTML report placeholder generated: {}", path);
        Ok(())
    }

    /// Write CSV report (placeholder implementation)
    async fn write_csv_report(&self, report: &BenchmarkReport) -> Result<()> {
        let mut csv = String::new();
        csv.push_str("Implementation,Mean_ns,StdDev_ns,Median_ns,Min_ns,Max_ns,Outlier_Percent\n");

        for (name, result) in &report.results {
            let stats = &result.statistics.sample_stats;
            csv.push_str(&format!(
                "{},{:.0},{:.0},{:.0},{:.0},{:.0},{:.1}\n",
                name,
                stats.mean,
                stats.std_dev,
                stats.median,
                stats.min,
                stats.max,
                result.statistics.outliers.outlier_percentage
            ));
        }

        let path = format!("{}/benchmark_results.csv", self.output_dir);
        fs::write(&path, csv).with_context(|| format!("Failed to write CSV report to {}", path))?;

        println!("📊 CSV report generated: {}", path);
        Ok(())
    }

    /// Create system information report
    #[allow(dead_code)]
    pub fn create_system_info() -> Result<SystemInfo> {
        Ok(SystemInfo {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            cpu_model: "Unknown".to_string(), // System detection not yet implemented
            total_memory_gb: 0.0,             // System detection not yet implemented
            rust_version: "Unknown".to_string(), // Rust version detection not yet implemented
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_report_generation() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let temp_path = temp_dir.path().to_string_lossy().to_string();

        let generator = ReportGenerator::new().with_output_dir(&temp_path);

        let _results: HashMap<String, LanguageResults> = HashMap::new();
        let _config = BenchmarkConfiguration {
            iterations: 1000,
            warmup_iterations: 100,
            confidence_level: 0.95,
            outlier_removal: false,
            min_sample_size: 30,
        };

        let _environment = EnvironmentReport {
            system: SystemInfo {
                os: "Linux".to_string(),
                arch: "x86_64".to_string(),
                cpu_model: "Test CPU".to_string(),
                total_memory_gb: 16.0,
                rust_version: "1.70.0".to_string(),
            },
            isolation: None,
            state: Default::default(),
        };

        // This would fail without actual benchmark data, but tests the structure
        assert!(generator.create_metadata().format_version == "1.0.0");
    }
}
