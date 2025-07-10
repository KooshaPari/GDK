use crate::{CommitNode, ThreadColor};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceAnalyzer {
    pub convergence_threshold: f64,
    pub stability_window: usize,
    pub quality_trend_window: usize,
    pub min_green_threads_ratio: f64,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceResult {
    pub is_converged: bool,
    pub confidence_score: f64,
    pub convergence_factors: ConvergenceFactors,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceFactors {
    pub quality_stability: f64,
    pub thread_health_ratio: f64,
    pub test_pass_consistency: f64,
    pub build_success_rate: f64,
    pub trend_improvement: f64,
}

impl ConvergenceAnalyzer {
    pub fn new() -> Self {
        Self::default()
    }

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

    pub fn analyze_convergence(&self, commit_history: &[CommitNode]) -> Result<ConvergenceResult> {
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
    ) -> Result<ConvergenceFactors> {
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

    fn calculate_quality_stability(&self, commit_history: &[CommitNode]) -> Result<f64> {
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

    fn calculate_thread_health_ratio(&self, commit_history: &[CommitNode]) -> Result<f64> {
        let latest_commit = commit_history
            .last()
            .ok_or_else(|| anyhow!("No commits available"))?;

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

    fn calculate_test_pass_consistency(&self, commit_history: &[CommitNode]) -> Result<f64> {
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

    fn calculate_build_success_rate(&self, commit_history: &[CommitNode]) -> Result<f64> {
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

    fn calculate_trend_improvement(&self, commit_history: &[CommitNode]) -> Result<f64> {
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

    pub fn predict_convergence_time(&self, commit_history: &[CommitNode]) -> Result<Option<u32>> {
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
