//! # GDK - Git Workflow Deep Knowledge
//!
//! A comprehensive git workflow system for AI agents featuring:
//! - Thread-based quality tracking with color coding
//! - Infinite monkey theorem convergence algorithms  
//! - Spiral branching with intelligent revert points
//! - Multi-format tree visualization
//! - Agent workflow management and analytics
//!
//! ## Core Types
//!
//! - [`CommitNode`]: Represents a git commit with quality metrics
//! - [`FileThread`]: Tracks quality across multiple dimensions per file
//! - [`ThreadColor`]: Visual quality indicator (Red → Green)
//! - [`ConvergenceMetrics`]: Mathematical convergence analysis
//! - [`RevertPoint`]: Intelligent checkpoint for state restoration

pub mod agent;
pub mod convergence;
pub mod core;
pub mod errors;
pub mod git;
pub mod performance;
pub mod quality_metrics;
pub mod threads;
pub mod validation;
pub mod visualization;

// Re-export commonly used types
pub use errors::{GdkError, GdkResult, GdkResultExt};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use uuid::Uuid;

/// Represents a git commit enhanced with quality metrics and thread analysis
///
/// Each commit node contains:
/// - Basic git metadata (hash, message, timestamp)
/// - Quality threads for each modified file
/// - Overall health score and convergence metrics
/// - Parent relationship for tree construction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommitNode {
    /// Unique identifier for this commit node
    pub id: String,
    /// Git commit hash (SHA-1)
    pub hash: String,
    /// Parent commit hashes for tree structure
    pub parent_hashes: Vec<String>,
    /// Git commit message
    pub message: String,
    /// Unix timestamp of commit creation
    pub timestamp: u64,
    /// Quality threads for each file modified in this commit
    pub file_threads: HashMap<String, FileThread>,
    /// Overall health score (0.0-1.0) derived from thread metrics
    pub health_score: f64,
    /// Convergence analysis for this commit
    pub convergence_metrics: ConvergenceMetrics,
}

/// Quality tracking thread for a specific file across multiple dimensions
///
/// Each file has a thread that monitors:
/// - Code quality (lint, type checking)
/// - Test coverage and functionality
/// - Historical quality progression
/// - Visual color status for quick assessment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileThread {
    /// Relative path to the file being tracked
    pub file_path: String,
    /// Unique identifier for this thread
    pub thread_id: Uuid,
    /// Current visual quality status (Red → Green)
    pub color_status: ThreadColor,
    /// Linting score (0.0-1.0): syntax, style, best practices
    pub lint_score: f64,
    /// Type checking score (0.0-1.0): compilation, type safety
    pub type_check_score: f64,
    /// Test coverage percentage (0.0-1.0)
    pub test_coverage: f64,
    /// Functional correctness score (0.0-1.0): runtime behavior
    pub functionality_score: f64,
    /// Historical progression of quality metrics
    pub history: Vec<ThreadState>,
}

/// Visual quality indicator using color coding system
///
/// Maps quality scores to intuitive colors:
/// - 🔴 Red (0.0-0.3): Critical issues, broken code
/// - 🟠 Orange (0.3-0.5): Major issues, needs attention  
/// - 🟡 Yellow (0.5-0.7): Minor issues, acceptable
/// - 🟢 LightGreen (0.7-0.9): Good quality, minor improvements
/// - 💚 Green (0.9-1.0): Excellent quality, production ready
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ThreadColor {
    /// Critical issues present (0.0-0.3)
    Red,
    /// Major issues that need attention (0.3-0.5)
    Orange,
    /// Minor issues, generally acceptable (0.5-0.7)
    Yellow,
    /// Good quality with room for improvement (0.7-0.9)
    LightGreen,
    /// Excellent quality, production ready (0.9-1.0)
    Green,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThreadState {
    pub commit_hash: String,
    pub diff_content: String,
    pub metrics: ThreadMetrics,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThreadMetrics {
    pub lines_added: u32,
    pub lines_removed: u32,
    pub complexity_delta: f64,
    pub quality_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConvergenceMetrics {
    pub attempts: u32,
    pub successful_builds: u32,
    pub test_pass_rate: f64,
    pub quality_trend: Vec<f64>,
    pub is_converged: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RevertPoint {
    pub commit_hash: String,
    pub branch_name: String,
    pub snapshot_id: Uuid,
    pub file_states: HashMap<String, FileSnapshot>,
    pub metadata: RevertMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileSnapshot {
    pub content_hash: String,
    pub thread_state: FileThread,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RevertMetadata {
    pub reason: String,
    pub agent_id: String,
    pub timestamp: u64,
    pub convergence_state: ConvergenceMetrics,
}

#[async_trait::async_trait(?Send)]
pub trait GitWorkflow {
    async fn create_commit_node(&mut self, message: &str) -> Result<CommitNode>;
    async fn create_revert_point(&mut self, reason: &str) -> Result<RevertPoint>;
    async fn revert_to_point(&mut self, point: &RevertPoint) -> Result<()>;
    async fn analyze_convergence(&self) -> Result<ConvergenceMetrics>;
    async fn update_thread_colors(&mut self) -> Result<()>;
    async fn validate_ci_cd(&self, commit_hash: &str) -> Result<bool>;
}

impl fmt::Display for ThreadColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (emoji, name) = match self {
            ThreadColor::Red => ("🔴", "Red"),
            ThreadColor::Orange => ("🟠", "Orange"),
            ThreadColor::Yellow => ("🟡", "Yellow"),
            ThreadColor::LightGreen => ("🟢", "Light Green"),
            ThreadColor::Green => ("💚", "Green"),
        };
        write!(f, "{} {}", emoji, name)
    }
}

impl ThreadColor {
    pub fn from_scores(lint: f64, type_check: f64, test_coverage: f64, functionality: f64) -> Self {
        let overall = (lint + type_check + test_coverage + functionality) / 4.0;
        match overall {
            x if x >= 0.9 => ThreadColor::Green,
            x if x >= 0.7 => ThreadColor::LightGreen,
            x if x >= 0.5 => ThreadColor::Yellow,
            x if x >= 0.3 => ThreadColor::Orange,
            _ => ThreadColor::Red,
        }
    }

    pub fn to_score(&self) -> f64 {
        match self {
            ThreadColor::Green => 1.0,
            ThreadColor::LightGreen => 0.8,
            ThreadColor::Yellow => 0.6,
            ThreadColor::Orange => 0.4,
            ThreadColor::Red => 0.2,
        }
    }
}