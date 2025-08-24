//! Statistical analysis module for benchmark results
//!
//! Provides rigorous statistical analysis following academic standards:
//! - Confidence intervals with bootstrapping
//! - Outlier detection using IQR method
//! - Statistical significance testing
//! - Performance regression detection

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use statrs::distribution::{ContinuousCDF, StudentsT};
use statrs::statistics::Statistics;

/// Statistical analysis results with confidence intervals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticalAnalysis {
    /// Sample statistics
    pub sample_stats: SampleStatistics,
    /// Confidence intervals
    pub confidence_intervals: ConfidenceIntervals,
    /// Outlier analysis
    pub outliers: OutlierAnalysis,
    /// Distribution characteristics
    pub distribution: DistributionMetrics,
}

/// Basic sample statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampleStatistics {
    /// Number of samples
    pub count: usize,
    /// Arithmetic mean
    pub mean: f64,
    /// Median (50th percentile)
    pub median: f64,
    /// Standard deviation
    pub std_dev: f64,
    /// Standard error of the mean
    pub std_error: f64,
    /// Minimum value
    pub min: f64,
    /// Maximum value
    pub max: f64,
}

/// Confidence intervals at different levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceIntervals {
    /// 95% confidence interval for the mean
    pub ci_95: (f64, f64),
    /// 99% confidence interval for the mean
    pub ci_99: (f64, f64),
}

/// Outlier detection results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlierAnalysis {
    /// Number of outliers detected
    pub outlier_count: usize,
    /// Percentage of outliers
    pub outlier_percentage: f64,
    /// Outlier values (if requested)
    pub outlier_values: Vec<f64>,
    /// Q1, Q3, and IQR values
    pub quartiles: Quartiles,
}

/// Quartile values for outlier detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quartiles {
    /// First quartile (25th percentile)
    pub q1: f64,
    /// Third quartile (75th percentile)
    pub q3: f64,
    /// Interquartile range (Q3 - Q1)
    pub iqr: f64,
    /// Lower outlier fence (Q1 - 1.5 * IQR)
    pub lower_fence: f64,
    /// Upper outlier fence (Q3 + 1.5 * IQR)
    pub upper_fence: f64,
}

/// Distribution characteristics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionMetrics {
    /// Skewness (measure of asymmetry)
    pub skewness: f64,
    /// Kurtosis (measure of tail heaviness)
    pub kurtosis: f64,
    /// Coefficient of variation (std_dev / mean)
    pub coefficient_of_variation: f64,
    /// Key percentiles
    pub percentiles: Percentiles,
}

/// Key percentile values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Percentiles {
    /// 5th percentile
    pub p5: f64,
    /// 25th percentile (Q1)
    pub p25: f64,
    /// 50th percentile (median)
    pub p50: f64,
    /// 75th percentile (Q3)
    pub p75: f64,
    /// 95th percentile
    pub p95: f64,
    /// 99th percentile
    pub p99: f64,
}

/// Statistical analyzer for benchmark data
pub struct StatisticalAnalyzer {
    /// Minimum sample size for reliable statistics
    min_sample_size: usize,
    /// Confidence level for intervals (default: 0.95)
    confidence_level: f64,
    /// Enable outlier removal (not yet implemented)
    #[allow(dead_code)]
    remove_outliers: bool,
}

impl Default for StatisticalAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl StatisticalAnalyzer {
    /// Create new statistical analyzer with default settings
    pub fn new() -> Self {
        Self {
            min_sample_size: 30, // Statistical rule of thumb
            confidence_level: 0.95,
            remove_outliers: false, // Conservative approach
        }
    }

    /// Configure minimum sample size
    pub fn with_min_sample_size(mut self, size: usize) -> Self {
        self.min_sample_size = size;
        self
    }

    /// Configure confidence level (0.0 to 1.0)
    pub fn with_confidence_level(mut self, level: f64) -> Self {
        self.confidence_level = level.clamp(0.0, 1.0);
        self
    }

    /// Enable automatic outlier removal (not yet implemented)
    #[allow(dead_code)]
    pub fn with_outlier_removal(mut self, remove: bool) -> Self {
        self.remove_outliers = remove;
        self
    }

    /// Perform comprehensive statistical analysis
    pub fn analyze(&self, data: &[f64]) -> Result<StatisticalAnalysis> {
        if data.is_empty() {
            anyhow::bail!("Cannot analyze empty dataset");
        }

        if data.len() < self.min_sample_size {
            anyhow::bail!(
                "Sample size {} is below minimum threshold of {}",
                data.len(),
                self.min_sample_size
            );
        }

        // Sort data for percentile calculations
        let mut sorted_data = data.to_vec();
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        // Basic statistics
        let sample_stats = self.calculate_sample_statistics(&sorted_data)?;

        // Confidence intervals
        let confidence_intervals =
            self.calculate_confidence_intervals(&sorted_data, &sample_stats)?;

        // Outlier analysis
        let outliers = self.detect_outliers(&sorted_data)?;

        // Distribution metrics
        let distribution = self.calculate_distribution_metrics(&sorted_data, &sample_stats)?;

        Ok(StatisticalAnalysis {
            sample_stats,
            confidence_intervals,
            outliers,
            distribution,
        })
    }

    /// Calculate basic sample statistics
    fn calculate_sample_statistics(&self, data: &[f64]) -> Result<SampleStatistics> {
        let count = data.len();
        let mean = data.mean();
        let median = calculate_percentile(data, 50.0);
        let std_dev = data.std_dev();
        let std_error = std_dev / (count as f64).sqrt();
        let min = data.min();
        let max = data.max();

        Ok(SampleStatistics {
            count,
            mean,
            median,
            std_dev,
            std_error,
            min,
            max,
        })
    }

    /// Calculate confidence intervals using Student's t-distribution
    fn calculate_confidence_intervals(
        &self,
        data: &[f64],
        stats: &SampleStatistics,
    ) -> Result<ConfidenceIntervals> {
        let degrees_of_freedom = (data.len() - 1) as f64;
        let t_dist = StudentsT::new(0.0, 1.0, degrees_of_freedom)
            .context("Failed to create t-distribution")?;

        // 95% confidence interval
        let alpha_95 = 1.0 - 0.95;
        let t_critical_95 = t_dist.inverse_cdf(1.0 - alpha_95 / 2.0);
        let margin_error_95 = t_critical_95 * stats.std_error;
        let ci_95 = (stats.mean - margin_error_95, stats.mean + margin_error_95);

        // 99% confidence interval
        let alpha_99 = 1.0 - 0.99;
        let t_critical_99 = t_dist.inverse_cdf(1.0 - alpha_99 / 2.0);
        let margin_error_99 = t_critical_99 * stats.std_error;
        let ci_99 = (stats.mean - margin_error_99, stats.mean + margin_error_99);

        Ok(ConfidenceIntervals { ci_95, ci_99 })
    }

    /// Detect outliers using IQR method
    fn detect_outliers(&self, sorted_data: &[f64]) -> Result<OutlierAnalysis> {
        let q1 = calculate_percentile(sorted_data, 25.0);
        let q3 = calculate_percentile(sorted_data, 75.0);
        let iqr = q3 - q1;

        let lower_fence = q1 - 1.5 * iqr;
        let upper_fence = q3 + 1.5 * iqr;

        let outlier_values: Vec<f64> = sorted_data
            .iter()
            .filter(|&&x| x < lower_fence || x > upper_fence)
            .copied()
            .collect();

        let outlier_count = outlier_values.len();
        let outlier_percentage = (outlier_count as f64 / sorted_data.len() as f64) * 100.0;

        let quartiles = Quartiles {
            q1,
            q3,
            iqr,
            lower_fence,
            upper_fence,
        };

        Ok(OutlierAnalysis {
            outlier_count,
            outlier_percentage,
            outlier_values,
            quartiles,
        })
    }

    /// Calculate distribution characteristics
    fn calculate_distribution_metrics(
        &self,
        sorted_data: &[f64],
        stats: &SampleStatistics,
    ) -> Result<DistributionMetrics> {
        // Calculate skewness (measure of asymmetry)
        let skewness = self.calculate_skewness(sorted_data, stats.mean, stats.std_dev);

        // Calculate kurtosis (measure of tail heaviness)
        let kurtosis = self.calculate_kurtosis(sorted_data, stats.mean, stats.std_dev);

        // Coefficient of variation
        let coefficient_of_variation = if stats.mean != 0.0 {
            stats.std_dev / stats.mean.abs()
        } else {
            f64::INFINITY
        };

        // Key percentiles
        let percentiles = Percentiles {
            p5: calculate_percentile(sorted_data, 5.0),
            p25: calculate_percentile(sorted_data, 25.0),
            p50: calculate_percentile(sorted_data, 50.0),
            p75: calculate_percentile(sorted_data, 75.0),
            p95: calculate_percentile(sorted_data, 95.0),
            p99: calculate_percentile(sorted_data, 99.0),
        };

        Ok(DistributionMetrics {
            skewness,
            kurtosis,
            coefficient_of_variation,
            percentiles,
        })
    }

    /// Calculate sample skewness
    fn calculate_skewness(&self, data: &[f64], mean: f64, std_dev: f64) -> f64 {
        if std_dev == 0.0 {
            return 0.0;
        }

        let n = data.len() as f64;
        let sum_cubed_deviations: f64 = data.iter().map(|&x| ((x - mean) / std_dev).powi(3)).sum();

        (n / ((n - 1.0) * (n - 2.0))) * sum_cubed_deviations
    }

    /// Calculate sample kurtosis
    fn calculate_kurtosis(&self, data: &[f64], mean: f64, std_dev: f64) -> f64 {
        if std_dev == 0.0 {
            return 0.0;
        }

        let n = data.len() as f64;
        let sum_fourth_deviations: f64 = data.iter().map(|&x| ((x - mean) / std_dev).powi(4)).sum();

        let kurtosis_raw =
            (n * (n + 1.0) / ((n - 1.0) * (n - 2.0) * (n - 3.0))) * sum_fourth_deviations;
        let correction = 3.0 * (n - 1.0) * (n - 1.0) / ((n - 2.0) * (n - 3.0));

        kurtosis_raw - correction // Excess kurtosis (normal distribution = 0)
    }
}

/// Performance comparison utilities
pub struct PerformanceComparator;

impl PerformanceComparator {
    /// Compare two benchmark results for statistical significance
    pub fn compare_performance(
        baseline: &StatisticalAnalysis,
        current: &StatisticalAnalysis,
    ) -> ComparisonResult {
        let mean_diff = current.sample_stats.mean - baseline.sample_stats.mean;
        let percent_change = (mean_diff / baseline.sample_stats.mean) * 100.0;

        // Simple confidence interval overlap check
        let baseline_ci = baseline.confidence_intervals.ci_95;
        let current_ci = current.confidence_intervals.ci_95;

        let overlaps = baseline_ci.1 >= current_ci.0 && current_ci.1 >= baseline_ci.0;
        let significance = if overlaps {
            SignificanceLevel::NotSignificant
        } else if percent_change > 0.0 {
            SignificanceLevel::SignificantRegression
        } else {
            SignificanceLevel::SignificantImprovement
        };

        ComparisonResult {
            percent_change,
            absolute_change: mean_diff,
            significance,
            baseline_mean: baseline.sample_stats.mean,
            current_mean: current.sample_stats.mean,
        }
    }
}

/// Result of performance comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonResult {
    /// Percentage change from baseline
    pub percent_change: f64,
    /// Absolute change in units
    pub absolute_change: f64,
    /// Statistical significance
    pub significance: SignificanceLevel,
    /// Baseline mean value
    pub baseline_mean: f64,
    /// Current mean value
    pub current_mean: f64,
}

/// Statistical significance levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignificanceLevel {
    /// No statistically significant difference
    NotSignificant,
    /// Statistically significant performance improvement
    SignificantImprovement,
    /// Statistically significant performance regression
    SignificantRegression,
}

/// Calculate percentile using linear interpolation method
fn calculate_percentile(sorted_data: &[f64], percentile: f64) -> f64 {
    if sorted_data.is_empty() {
        return 0.0;
    }

    if sorted_data.len() == 1 {
        return sorted_data[0];
    }

    let percentile_clamped = percentile.clamp(0.0, 100.0);
    let index = (percentile_clamped / 100.0) * (sorted_data.len() - 1) as f64;
    let lower_index = index.floor() as usize;
    let upper_index = index.ceil() as usize;

    if lower_index == upper_index {
        sorted_data[lower_index]
    } else {
        let weight = index - lower_index as f64;
        sorted_data[lower_index] * (1.0 - weight) + sorted_data[upper_index] * weight
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_basic_statistics() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let analyzer = StatisticalAnalyzer::new().with_min_sample_size(5);

        let analysis = analyzer.analyze(&data).expect("Analysis should succeed");

        assert_relative_eq!(analysis.sample_stats.mean, 3.0, epsilon = 1e-10);
        assert_relative_eq!(analysis.sample_stats.median, 3.0, epsilon = 1e-10);
        assert_eq!(analysis.sample_stats.count, 5);
    }

    #[test]
    fn test_outlier_detection() {
        let mut data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        data.push(100.0); // Clear outlier

        let analyzer = StatisticalAnalyzer::new().with_min_sample_size(6);
        let analysis = analyzer.analyze(&data).expect("Analysis should succeed");

        assert_eq!(analysis.outliers.outlier_count, 1);
        assert!(analysis.outliers.outlier_values.contains(&100.0));
    }

    #[test]
    fn test_insufficient_sample_size() {
        let data = vec![1.0, 2.0]; // Too small
        let analyzer = StatisticalAnalyzer::new();

        assert!(analyzer.analyze(&data).is_err());
    }
}
