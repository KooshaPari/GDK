use crate::{CommitNode, ConvergenceMetrics, GitWorkflow, RevertPoint};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSession {
    pub session_id: Uuid,
    pub agent_id: String,
    pub workflow: String,
    pub start_time: u64,
    pub current_commit: Option<String>,
    pub revert_stack: Vec<RevertPoint>,
    pub convergence_history: Vec<ConvergenceMetrics>,
    pub spiral_attempts: u32,
    pub max_spiral_attempts: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentAction {
    pub action_id: Uuid,
    pub agent_id: String,
    pub action_type: ActionType,
    pub timestamp: u64,
    pub commit_before: Option<String>,
    pub commit_after: Option<String>,
    pub success: bool,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    CommitCreate,
    RevertToPoint,
    SpiralBranch,
    ConvergenceCheck,
    QualityValidation,
    CiCdValidation,
    InfiniteMonkeyIteration,
}

#[derive(Debug)]
pub struct AgentWorkflowController<T: GitWorkflow> {
    pub workflow: T,
    pub active_sessions: HashMap<String, AgentSession>,
    pub action_history: Vec<AgentAction>,
}

impl<T: GitWorkflow> AgentWorkflowController<T> {
    pub fn new(workflow: T) -> Self {
        Self {
            workflow,
            active_sessions: HashMap::new(),
            action_history: Vec::new(),
        }
    }

    pub async fn start_agent_session(&mut self, agent_id: &str) -> Result<Uuid> {
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

    pub async fn execute_infinite_monkey_workflow(
        &mut self,
        agent_id: &str,
        target_convergence: f64,
    ) -> Result<CommitNode> {
        let initial_revert_point = self
            .workflow
            .create_revert_point("infinite_monkey_start")
            .await?;

        {
            let session = self.get_session_mut(agent_id)?;
            session.revert_stack.push(initial_revert_point.clone());
        }

        loop {
            let (spiral_attempts, max_attempts) = {
                let session = self.get_session_mut(agent_id)?;
                session.spiral_attempts += 1;
                (session.spiral_attempts, session.max_spiral_attempts)
            };

            if spiral_attempts > max_attempts {
                return Err(anyhow!("Max spiral attempts reached without convergence"));
            }

            let action = self
                .log_action(agent_id, ActionType::InfiniteMonkeyIteration)
                .await?;

            let commit_node = self
                .workflow
                .create_commit_node(&format!("Infinite monkey attempt {spiral_attempts}"))
                .await?;

            {
                let session = self.get_session_mut(agent_id)?;
                session.current_commit = Some(commit_node.hash.clone());
            }

            let convergence = self.workflow.analyze_convergence().await?;

            {
                let session = self.get_session_mut(agent_id)?;
                session.convergence_history.push(convergence.clone());
            }

            self.complete_action(&action, true, Some(&commit_node.hash))
                .await?;

            if convergence.test_pass_rate >= target_convergence && convergence.is_converged {
                tracing::info!(
                    "Agent {} achieved convergence after {} attempts",
                    agent_id,
                    spiral_attempts
                );
                return Ok(commit_node);
            }

            self.workflow.revert_to_point(&initial_revert_point).await?;
        }
    }

    pub async fn create_spiral_checkpoint(
        &mut self,
        agent_id: &str,
        reason: &str,
    ) -> Result<RevertPoint> {
        let action = self.log_action(agent_id, ActionType::RevertToPoint).await?;

        let revert_point = self.workflow.create_revert_point(reason).await?;

        if let Some(session) = self.active_sessions.get_mut(agent_id) {
            session.revert_stack.push(revert_point.clone());
        }

        self.complete_action(&action, true, Some(&revert_point.commit_hash))
            .await?;

        Ok(revert_point)
    }

    pub async fn revert_to_last_checkpoint(&mut self, agent_id: &str) -> Result<()> {
        let revert_point = {
            let session = self.get_session_mut(agent_id)?;
            session
                .revert_stack
                .pop()
                .ok_or_else(|| anyhow!("No revert points available for agent {}", agent_id))?
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

    pub async fn validate_and_commit(
        &mut self,
        agent_id: &str,
        message: &str,
    ) -> Result<CommitNode> {
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
