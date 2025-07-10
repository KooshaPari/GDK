//! Integration tests for the GDK system
//!
//! These tests verify the complete workflow functionality:
//! - End-to-end git workflow operations
//! - Multi-agent session management
//! - Quality convergence algorithms
//! - Visualization generation
//! - Error handling and recovery

use gdk::core::GitWorkflowManager;
use gdk::{CommitNode, ThreadColor, GdkResult, GitWorkflow};
use std::fs;
use tempfile::TempDir;
use tokio::test;

/// Helper function to create a temporary git repository for testing
async fn setup_test_repo() -> GdkResult<(TempDir, GitWorkflowManager)> {
    let temp_dir = TempDir::new().map_err(|e| {
        gdk::GdkError::file_system_error("temp", "Failed to create temp directory", e)
    })?;
    
    let repo_path = temp_dir.path().to_str().unwrap();
    let manager = GitWorkflowManager::new(repo_path)?;
    
    // Create initial test file
    let test_file = temp_dir.path().join("src").join("lib.rs");
    fs::create_dir_all(test_file.parent().unwrap()).map_err(|e| {
        gdk::GdkError::file_system_error("src", "Failed to create src directory", e)
    })?;
    
    fs::write(&test_file, r#"
//! Test library for GDK integration tests

/// A simple function to test
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
"#).map_err(|e| {
        gdk::GdkError::file_system_error(test_file.to_string_lossy(), "Failed to write test file", e)
    })?;

    // Create Cargo.toml
    let cargo_toml = temp_dir.path().join("Cargo.toml");
    fs::write(&cargo_toml, r#"
[package]
name = "test-project"
version = "0.1.0"
edition = "2021"
"#).map_err(|e| {
        gdk::GdkError::file_system_error(cargo_toml.to_string_lossy(), "Failed to write Cargo.toml", e)
    })?;

    Ok((temp_dir, manager))
}

#[test]
async fn test_basic_workflow_creation() -> GdkResult<()> {
    let (_temp_dir, mut manager) = setup_test_repo().await?;
    
    // Test that we can create a basic workflow manager
    assert_eq!(manager.commit_history.len(), 0);
    assert_eq!(manager.revert_points.len(), 0);
    assert!(manager.current_branch.contains("main") || manager.current_branch.contains("master"));
    
    Ok(())
}

#[test]
async fn test_commit_node_creation() -> GdkResult<()> {
    let (_temp_dir, mut manager) = setup_test_repo().await?;
    
    // Create initial commit
    let commit = manager.create_commit_node("Initial test commit").await?;
    
    // Verify commit structure
    assert!(!commit.id.is_empty());
    assert!(!commit.hash.is_empty());
    assert_eq!(commit.message, "Initial test commit");
    assert!(commit.timestamp > 0);
    assert!(commit.health_score >= 0.0 && commit.health_score <= 1.0);
    
    // Check that commit was added to history
    assert_eq!(manager.commit_history.len(), 1);
    
    Ok(())
}

#[test]
async fn test_revert_point_creation() -> GdkResult<()> {
    let (_temp_dir, mut manager) = setup_test_repo().await?;
    
    // Create a commit first
    let _commit = manager.create_commit_node("Base commit").await?;
    
    // Create revert point
    let revert_point = manager.create_revert_point("Test checkpoint").await?;
    
    // Verify revert point structure
    assert!(!revert_point.commit_hash.is_empty());
    assert_eq!(revert_point.branch_name, manager.current_branch);
    assert_eq!(revert_point.metadata.reason, "Test checkpoint");
    assert_eq!(revert_point.metadata.agent_id, "gdk-system");
    
    Ok(())
}

#[test]
async fn test_thread_color_scoring() {
    // Test thread color calculation from scores
    assert_eq!(ThreadColor::from_scores(1.0, 1.0, 1.0, 1.0), ThreadColor::Green);
    assert_eq!(ThreadColor::from_scores(0.8, 0.8, 0.8, 0.8), ThreadColor::LightGreen);
    assert_eq!(ThreadColor::from_scores(0.6, 0.6, 0.6, 0.6), ThreadColor::Yellow);
    assert_eq!(ThreadColor::from_scores(0.4, 0.4, 0.4, 0.4), ThreadColor::Orange);
    assert_eq!(ThreadColor::from_scores(0.1, 0.1, 0.1, 0.1), ThreadColor::Red);
    
    // Test mixed scores
    assert_eq!(ThreadColor::from_scores(1.0, 0.8, 0.6, 0.4), ThreadColor::LightGreen); // avg = 0.7
    assert_eq!(ThreadColor::from_scores(0.9, 0.9, 0.9, 1.0), ThreadColor::Green); // avg = 0.925
}

#[test]
async fn test_convergence_analysis() -> GdkResult<()> {
    let (_temp_dir, mut manager) = setup_test_repo().await?;
    
    // Create multiple commits to establish a trend
    for i in 1..=5 {
        let _commit = manager.create_commit_node(&format!("Commit {}", i)).await?;
    }
    
    // Analyze convergence
    let convergence = manager.analyze_convergence().await?;
    
    // Verify convergence metrics
    assert_eq!(convergence.attempts, 5);
    assert!(convergence.successful_builds <= 5);
    assert!(convergence.test_pass_rate >= 0.0 && convergence.test_pass_rate <= 1.0);
    assert_eq!(convergence.quality_trend.len(), 5);
    
    Ok(())
}

#[test]
async fn test_spiral_branching() -> GdkResult<()> {
    let (_temp_dir, mut manager) = setup_test_repo().await?;
    
    // Create base commit
    let base_commit = manager.create_commit_node("Base for spiral").await?;
    
    // Create spiral branch
    let spiral_branch = manager.create_spiral_branch(&base_commit.hash).await?;
    
    // Verify spiral branch properties
    assert!(spiral_branch.starts_with("spiral-"));
    assert_eq!(manager.current_branch, spiral_branch);
    
    Ok(())
}

#[test]
async fn test_infinite_monkey_iteration() -> GdkResult<()> {
    let (_temp_dir, mut manager) = setup_test_repo().await?;
    
    // Test with small number of iterations to avoid long test runs
    let result = manager.infinite_monkey_iteration(3).await;
    
    // Should either converge or reach max attempts
    match result {
        Ok(commit) => {
            // Successful convergence
            assert!(commit.convergence_metrics.is_converged);
            assert!(commit.health_score > 0.8);
        }
        Err(gdk::GdkError::ConvergenceError { iterations, .. }) => {
            // Reached max attempts without convergence
            assert_eq!(iterations, 3);
        }
        Err(e) => {
            panic!("Unexpected error type: {:?}", e);
        }
    }
    
    Ok(())
}

#[test]
async fn test_error_handling() -> GdkResult<()> {
    // Test error creation and categorization
    let git_error = gdk::GdkError::git_error(
        "test operation",
        git2::Error::from_str("test error")
    );
    assert_eq!(git_error.category(), "git");
    assert!(!git_error.is_recoverable()); // Generic git errors not recoverable
    
    let validation_error = gdk::GdkError::validation_error(
        "lint",
        "style_check",
        "Missing documentation"
    );
    assert_eq!(validation_error.category(), "validation");
    assert!(!validation_error.is_recoverable()); // Code issues need fixing
    
    let convergence_error = gdk::GdkError::convergence_error(
        "Failed to meet threshold",
        10,
        0.7,
        0.8
    );
    assert_eq!(convergence_error.category(), "convergence");
    assert!(convergence_error.is_recoverable()); // Can retry convergence
    
    Ok(())
}

#[test]
async fn test_file_thread_creation() -> GdkResult<()> {
    let (_temp_dir, mut manager) = setup_test_repo().await?;
    
    // Create commit with file changes
    let commit = manager.create_commit_node("Test file threads").await?;
    
    // Verify file threads were created
    assert!(!commit.file_threads.is_empty());
    
    for (file_path, thread) in &commit.file_threads {
        assert!(!file_path.is_empty());
        assert!(!thread.thread_id.to_string().is_empty());
        assert!(thread.lint_score >= 0.0 && thread.lint_score <= 1.0);
        assert!(thread.type_check_score >= 0.0 && thread.type_check_score <= 1.0);
        assert!(thread.test_coverage >= 0.0 && thread.test_coverage <= 1.0);
        assert!(thread.functionality_score >= 0.0 && thread.functionality_score <= 1.0);
        assert!(!thread.history.is_empty());
    }
    
    Ok(())
}

#[test]
async fn test_thread_color_display() {
    // Test Display implementation for ThreadColor
    assert_eq!(format!("{}", ThreadColor::Red), "🔴 Red");
    assert_eq!(format!("{}", ThreadColor::Orange), "🟠 Orange");
    assert_eq!(format!("{}", ThreadColor::Yellow), "🟡 Yellow");
    assert_eq!(format!("{}", ThreadColor::LightGreen), "🟢 Light Green");
    assert_eq!(format!("{}", ThreadColor::Green), "💚 Green");
}

#[test]
async fn test_quality_metrics_bounds() {
    // Test that all quality scores are properly bounded
    let colors = [
        ThreadColor::Red,
        ThreadColor::Orange,
        ThreadColor::Yellow,
        ThreadColor::LightGreen,
        ThreadColor::Green,
    ];
    
    for color in &colors {
        let score = color.to_score();
        assert!(score >= 0.0 && score <= 1.0, "Score {} out of bounds for {:?}", score, color);
    }
}

#[test]
async fn test_serialization() -> GdkResult<()> {
    let (_temp_dir, mut manager) = setup_test_repo().await?;
    
    // Create a commit with full data
    let commit = manager.create_commit_node("Serialization test").await?;
    
    // Test JSON serialization
    let json = serde_json::to_string(&commit).map_err(|e| {
        gdk::GdkError::SerializationError {
            format: "JSON".to_string(),
            context: "commit serialization".to_string(),
            source: e,
        }
    })?;
    
    // Test JSON deserialization
    let deserialized: CommitNode = serde_json::from_str(&json).map_err(|e| {
        gdk::GdkError::SerializationError {
            format: "JSON".to_string(),
            context: "commit deserialization".to_string(),
            source: e,
        }
    })?;
    
    // Verify round-trip integrity
    assert_eq!(commit.id, deserialized.id);
    assert_eq!(commit.hash, deserialized.hash);
    assert_eq!(commit.message, deserialized.message);
    assert_eq!(commit.health_score, deserialized.health_score);
    
    Ok(())
}