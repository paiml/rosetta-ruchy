//! Performance regression detection and alerting system
//!
//! Implements Toyota Way Jidoka principle: stop production when quality
//! violations are detected. Provides automated performance regression
//! detection with configurable thresholds and comprehensive alerting.

use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::statistics::{
    ComparisonResult, PerformanceComparator, SignificanceLevel, StatisticalAnalysis,
};

/// Performance regression detector with configurable thresholds
pub struct RegressionDetector {
    /// Regression threshold as percentage (default: 5.0%)
    threshold_percent: f64,
    /// Baseline storage directory
    baselines_dir: PathBuf,
    /// History retention period in days
    history_retention_days: i64,
    /// Minimum confidence level for regression detection
    #[allow(dead_code)]
    min_confidence_level: f64,
}

/// Performance baseline for comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBaseline {
    /// Baseline identifier (language/implementation)
    pub implementation: String,
    /// Example/benchmark name
    pub example: String,
    /// Statistical analysis of baseline performance
    pub statistics: StatisticalAnalysis,
    /// When this baseline was established
    pub timestamp: DateTime<Utc>,
    /// Benchmark configuration used
    pub configuration: BaselineConfiguration,
    /// Git commit hash (if available)
    pub git_commit: Option<String>,
    /// Environment information
    pub environment_fingerprint: String,
}

/// Configuration used when establishing baseline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineConfiguration {
    /// Number of iterations
    pub iterations: usize,
    /// Warmup iterations
    pub warmup_iterations: usize,
    /// Statistical confidence level
    pub confidence_level: f64,
}

/// Regression detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionAnalysis {
    /// Whether a regression was detected
    pub regression_detected: bool,
    /// Performance comparison results
    pub comparisons: Vec<ImplementationRegression>,
    /// Overall assessment
    pub overall_status: RegressionStatus,
    /// Actionable recommendations
    pub recommendations: Vec<String>,
    /// Analysis timestamp
    pub analyzed_at: DateTime<Utc>,
}

/// Regression analysis for a specific implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationRegression {
    /// Implementation name
    pub implementation: String,
    /// Comparison with baseline
    pub comparison: ComparisonResult,
    /// Regression severity
    pub severity: RegressionSeverity,
    /// Whether this violates quality gates
    pub quality_gate_violation: bool,
    /// Specific recommendations for this implementation
    pub recommendations: Vec<String>,
}

/// Overall regression status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegressionStatus {
    /// No regressions detected - continue deployment
    Healthy,
    /// Minor regressions within acceptable bounds
    Warning,
    /// Significant regressions detected - halt deployment
    Critical,
    /// Insufficient data for regression analysis
    Inconclusive,
}

/// Severity classification for regressions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegressionSeverity {
    /// No regression detected
    None,
    /// Minor regression (< 5% slower)
    Minor,
    /// Moderate regression (5-15% slower)
    Moderate,
    /// Major regression (15-30% slower)
    Major,
    /// Critical regression (> 30% slower)
    Critical,
}

/// Regression alerting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct AlertConfiguration {
    /// Enable regression alerts
    pub enabled: bool,
    /// Webhook URL for notifications (if available)
    pub webhook_url: Option<String>,
    /// Email notifications (if configured)
    pub email_notifications: bool,
    /// Alert on warnings or only critical issues
    pub alert_threshold: RegressionSeverity,
}

impl Default for RegressionDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl RegressionDetector {
    /// Create new regression detector with default settings
    pub fn new() -> Self {
        Self {
            threshold_percent: 5.0, // 5% regression threshold (Toyota Way)
            baselines_dir: PathBuf::from("baselines"),
            history_retention_days: 90, // Keep 3 months of history
            min_confidence_level: 0.95,
        }
    }

    /// Configure regression threshold percentage
    pub fn with_threshold(mut self, threshold_percent: f64) -> Self {
        self.threshold_percent = threshold_percent.abs();
        self
    }

    /// Configure baselines directory
    pub fn with_baselines_dir(mut self, dir: PathBuf) -> Self {
        self.baselines_dir = dir;
        self
    }

    /// Configure history retention
    pub fn with_history_retention_days(mut self, days: i64) -> Self {
        self.history_retention_days = days;
        self
    }

    /// Establish performance baseline from benchmark results
    pub async fn establish_baseline(
        &self,
        implementation: &str,
        example: &str,
        statistics: StatisticalAnalysis,
        config: BaselineConfiguration,
    ) -> Result<()> {
        let baseline = PerformanceBaseline {
            implementation: implementation.to_string(),
            example: example.to_string(),
            statistics,
            timestamp: Utc::now(),
            configuration: config,
            git_commit: self.get_git_commit().await.ok(),
            environment_fingerprint: self.generate_environment_fingerprint().await,
        };

        self.save_baseline(&baseline).await.with_context(|| {
            format!("Failed to save baseline for {}/{}", implementation, example)
        })?;

        println!(
            "📊 Performance baseline established for {}/{}",
            implementation, example
        );
        println!(
            "   Mean: {:.2}ms (±{:.2}ms)",
            baseline.statistics.sample_stats.mean / 1_000_000.0,
            baseline.statistics.sample_stats.std_error / 1_000_000.0
        );

        Ok(())
    }

    /// Detect regressions by comparing current results with baseline
    pub async fn detect_regressions(
        &self,
        current_results: &HashMap<String, StatisticalAnalysis>,
        example: &str,
    ) -> Result<RegressionAnalysis> {
        let mut comparisons = Vec::new();
        let mut has_critical_regression = false;
        let mut has_warning_regression = false;

        for (implementation, current_stats) in current_results {
            if let Some(baseline) = self.load_baseline(implementation, example).await? {
                let comparison =
                    PerformanceComparator::compare_performance(&baseline.statistics, current_stats);

                let severity = self.classify_regression_severity(&comparison);
                let quality_gate_violation = self.is_quality_gate_violation(&severity, &comparison);

                if matches!(
                    severity,
                    RegressionSeverity::Major | RegressionSeverity::Critical
                ) {
                    has_critical_regression = true;
                } else if matches!(severity, RegressionSeverity::Moderate) {
                    has_warning_regression = true;
                }

                let recommendations =
                    self.generate_regression_recommendations(&severity, &comparison);

                comparisons.push(ImplementationRegression {
                    implementation: implementation.clone(),
                    comparison,
                    severity,
                    quality_gate_violation,
                    recommendations,
                });
            }
        }

        let overall_status = if has_critical_regression {
            RegressionStatus::Critical
        } else if has_warning_regression {
            RegressionStatus::Warning
        } else if comparisons.is_empty() {
            RegressionStatus::Inconclusive
        } else {
            RegressionStatus::Healthy
        };

        let overall_recommendations =
            self.generate_overall_recommendations(&overall_status, &comparisons);

        let analysis = RegressionAnalysis {
            regression_detected: has_critical_regression || has_warning_regression,
            comparisons,
            overall_status,
            recommendations: overall_recommendations,
            analyzed_at: Utc::now(),
        };

        Ok(analysis)
    }

    /// Generate regression report
    pub async fn generate_regression_report(
        &self,
        analysis: &RegressionAnalysis,
    ) -> Result<String> {
        let mut report = String::new();

        report.push_str("# Performance Regression Analysis Report\n\n");
        report.push_str(&format!(
            "Generated: {}\n\n",
            analysis.analyzed_at.format("%Y-%m-%d %H:%M:%S UTC")
        ));

        // Executive summary
        report.push_str("## Executive Summary\n\n");
        match analysis.overall_status {
            RegressionStatus::Healthy => {
                report.push_str("✅ **Status: HEALTHY** - No performance regressions detected\n");
            }
            RegressionStatus::Warning => {
                report
                    .push_str("⚠️ **Status: WARNING** - Minor performance degradation detected\n");
            }
            RegressionStatus::Critical => {
                report.push_str(
                    "🚨 **Status: CRITICAL** - Significant performance regression detected\n",
                );
                report.push_str(
                    "**RECOMMENDED ACTION**: Halt deployment and investigate performance issues\n",
                );
            }
            RegressionStatus::Inconclusive => {
                report.push_str(
                    "❓ **Status: INCONCLUSIVE** - Insufficient baseline data for analysis\n",
                );
            }
        }

        report.push_str(&format!(
            "\n- **Implementations Analyzed**: {}\n",
            analysis.comparisons.len()
        ));
        report.push_str(&format!(
            "- **Regression Threshold**: {:.1}%\n\n",
            self.threshold_percent
        ));

        // Detailed analysis
        if !analysis.comparisons.is_empty() {
            report.push_str("## Detailed Analysis\n\n");

            for comparison in &analysis.comparisons {
                report.push_str(&format!(
                    "### {} Implementation\n\n",
                    comparison.implementation
                ));

                let status_emoji = match comparison.severity {
                    RegressionSeverity::None => "✅",
                    RegressionSeverity::Minor => "💚",
                    RegressionSeverity::Moderate => "⚠️",
                    RegressionSeverity::Major => "🔶",
                    RegressionSeverity::Critical => "🚨",
                };

                report.push_str(&format!(
                    "{} **Performance Change**: {:.1}%\n",
                    status_emoji, comparison.comparison.percent_change
                ));
                report.push_str(&format!("- **Severity**: {:?}\n", comparison.severity));
                report.push_str(&format!(
                    "- **Statistical Significance**: {:?}\n",
                    comparison.comparison.significance
                ));

                if comparison.quality_gate_violation {
                    report.push_str("- **Quality Gate**: ❌ VIOLATION\n");
                } else {
                    report.push_str("- **Quality Gate**: ✅ PASSED\n");
                }

                if !comparison.recommendations.is_empty() {
                    report.push_str("\n**Recommendations**:\n");
                    for rec in &comparison.recommendations {
                        report.push_str(&format!("- {}\n", rec));
                    }
                }

                report.push('\n');
            }
        }

        // Overall recommendations
        if !analysis.recommendations.is_empty() {
            report.push_str("## Action Items\n\n");
            for rec in &analysis.recommendations {
                report.push_str(&format!("- {}\n", rec));
            }
        }

        Ok(report)
    }

    /// Clean up old baseline files
    #[allow(dead_code)]
    pub async fn cleanup_old_baselines(&self) -> Result<usize> {
        if !self.baselines_dir.exists() {
            return Ok(0);
        }

        let cutoff_date = Utc::now() - Duration::days(self.history_retention_days);
        let mut cleaned_count = 0;

        for entry in fs::read_dir(&self.baselines_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().is_some_and(|ext| ext == "json") {
                if let Ok(baseline) = self.load_baseline_from_file(&path).await {
                    if baseline.timestamp < cutoff_date {
                        fs::remove_file(&path).with_context(|| {
                            format!("Failed to remove old baseline: {}", path.display())
                        })?;
                        cleaned_count += 1;
                    }
                }
            }
        }

        if cleaned_count > 0 {
            println!("🧹 Cleaned up {} old baseline files", cleaned_count);
        }

        Ok(cleaned_count)
    }

    /// Save baseline to disk
    async fn save_baseline(&self, baseline: &PerformanceBaseline) -> Result<()> {
        fs::create_dir_all(&self.baselines_dir).context("Failed to create baselines directory")?;

        let filename = format!("{}_baseline.json", baseline.implementation);
        let path = self.baselines_dir.join(filename);

        let json =
            serde_json::to_string_pretty(baseline).context("Failed to serialize baseline")?;

        fs::write(&path, json)
            .with_context(|| format!("Failed to write baseline to {}", path.display()))?;

        Ok(())
    }

    /// Load baseline from disk
    async fn load_baseline(
        &self,
        implementation: &str,
        _example: &str,
    ) -> Result<Option<PerformanceBaseline>> {
        let filename = format!("{}_baseline.json", implementation);
        let path = self.baselines_dir.join(filename);

        if !path.exists() {
            return Ok(None);
        }

        match self.load_baseline_from_file(&path).await {
            Ok(baseline) => Ok(Some(baseline)),
            Err(_) => Ok(None), // Ignore corrupted baseline files
        }
    }

    /// Load baseline from specific file
    async fn load_baseline_from_file(&self, path: &Path) -> Result<PerformanceBaseline> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read baseline file: {}", path.display()))?;

        let baseline: PerformanceBaseline = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse baseline file: {}", path.display()))?;

        Ok(baseline)
    }

    /// Classify regression severity based on performance change
    fn classify_regression_severity(&self, comparison: &ComparisonResult) -> RegressionSeverity {
        // Only consider statistically significant regressions
        if !matches!(
            comparison.significance,
            SignificanceLevel::SignificantRegression
        ) {
            return RegressionSeverity::None;
        }

        let percent_change = comparison.percent_change;
        if percent_change < self.threshold_percent {
            RegressionSeverity::Minor
        } else if percent_change < 15.0 {
            RegressionSeverity::Moderate
        } else if percent_change < 30.0 {
            RegressionSeverity::Major
        } else {
            RegressionSeverity::Critical
        }
    }

    /// Check if regression violates quality gates
    fn is_quality_gate_violation(
        &self,
        severity: &RegressionSeverity,
        comparison: &ComparisonResult,
    ) -> bool {
        // Toyota Way: Any statistically significant regression above threshold violates quality
        matches!(
            severity,
            RegressionSeverity::Moderate | RegressionSeverity::Major | RegressionSeverity::Critical
        ) && matches!(
            comparison.significance,
            SignificanceLevel::SignificantRegression
        )
    }

    /// Generate recommendations for specific regression
    fn generate_regression_recommendations(
        &self,
        severity: &RegressionSeverity,
        _comparison: &ComparisonResult,
    ) -> Vec<String> {
        match severity {
            RegressionSeverity::None => {
                vec!["Performance within acceptable bounds - continue monitoring".to_string()]
            }
            RegressionSeverity::Minor => vec![
                "Minor performance change detected - monitor for trends".to_string(),
                "Consider profiling to identify optimization opportunities".to_string(),
            ],
            RegressionSeverity::Moderate => vec![
                "Moderate regression detected - investigate recent changes".to_string(),
                "Run performance profiling to identify bottlenecks".to_string(),
                "Consider reverting recent commits if no clear cause".to_string(),
            ],
            RegressionSeverity::Major => vec![
                "Major regression detected - halt deployment".to_string(),
                "Immediately investigate performance-critical code changes".to_string(),
                "Run detailed profiling and memory analysis".to_string(),
                "Consider performance optimization before deployment".to_string(),
            ],
            RegressionSeverity::Critical => vec![
                "Critical regression detected - block all deployments".to_string(),
                "Emergency investigation required for performance issues".to_string(),
                "Revert to last known good baseline immediately".to_string(),
                "Conduct thorough performance audit before proceeding".to_string(),
            ],
        }
    }

    /// Generate overall recommendations
    fn generate_overall_recommendations(
        &self,
        status: &RegressionStatus,
        comparisons: &[ImplementationRegression],
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        match status {
            RegressionStatus::Healthy => {
                recommendations
                    .push("All implementations performing within expected bounds".to_string());
                recommendations
                    .push("Continue with deployment - monitoring recommended".to_string());
            }
            RegressionStatus::Warning => {
                recommendations.push("Minor performance degradations detected".to_string());
                recommendations
                    .push("Schedule performance review for affected implementations".to_string());
                recommendations
                    .push("Consider additional testing before full deployment".to_string());
            }
            RegressionStatus::Critical => {
                recommendations.push("CRITICAL: Halt deployment immediately".to_string());
                recommendations
                    .push("Investigate performance regressions before proceeding".to_string());
                recommendations
                    .push("Consider rolling back to previous stable version".to_string());
                recommendations.push("Run comprehensive performance analysis".to_string());
            }
            RegressionStatus::Inconclusive => {
                recommendations
                    .push("Establish performance baselines before regression analysis".to_string());
                recommendations.push("Run benchmark suite to collect baseline data".to_string());
            }
        }

        // Add specific recommendations based on violations
        let violations: Vec<_> = comparisons
            .iter()
            .filter(|c| c.quality_gate_violation)
            .collect();

        if !violations.is_empty() {
            recommendations.push(format!(
                "{} quality gate violations require immediate attention",
                violations.len()
            ));
        }

        recommendations
    }

    /// Get current git commit hash
    async fn get_git_commit(&self) -> Result<String> {
        let output = tokio::process::Command::new("git")
            .arg("rev-parse")
            .arg("HEAD")
            .output()
            .await
            .context("Failed to execute git command")?;

        if output.status.success() {
            let commit = String::from_utf8(output.stdout)
                .context("Invalid UTF-8 in git output")?
                .trim()
                .to_string();
            Ok(commit)
        } else {
            anyhow::bail!("Git command failed")
        }
    }

    /// Generate environment fingerprint for baseline validation
    async fn generate_environment_fingerprint(&self) -> String {
        // Simple fingerprint based on system characteristics
        // In production, this would include more detailed system information
        format!(
            "{}-{}-{}",
            std::env::consts::OS,
            std::env::consts::ARCH,
            chrono::Utc::now().format("%Y%m")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_regression_severity_classification() {
        let detector = RegressionDetector::new();

        // Test different regression levels
        let comparison = ComparisonResult {
            percent_change: 3.0,
            absolute_change: 30000.0,
            significance: SignificanceLevel::SignificantRegression,
            baseline_mean: 1000000.0,
            current_mean: 1030000.0,
        };

        let severity = detector.classify_regression_severity(&comparison);
        assert!(matches!(severity, RegressionSeverity::Minor));

        let comparison = ComparisonResult {
            percent_change: 10.0,
            absolute_change: 100000.0,
            significance: SignificanceLevel::SignificantRegression,
            baseline_mean: 1000000.0,
            current_mean: 1100000.0,
        };

        let severity = detector.classify_regression_severity(&comparison);
        assert!(matches!(severity, RegressionSeverity::Moderate));
    }

    #[tokio::test]
    async fn test_baseline_storage() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let _detector = RegressionDetector::new().with_baselines_dir(temp_dir.path().to_path_buf());

        // This test would require actual statistical data to work fully
        // but demonstrates the structure

        Ok(())
    }

    #[test]
    fn test_detector_builder_pattern() {
        let detector = RegressionDetector::new()
            .with_threshold(10.0)
            .with_history_retention_days(30);

        assert_eq!(detector.threshold_percent, 10.0);
        assert_eq!(detector.history_retention_days, 30);
    }

    #[test]
    fn test_regression_severity_minor() {
        let detector = RegressionDetector::new();
        let comparison = ComparisonResult {
            percent_change: 3.0,
            absolute_change: 30000.0,
            significance: SignificanceLevel::SignificantRegression,
            baseline_mean: 1000000.0,
            current_mean: 1030000.0,
        };

        let severity = detector.classify_regression_severity(&comparison);
        assert!(matches!(severity, RegressionSeverity::Minor));
    }

    #[test]
    fn test_regression_severity_moderate() {
        let detector = RegressionDetector::new();
        let comparison = ComparisonResult {
            percent_change: 7.0,
            absolute_change: 70000.0,
            significance: SignificanceLevel::SignificantRegression,
            baseline_mean: 1000000.0,
            current_mean: 1070000.0,
        };

        let severity = detector.classify_regression_severity(&comparison);
        assert!(matches!(severity, RegressionSeverity::Moderate));
    }

    #[test]
    fn test_regression_severity_major() {
        let detector = RegressionDetector::new();
        let comparison = ComparisonResult {
            percent_change: 20.0,
            absolute_change: 200000.0,
            significance: SignificanceLevel::SignificantRegression,
            baseline_mean: 1000000.0,
            current_mean: 1200000.0,
        };

        let severity = detector.classify_regression_severity(&comparison);
        assert!(matches!(severity, RegressionSeverity::Major));
    }

    #[test]
    fn test_regression_severity_critical() {
        let detector = RegressionDetector::new();
        let comparison = ComparisonResult {
            percent_change: 35.0,
            absolute_change: 350000.0,
            significance: SignificanceLevel::SignificantRegression,
            baseline_mean: 1000000.0,
            current_mean: 1350000.0,
        };

        let severity = detector.classify_regression_severity(&comparison);
        assert!(matches!(severity, RegressionSeverity::Critical));
    }

    #[test]
    fn test_no_regression_when_improvement() {
        let detector = RegressionDetector::new();
        let comparison = ComparisonResult {
            percent_change: -5.0,
            absolute_change: -50000.0,
            significance: SignificanceLevel::SignificantImprovement,
            baseline_mean: 1000000.0,
            current_mean: 950000.0,
        };

        let severity = detector.classify_regression_severity(&comparison);
        assert!(matches!(severity, RegressionSeverity::None));
    }

    #[test]
    fn test_no_regression_when_not_significant() {
        let detector = RegressionDetector::new();
        let comparison = ComparisonResult {
            percent_change: 1.0,
            absolute_change: 10000.0,
            significance: SignificanceLevel::NotSignificant,
            baseline_mean: 1000000.0,
            current_mean: 1010000.0,
        };

        let severity = detector.classify_regression_severity(&comparison);
        assert!(matches!(severity, RegressionSeverity::None));
    }

    #[test]
    fn test_performance_baseline_creation() {
        use crate::statistics::{SampleStatistics, OutlierAnalysis, Quartiles, DistributionMetrics, ConfidenceIntervals, Percentiles};

        let baseline = PerformanceBaseline {
            implementation: "rust".to_string(),
            example: "fibonacci".to_string(),
            statistics: StatisticalAnalysis {
                sample_stats: SampleStatistics {
                    count: 1000,
                    mean: 5000000.0,
                    median: 4900000.0,
                    std_dev: 500000.0,
                    std_error: 15811.0,
                    min: 4000000.0,
                    max: 6500000.0,
                },
                confidence_intervals: ConfidenceIntervals {
                    ci_95: (4968622.0, 5031378.0),
                    ci_99: (4959271.0, 5040729.0),
                },
                outliers: OutlierAnalysis {
                    outlier_count: 5,
                    outlier_percentage: 0.5,
                    outlier_values: vec![],
                    quartiles: Quartiles {
                        q1: 4500000.0,
                        q3: 5500000.0,
                        iqr: 1000000.0,
                        lower_fence: 3000000.0,
                        upper_fence: 7000000.0,
                    },
                },
                distribution: DistributionMetrics {
                    skewness: 0.1,
                    kurtosis: 0.0,
                    coefficient_of_variation: 0.1,
                    percentiles: Percentiles {
                        p5: 4200000.0,
                        p25: 4500000.0,
                        p50: 4900000.0,
                        p75: 5500000.0,
                        p95: 6000000.0,
                        p99: 6400000.0,
                    },
                },
            },
            timestamp: Utc::now(),
            configuration: BaselineConfiguration {
                iterations: 1000,
                warmup_iterations: 100,
                confidence_level: 0.95,
            },
            git_commit: Some("abc123".to_string()),
            environment_fingerprint: "test_env".to_string(),
        };

        assert_eq!(baseline.implementation, "rust");
        assert_eq!(baseline.example, "fibonacci");
        assert_eq!(baseline.configuration.iterations, 1000);
    }

    #[test]
    fn test_implementation_regression_structure() {
        let regression = ImplementationRegression {
            implementation: "python".to_string(),
            comparison: ComparisonResult {
                percent_change: 15.0,
                absolute_change: 750000.0,
                significance: SignificanceLevel::SignificantRegression,
                baseline_mean: 5000000.0,
                current_mean: 5750000.0,
            },
            severity: RegressionSeverity::Critical,
            quality_gate_violation: true,
            recommendations: vec!["Investigate performance drop".to_string()],
        };

        assert_eq!(regression.implementation, "python");
        assert_eq!(regression.comparison.percent_change, 15.0);
        assert!(regression.quality_gate_violation);
        assert!(matches!(regression.severity, RegressionSeverity::Critical));
    }

    #[test]
    fn test_regression_analysis_structure() {
        let analysis = RegressionAnalysis {
            regression_detected: true,
            comparisons: vec![],
            overall_status: RegressionStatus::Warning,
            recommendations: vec!["Monitor performance".to_string()],
            analyzed_at: Utc::now(),
        };

        assert!(analysis.regression_detected);
        assert!(matches!(analysis.overall_status, RegressionStatus::Warning));
        assert_eq!(analysis.recommendations.len(), 1);
    }

    #[test]
    fn test_baseline_configuration() {
        let config = BaselineConfiguration {
            iterations: 1000,
            warmup_iterations: 100,
            confidence_level: 0.95,
        };

        assert_eq!(config.iterations, 1000);
        assert_eq!(config.warmup_iterations, 100);
        assert_eq!(config.confidence_level, 0.95);
    }
}
