//! Convergence analysis for the GDK infinite monkey theorem implementation
//!
//! This module provides mathematical analysis of workflow convergence:
//! - Quality stability detection across commit history
//! - Thread health ratio calculation for overall system health
//! - Test pass consistency analysis for reliability metrics
//! - Build success rate tracking for compilation health
//! - Trend improvement analysis using linear regression
//! - Convergence prediction algorithms
//!
//! # Mathematical Foundation
//!
//! The convergence algorithm uses weighted factors:
//! - **Quality Stability (30%)**: Variance analysis of health scores
//! - **Thread Health Ratio (25%)**: Percentage of green/light-green threads
//! - **Test Pass Consistency (20%)**: Reliability of test execution
//! - **Build Success Rate (15%)**: Compilation and build health
//! - **Trend Improvement (10%)**: Linear regression slope of quality
//!
//! # Example Usage
//!
//! ```rust,no_run
//! use gdk::convergence::ConvergenceAnalyzer;
//!
//! let analyzer = ConvergenceAnalyzer::new();
//! let result = analyzer.analyze_convergence(&commit_history)?;
//!
//! if result.is_converged {
//!     println!("Converged with confidence: {:.3}", result.confidence_score);
//! } else {
//!     for recommendation in result.recommendations {
//!         println!("Recommendation: {}", recommendation);
//!     }
//! }
//! ```

use crate::{CommitNode, ThreadColor, GdkResult, GdkError};
use serde::{Deserialize, Serialize};

/// Mathematical analyzer for detecting workflow convergence
///
/// Uses statistical analysis to determine when an agent's workflow
/// has achieved stable, high-quality output. Configurable thresholds
/// allow tuning for different project requirements.
///
/// # Configuration Parameters
///
/// - **convergence_threshold**: Minimum weighted score for convergence (0.0-1.0)
/// - **stability_window**: Number of recent commits to analyze for stability
/// - **quality_trend_window**: Number of commits for trend analysis
/// - **min_green_threads_ratio**: Required percentage of healthy threads
/// - **variance_threshold**: Maximum allowed variance in quality scores
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConvergenceAnalyzer {
    /// Minimum weighted confidence score required for convergence (default: 0.8)
    pub convergence_threshold: f64,
    /// Number of recent commits to analyze for stability (default: 5)
    pub stability_window: usize,
    /// Number of commits to analyze for quality trends (default: 10)
    pub quality_trend_window: usize,
    /// Minimum ratio of healthy (green/light-green) threads (default: 0.7)
    pub min_green_threads_ratio: f64,
    /// Maximum allowed variance in quality scores for stability (default: 0.02)
    pub variance_threshold: f64,
}

impl Default for ConvergenceAnalyzer {
    fn default() -> Self {
        Self {
            convergence_threshold: 0.8,
            stability_window: 5,
            quality_trend_window: 10,
            min_green_threads_ratio: 0.7,
            variance_threshold: 0.02,
        }
    }
}

/// Result of convergence analysis with detailed metrics and recommendations
///
/// Provides comprehensive assessment of workflow state including:
/// - Binary convergence decision based on weighted factors
/// - Confidence score indicating strength of convergence
/// - Detailed breakdown of contributing factors
/// - Actionable recommendations for improvement
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConvergenceResult {
    /// Whether the workflow has achieved convergence
    pub is_converged: bool,
    /// Weighted confidence score (0.0-1.0) indicating convergence strength
    pub confidence_score: f64,
    /// Detailed breakdown of individual convergence factors
    pub convergence_factors: ConvergenceFactors,
    /// Human-readable recommendations for improving convergence
    pub recommendations: Vec<String>,
}

/// Individual factors contributing to overall convergence assessment
///
/// Each factor is normalized to 0.0-1.0 range where:
/// - 0.0 indicates poor performance in this dimension
/// - 1.0 indicates excellent performance in this dimension
///
/// Factors are weighted differently in the final convergence calculation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConvergenceFactors {
    /// Stability of quality scores over recent commits (0.0-1.0)
    pub quality_stability: f64,
    /// Ratio of healthy threads to total threads (0.0-1.0)
    pub thread_health_ratio: f64,
    /// Consistency of test pass rates across recent commits (0.0-1.0)
    pub test_pass_consistency: f64,
    /// Success rate of builds/compilations (0.0-1.0)
    pub build_success_rate: f64,
    /// Linear trend improvement in quality scores (0.0-1.0)
    pub trend_improvement: f64,
}

impl ConvergenceAnalyzer {
    /// Create a new convergence analyzer with default configuration
    ///
    /// Default settings are suitable for most Rust projects:
    /// - convergence_threshold: 0.8 (80% confidence required)
    /// - stability_window: 5 commits
    /// - quality_trend_window: 10 commits
    /// - min_green_threads_ratio: 0.7 (70% healthy threads)
    /// - variance_threshold: 0.02 (2% variance allowed)
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a convergence analyzer with custom configuration
    ///
    /// # Arguments
    ///
    /// * `convergence_threshold` - Minimum confidence score for convergence (0.0-1.0)
    /// * `stability_window` - Number of recent commits for stability analysis
    /// * `quality_trend_window` - Number of commits for trend analysis
    /// * `min_green_threads_ratio` - Required ratio of healthy threads (0.0-1.0)
    /// * `variance_threshold` - Maximum quality score variance for stability
    ///
    /// # Example
    ///
    /// ```rust
    /// use gdk::convergence::ConvergenceAnalyzer;
    ///
    /// // Stricter convergence requirements
    /// let analyzer = ConvergenceAnalyzer::with_config(
    ///     0.9,  // 90% confidence required
    ///     3,    // Only look at last 3 commits
    ///     5,    // Shorter trend window
    ///     0.8,  // 80% healthy threads required
    ///     0.01  // Very low variance allowed
    /// );
    /// ```
    pub fn with_config(
        convergence_threshold: f64,
        stability_window: usize,
        quality_trend_window: usize,
        min_green_threads_ratio: f64,
        variance_threshold: f64,
    ) -> Self {
        Self {
            convergence_threshold,
            stability_window,
            quality_trend_window,
            min_green_threads_ratio,
            variance_threshold,
        }
    }

    /// Analyze commit history to determine convergence status
    ///
    /// Performs comprehensive mathematical analysis of the workflow:
    /// 1. Calculate individual convergence factors
    /// 2. Apply weighted scoring to determine overall confidence
    /// 3. Generate actionable recommendations
    /// 4. Make binary convergence decision
    ///
    /// # Arguments
    ///
    /// * `commit_history` - Chronological list of commits with quality metrics
    ///
    /// # Returns
    ///
    /// Detailed convergence analysis with recommendations
    ///
    /// # Mathematical Details
    ///
    /// The confidence score is calculated as:
    /// ```text
    /// confidence = 0.30 * quality_stability +
    ///              0.25 * thread_health_ratio +
    ///              0.20 * test_pass_consistency +
    ///              0.15 * build_success_rate +
    ///              0.10 * trend_improvement
    /// ```
    pub fn analyze_convergence(&self, commit_history: &[CommitNode]) -> GdkResult<ConvergenceResult> {
        if commit_history.is_empty() {
            return Ok(ConvergenceResult {
                is_converged: false,
                confidence_score: 0.0,
                convergence_factors: ConvergenceFactors {
                    quality_stability: 0.0,
                    thread_health_ratio: 0.0,
                    test_pass_consistency: 0.0,
                    build_success_rate: 0.0,
                    trend_improvement: 0.0,
                },
                recommendations: vec!["No commit history available".to_string()],
            });
        }

        let factors = self.calculate_convergence_factors(commit_history)?;
        let (is_converged, confidence_score) = self.determine_convergence(&factors);
        let recommendations = self.generate_recommendations(&factors, commit_history);

        Ok(ConvergenceResult {
            is_converged,
            confidence_score,
            convergence_factors: factors,
            recommendations,
        })
    }

    fn calculate_convergence_factors(
        &self,
        commit_history: &[CommitNode],
    ) -> GdkResult<ConvergenceFactors> {
        let quality_stability = self.calculate_quality_stability(commit_history)?;
        let thread_health_ratio = self.calculate_thread_health_ratio(commit_history)?;
        let test_pass_consistency = self.calculate_test_pass_consistency(commit_history)?;
        let build_success_rate = self.calculate_build_success_rate(commit_history)?;
        let trend_improvement = self.calculate_trend_improvement(commit_history)?;

        Ok(ConvergenceFactors {
            quality_stability,
            thread_health_ratio,
            test_pass_consistency,
            build_success_rate,
            trend_improvement,
        })
    }

    fn calculate_quality_stability(&self, commit_history: &[CommitNode]) -> GdkResult<f64> {
        let recent_commits: Vec<&CommitNode> = commit_history
            .iter()
            .rev()
            .take(self.stability_window)
            .collect();

        if recent_commits.len() < 3 {
            return Ok(0.0);
        }

        let quality_scores: Vec<f64> = recent_commits
            .iter()
            .map(|commit| commit.health_score)
            .collect();

        let mean = quality_scores.iter().sum::<f64>() / quality_scores.len() as f64;
        let variance = quality_scores
            .iter()
            .map(|&score| (score - mean).powi(2))
            .sum::<f64>()
            / quality_scores.len() as f64;

        let stability = if variance <= self.variance_threshold && mean >= self.convergence_threshold
        {
            1.0 - (variance / self.variance_threshold).min(1.0)
        } else {
            0.0
        };

        Ok(stability)
    }

    fn calculate_thread_health_ratio(&self, commit_history: &[CommitNode]) -> GdkResult<f64> {
        let latest_commit = commit_history
            .last()
            .ok_or_else(|| GdkError::validation_error(
                "no_commits",
                "No commits available for thread health analysis".to_string(),
                "Commit history is empty".to_string(),
            ))?;

        let total_threads = latest_commit.file_threads.len();
        if total_threads == 0 {
            return Ok(0.0);
        }

        let healthy_threads = latest_commit
            .file_threads
            .values()
            .filter(|thread| {
                matches!(
                    thread.color_status,
                    ThreadColor::Green | ThreadColor::LightGreen
                )
            })
            .count();

        let ratio = healthy_threads as f64 / total_threads as f64;

        if ratio >= self.min_green_threads_ratio {
            Ok(ratio)
        } else {
            Ok(0.0)
        }
    }

    fn calculate_test_pass_consistency(&self, commit_history: &[CommitNode]) -> GdkResult<f64> {
        let recent_commits: Vec<&CommitNode> = commit_history
            .iter()
            .rev()
            .take(self.stability_window)
            .collect();

        if recent_commits.is_empty() {
            return Ok(0.0);
        }

        let pass_rates: Vec<f64> = recent_commits
            .iter()
            .map(|commit| commit.convergence_metrics.test_pass_rate)
            .collect();

        let high_pass_rate_count = pass_rates
            .iter()
            .filter(|&&rate| rate >= self.convergence_threshold)
            .count();

        Ok(high_pass_rate_count as f64 / pass_rates.len() as f64)
    }

    fn calculate_build_success_rate(&self, commit_history: &[CommitNode]) -> GdkResult<f64> {
        let recent_commits: Vec<&CommitNode> = commit_history
            .iter()
            .rev()
            .take(self.stability_window)
            .collect();

        if recent_commits.is_empty() {
            return Ok(0.0);
        }

        let successful_builds: usize = recent_commits
            .iter()
            .map(|commit| commit.convergence_metrics.successful_builds)
            .sum::<u32>() as usize;

        let total_attempts: usize = recent_commits
            .iter()
            .map(|commit| commit.convergence_metrics.attempts)
            .sum::<u32>() as usize;

        if total_attempts == 0 {
            Ok(0.0)
        } else {
            Ok(successful_builds as f64 / total_attempts as f64)
        }
    }

    fn calculate_trend_improvement(&self, commit_history: &[CommitNode]) -> GdkResult<f64> {
        let trend_commits: Vec<&CommitNode> = commit_history
            .iter()
            .rev()
            .take(self.quality_trend_window)
            .collect();

        if trend_commits.len() < 3 {
            return Ok(0.0);
        }

        let quality_scores: Vec<f64> = trend_commits
            .iter()
            .rev()
            .map(|commit| commit.health_score)
            .collect();

        // Calculate linear regression slope to determine trend
        let n = quality_scores.len() as f64;
        let sum_x: f64 = (0..quality_scores.len()).map(|i| i as f64).sum();
        let sum_y: f64 = quality_scores.iter().sum();
        let sum_xy: f64 = quality_scores
            .iter()
            .enumerate()
            .map(|(i, &y)| i as f64 * y)
            .sum();
        let sum_x2: f64 = (0..quality_scores.len()).map(|i| (i as f64).powi(2)).sum();

        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x.powi(2));

        // Normalize slope to 0-1 range (positive slope indicates improvement)
        if slope > 0.0 {
            Ok(slope.min(1.0))
        } else {
            Ok(0.0)
        }
    }

    fn determine_convergence(&self, factors: &ConvergenceFactors) -> (bool, f64) {
        let weights = [0.3, 0.25, 0.2, 0.15, 0.1]; // Weights for each factor
        let factor_values = [
            factors.quality_stability,
            factors.thread_health_ratio,
            factors.test_pass_consistency,
            factors.build_success_rate,
            factors.trend_improvement,
        ];

        let confidence_score = factor_values
            .iter()
            .zip(weights.iter())
            .map(|(&value, &weight)| value * weight)
            .sum::<f64>();

        let is_converged = confidence_score >= self.convergence_threshold
            && factors.quality_stability > 0.8
            && factors.thread_health_ratio >= self.min_green_threads_ratio;

        (is_converged, confidence_score)
    }

    fn generate_recommendations(
        &self,
        factors: &ConvergenceFactors,
        commit_history: &[CommitNode],
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        if factors.quality_stability < 0.5 {
            recommendations
                .push("Quality scores are unstable. Focus on consistent improvements.".to_string());
        }

        if factors.thread_health_ratio < self.min_green_threads_ratio {
            recommendations.push(format!(
                "Only {:.1}% of threads are healthy. Target: {:.1}%. Focus on improving failing files.",
                factors.thread_health_ratio * 100.0,
                self.min_green_threads_ratio * 100.0
            ));
        }

        if factors.test_pass_consistency < 0.7 {
            recommendations
                .push("Test pass rates are inconsistent. Improve test reliability.".to_string());
        }

        if factors.build_success_rate < 0.8 {
            recommendations
                .push("Build success rate is low. Fix compilation and build issues.".to_string());
        }

        if factors.trend_improvement < 0.1 {
            recommendations
                .push("Quality trend is not improving. Consider different strategies.".to_string());
        }

        if let Some(latest_commit) = commit_history.last() {
            let red_threads: Vec<_> = latest_commit
                .file_threads
                .iter()
                .filter(|(_, thread)| thread.color_status == ThreadColor::Red)
                .collect();

            if !red_threads.is_empty() {
                recommendations.push(format!(
                    "Critical: {} files have red status: {}",
                    red_threads.len(),
                    red_threads
                        .iter()
                        .take(3)
                        .map(|(path, _)| path.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
        }

        if recommendations.is_empty() {
            recommendations.push(
                "System appears to be converging well. Continue current approach.".to_string(),
            );
        }

        recommendations
    }

    pub fn predict_convergence_time(&self, commit_history: &[CommitNode]) -> GdkResult<Option<u32>> {
        if commit_history.len() < 3 {
            return Ok(None);
        }

        let recent_scores: Vec<f64> = commit_history
            .iter()
            .rev()
            .take(10)
            .map(|commit| commit.health_score)
            .collect();

        if recent_scores.len() < 3 {
            return Ok(None);
        }

        // Calculate improvement rate
        let current_score = recent_scores[0];
        let oldest_score = recent_scores[recent_scores.len() - 1];
        let improvement_rate = (current_score - oldest_score) / recent_scores.len() as f64;

        if improvement_rate <= 0.0 {
            return Ok(None); // No improvement trend
        }

        let remaining_improvement = self.convergence_threshold - current_score;
        if remaining_improvement <= 0.0 {
            return Ok(Some(0)); // Already converged
        }

        let predicted_iterations = (remaining_improvement / improvement_rate).ceil() as u32;

        // Cap prediction at reasonable upper bound
        if predicted_iterations > 100 {
            Ok(None)
        } else {
            Ok(Some(predicted_iterations))
        }
    }
}
