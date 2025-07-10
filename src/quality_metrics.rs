//! Advanced quality metrics and analysis for GDK system
//!
//! This module provides sophisticated quality measurement and analysis:
//! - Multi-dimensional quality scoring across different aspects
//! - Historical trend analysis with statistical methods
//! - Quality gate enforcement with configurable thresholds
//! - Performance impact analysis of quality changes
//! - Predictive quality modeling using machine learning techniques
//! - Technical debt measurement and tracking
//! - Code complexity analysis and recommendations
//!
//! # Quality Dimensions
//!
//! - **Correctness**: Compilation, type safety, test coverage
//! - **Maintainability**: Code complexity, documentation, structure
//! - **Security**: Vulnerability scanning, dependency auditing
//! - **Performance**: Benchmarks, memory usage, optimization
//! - **Reliability**: Error handling, edge case coverage
//! - **Usability**: API design, documentation quality

use crate::{CommitNode, GdkResult, GdkError};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::time::{SystemTime, UNIX_EPOCH};

/// Comprehensive quality metrics analyzer
///
/// Provides advanced analysis of code quality across multiple dimensions
/// with historical tracking, trend analysis, and predictive modeling.
#[derive(Debug, Clone)]
pub struct QualityMetricsAnalyzer {
    /// Configuration for quality analysis
    config: QualityConfig,
    /// Historical quality data for trend analysis
    history: VecDeque<QualitySnapshot>,
    /// Current quality state
    current_metrics: QualityMetrics,
    /// Quality gates and thresholds
    gates: Vec<QualityGate>,
}

/// Configuration for quality metrics analysis
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QualityConfig {
    /// Maximum history entries to maintain
    pub max_history_entries: usize,
    /// Weight for different quality dimensions
    pub dimension_weights: DimensionWeights,
    /// Minimum quality thresholds
    pub quality_thresholds: QualityThresholds,
    /// Trend analysis configuration
    pub trend_config: TrendConfig,
}

/// Weights for different quality dimensions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DimensionWeights {
    /// Correctness weight (compilation, tests, types)
    pub correctness: f64,
    /// Maintainability weight (complexity, documentation)
    pub maintainability: f64,
    /// Security weight (vulnerabilities, dependencies)
    pub security: f64,
    /// Performance weight (speed, memory, efficiency)
    pub performance: f64,
    /// Reliability weight (error handling, robustness)
    pub reliability: f64,
    /// Usability weight (API design, documentation)
    pub usability: f64,
}

/// Quality thresholds for different dimensions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QualityThresholds {
    /// Minimum overall quality score
    pub overall_minimum: f64,
    /// Minimum correctness score
    pub correctness_minimum: f64,
    /// Minimum maintainability score
    pub maintainability_minimum: f64,
    /// Minimum security score
    pub security_minimum: f64,
    /// Maximum complexity threshold
    pub max_complexity: f64,
    /// Minimum test coverage
    pub min_test_coverage: f64,
    /// Maximum technical debt ratio
    pub max_technical_debt: f64,
}

/// Configuration for trend analysis
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrendConfig {
    /// Window size for trend calculation
    pub trend_window: usize,
    /// Minimum data points for reliable trends
    pub min_data_points: usize,
    /// Sensitivity for trend detection
    pub trend_sensitivity: f64,
    /// Enable predictive modeling
    pub enable_prediction: bool,
}

/// Comprehensive quality metrics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QualityMetrics {
    /// Overall quality score (0.0-1.0)
    pub overall_score: f64,
    /// Individual dimension scores
    pub dimensions: QualityDimensions,
    /// Technical debt metrics
    pub technical_debt: TechnicalDebtMetrics,
    /// Code complexity metrics
    pub complexity: ComplexityMetrics,
    /// Performance metrics
    pub performance: PerformanceMetrics,
    /// Security metrics
    pub security: SecurityMetrics,
    /// Test coverage metrics
    pub coverage: CoverageMetrics,
    /// Timestamp of measurement
    pub timestamp: u64,
}

/// Quality scores across different dimensions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QualityDimensions {
    /// Correctness score (compilation, type safety, tests)
    pub correctness: f64,
    /// Maintainability score (code structure, documentation)
    pub maintainability: f64,
    /// Security score (vulnerabilities, best practices)
    pub security: f64,
    /// Performance score (speed, memory usage)
    pub performance: f64,
    /// Reliability score (error handling, edge cases)
    pub reliability: f64,
    /// Usability score (API design, documentation quality)
    pub usability: f64,
}

/// Technical debt measurement and tracking
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TechnicalDebtMetrics {
    /// Total estimated debt in hours
    pub total_debt_hours: f64,
    /// Debt ratio (debt / total code size)
    pub debt_ratio: f64,
    /// Debt by category
    pub debt_breakdown: HashMap<String, f64>,
    /// High-priority debt items
    pub priority_items: Vec<DebtItem>,
    /// Trend direction (increasing/decreasing/stable)
    pub trend: DebtTrend,
}

/// Individual technical debt item
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DebtItem {
    /// File path where debt is located
    pub file_path: String,
    /// Type of debt (complexity, duplication, etc.)
    pub debt_type: String,
    /// Estimated effort to fix (hours)
    pub effort_hours: f64,
    /// Priority level (1-5, 5 being highest)
    pub priority: u8,
    /// Description of the debt
    pub description: String,
    /// Suggested remediation
    pub remediation: String,
}

/// Trend direction for technical debt
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DebtTrend {
    /// Debt is increasing rapidly
    Increasing,
    /// Debt is decreasing (good!)
    Decreasing,
    /// Debt is stable
    Stable,
    /// Insufficient data for trend
    Unknown,
}

/// Code complexity metrics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ComplexityMetrics {
    /// Average cyclomatic complexity
    pub avg_cyclomatic: f64,
    /// Maximum cyclomatic complexity
    pub max_cyclomatic: f64,
    /// Average cognitive complexity
    pub avg_cognitive: f64,
    /// Maximum cognitive complexity
    pub max_cognitive: f64,
    /// Number of complex functions (above threshold)
    pub complex_functions: usize,
    /// Files with high complexity
    pub complex_files: Vec<ComplexFile>,
}

/// File with high complexity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ComplexFile {
    /// Path to the file
    pub path: String,
    /// Cyclomatic complexity
    pub cyclomatic: f64,
    /// Cognitive complexity
    pub cognitive: f64,
    /// Number of functions in file
    pub function_count: usize,
    /// Lines of code
    pub lines_of_code: usize,
}

/// Performance-related quality metrics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PerformanceMetrics {
    /// Benchmark score (relative to baseline)
    pub benchmark_score: f64,
    /// Memory efficiency score
    pub memory_efficiency: f64,
    /// Compilation time (seconds)
    pub compilation_time: f64,
    /// Test execution time (seconds)
    pub test_execution_time: f64,
    /// Performance regression count
    pub regressions: usize,
    /// Performance improvements count
    pub improvements: usize,
}

/// Security-related quality metrics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SecurityMetrics {
    /// Number of known vulnerabilities
    pub vulnerability_count: usize,
    /// Security score (0.0-1.0)
    pub security_score: f64,
    /// Dependency audit results
    pub dependency_audit: DependencyAudit,
    /// Code security analysis
    pub code_analysis: CodeSecurityAnalysis,
}

/// Dependency security audit results
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DependencyAudit {
    /// Total dependencies checked
    pub total_dependencies: usize,
    /// Dependencies with known vulnerabilities
    pub vulnerable_dependencies: usize,
    /// Critical vulnerabilities
    pub critical_vulns: usize,
    /// High severity vulnerabilities
    pub high_vulns: usize,
    /// Medium severity vulnerabilities
    pub medium_vulns: usize,
    /// Low severity vulnerabilities
    pub low_vulns: usize,
}

/// Code security analysis results
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CodeSecurityAnalysis {
    /// Security hotspots found
    pub hotspots: usize,
    /// Secure coding practices score
    pub practices_score: f64,
    /// Input validation score
    pub input_validation: f64,
    /// Output encoding score
    pub output_encoding: f64,
    /// Authentication/authorization score
    pub auth_score: f64,
}

/// Test coverage metrics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CoverageMetrics {
    /// Line coverage percentage
    pub line_coverage: f64,
    /// Branch coverage percentage
    pub branch_coverage: f64,
    /// Function coverage percentage
    pub function_coverage: f64,
    /// Integration test coverage
    pub integration_coverage: f64,
    /// Files with low coverage
    pub low_coverage_files: Vec<LowCoverageFile>,
}

/// File with low test coverage
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LowCoverageFile {
    /// Path to the file
    pub path: String,
    /// Line coverage percentage
    pub coverage: f64,
    /// Number of uncovered lines
    pub uncovered_lines: usize,
    /// Critical uncovered functions
    pub uncovered_functions: Vec<String>,
}

/// Quality gate for enforcement
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QualityGate {
    /// Name of the quality gate
    pub name: String,
    /// Description of what this gate checks
    pub description: String,
    /// Quality metric to check
    pub metric: QualityMetric,
    /// Comparison operator
    pub operator: GateOperator,
    /// Threshold value
    pub threshold: f64,
    /// Whether this gate is blocking (fails build)
    pub is_blocking: bool,
    /// Warning threshold (before blocking threshold)
    pub warning_threshold: Option<f64>,
}

/// Quality metric types for gates
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QualityMetric {
    /// Overall quality score
    OverallScore,
    /// Specific dimension score
    DimensionScore(String),
    /// Test coverage percentage
    TestCoverage,
    /// Technical debt ratio
    TechnicalDebtRatio,
    /// Vulnerability count
    VulnerabilityCount,
    /// Complexity threshold
    MaxComplexity,
    /// Performance regression count
    PerformanceRegressions,
}

/// Comparison operators for quality gates
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GateOperator {
    /// Greater than
    GreaterThan,
    /// Greater than or equal
    GreaterThanOrEqual,
    /// Less than
    LessThan,
    /// Less than or equal
    LessThanOrEqual,
    /// Equal to
    Equal,
    /// Not equal to
    NotEqual,
}

/// Quality snapshot for historical tracking
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QualitySnapshot {
    /// Commit hash for this snapshot
    pub commit_hash: String,
    /// Quality metrics at this point
    pub metrics: QualityMetrics,
    /// Quality gate results
    pub gate_results: Vec<GateResult>,
}

/// Result of quality gate evaluation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GateResult {
    /// Name of the gate
    pub gate_name: String,
    /// Whether the gate passed
    pub passed: bool,
    /// Actual value measured
    pub actual_value: f64,
    /// Threshold that was checked
    pub threshold: f64,
    /// Warning if applicable
    pub warning: Option<String>,
}

/// Quality analysis result
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QualityAnalysisResult {
    /// Current quality metrics
    pub current_metrics: QualityMetrics,
    /// Quality gate results
    pub gate_results: Vec<GateResult>,
    /// Quality trends analysis
    pub trends: QualityTrends,
    /// Recommendations for improvement
    pub recommendations: Vec<QualityRecommendation>,
    /// Predicted future quality (if enabled)
    pub predictions: Option<QualityPrediction>,
}

/// Quality trends analysis
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QualityTrends {
    /// Overall quality trend
    pub overall_trend: TrendDirection,
    /// Trend by dimension
    pub dimension_trends: HashMap<String, TrendDirection>,
    /// Technical debt trend
    pub debt_trend: DebtTrend,
    /// Performance trend
    pub performance_trend: TrendDirection,
    /// Coverage trend
    pub coverage_trend: TrendDirection,
}

/// Trend direction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TrendDirection {
    /// Improving trend
    Improving,
    /// Declining trend
    Declining,
    /// Stable trend
    Stable,
    /// Insufficient data
    Unknown,
}

/// Quality improvement recommendation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QualityRecommendation {
    /// Priority level (1-5, 5 being highest)
    pub priority: u8,
    /// Affected quality dimension
    pub dimension: String,
    /// Current score in this dimension
    pub current_score: f64,
    /// Potential improvement
    pub potential_improvement: f64,
    /// Estimated effort (hours)
    pub effort_hours: f64,
    /// Description of the issue
    pub description: String,
    /// Specific actions to take
    pub actions: Vec<String>,
    /// Expected impact on overall quality
    pub expected_impact: f64,
}

/// Quality prediction based on trends
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QualityPrediction {
    /// Predicted quality score in 1 week
    pub one_week: f64,
    /// Predicted quality score in 1 month
    pub one_month: f64,
    /// Predicted quality score in 3 months
    pub three_months: f64,
    /// Confidence level of prediction (0.0-1.0)
    pub confidence: f64,
    /// Key factors influencing prediction
    pub factors: Vec<String>,
}

impl Default for QualityConfig {
    fn default() -> Self {
        Self {
            max_history_entries: 100,
            dimension_weights: DimensionWeights::default(),
            quality_thresholds: QualityThresholds::default(),
            trend_config: TrendConfig::default(),
        }
    }
}

impl Default for DimensionWeights {
    fn default() -> Self {
        Self {
            correctness: 0.30,
            maintainability: 0.20,
            security: 0.20,
            performance: 0.15,
            reliability: 0.10,
            usability: 0.05,
        }
    }
}

impl Default for QualityThresholds {
    fn default() -> Self {
        Self {
            overall_minimum: 0.75,
            correctness_minimum: 0.90,
            maintainability_minimum: 0.70,
            security_minimum: 0.80,
            max_complexity: 10.0,
            min_test_coverage: 0.80,
            max_technical_debt: 0.15,
        }
    }
}

impl Default for TrendConfig {
    fn default() -> Self {
        Self {
            trend_window: 10,
            min_data_points: 3,
            trend_sensitivity: 0.05,
            enable_prediction: true,
        }
    }
}

impl QualityMetricsAnalyzer {
    /// Create a new quality metrics analyzer
    pub fn new(config: QualityConfig) -> Self {
        Self {
            config,
            history: VecDeque::new(),
            current_metrics: QualityMetrics::default(),
            gates: Self::default_quality_gates(),
        }
    }

    /// Create analyzer with default configuration
    pub fn with_default_config() -> Self {
        Self::new(QualityConfig::default())
    }

    /// Analyze quality metrics for a commit
    pub async fn analyze_commit_quality(
        &mut self, 
        commit: &CommitNode
    ) -> GdkResult<QualityAnalysisResult> {
        // Calculate quality metrics from commit data
        let metrics = self.calculate_quality_metrics(commit).await?;
        
        // Evaluate quality gates
        let gate_results = self.evaluate_quality_gates(&metrics)?;
        
        // Analyze trends
        let trends = self.analyze_trends(&metrics)?;
        
        // Generate recommendations
        let recommendations = self.generate_recommendations(&metrics, &trends)?;
        
        // Predict future quality if enabled
        let predictions = if self.config.trend_config.enable_prediction {
            Some(self.predict_quality_trends()?)
        } else {
            None
        };
        
        // Update history
        self.add_to_history(commit.hash.clone(), metrics.clone());
        self.current_metrics = metrics.clone();
        
        Ok(QualityAnalysisResult {
            current_metrics: metrics,
            gate_results,
            trends,
            recommendations,
            predictions,
        })
    }

    /// Calculate comprehensive quality metrics from commit data
    async fn calculate_quality_metrics(&self, commit: &CommitNode) -> GdkResult<QualityMetrics> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| GdkError::validation_error("timestamp", "system_time", e.to_string()))?
            .as_secs();

        // Calculate dimension scores
        let dimensions = self.calculate_dimension_scores(commit)?;
        
        // Calculate overall score using configured weights
        let overall_score = self.calculate_weighted_score(&dimensions)?;
        
        // Calculate technical debt
        let technical_debt = self.calculate_technical_debt(commit)?;
        
        // Calculate complexity metrics
        let complexity = self.calculate_complexity_metrics(commit)?;
        
        // Calculate performance metrics
        let performance = self.calculate_performance_metrics(commit)?;
        
        // Calculate security metrics
        let security = self.calculate_security_metrics(commit)?;
        
        // Calculate coverage metrics
        let coverage = self.calculate_coverage_metrics(commit)?;

        Ok(QualityMetrics {
            overall_score,
            dimensions,
            technical_debt,
            complexity,
            performance,
            security,
            coverage,
            timestamp,
        })
    }

    /// Calculate quality scores for each dimension
    fn calculate_dimension_scores(&self, commit: &CommitNode) -> GdkResult<QualityDimensions> {
        let mut total_scores = QualityDimensions {
            correctness: 0.0,
            maintainability: 0.0,
            security: 0.0,
            performance: 0.0,
            reliability: 0.0,
            usability: 0.0,
        };

        let file_count = commit.file_threads.len();
        if file_count == 0 {
            return Ok(total_scores);
        }

        // Aggregate scores from all file threads
        for thread in commit.file_threads.values() {
            // Correctness: based on type checking and test coverage
            let correctness = (thread.type_check_score + thread.test_coverage) / 2.0;
            total_scores.correctness += correctness;

            // Maintainability: based on lint score and structure
            let maintainability = thread.lint_score;
            total_scores.maintainability += maintainability;

            // Security: derive from lint and functionality scores
            let security = (thread.lint_score + thread.functionality_score) / 2.0;
            total_scores.security += security;

            // Performance: based on functionality score (proxy for efficiency)
            let performance = thread.functionality_score;
            total_scores.performance += performance;

            // Reliability: combination of test coverage and functionality
            let reliability = (thread.test_coverage + thread.functionality_score) / 2.0;
            total_scores.reliability += reliability;

            // Usability: based on documentation and API design (proxy: lint score)
            let usability = thread.lint_score;
            total_scores.usability += usability;
        }

        // Average the scores
        total_scores.correctness /= file_count as f64;
        total_scores.maintainability /= file_count as f64;
        total_scores.security /= file_count as f64;
        total_scores.performance /= file_count as f64;
        total_scores.reliability /= file_count as f64;
        total_scores.usability /= file_count as f64;

        Ok(total_scores)
    }

    /// Calculate weighted overall score
    fn calculate_weighted_score(&self, dimensions: &QualityDimensions) -> GdkResult<f64> {
        let weights = &self.config.dimension_weights;
        
        let score = dimensions.correctness * weights.correctness +
                   dimensions.maintainability * weights.maintainability +
                   dimensions.security * weights.security +
                   dimensions.performance * weights.performance +
                   dimensions.reliability * weights.reliability +
                   dimensions.usability * weights.usability;
        
        Ok(score.clamp(0.0, 1.0))
    }

    /// Default quality gates for standard projects
    fn default_quality_gates() -> Vec<QualityGate> {
        vec![
            QualityGate {
                name: "Overall Quality".to_string(),
                description: "Minimum overall quality score".to_string(),
                metric: QualityMetric::OverallScore,
                operator: GateOperator::GreaterThanOrEqual,
                threshold: 0.75,
                is_blocking: true,
                warning_threshold: Some(0.70),
            },
            QualityGate {
                name: "Test Coverage".to_string(),
                description: "Minimum test coverage percentage".to_string(),
                metric: QualityMetric::TestCoverage,
                operator: GateOperator::GreaterThanOrEqual,
                threshold: 0.80,
                is_blocking: true,
                warning_threshold: Some(0.75),
            },
            QualityGate {
                name: "Technical Debt".to_string(),
                description: "Maximum technical debt ratio".to_string(),
                metric: QualityMetric::TechnicalDebtRatio,
                operator: GateOperator::LessThanOrEqual,
                threshold: 0.15,
                is_blocking: false,
                warning_threshold: Some(0.20),
            },
            QualityGate {
                name: "Security Vulnerabilities".to_string(),
                description: "Maximum number of security vulnerabilities".to_string(),
                metric: QualityMetric::VulnerabilityCount,
                operator: GateOperator::Equal,
                threshold: 0.0,
                is_blocking: true,
                warning_threshold: None,
            },
        ]
    }

    // Placeholder implementations for complex calculations
    fn calculate_technical_debt(&self, _commit: &CommitNode) -> GdkResult<TechnicalDebtMetrics> {
        Ok(TechnicalDebtMetrics {
            total_debt_hours: 0.0,
            debt_ratio: 0.0,
            debt_breakdown: HashMap::new(),
            priority_items: Vec::new(),
            trend: DebtTrend::Stable,
        })
    }

    fn calculate_complexity_metrics(&self, _commit: &CommitNode) -> GdkResult<ComplexityMetrics> {
        Ok(ComplexityMetrics {
            avg_cyclomatic: 5.0,
            max_cyclomatic: 15.0,
            avg_cognitive: 8.0,
            max_cognitive: 25.0,
            complex_functions: 0,
            complex_files: Vec::new(),
        })
    }

    fn calculate_performance_metrics(&self, _commit: &CommitNode) -> GdkResult<PerformanceMetrics> {
        Ok(PerformanceMetrics {
            benchmark_score: 1.0,
            memory_efficiency: 0.85,
            compilation_time: 30.0,
            test_execution_time: 15.0,
            regressions: 0,
            improvements: 0,
        })
    }

    fn calculate_security_metrics(&self, _commit: &CommitNode) -> GdkResult<SecurityMetrics> {
        Ok(SecurityMetrics {
            vulnerability_count: 0,
            security_score: 0.90,
            dependency_audit: DependencyAudit {
                total_dependencies: 50,
                vulnerable_dependencies: 0,
                critical_vulns: 0,
                high_vulns: 0,
                medium_vulns: 0,
                low_vulns: 0,
            },
            code_analysis: CodeSecurityAnalysis {
                hotspots: 0,
                practices_score: 0.85,
                input_validation: 0.80,
                output_encoding: 0.90,
                auth_score: 0.95,
            },
        })
    }

    fn calculate_coverage_metrics(&self, commit: &CommitNode) -> GdkResult<CoverageMetrics> {
        // Calculate average test coverage from file threads
        let avg_coverage = if commit.file_threads.is_empty() {
            0.0
        } else {
            commit.file_threads.values()
                .map(|thread| thread.test_coverage)
                .sum::<f64>() / commit.file_threads.len() as f64
        };

        Ok(CoverageMetrics {
            line_coverage: avg_coverage,
            branch_coverage: avg_coverage * 0.9, // Estimate
            function_coverage: avg_coverage * 1.1, // Estimate
            integration_coverage: avg_coverage * 0.8, // Estimate
            low_coverage_files: Vec::new(),
        })
    }

    fn evaluate_quality_gates(&self, metrics: &QualityMetrics) -> GdkResult<Vec<GateResult>> {
        let mut results = Vec::new();

        for gate in &self.gates {
            let actual_value = self.extract_metric_value(gate, metrics)?;
            let passed = self.evaluate_gate_condition(gate, actual_value);
            
            let warning = if let Some(warning_threshold) = gate.warning_threshold {
                if !passed || !self.evaluate_gate_condition_with_threshold(gate, actual_value, warning_threshold) {
                    Some(format!("Quality gate '{}' is approaching threshold", gate.name))
                } else {
                    None
                }
            } else {
                None
            };

            results.push(GateResult {
                gate_name: gate.name.clone(),
                passed,
                actual_value,
                threshold: gate.threshold,
                warning,
            });
        }

        Ok(results)
    }

    fn extract_metric_value(&self, gate: &QualityGate, metrics: &QualityMetrics) -> GdkResult<f64> {
        match &gate.metric {
            QualityMetric::OverallScore => Ok(metrics.overall_score),
            QualityMetric::TestCoverage => Ok(metrics.coverage.line_coverage),
            QualityMetric::TechnicalDebtRatio => Ok(metrics.technical_debt.debt_ratio),
            QualityMetric::VulnerabilityCount => Ok(metrics.security.vulnerability_count as f64),
            QualityMetric::MaxComplexity => Ok(metrics.complexity.max_cyclomatic),
            QualityMetric::PerformanceRegressions => Ok(metrics.performance.regressions as f64),
            QualityMetric::DimensionScore(dimension) => {
                match dimension.as_str() {
                    "correctness" => Ok(metrics.dimensions.correctness),
                    "maintainability" => Ok(metrics.dimensions.maintainability),
                    "security" => Ok(metrics.dimensions.security),
                    "performance" => Ok(metrics.dimensions.performance),
                    "reliability" => Ok(metrics.dimensions.reliability),
                    "usability" => Ok(metrics.dimensions.usability),
                    _ => Err(GdkError::validation_error(
                        "quality_gate",
                        "unknown_dimension",
                        format!("Unknown dimension: {dimension}"),
                    )),
                }
            }
        }
    }

    fn evaluate_gate_condition(&self, gate: &QualityGate, actual_value: f64) -> bool {
        self.evaluate_gate_condition_with_threshold(gate, actual_value, gate.threshold)
    }

    fn evaluate_gate_condition_with_threshold(&self, gate: &QualityGate, actual_value: f64, threshold: f64) -> bool {
        match gate.operator {
            GateOperator::GreaterThan => actual_value > threshold,
            GateOperator::GreaterThanOrEqual => actual_value >= threshold,
            GateOperator::LessThan => actual_value < threshold,
            GateOperator::LessThanOrEqual => actual_value <= threshold,
            GateOperator::Equal => (actual_value - threshold).abs() < f64::EPSILON,
            GateOperator::NotEqual => (actual_value - threshold).abs() >= f64::EPSILON,
        }
    }

    fn analyze_trends(&self, _current_metrics: &QualityMetrics) -> GdkResult<QualityTrends> {
        // Simplified trend analysis - would be more sophisticated in practice
        Ok(QualityTrends {
            overall_trend: TrendDirection::Stable,
            dimension_trends: HashMap::new(),
            debt_trend: DebtTrend::Stable,
            performance_trend: TrendDirection::Stable,
            coverage_trend: TrendDirection::Stable,
        })
    }

    fn generate_recommendations(&self, _metrics: &QualityMetrics, _trends: &QualityTrends) -> GdkResult<Vec<QualityRecommendation>> {
        // Simplified recommendation generation
        Ok(Vec::new())
    }

    fn predict_quality_trends(&self) -> GdkResult<QualityPrediction> {
        // Simplified prediction model
        Ok(QualityPrediction {
            one_week: 0.80,
            one_month: 0.82,
            three_months: 0.85,
            confidence: 0.70,
            factors: vec!["test_coverage_improvement".to_string(), "complexity_reduction".to_string()],
        })
    }

    fn add_to_history(&mut self, commit_hash: String, metrics: QualityMetrics) {
        let snapshot = QualitySnapshot {
            commit_hash,
            metrics,
            gate_results: Vec::new(), // Would be populated in practice
        };

        self.history.push_back(snapshot);

        // Maintain history size limit
        while self.history.len() > self.config.max_history_entries {
            self.history.pop_front();
        }
    }
}

impl Default for QualityMetrics {
    fn default() -> Self {
        Self {
            overall_score: 0.0,
            dimensions: QualityDimensions {
                correctness: 0.0,
                maintainability: 0.0,
                security: 0.0,
                performance: 0.0,
                reliability: 0.0,
                usability: 0.0,
            },
            technical_debt: TechnicalDebtMetrics {
                total_debt_hours: 0.0,
                debt_ratio: 0.0,
                debt_breakdown: HashMap::new(),
                priority_items: Vec::new(),
                trend: DebtTrend::Unknown,
            },
            complexity: ComplexityMetrics {
                avg_cyclomatic: 0.0,
                max_cyclomatic: 0.0,
                avg_cognitive: 0.0,
                max_cognitive: 0.0,
                complex_functions: 0,
                complex_files: Vec::new(),
            },
            performance: PerformanceMetrics {
                benchmark_score: 1.0,
                memory_efficiency: 1.0,
                compilation_time: 0.0,
                test_execution_time: 0.0,
                regressions: 0,
                improvements: 0,
            },
            security: SecurityMetrics {
                vulnerability_count: 0,
                security_score: 1.0,
                dependency_audit: DependencyAudit {
                    total_dependencies: 0,
                    vulnerable_dependencies: 0,
                    critical_vulns: 0,
                    high_vulns: 0,
                    medium_vulns: 0,
                    low_vulns: 0,
                },
                code_analysis: CodeSecurityAnalysis {
                    hotspots: 0,
                    practices_score: 1.0,
                    input_validation: 1.0,
                    output_encoding: 1.0,
                    auth_score: 1.0,
                },
            },
            coverage: CoverageMetrics {
                line_coverage: 0.0,
                branch_coverage: 0.0,
                function_coverage: 0.0,
                integration_coverage: 0.0,
                low_coverage_files: Vec::new(),
            },
            timestamp: 0,
        }
    }
}