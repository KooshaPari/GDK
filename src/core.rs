//! Core workflow management implementation
//!
//! This module provides the primary [`GitWorkflowManager`] that orchestrates:
//! - Infinite monkey theorem iterations with convergence detection
//! - Spiral branching for experimental changes with automatic revert
//! - Quality assessment across multiple dimensions (lint, types, tests, functionality)
//! - Commit node creation with comprehensive thread analysis
//! - Revert point management for intelligent state restoration

use crate::{
    CommitNode, ConvergenceMetrics, FileThread, GitWorkflow, RevertPoint, ThreadColor,
    ThreadMetrics, ThreadState, GdkError, GdkResult, GdkResultExt,
};
use anyhow::{anyhow, Context};
use git2::{Repository, Signature};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::process::Command;
use uuid::Uuid;

/// Primary workflow manager implementing the GDK git workflow system
///
/// Manages the complete lifecycle of AI agent interactions with git:
/// - Repository state and commit history tracking
/// - Quality-based thread management for files
/// - Convergence analysis and spiral branching
/// - Intelligent revert point creation and restoration
///
/// # Thread Safety
///
/// This struct is designed for single-threaded use within async contexts.
/// For multi-agent scenarios, create separate instances per agent.
///
/// # Example
///
/// ```rust,no_run
/// use gdk::core::GitWorkflowManager;
///
/// #[tokio::main]
/// async fn main() -> gdk::GdkResult<()> {
///     let mut manager = GitWorkflowManager::new("./my-project")?;
///     
///     // Create a checkpoint before risky changes
///     let checkpoint = manager.create_revert_point("Before refactoring").await?;
///     
///     // Attempt convergence through iteration
///     let result = manager.infinite_monkey_iteration(10).await?;
///     
///     println!("Converged with score: {}", result.health_score);
///     Ok(())
/// }
/// ```
pub struct GitWorkflowManager {
    /// Git repository handle for all git operations
    pub repo: Repository,
    /// Absolute path to the repository root
    pub repo_path: String,
    /// Complete history of commit nodes with quality analysis
    pub commit_history: Vec<CommitNode>,
    /// Stack of revert points for state restoration
    pub revert_points: Vec<RevertPoint>,
    /// Current active branch name
    pub current_branch: String,
}

impl GitWorkflowManager {
    /// Create a new workflow manager for the specified repository
    ///
    /// # Arguments
    ///
    /// * `repo_path` - Path to git repository (will be created if doesn't exist)
    ///
    /// # Returns
    ///
    /// A configured [`GitWorkflowManager`] ready for workflow operations
    ///
    /// # Errors
    ///
    /// Returns [`GdkError::GitError`] if:
    /// - Repository cannot be opened or created
    /// - Current HEAD cannot be determined
    /// - File system permissions prevent access
    pub fn new(repo_path: &str) -> GdkResult<Self> {
        let repo = Repository::open(repo_path)
            .or_else(|_| Repository::init(repo_path))
            .with_git_context("opening or creating repository")?;

        let current_branch = repo
            .head()
            .with_git_context("determining current branch")?
            .shorthand()
            .unwrap_or("main")
            .to_string();

        Ok(Self {
            repo,
            repo_path: repo_path.to_string(),
            commit_history: Vec::new(),
            revert_points: Vec::new(),
            current_branch,
        })
    }

    /// Execute infinite monkey theorem convergence algorithm
    ///
    /// Attempts to reach convergence through iterative improvement:
    /// 1. Create commit with current state
    /// 2. Analyze quality metrics across all threads
    /// 3. Check convergence criteria (score > 0.8, stable trend)
    /// 4. If not converged, revert and try again
    /// 5. Repeat until convergence or max attempts reached
    ///
    /// # Arguments
    ///
    /// * `max_attempts` - Maximum iterations before giving up
    ///
    /// # Returns
    ///
    /// The final [`CommitNode`] that achieved convergence
    ///
    /// # Errors
    ///
    /// Returns [`GdkError::ConvergenceError`] if convergence not achieved
    pub async fn infinite_monkey_iteration(&mut self, max_attempts: u32) -> GdkResult<CommitNode> {
        let initial_revert_point = self.create_revert_point("infinite_monkey_start").await?;

        for attempt in 1..=max_attempts {
            tracing::info!("Infinite monkey attempt {}/{}", attempt, max_attempts);

            let commit_node = self
                .create_commit_node(&format!("Iteration attempt {attempt}"))
                .await?;

            let convergence = self.analyze_convergence().await?;
            if convergence.is_converged {
                tracing::info!("Convergence achieved at attempt {}", attempt);
                return Ok(commit_node);
            }

            if attempt < max_attempts {
                self.revert_to_point(&initial_revert_point).await?;
            }
        }

        let last_score = self.commit_history.last()
            .map(|c| c.health_score)
            .unwrap_or(0.0);
            
        Err(GdkError::convergence_error(
            "Maximum iterations reached without convergence",
            max_attempts,
            last_score,
            0.8,
        ))
    }

    pub async fn create_spiral_branch(&mut self, base_commit: &str) -> Result<String> {
        let uuid_str = Uuid::new_v4().to_string();
        let spiral_branch_name = format!("spiral-{}", &uuid_str[..8]);

        let base_commit_obj = self.repo.find_commit(
            git2::Oid::from_str(base_commit).map_err(|e| anyhow!("Invalid commit hash: {}", e))?,
        )?;

        self.repo
            .branch(&spiral_branch_name, &base_commit_obj, false)?;

        self.repo
            .set_head(&format!("refs/heads/{spiral_branch_name}"))?;
        self.repo
            .checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;

        self.current_branch = spiral_branch_name.clone();

        Ok(spiral_branch_name)
    }

    async fn run_quality_checks(&self, file_path: &str) -> Result<(f64, f64, f64, f64)> {
        let lint_score = self.run_lint_check(file_path).await.unwrap_or(0.0);
        let type_check_score = self.run_type_check(file_path).await.unwrap_or(0.0);
        let test_coverage = self.get_test_coverage(file_path).await.unwrap_or(0.0);
        let functionality_score = self.assess_functionality(file_path).await.unwrap_or(0.0);

        Ok((
            lint_score,
            type_check_score,
            test_coverage,
            functionality_score,
        ))
    }

    async fn run_lint_check(&self, _file_path: &str) -> Result<f64> {
        let output = Command::new("cargo")
            .args(["clippy", "--", "-D", "warnings"])
            .current_dir(&self.repo_path)
            .output()
            .await?;

        if output.status.success() {
            Ok(1.0)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let warning_count = stderr.matches("warning:").count();
            let error_count = stderr.matches("error:").count();

            let penalty = (warning_count as f64 * 0.1) + (error_count as f64 * 0.5);
            Ok((1.0 - penalty).max(0.0))
        }
    }

    async fn run_type_check(&self, _file_path: &str) -> Result<f64> {
        let output = Command::new("cargo")
            .args(["check"])
            .current_dir(&self.repo_path)
            .output()
            .await?;

        Ok(if output.status.success() { 1.0 } else { 0.0 })
    }

    async fn get_test_coverage(&self, _file_path: &str) -> Result<f64> {
        let output = Command::new("cargo")
            .args(["test"])
            .current_dir(&self.repo_path)
            .output()
            .await?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);

            if let Some(line) = stdout.lines().find(|l| l.contains("test result:")) {
                if let Some(passed_part) = line.split_whitespace().nth(2) {
                    if let Ok(passed) = passed_part.parse::<u32>() {
                        if let Some(total_part) = line.split_whitespace().nth(4) {
                            if let Ok(total) = total_part.parse::<u32>() {
                                return Ok(passed as f64 / total as f64);
                            }
                        }
                    }
                }
            }
        }

        Ok(0.5)
    }

    async fn assess_functionality(&self, file_path: &str) -> Result<f64> {
        let content = tokio::fs::read_to_string(file_path).await?;

        let lines = content.lines().count() as f64;
        let non_empty_lines = content.lines().filter(|l| !l.trim().is_empty()).count() as f64;
        let comment_lines = content
            .lines()
            .filter(|l| l.trim().starts_with("//"))
            .count() as f64;

        let density = non_empty_lines / lines.max(1.0);
        let documentation_ratio = comment_lines / non_empty_lines.max(1.0);

        Ok((density * 0.7 + documentation_ratio * 0.3).min(1.0))
    }

    pub async fn get_current_commit_hash(&self) -> Result<String> {
        let head = self.repo.head()?;
        let commit = head.peel_to_commit()?;
        Ok(commit.id().to_string())
    }
}

#[async_trait::async_trait(?Send)]
impl GitWorkflow for GitWorkflowManager {
    async fn create_commit_node(&mut self, message: &str) -> Result<CommitNode> {
        // Perform all git operations synchronously first
        let (commit_hash, parent_hashes) = {
            let mut index = self.repo.index()?;
            index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
            index.write()?;

            let tree_id = index.write_tree()?;
            let tree = self.repo.find_tree(tree_id)?;

            let signature = Signature::now("GDK System", "gdk@system.local")?;
            let parent_commit = self.repo.head()?.peel_to_commit().ok();

            let parent_hashes = if let Some(ref parent) = parent_commit {
                vec![parent.id().to_string()]
            } else {
                vec![]
            };

            let parents: Vec<_> = if let Some(ref parent) = parent_commit {
                vec![parent]
            } else {
                vec![]
            };

            let commit_id = self.repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                message,
                &tree,
                &parents,
            )?;

            (commit_id.to_string(), parent_hashes)
        };

        let mut file_threads = HashMap::new();

        let changed_files = self.get_changed_files().await?;
        for file_path in &changed_files {
            let (lint, type_check, test_coverage, functionality) =
                self.run_quality_checks(&file_path).await?;

            let color_status =
                ThreadColor::from_scores(lint, type_check, test_coverage, functionality);

            let thread = FileThread {
                file_path: file_path.to_string(),
                thread_id: Uuid::new_v4(),
                color_status,
                lint_score: lint,
                type_check_score: type_check,
                test_coverage,
                functionality_score: functionality,
                history: vec![ThreadState {
                    commit_hash: commit_hash.clone(),
                    diff_content: self.get_file_diff(&file_path).await.unwrap_or_default(),
                    metrics: ThreadMetrics {
                        lines_added: 0,
                        lines_removed: 0,
                        complexity_delta: 0.0,
                        quality_score: (lint + type_check + test_coverage + functionality) / 4.0,
                    },
                    timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
                }],
            };

            file_threads.insert(file_path.clone(), thread);
        }

        let convergence_metrics = self.analyze_convergence().await?;
        let health_score = file_threads
            .values()
            .map(|t| t.functionality_score)
            .sum::<f64>()
            / file_threads.len().max(1) as f64;

        let commit_node = CommitNode {
            id: Uuid::new_v4().to_string(),
            hash: commit_hash,
            parent_hashes,
            message: message.to_string(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
            file_threads,
            health_score,
            convergence_metrics,
        };

        self.commit_history.push(commit_node.clone());

        Ok(commit_node)
    }

    async fn create_revert_point(&mut self, reason: &str) -> Result<RevertPoint> {
        let commit_hash = {
            let head = self.repo.head()?;
            let commit = head.peel_to_commit()?;
            commit.id().to_string()
        };

        let convergence_state = self.analyze_convergence().await?;

        Ok(RevertPoint {
            commit_hash,
            branch_name: self.current_branch.clone(),
            snapshot_id: Uuid::new_v4(),
            file_states: HashMap::new(),
            metadata: crate::RevertMetadata {
                reason: reason.to_string(),
                agent_id: "gdk-system".to_string(),
                timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
                convergence_state,
            },
        })
    }

    async fn revert_to_point(&mut self, point: &RevertPoint) -> Result<()> {
        let commit_oid = git2::Oid::from_str(&point.commit_hash)?;
        let commit = self.repo.find_commit(commit_oid)?;

        self.repo
            .reset(commit.as_object(), git2::ResetType::Hard, None)?;

        if point.branch_name != self.current_branch {
            self.repo
                .set_head(&format!("refs/heads/{}", point.branch_name))?;
            self.current_branch = point.branch_name.clone();
        }

        Ok(())
    }

    async fn analyze_convergence(&self) -> Result<ConvergenceMetrics> {
        let recent_commits = self.commit_history.iter().rev().take(10);

        let quality_trend: Vec<f64> = recent_commits.map(|c| c.health_score).collect();

        let is_converged = if quality_trend.len() >= 3 {
            let recent_avg = quality_trend.iter().take(3).sum::<f64>() / 3.0;
            recent_avg > 0.8 && quality_trend.windows(2).all(|w| w[0] <= w[1])
        } else {
            false
        };

        Ok(ConvergenceMetrics {
            attempts: self.commit_history.len() as u32,
            successful_builds: quality_trend.iter().filter(|&&q| q > 0.7).count() as u32,
            test_pass_rate: quality_trend.iter().sum::<f64>() / quality_trend.len().max(1) as f64,
            quality_trend,
            is_converged,
        })
    }

    async fn update_thread_colors(&mut self) -> Result<()> {
        for commit in &mut self.commit_history {
            for thread in commit.file_threads.values_mut() {
                thread.color_status = ThreadColor::from_scores(
                    thread.lint_score,
                    thread.type_check_score,
                    thread.test_coverage,
                    thread.functionality_score,
                );
            }
        }
        Ok(())
    }

    async fn validate_ci_cd(&self, _commit_hash: &str) -> Result<bool> {
        let output = Command::new("cargo")
            .args(["test", "--", "--test-threads=1"])
            .current_dir(&self.repo_path)
            .output()
            .await?;

        Ok(output.status.success())
    }
}

impl GitWorkflowManager {
    async fn get_changed_files(&self) -> Result<Vec<String>> {
        let output = Command::new("git")
            .args(["diff", "--name-only", "HEAD~1..HEAD"])
            .current_dir(&self.repo_path)
            .output()
            .await?;

        let files = String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|s| s.to_string())
            .collect();

        Ok(files)
    }

    async fn get_file_diff(&self, file_path: &str) -> Result<String> {
        let output = Command::new("git")
            .args(["diff", "HEAD~1..HEAD", "--", file_path])
            .current_dir(&self.repo_path)
            .output()
            .await?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
