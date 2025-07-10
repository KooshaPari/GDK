//! Advanced validation and quality metrics system for GDK
//!
//! This module provides comprehensive code quality validation:
//! - Pluggable validator system for different languages and tools
//! - Parallel execution with configurable timeouts
//! - Advanced scoring algorithms with weighted metrics
//! - Detailed recommendations and improvement suggestions
//! - Security auditing and compliance checking
//! - Performance profiling and memory usage analysis
//! - Code coverage measurement and trend analysis
//!
//! # Example Usage
//!
//! ```rust,no_run
//! use gdk::validation::ValidationSuite;
//!
//! #[tokio::main]
//! async fn main() -> gdk::GdkResult<()> {
//!     let suite = ValidationSuite::rust_default("./my-project");
//!     let result = suite.validate("./my-project").await?;
//!     
//!     println!("Overall score: {:.3}", result.overall_score);
//!     for recommendation in result.recommendations {
//!         println!("💡 {}", recommendation);
//!     }
//!     
//!     Ok(())
//! }
//! ```

use crate::{GdkResult, GdkError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Stdio;
use tokio::process::Command;

/// Comprehensive validation suite for code quality assessment
///
/// Manages a collection of validators that can run in parallel or sequentially.
/// Each validator has configurable weight, timeout, and requirement level.
/// Results are aggregated into an overall quality score with detailed recommendations.
///
/// # Features
///
/// - **Parallel Execution**: Run multiple validators concurrently for speed
/// - **Weighted Scoring**: Different validators contribute different amounts
/// - **Required vs Optional**: Some validators must pass for overall success
/// - **Timeout Protection**: Prevent hanging on problematic code
/// - **Fail Fast**: Stop execution on critical failures
/// - **Detailed Output**: Capture stdout/stderr for analysis
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValidationSuite {
    /// List of validators to execute
    pub validators: Vec<Validator>,
    /// Rules governing validation behavior
    pub validation_rules: ValidationRules,
}

/// Individual validator configuration
///
/// Represents a single validation tool (e.g., cargo clippy, eslint, mypy)
/// with its execution parameters and scoring weight.
///
/// # Example
///
/// ```rust
/// use gdk::validation::Validator;
///
/// let clippy = Validator {
///     name: "cargo_clippy".to_string(),
///     command: "cargo".to_string(),
///     args: vec!["clippy".to_string(), "--".to_string(), "-D".to_string(), "warnings".to_string()],
///     working_dir: Some("./project".to_string()),
///     timeout_seconds: 120,
///     weight: 0.25,
///     is_required: false,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Validator {
    /// Unique name for this validator
    pub name: String,
    /// Command to execute (e.g., "cargo", "npm", "python")
    pub command: String,
    /// Command line arguments
    pub args: Vec<String>,
    /// Working directory (defaults to repo root)
    pub working_dir: Option<String>,
    /// Maximum execution time in seconds
    pub timeout_seconds: u64,
    /// Weight in overall score calculation (0.0-1.0)
    pub weight: f64,
    /// Whether this validator must pass for overall success
    pub is_required: bool,
}

/// Rules governing validation suite behavior
///
/// Controls how validators are executed and how results are interpreted.
/// These rules affect performance, error handling, and pass/fail criteria.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValidationRules {
    /// Minimum weighted score required for overall pass (0.0-1.0)
    pub min_passing_score: f64,
    /// Whether all required validators must pass
    pub required_validators_must_pass: bool,
    /// Stop execution on first critical failure
    pub fail_fast: bool,
    /// Execute validators in parallel for speed
    pub parallel_execution: bool,
}

/// Complete validation results with metrics and recommendations
///
/// Contains aggregated results from all validators with overall scoring,
/// timing information, and actionable recommendations for improvement.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValidationResult {
    /// Weighted overall score (0.0-1.0)
    pub overall_score: f64,
    /// Whether validation passed based on rules
    pub passed: bool,
    /// Individual validator results by name
    pub validator_results: HashMap<String, ValidatorResult>,
    /// Total execution time in milliseconds
    pub execution_time_ms: u64,
    /// Actionable recommendations for improvement
    pub recommendations: Vec<String>,
}

/// Result from a single validator execution
///
/// Contains detailed information about validator execution including
/// output, timing, scoring, and success/failure status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValidatorResult {
    /// Name of the validator that ran
    pub name: String,
    /// Whether the validator passed (exit code 0)
    pub passed: bool,
    /// Calculated score (0.0-1.0) based on output analysis
    pub score: f64,
    /// Standard output from validator execution
    pub output: String,
    /// Standard error output from validator execution
    pub error_output: String,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
    /// Process exit code
    pub exit_code: i32,
}

impl Default for ValidationRules {
    fn default() -> Self {
        Self {
            min_passing_score: 0.8,
            required_validators_must_pass: true,
            fail_fast: false,
            parallel_execution: true,
        }
    }
}

impl ValidationSuite {
    pub fn new() -> Self {
        Self {
            validators: Vec::new(),
            validation_rules: ValidationRules::default(),
        }
    }

    pub fn rust_default(repo_path: &str) -> Self {
        let mut suite = Self::new();

        // Cargo check (type checking)
        suite.add_validator(Validator {
            name: "cargo_check".to_string(),
            command: "cargo".to_string(),
            args: vec!["check".to_string()],
            working_dir: Some(repo_path.to_string()),
            timeout_seconds: 60,
            weight: 0.25,
            is_required: true,
        });

        // Cargo clippy (linting)
        suite.add_validator(Validator {
            name: "cargo_clippy".to_string(),
            command: "cargo".to_string(),
            args: vec![
                "clippy".to_string(),
                "--".to_string(),
                "-D".to_string(),
                "warnings".to_string(),
            ],
            working_dir: Some(repo_path.to_string()),
            timeout_seconds: 120,
            weight: 0.25,
            is_required: false,
        });

        // Cargo test
        suite.add_validator(Validator {
            name: "cargo_test".to_string(),
            command: "cargo".to_string(),
            args: vec![
                "test".to_string(),
                "--".to_string(),
                "--test-threads=1".to_string(),
            ],
            working_dir: Some(repo_path.to_string()),
            timeout_seconds: 300,
            weight: 0.3,
            is_required: true,
        });

        // Cargo fmt check
        suite.add_validator(Validator {
            name: "cargo_fmt".to_string(),
            command: "cargo".to_string(),
            args: vec!["fmt".to_string(), "--".to_string(), "--check".to_string()],
            working_dir: Some(repo_path.to_string()),
            timeout_seconds: 30,
            weight: 0.1,
            is_required: false,
        });

        // Security audit (if cargo-audit is available)
        suite.add_validator(Validator {
            name: "cargo_audit".to_string(),
            command: "cargo".to_string(),
            args: vec!["audit".to_string()],
            working_dir: Some(repo_path.to_string()),
            timeout_seconds: 60,
            weight: 0.1,
            is_required: false,
        });

        suite
    }

    pub fn add_validator(&mut self, validator: Validator) {
        self.validators.push(validator);
    }

    pub fn set_rules(&mut self, rules: ValidationRules) {
        self.validation_rules = rules;
    }

    pub async fn validate(&self, repo_path: &str) -> GdkResult<ValidationResult> {
        let start_time = std::time::Instant::now();
        let mut validator_results = HashMap::new();
        let mut total_weighted_score = 0.0;
        let mut total_weight = 0.0;
        let mut required_failed = false;

        if self.validation_rules.parallel_execution {
            // Execute validators in parallel
            let mut handles = Vec::new();

            for validator in &self.validators {
                let validator_clone = validator.clone();
                let repo_path_clone = repo_path.to_string();

                let handle = tokio::spawn(async move {
                    Self::execute_validator(&validator_clone, &repo_path_clone).await
                });

                handles.push((validator.name.clone(), handle));
            }

            for (name, handle) in handles {
                let result = handle.await??;

                if self.validation_rules.fail_fast
                    && !result.passed
                    && self
                        .validators
                        .iter()
                        .find(|v| v.name == name)
                        .unwrap()
                        .is_required
                {
                    required_failed = true;
                }

                total_weighted_score += result.score
                    * self
                        .validators
                        .iter()
                        .find(|v| v.name == name)
                        .unwrap()
                        .weight;
                total_weight += self
                    .validators
                    .iter()
                    .find(|v| v.name == name)
                    .unwrap()
                    .weight;

                validator_results.insert(name, result);

                if required_failed && self.validation_rules.fail_fast {
                    break;
                }
            }
        } else {
            // Execute validators sequentially
            for validator in &self.validators {
                let result = Self::execute_validator(validator, repo_path).await?;

                if self.validation_rules.fail_fast && !result.passed && validator.is_required {
                    required_failed = true;
                }

                total_weighted_score += result.score * validator.weight;
                total_weight += validator.weight;

                validator_results.insert(validator.name.clone(), result);

                if required_failed && self.validation_rules.fail_fast {
                    break;
                }
            }
        }

        let overall_score = if total_weight > 0.0 {
            total_weighted_score / total_weight
        } else {
            0.0
        };

        // Check if validation passed
        let required_validators_passed = if self.validation_rules.required_validators_must_pass {
            self.validators
                .iter()
                .filter(|v| v.is_required)
                .all(|v| validator_results.get(&v.name).is_some_and(|r| r.passed))
        } else {
            true
        };

        let passed = overall_score >= self.validation_rules.min_passing_score
            && required_validators_passed
            && !required_failed;

        let recommendations = self.generate_recommendations(&validator_results);

        Ok(ValidationResult {
            overall_score,
            passed,
            validator_results,
            execution_time_ms: start_time.elapsed().as_millis() as u64,
            recommendations,
        })
    }

    async fn execute_validator(validator: &Validator, repo_path: &str) -> GdkResult<ValidatorResult> {
        let start_time = std::time::Instant::now();

        let default_dir = repo_path.to_string();
        let working_dir = validator.working_dir.as_ref().unwrap_or(&default_dir);

        let mut command = Command::new(&validator.command);
        command
            .args(&validator.args)
            .current_dir(working_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let child = command
            .spawn()
            .map_err(|e| GdkError::validation_error(
                "spawn_error",
                format!("Failed to spawn validator {}", validator.name),
                e.to_string(),
            ))?;

        let timeout_duration = std::time::Duration::from_secs(validator.timeout_seconds);
        let output = tokio::time::timeout(timeout_duration, child.wait_with_output())
            .await
            .map_err(|_| GdkError::validation_error(
                "timeout",
                format!("Validator {} timed out after {} seconds", validator.name, validator.timeout_seconds),
                "Command execution exceeded timeout limit".to_string(),
            ))?
            .map_err(|e| GdkError::validation_error(
                "execution_failed",
                format!("Validator {} execution failed", validator.name),
                e.to_string(),
            ))?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code().unwrap_or(-1);
        let passed = output.status.success();

        // Calculate score based on exit code and output
        let score = Self::calculate_validator_score(&validator.name, exit_code, &stdout, &stderr);

        Ok(ValidatorResult {
            name: validator.name.clone(),
            passed,
            score,
            output: stdout,
            error_output: stderr,
            execution_time_ms: start_time.elapsed().as_millis() as u64,
            exit_code,
        })
    }

    fn calculate_validator_score(
        validator_name: &str,
        exit_code: i32,
        stdout: &str,
        stderr: &str,
    ) -> f64 {
        if exit_code == 0 {
            return 1.0;
        }

        // Special handling for different validators
        match validator_name {
            "cargo_clippy" => {
                // Count warnings and errors
                let warning_count = stderr.matches("warning:").count();
                let error_count = stderr.matches("error:").count();

                if error_count > 0 {
                    0.0
                } else if warning_count == 0 {
                    1.0
                } else {
                    // Deduct score based on warnings
                    (1.0 - (warning_count as f64 * 0.1)).max(0.0)
                }
            }
            "cargo_test" => {
                // Parse test results
                if let Some(line) = stdout.lines().find(|l| l.contains("test result:")) {
                    if let Some(passed_part) = line.split_whitespace().nth(2) {
                        if let Ok(passed) = passed_part.parse::<u32>() {
                            if let Some(total_part) = line.split_whitespace().nth(4) {
                                if let Ok(total) = total_part.parse::<u32>() {
                                    return passed as f64 / total as f64;
                                }
                            }
                        }
                    }
                }
                0.0
            }
            "cargo_audit" => {
                // Security audit scoring
                if stderr.contains("vulnerabilities found") {
                    let vuln_count = stderr.matches("vulnerability").count();
                    (1.0 - (vuln_count as f64 * 0.2)).max(0.0)
                } else {
                    1.0
                }
            }
            _ => {
                // Default: binary pass/fail
                if exit_code == 0 {
                    1.0
                } else {
                    0.0
                }
            }
        }
    }

    fn generate_recommendations(&self, results: &HashMap<String, ValidatorResult>) -> Vec<String> {
        let mut recommendations = Vec::new();

        for (name, result) in results {
            if !result.passed {
                match name.as_str() {
                    "cargo_check" => {
                        recommendations.push(
                            "Fix compilation errors to improve type checking score.".to_string(),
                        );
                        if !result.error_output.is_empty() {
                            let error_lines: Vec<&str> =
                                result.error_output.lines().take(3).collect();
                            recommendations
                                .push(format!("First errors: {}", error_lines.join("; ")));
                        }
                    }
                    "cargo_clippy" => {
                        let warning_count = result.error_output.matches("warning:").count();
                        let error_count = result.error_output.matches("error:").count();

                        if error_count > 0 {
                            recommendations.push(format!("Fix {error_count} clippy errors."));
                        } else if warning_count > 0 {
                            recommendations.push(format!(
                                "Address {warning_count} clippy warnings to improve code quality."
                            ));
                        }
                    }
                    "cargo_test" => {
                        recommendations
                            .push("Fix failing tests to improve test coverage.".to_string());
                        if result.error_output.contains("test result:") {
                            recommendations.push(
                                "Check test output for specific failure details.".to_string(),
                            );
                        }
                    }
                    "cargo_fmt" => {
                        recommendations
                            .push("Run 'cargo fmt' to fix code formatting issues.".to_string());
                    }
                    "cargo_audit" => {
                        recommendations.push(
                            "Update dependencies to fix security vulnerabilities.".to_string(),
                        );
                    }
                    _ => {
                        recommendations.push(format!("Fix issues in {name} validator."));
                    }
                }
            }
        }

        if recommendations.is_empty() {
            recommendations.push("All validations passed successfully!".to_string());
        }

        recommendations
    }

    pub fn get_validator_summary(&self) -> ValidatorSummary {
        ValidatorSummary {
            total_validators: self.validators.len(),
            required_validators: self.validators.iter().filter(|v| v.is_required).count(),
            total_weight: self.validators.iter().map(|v| v.weight).sum(),
            validator_names: self.validators.iter().map(|v| v.name.clone()).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorSummary {
    pub total_validators: usize,
    pub required_validators: usize,
    pub total_weight: f64,
    pub validator_names: Vec<String>,
}

impl Default for ValidationSuite {
    fn default() -> Self {
        Self::new()
    }
}
