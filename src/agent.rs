//! Agent workflow management for GDK system
//!
//! This module provides intelligent agent coordination for git workflows:
//! - Multi-agent session tracking with isolated state
//! - Infinite monkey theorem implementation with convergence detection
//! - Spiral branching with automatic revert capabilities
//! - Action logging and statistical analysis
//! - Quality validation and CI/CD integration
//!
//! # Example Usage
//!
//! ```rust,no_run
//! use gdk::agent::AgentWorkflowController;
//! use gdk::core::GitWorkflowManager;
//!
//! #[tokio::main]
//! async fn main() -> gdk::GdkResult<()> {
//!     let workflow = GitWorkflowManager::new("./repo")?;
//!     let mut controller = AgentWorkflowController::new(workflow);
//!     
//!     // Start agent session
//!     let session_id = controller.start_agent_session("agent-1").await?;
//!     
//!     // Execute convergence workflow
//!     let result = controller.execute_infinite_monkey_workflow("agent-1", 0.8).await?;
//!     
//!     println!("Converged with health score: {}", result.health_score);
//!     Ok(())
//! }
//! ```

use crate::{CommitNode, ConvergenceMetrics, GitWorkflow, RevertPoint, GdkError, GdkResult};
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// Represents an active agent session with workflow state
///
/// Each agent maintains isolated session state including:
/// - Current commit tracking for state management
/// - Revert point stack for intelligent backtracking
/// - Convergence history for trend analysis
/// - Spiral attempt tracking with configurable limits
///
/// # Thread Safety
///
/// AgentSession is designed for single-threaded use within async contexts.
/// For concurrent access, use appropriate synchronization primitives.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentSession {
    /// Unique identifier for this agent session
    pub session_id: Uuid,
    /// Agent identifier for tracking across the system
    pub agent_id: String,
    /// Workflow type identifier (typically "gdk-workflow")
    pub workflow: String,
    /// Unix timestamp when session was started
    pub start_time: u64,
    /// Current git commit hash the agent is working on
    pub current_commit: Option<String>,
    /// Stack of revert points for intelligent backtracking
    pub revert_stack: Vec<RevertPoint>,
    /// Historical convergence metrics for trend analysis
    pub convergence_history: Vec<ConvergenceMetrics>,
    /// Current number of spiral attempts (infinite monkey iterations)
    pub spiral_attempts: u32,
    /// Maximum allowed spiral attempts before giving up
    pub max_spiral_attempts: u32,
}

/// Represents a single action taken by an agent during workflow execution
///
/// Actions are logged for:
/// - Debugging workflow failures and bottlenecks
/// - Statistical analysis of agent performance
/// - Audit trails for compliance and review
/// - Pattern recognition for optimization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentAction {
    /// Unique identifier for this specific action
    pub action_id: Uuid,
    /// Agent that performed this action
    pub agent_id: String,
    /// Type of action performed (see ActionType enum)
    pub action_type: ActionType,
    /// Unix timestamp when action was initiated
    pub timestamp: u64,
    /// Git commit hash before action execution
    pub commit_before: Option<String>,
    /// Git commit hash after action completion
    pub commit_after: Option<String>,
    /// Whether the action completed successfully
    pub success: bool,
    /// Additional metadata specific to the action type
    pub metadata: HashMap<String, String>,
}

/// Types of actions that agents can perform in the workflow
///
/// Each action type represents a different phase of the infinite monkey workflow:
/// - **CommitCreate**: Create new commit with quality analysis
/// - **RevertToPoint**: Restore to previous checkpoint
/// - **SpiralBranch**: Create experimental branch for risky changes
/// - **ConvergenceCheck**: Analyze current convergence status
/// - **QualityValidation**: Run quality checks (lint, tests, etc.)
/// - **CiCdValidation**: Validate through CI/CD pipeline
/// - **InfiniteMonkeyIteration**: Complete iteration of convergence algorithm
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ActionType {
    /// Create a new commit with quality thread analysis
    CommitCreate,
    /// Revert repository state to a previous checkpoint
    RevertToPoint,
    /// Create experimental branch for testing risky changes
    SpiralBranch,
    /// Analyze convergence metrics and trend detection
    ConvergenceCheck,
    /// Execute quality validation (lint, typecheck, tests)
    QualityValidation,
    /// Validate changes through CI/CD pipeline
    CiCdValidation,
    /// Complete iteration of infinite monkey theorem algorithm
    InfiniteMonkeyIteration,
}

/// Multi-agent workflow controller implementing the infinite monkey theorem
///
/// Coordinates multiple AI agents working simultaneously on git workflows:
/// - Isolated session management per agent
/// - Convergence algorithm execution with automatic revert
/// - Quality validation and CI/CD integration
/// - Statistical tracking and recommendation engine
///
/// # Type Parameters
///
/// * `T` - Implementation of GitWorkflow trait (typically GitWorkflowManager)
///
/// # Example
///
/// ```rust,no_run
/// use gdk::agent::AgentWorkflowController;
/// use gdk::core::GitWorkflowManager;
///
/// let workflow = GitWorkflowManager::new("./repo")?;
/// let mut controller = AgentWorkflowController::new(workflow);
///
/// // Start multiple agents
/// controller.start_agent_session("agent-1").await?;
/// controller.start_agent_session("agent-2").await?;
///
/// // Execute convergence workflows in parallel
/// tokio::join!(
///     controller.execute_infinite_monkey_workflow("agent-1", 0.8),
///     controller.execute_infinite_monkey_workflow("agent-2", 0.8)
/// );
/// ```
#[derive(Debug)]
pub struct AgentWorkflowController<T: GitWorkflow> {
    /// Git workflow implementation (typically GitWorkflowManager)
    pub workflow: T,
    /// Active agent sessions indexed by agent_id
    pub active_sessions: HashMap<String, AgentSession>,
    /// Complete history of all agent actions for analysis
    pub action_history: Vec<AgentAction>,
}

impl<T: GitWorkflow> AgentWorkflowController<T> {
    /// Create a new agent workflow controller
    ///
    /// # Arguments
    ///
    /// * `workflow` - Git workflow implementation to manage
    ///
    /// # Returns
    ///
    /// A new controller ready to manage agent sessions
    pub fn new(workflow: T) -> Self {
        Self {
            workflow,
            active_sessions: HashMap::new(),
            action_history: Vec::new(),
        }
    }

    /// Start a new agent session with default configuration
    ///
    /// Creates an isolated session for the specified agent with:
    /// - Unique session ID for tracking
    /// - Empty revert stack for checkpoints
    /// - Default spiral attempt limits (100)
    /// - Convergence history tracking
    ///
    /// # Arguments
    ///
    /// * `agent_id` - Unique identifier for the agent
    ///
    /// # Returns
    ///
    /// Session UUID for tracking this agent's workflow
    ///
    /// # Errors
    ///
    /// Returns error if system time cannot be determined
    pub async fn start_agent_session(&mut self, agent_id: &str) -> GdkResult<Uuid> {
        let session_id = Uuid::new_v4();
        let session = AgentSession {
            session_id,
            agent_id: agent_id.to_string(),
            workflow: "gdk-workflow".to_string(),
            start_time: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
            current_commit: None,
            revert_stack: Vec::new(),
            convergence_history: Vec::new(),
            spiral_attempts: 0,
            max_spiral_attempts: 100,
        };

        self.active_sessions.insert(agent_id.to_string(), session);
        Ok(session_id)
    }

    /// Execute the infinite monkey theorem convergence algorithm
    ///
    /// Implements the core GDK workflow:
    /// 1. Create initial revert point for safe experimentation
    /// 2. Iteratively attempt solutions with quality validation
    /// 3. Revert unsuccessful attempts automatically
    /// 4. Continue until convergence criteria are met
    /// 5. Return final converged commit with quality metrics
    ///
    /// # Arguments
    ///
    /// * `agent_id` - Agent to execute workflow for
    /// * `target_convergence` - Required test pass rate (0.0-1.0)
    ///
    /// # Returns
    ///
    /// Final commit node that achieved convergence
    ///
    /// # Errors
    ///
    /// Returns [`GdkError::ConvergenceError`] if:
    /// - Maximum spiral attempts reached without convergence
    /// - Agent session not found
    /// - Git operations fail during iteration
    pub async fn execute_infinite_monkey_workflow(
        &mut self,
        agent_id: &str,
        target_convergence: f64,
    ) -> GdkResult<CommitNode> {
        let initial_revert_point = self
            .workflow
            .create_revert_point("infinite_monkey_start")
            .await?;

        {
            let session = self.get_session_mut(agent_id)?;
            session.revert_stack.push(initial_revert_point.clone());
        }

        loop {
            // Increment spiral attempts and check limits
            let (spiral_attempts, max_attempts) = {
                let session = self.get_session_mut(agent_id)?;
                session.spiral_attempts += 1;
                (session.spiral_attempts, session.max_spiral_attempts)
            };

            // Prevent infinite loops by enforcing attempt limits
            if spiral_attempts > max_attempts {
                return Err(GdkError::convergence_error(
                    "Maximum spiral attempts reached without convergence",
                    spiral_attempts,
                    0.0, // No final score available
                    target_convergence,
                ));
            }

            let action = self
                .log_action(agent_id, ActionType::InfiniteMonkeyIteration)
                .await?;

            // Create commit with current state and analyze quality
            let commit_node = self
                .workflow
                .create_commit_node(&format!("Infinite monkey attempt {spiral_attempts}"))
                .await?;

            // Update session with new commit
            {
                let session = self.get_session_mut(agent_id)?;
                session.current_commit = Some(commit_node.hash.clone());
            }

            // Analyze convergence metrics for this iteration
            let convergence = self.workflow.analyze_convergence().await?;

            // Store convergence data for trend analysis
            {
                let session = self.get_session_mut(agent_id)?;
                session.convergence_history.push(convergence.clone());
            }

            self.complete_action(&action, true, Some(&commit_node.hash))
                .await?;

            // Check if convergence criteria are met
            if convergence.test_pass_rate >= target_convergence && convergence.is_converged {
                tracing::info!(
                    "Agent {} achieved convergence after {} attempts with score {:.3}",
                    agent_id,
                    spiral_attempts,
                    convergence.test_pass_rate
                );
                return Ok(commit_node);
            }

            // Revert to starting point for next iteration
            tracing::debug!(
                "Agent {} attempt {} failed (score: {:.3}), reverting",
                agent_id,
                spiral_attempts,
                convergence.test_pass_rate
            );
            self.workflow.revert_to_point(&initial_revert_point).await?;
        }
    }

    /// Create a revert checkpoint for experimental changes
    ///
    /// Establishes a safe point that the agent can return to if
    /// experimental changes fail. Useful for:
    /// - Before attempting risky refactoring
    /// - Prior to implementing complex features
    /// - When entering uncertain code paths
    ///
    /// # Arguments
    ///
    /// * `agent_id` - Agent creating the checkpoint
    /// * `reason` - Human-readable reason for the checkpoint
    ///
    /// # Returns
    ///
    /// Revert point that can be used for restoration
    pub async fn create_spiral_checkpoint(
        &mut self,
        agent_id: &str,
        reason: &str,
    ) -> GdkResult<RevertPoint> {
        let action = self.log_action(agent_id, ActionType::RevertToPoint).await?;

        let revert_point = self.workflow.create_revert_point(reason).await?;

        if let Some(session) = self.active_sessions.get_mut(agent_id) {
            session.revert_stack.push(revert_point.clone());
        }

        self.complete_action(&action, true, Some(&revert_point.commit_hash))
            .await?;

        Ok(revert_point)
    }

    /// Revert to the most recent checkpoint for this agent
    ///
    /// Restores repository state to the last revert point created by
    /// this agent. Useful when:
    /// - Experimental changes have failed
    /// - Quality metrics have degraded significantly
    /// - Manual intervention is needed
    ///
    /// # Arguments
    ///
    /// * `agent_id` - Agent to revert
    ///
    /// # Errors
    ///
    /// Returns error if no revert points are available
    pub async fn revert_to_last_checkpoint(&mut self, agent_id: &str) -> GdkResult<()> {
        // Pop the most recent revert point from the stack
        let revert_point = {
            let session = self.get_session_mut(agent_id)?;
            session
                .revert_stack
                .pop()
                .ok_or_else(|| GdkError::validation_error(
                    "No revert points available",
                    "revert_stack",
                    format!("Agent {} has no checkpoints to revert to", agent_id),
                ))?
        };

        let action = self.log_action(agent_id, ActionType::RevertToPoint).await?;

        self.workflow.revert_to_point(&revert_point).await?;

        {
            let session = self.get_session_mut(agent_id)?;
            session.current_commit = Some(revert_point.commit_hash.clone());
        }

        self.complete_action(&action, true, Some(&revert_point.commit_hash))
            .await?;

        Ok(())
    }

    /// Validate current state and create commit if quality standards are met
    ///
    /// Performs comprehensive validation:
    /// 1. Update thread colors based on quality metrics
    /// 2. Create commit with quality analysis
    /// 3. Validate through CI/CD pipeline
    /// 4. Roll back if validation fails
    ///
    /// # Arguments
    ///
    /// * `agent_id` - Agent requesting the commit
    /// * `message` - Git commit message
    ///
    /// # Returns
    ///
    /// Commit node if validation passes
    ///
    /// # Errors
    ///
    /// Returns error if CI/CD validation fails
    pub async fn validate_and_commit(
        &mut self,
        agent_id: &str,
        message: &str,
    ) -> GdkResult<CommitNode> {
        let action = self
            .log_action(agent_id, ActionType::QualityValidation)
            .await?;

        self.workflow.update_thread_colors().await?;

        let commit_node = self.workflow.create_commit_node(message).await?;

        let ci_validation_action = self
            .log_action(agent_id, ActionType::CiCdValidation)
            .await?;
        let ci_success = self.workflow.validate_ci_cd(&commit_node.hash).await?;

        if !ci_success {
            self.complete_action(&ci_validation_action, false, Some(&commit_node.hash))
                .await?;
            self.complete_action(&action, false, Some(&commit_node.hash))
                .await?;
            return Err(anyhow!(
                "CI/CD validation failed for commit {}",
                commit_node.hash
            ));
        }

        if let Some(session) = self.active_sessions.get_mut(agent_id) {
            session.current_commit = Some(commit_node.hash.clone());
        }

        self.complete_action(&ci_validation_action, true, Some(&commit_node.hash))
            .await?;
        self.complete_action(&action, true, Some(&commit_node.hash))
            .await?;

        Ok(commit_node)
    }

    pub async fn get_convergence_status(&mut self, agent_id: &str) -> Result<ConvergenceMetrics> {
        let action = self
            .log_action(agent_id, ActionType::ConvergenceCheck)
            .await?;

        let convergence = self.workflow.analyze_convergence().await?;

        if let Some(session) = self.active_sessions.get_mut(agent_id) {
            session.convergence_history.push(convergence.clone());
        }

        self.complete_action(&action, true, None).await?;

        Ok(convergence)
    }

    pub async fn suggest_next_action(&self, agent_id: &str) -> Result<String> {
        let session = self.get_session(agent_id)?;

        if let Some(latest_convergence) = session.convergence_history.last() {
            if latest_convergence.is_converged {
                return Ok(
                    "CONVERGED: Consider creating a new spiral branch for further exploration"
                        .to_string(),
                );
            }

            if latest_convergence.test_pass_rate < 0.5 {
                return Ok(
                    "REVERT: Test pass rate is low, consider reverting to last checkpoint"
                        .to_string(),
                );
            }

            if session.spiral_attempts > session.max_spiral_attempts / 2 {
                return Ok("CHECKPOINT: Create a revert point before continuing".to_string());
            }
        }

        Ok("CONTINUE: Execute next iteration of infinite monkey workflow".to_string())
    }

    fn get_session(&self, agent_id: &str) -> Result<&AgentSession> {
        self.active_sessions
            .get(agent_id)
            .ok_or_else(|| anyhow!("No active session for agent {}", agent_id))
    }

    fn get_session_mut(&mut self, agent_id: &str) -> Result<&mut AgentSession> {
        self.active_sessions
            .get_mut(agent_id)
            .ok_or_else(|| anyhow!("No active session for agent {}", agent_id))
    }

    async fn log_action(&mut self, agent_id: &str, action_type: ActionType) -> Result<AgentAction> {
        let session = self.get_session(agent_id)?;

        let action = AgentAction {
            action_id: Uuid::new_v4(),
            agent_id: agent_id.to_string(),
            action_type,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
            commit_before: session.current_commit.clone(),
            commit_after: None,
            success: false,
            metadata: HashMap::new(),
        };

        Ok(action)
    }

    async fn complete_action(
        &mut self,
        action: &AgentAction,
        success: bool,
        commit_after: Option<&str>,
    ) -> Result<()> {
        let completed_action = AgentAction {
            action_id: action.action_id,
            agent_id: action.agent_id.clone(),
            action_type: action.action_type.clone(),
            timestamp: action.timestamp,
            commit_before: action.commit_before.clone(),
            commit_after: commit_after.map(|s| s.to_string()),
            success,
            metadata: action.metadata.clone(),
        };

        self.action_history.push(completed_action);
        Ok(())
    }

    pub fn get_agent_statistics(&self, agent_id: &str) -> Result<AgentStatistics> {
        let session = self.get_session(agent_id)?;
        let agent_actions: Vec<_> = self
            .action_history
            .iter()
            .filter(|a| a.agent_id == agent_id)
            .collect();

        let total_actions = agent_actions.len();
        let successful_actions = agent_actions.iter().filter(|a| a.success).count();
        let success_rate = if total_actions > 0 {
            successful_actions as f64 / total_actions as f64
        } else {
            0.0
        };

        let latest_convergence = session
            .convergence_history
            .last()
            .cloned()
            .unwrap_or_else(|| ConvergenceMetrics {
                attempts: 0,
                successful_builds: 0,
                test_pass_rate: 0.0,
                quality_trend: Vec::new(),
                is_converged: false,
            });

        Ok(AgentStatistics {
            agent_id: agent_id.to_string(),
            total_actions,
            success_rate,
            spiral_attempts: session.spiral_attempts,
            convergence_state: latest_convergence,
            revert_points_used: session.revert_stack.len(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStatistics {
    pub agent_id: String,
    pub total_actions: usize,
    pub success_rate: f64,
    pub spiral_attempts: u32,
    pub convergence_state: ConvergenceMetrics,
    pub revert_points_used: usize,
}
