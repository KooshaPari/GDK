pub mod agent;
pub mod convergence;
pub mod core;
pub mod git;
pub mod threads;
pub mod validation;
pub mod visualization;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitNode {
    pub id: String,
    pub hash: String,
    pub parent_hashes: Vec<String>,
    pub message: String,
    pub timestamp: u64,
    pub file_threads: HashMap<String, FileThread>,
    pub health_score: f64,
    pub convergence_metrics: ConvergenceMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileThread {
    pub file_path: String,
    pub thread_id: Uuid,
    pub color_status: ThreadColor,
    pub lint_score: f64,
    pub type_check_score: f64,
    pub test_coverage: f64,
    pub functionality_score: f64,
    pub history: Vec<ThreadState>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ThreadColor {
    Red,
    Orange,
    Yellow,
    LightGreen,
    Green,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadState {
    pub commit_hash: String,
    pub diff_content: String,
    pub metrics: ThreadMetrics,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadMetrics {
    pub lines_added: u32,
    pub lines_removed: u32,
    pub complexity_delta: f64,
    pub quality_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceMetrics {
    pub attempts: u32,
    pub successful_builds: u32,
    pub test_pass_rate: f64,
    pub quality_trend: Vec<f64>,
    pub is_converged: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevertPoint {
    pub commit_hash: String,
    pub branch_name: String,
    pub snapshot_id: Uuid,
    pub file_states: HashMap<String, FileSnapshot>,
    pub metadata: RevertMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSnapshot {
    pub content_hash: String,
    pub thread_state: FileThread,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
