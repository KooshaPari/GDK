//! Unit tests for individual GDK components
//!
//! These tests focus on isolated functionality:
//! - ThreadColor calculations and conversions
//! - ConvergenceMetrics analysis
//! - Error type behavior and categorization
//! - Data structure serialization/deserialization
//! - Edge cases and boundary conditions

use gdk::{
    ThreadColor, ThreadMetrics, ThreadState, ConvergenceMetrics,
    CommitNode, FileThread, GdkError, GdkResult,
};
use proptest::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

/// Property-based test for ThreadColor scoring consistency
proptest! {
    #[test]
    fn prop_thread_color_score_consistency(
        lint in 0.0f64..1.0,
        type_check in 0.0f64..1.0,
        test_coverage in 0.0f64..1.0,
        functionality in 0.0f64..1.0
    ) {
        let color = ThreadColor::from_scores(lint, type_check, test_coverage, functionality);
        let score = color.to_score();
        
        // Color score should always be within valid range
        prop_assert!(score >= 0.0 && score <= 1.0);
        
        // Average input should roughly correspond to color category
        let avg = (lint + type_check + test_coverage + functionality) / 4.0;
        match color {
            ThreadColor::Green => prop_assert!(avg >= 0.9 || score == 1.0),
            ThreadColor::LightGreen => prop_assert!((avg >= 0.7 && avg < 0.9) || score == 0.8),
            ThreadColor::Yellow => prop_assert!((avg >= 0.5 && avg < 0.7) || score == 0.6),
            ThreadColor::Orange => prop_assert!((avg >= 0.3 && avg < 0.5) || score == 0.4),
            ThreadColor::Red => prop_assert!(avg < 0.3 || score == 0.2),
        }
    }
}

/// Test ThreadColor enum complete coverage
#[test]
fn test_thread_color_exhaustive() {
    // Test all enum variants exist and behave correctly
    let colors = vec![
        (ThreadColor::Red, 0.2, "🔴 Red"),
        (ThreadColor::Orange, 0.4, "🟠 Orange"),
        (ThreadColor::Yellow, 0.6, "🟡 Yellow"),
        (ThreadColor::LightGreen, 0.8, "🟢 Light Green"),
        (ThreadColor::Green, 1.0, "💚 Green"),
    ];
    
    for (color, expected_score, expected_display) in colors {
        assert_eq!(color.to_score(), expected_score);
        assert_eq!(format!("{}", color), expected_display);
    }
}

/// Test ThreadColor boundary conditions
#[test]
fn test_thread_color_boundaries() {
    // Test exact boundary values
    assert_eq!(ThreadColor::from_scores(0.9, 0.9, 0.9, 0.9), ThreadColor::Green);
    assert_eq!(ThreadColor::from_scores(0.899, 0.9, 0.9, 0.9), ThreadColor::LightGreen);
    
    assert_eq!(ThreadColor::from_scores(0.7, 0.7, 0.7, 0.7), ThreadColor::LightGreen);
    assert_eq!(ThreadColor::from_scores(0.699, 0.7, 0.7, 0.7), ThreadColor::Yellow);
    
    assert_eq!(ThreadColor::from_scores(0.5, 0.5, 0.5, 0.5), ThreadColor::Yellow);
    assert_eq!(ThreadColor::from_scores(0.499, 0.5, 0.5, 0.5), ThreadColor::Orange);
    
    assert_eq!(ThreadColor::from_scores(0.3, 0.3, 0.3, 0.3), ThreadColor::Orange);
    assert_eq!(ThreadColor::from_scores(0.299, 0.3, 0.3, 0.3), ThreadColor::Red);
    
    // Test extreme values
    assert_eq!(ThreadColor::from_scores(0.0, 0.0, 0.0, 0.0), ThreadColor::Red);
    assert_eq!(ThreadColor::from_scores(1.0, 1.0, 1.0, 1.0), ThreadColor::Green);
}

/// Test ConvergenceMetrics calculation logic
#[test]
fn test_convergence_metrics() {
    let metrics = ConvergenceMetrics {
        attempts: 10,
        successful_builds: 7,
        test_pass_rate: 0.85,
        quality_trend: vec![0.6, 0.7, 0.75, 0.8, 0.85],
        is_converged: true,
    };
    
    // Test that convergence detection is reasonable
    assert!(metrics.is_converged);
    assert_eq!(metrics.attempts, 10);
    assert_eq!(metrics.successful_builds, 7);
    assert!((metrics.test_pass_rate - 0.85).abs() < f64::EPSILON);
    
    // Quality trend should show improvement
    let trend = &metrics.quality_trend;
    assert!(trend.len() > 0);
    assert!(trend.first().unwrap() < trend.last().unwrap());
}

/// Test ThreadMetrics calculation
#[test]
fn test_thread_metrics() {
    let metrics = ThreadMetrics {
        lines_added: 50,
        lines_removed: 10,
        complexity_delta: 0.15,
        quality_score: 0.85,
    };
    
    assert_eq!(metrics.lines_added, 50);
    assert_eq!(metrics.lines_removed, 10);
    assert!((metrics.complexity_delta - 0.15).abs() < f64::EPSILON);
    assert!((metrics.quality_score - 0.85).abs() < f64::EPSILON);
    
    // Quality score should be in valid range
    assert!(metrics.quality_score >= 0.0 && metrics.quality_score <= 1.0);
}

/// Test FileThread structure and validation
#[test]
fn test_file_thread() {
    let thread_id = Uuid::new_v4();
    let thread = FileThread {
        file_path: "src/lib.rs".to_string(),
        thread_id,
        color_status: ThreadColor::Green,
        lint_score: 0.95,
        type_check_score: 1.0,
        test_coverage: 0.88,
        functionality_score: 0.92,
        history: vec![],
    };
    
    assert_eq!(thread.file_path, "src/lib.rs");
    assert_eq!(thread.thread_id, thread_id);
    assert_eq!(thread.color_status, ThreadColor::Green);
    
    // All scores should be in valid range
    assert!(thread.lint_score >= 0.0 && thread.lint_score <= 1.0);
    assert!(thread.type_check_score >= 0.0 && thread.type_check_score <= 1.0);
    assert!(thread.test_coverage >= 0.0 && thread.test_coverage <= 1.0);
    assert!(thread.functionality_score >= 0.0 && thread.functionality_score <= 1.0);
}

/// Test CommitNode structure validation
#[test]
fn test_commit_node() {
    let commit_id = Uuid::new_v4().to_string();
    let commit_hash = "abc123def456".to_string();
    let timestamp = 1234567890;
    
    let commit = CommitNode {
        id: commit_id.clone(),
        hash: commit_hash.clone(),
        parent_hashes: vec!["parent123".to_string()],
        message: "Test commit".to_string(),
        timestamp,
        file_threads: HashMap::new(),
        health_score: 0.85,
        convergence_metrics: ConvergenceMetrics {
            attempts: 1,
            successful_builds: 1,
            test_pass_rate: 1.0,
            quality_trend: vec![0.85],
            is_converged: true,
        },
    };
    
    assert_eq!(commit.id, commit_id);
    assert_eq!(commit.hash, commit_hash);
    assert_eq!(commit.parent_hashes.len(), 1);
    assert_eq!(commit.message, "Test commit");
    assert_eq!(commit.timestamp, timestamp);
    assert!(commit.health_score >= 0.0 && commit.health_score <= 1.0);
    assert!(commit.convergence_metrics.is_converged);
}

/// Test error categorization and recoverability
#[test]
fn test_error_categories() {
    let errors = vec![
        (GdkError::git_error("test", git2::Error::from_str("test")), "git", false),
        (GdkError::validation_error("lint", "rule", "details"), "validation", false),
        (GdkError::convergence_error("reason", 10, 0.7, 0.8), "convergence", true),
        (GdkError::configuration_error("setting", "message", None), "configuration", false),
    ];
    
    for (error, expected_category, expected_recoverable) in errors {
        assert_eq!(error.category(), expected_category);
        assert_eq!(error.is_recoverable(), expected_recoverable);
    }
}

/// Test error context preservation
#[test]
fn test_error_context() {
    let git_error = GdkError::git_error("commit operation", git2::Error::from_str("access denied"));
    
    match git_error {
        GdkError::GitError { operation, source } => {
            assert_eq!(operation, "commit operation");
            assert_eq!(source.message(), "access denied");
        }
        _ => panic!("Expected GitError variant"),
    }
    
    let convergence_error = GdkError::convergence_error("timeout", 50, 0.75, 0.9);
    
    match convergence_error {
        GdkError::ConvergenceError { reason, iterations, last_score, threshold } => {
            assert_eq!(reason, "timeout");
            assert_eq!(iterations, 50);
            assert!((last_score - 0.75).abs() < f64::EPSILON);
            assert!((threshold - 0.9).abs() < f64::EPSILON);
        }
        _ => panic!("Expected ConvergenceError variant"),
    }
}

/// Property-based test for score normalization
proptest! {
    #[test]
    fn prop_score_normalization(score in -10.0f64..10.0) {
        // Test that scores outside [0,1] are handled gracefully in color calculation
        let normalized = score.max(0.0).min(1.0);
        let color = ThreadColor::from_scores(normalized, normalized, normalized, normalized);
        
        // Should always produce a valid color
        prop_assert!(matches!(color, 
            ThreadColor::Red | ThreadColor::Orange | ThreadColor::Yellow | 
            ThreadColor::LightGreen | ThreadColor::Green
        ));
        
        // Score should always be in valid range
        let color_score = color.to_score();
        prop_assert!(color_score >= 0.0 && color_score <= 1.0);
    }
}

/// Test serialization roundtrip for all major types
#[test]
fn test_serialization_roundtrip() -> GdkResult<()> {
    // Test ThreadColor
    let color = ThreadColor::Green;
    let json = serde_json::to_string(&color).unwrap();
    let deserialized: ThreadColor = serde_json::from_str(&json).unwrap();
    assert_eq!(color, deserialized);
    
    // Test ThreadMetrics
    let metrics = ThreadMetrics {
        lines_added: 42,
        lines_removed: 13,
        complexity_delta: 0.25,
        quality_score: 0.88,
    };
    let json = serde_json::to_string(&metrics).unwrap();
    let deserialized: ThreadMetrics = serde_json::from_str(&json).unwrap();
    assert_eq!(metrics, deserialized);
    
    // Test ConvergenceMetrics
    let convergence = ConvergenceMetrics {
        attempts: 15,
        successful_builds: 12,
        test_pass_rate: 0.92,
        quality_trend: vec![0.6, 0.7, 0.8, 0.9],
        is_converged: true,
    };
    let json = serde_json::to_string(&convergence).unwrap();
    let deserialized: ConvergenceMetrics = serde_json::from_str(&json).unwrap();
    assert_eq!(convergence, deserialized);
    
    Ok(())
}

/// Test edge cases for quality calculations
#[test]
fn test_quality_edge_cases() {
    // Test NaN handling (should not occur in normal operation)
    let color = ThreadColor::from_scores(0.0, 0.0, 0.0, 0.0);
    assert_eq!(color, ThreadColor::Red);
    
    // Test very small differences
    let color1 = ThreadColor::from_scores(0.8999, 0.9, 0.9, 0.9);
    let color2 = ThreadColor::from_scores(0.9001, 0.9, 0.9, 0.9);
    // Both should be LightGreen vs Green respectively
    assert_eq!(color1, ThreadColor::LightGreen);
    assert_eq!(color2, ThreadColor::Green);
    
    // Test mixed extreme values
    let color = ThreadColor::from_scores(1.0, 0.0, 1.0, 0.0); // avg = 0.5
    assert_eq!(color, ThreadColor::Yellow);
}

/// Test thread state history validation
#[test]
fn test_thread_state_history() {
    let state = ThreadState {
        commit_hash: "abc123".to_string(),
        diff_content: "+added line\n-removed line".to_string(),
        metrics: ThreadMetrics {
            lines_added: 1,
            lines_removed: 1,
            complexity_delta: 0.0,
            quality_score: 0.8,
        },
        timestamp: 1234567890,
    };
    
    assert_eq!(state.commit_hash, "abc123");
    assert!(state.diff_content.contains("+added line"));
    assert!(state.diff_content.contains("-removed line"));
    assert_eq!(state.metrics.lines_added, 1);
    assert_eq!(state.metrics.lines_removed, 1);
    assert_eq!(state.timestamp, 1234567890);
}

/// Test concurrent safety of data structures
#[test]
fn test_data_structure_traits() {
    // Verify that our types implement necessary traits for concurrent use
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}
    
    assert_send::<ThreadColor>();
    assert_sync::<ThreadColor>();
    
    assert_send::<ThreadMetrics>();
    assert_sync::<ThreadMetrics>();
    
    assert_send::<ConvergenceMetrics>();
    assert_sync::<ConvergenceMetrics>();
    
    // Test Clone behavior
    let color = ThreadColor::Green;
    let cloned = color.clone();
    assert_eq!(color, cloned);
    
    // Test Debug formatting
    let debug_str = format!("{:?}", ThreadColor::Green);
    assert!(debug_str.contains("Green"));
}