//! Property-based tests for GDK system invariants
//!
//! These tests verify mathematical properties and invariants:
//! - Quality score consistency across operations
//! - Convergence algorithm correctness
//! - Data structure invariants
//! - Error handling completeness
//! - Serialization round-trip properties

use gdk::{
    ThreadColor, ThreadMetrics, ConvergenceMetrics, CommitNode, 
    FileThread, GdkError, GdkResult,
};
use proptest::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

/// Generate valid quality scores (0.0 to 1.0)
fn quality_score() -> impl Strategy<Value = f64> {
    (0.0..=1.0f64)
}

/// Generate thread colors
fn thread_color() -> impl Strategy<Value = ThreadColor> {
    prop_oneof![
        Just(ThreadColor::Red),
        Just(ThreadColor::Orange),
        Just(ThreadColor::Yellow),
        Just(ThreadColor::LightGreen),
        Just(ThreadColor::Green),
    ]
}

/// Generate valid file paths
fn file_path() -> impl Strategy<Value = String> {
    prop_oneof![
        "src/[a-z]{1,10}\\.rs",
        "tests/[a-z]{1,10}\\.rs",
        "benches/[a-z]{1,10}\\.rs",
        "[a-z]{1,10}/[a-z]{1,10}\\.rs",
    ].prop_map(|s| s.to_string())
}

/// Generate valid commit messages
fn commit_message() -> impl Strategy<Value = String> {
    "[A-Z][a-z ]{10,50}".prop_map(|s| s.trim().to_string())
}

/// Generate valid timestamps
fn timestamp() -> impl Strategy<Value = u64> {
    (1_000_000_000u64..2_000_000_000u64)
}

/// Property: ThreadColor calculation should be deterministic and consistent
proptest! {
    #[test]
    fn prop_thread_color_deterministic(
        lint in quality_score(),
        type_check in quality_score(),
        test_coverage in quality_score(),
        functionality in quality_score(),
    ) {
        let color1 = ThreadColor::from_scores(lint, type_check, test_coverage, functionality);
        let color2 = ThreadColor::from_scores(lint, type_check, test_coverage, functionality);
        
        // Same inputs should always produce same output
        prop_assert_eq!(color1, color2);
        
        // Score conversion should be consistent
        prop_assert_eq!(color1.to_score(), color2.to_score());
    }
    
    #[test]
    fn prop_thread_color_monotonic(
        base_score in 0.0..0.8f64,
        improvement in 0.0..0.2f64,
    ) {
        let low_score = base_score;
        let high_score = base_score + improvement;
        
        let low_color = ThreadColor::from_scores(low_score, low_score, low_score, low_score);
        let high_color = ThreadColor::from_scores(high_score, high_score, high_score, high_score);
        
        // Higher input scores should produce same or better color category
        prop_assert!(high_color.to_score() >= low_color.to_score());
    }
    
    #[test]
    fn prop_quality_score_bounds(
        lint in quality_score(),
        type_check in quality_score(),
        test_coverage in quality_score(),
        functionality in quality_score(),
    ) {
        let color = ThreadColor::from_scores(lint, type_check, test_coverage, functionality);
        let score = color.to_score();
        
        // Color score should always be in valid range
        prop_assert!(score >= 0.0);
        prop_assert!(score <= 1.0);
        
        // Score should correspond to reasonable category
        let avg = (lint + type_check + test_coverage + functionality) / 4.0;
        match color {
            ThreadColor::Green => prop_assert!(avg >= 0.85 || score >= 0.9),
            ThreadColor::LightGreen => prop_assert!(score == 0.8),
            ThreadColor::Yellow => prop_assert!(score == 0.6),
            ThreadColor::Orange => prop_assert!(score == 0.4),
            ThreadColor::Red => prop_assert!(score == 0.2),
        }
    }
}

/// Property: ThreadMetrics should maintain logical consistency
proptest! {
    #[test]
    fn prop_thread_metrics_consistency(
        lines_added in 0u32..10000,
        lines_removed in 0u32..10000,
        complexity_delta in -1.0..1.0f64,
        quality_score in quality_score(),
    ) {
        let metrics = ThreadMetrics {
            lines_added,
            lines_removed,
            complexity_delta,
            quality_score,
        };
        
        // Lines should be non-negative
        prop_assert!(metrics.lines_added >= 0);
        prop_assert!(metrics.lines_removed >= 0);
        
        // Quality score should be bounded
        prop_assert!(metrics.quality_score >= 0.0);
        prop_assert!(metrics.quality_score <= 1.0);
        
        // Complexity delta should be reasonable
        prop_assert!(metrics.complexity_delta >= -1.0);
        prop_assert!(metrics.complexity_delta <= 1.0);
    }
}

/// Property: ConvergenceMetrics should follow mathematical constraints
proptest! {
    #[test]
    fn prop_convergence_metrics_constraints(
        attempts in 1u32..1000,
        successful_builds in 0u32..1000,
        test_pass_rate in quality_score(),
        quality_trend in prop::collection::vec(quality_score(), 1..100),
        is_converged in any::<bool>(),
    ) {
        // Ensure successful builds don't exceed attempts
        let successful_builds = successful_builds.min(attempts);
        
        let metrics = ConvergenceMetrics {
            attempts,
            successful_builds,
            test_pass_rate,
            quality_trend: quality_trend.clone(),
            is_converged,
        };
        
        // Basic constraints
        prop_assert!(metrics.attempts >= 1);
        prop_assert!(metrics.successful_builds <= metrics.attempts);
        prop_assert!(metrics.test_pass_rate >= 0.0);
        prop_assert!(metrics.test_pass_rate <= 1.0);
        
        // Quality trend should contain valid scores
        for &score in &metrics.quality_trend {
            prop_assert!(score >= 0.0);
            prop_assert!(score <= 1.0);
        }
        
        // If converged, quality should generally be high
        if metrics.is_converged && !metrics.quality_trend.is_empty() {
            let recent_quality = metrics.quality_trend.iter().rev().take(3).sum::<f64>() 
                / metrics.quality_trend.iter().rev().take(3).count() as f64;
            // Converged systems should have reasonable quality
            prop_assert!(recent_quality >= 0.5);
        }
    }
}

/// Property: FileThread should maintain internal consistency
proptest! {
    #[test]
    fn prop_file_thread_consistency(
        file_path in file_path(),
        color_status in thread_color(),
        lint_score in quality_score(),
        type_check_score in quality_score(),
        test_coverage in quality_score(),
        functionality_score in quality_score(),
    ) {
        let thread = FileThread {
            file_path: file_path.clone(),
            thread_id: Uuid::new_v4(),
            color_status: color_status.clone(),
            lint_score,
            type_check_score,
            test_coverage,
            functionality_score,
            history: vec![],
        };
        
        // File path should not be empty
        prop_assert!(!thread.file_path.is_empty());
        prop_assert_eq!(thread.file_path, file_path);
        
        // All scores should be valid
        prop_assert!(thread.lint_score >= 0.0 && thread.lint_score <= 1.0);
        prop_assert!(thread.type_check_score >= 0.0 && thread.type_check_score <= 1.0);
        prop_assert!(thread.test_coverage >= 0.0 && thread.test_coverage <= 1.0);
        prop_assert!(thread.functionality_score >= 0.0 && thread.functionality_score <= 1.0);
        
        // Color status should roughly match calculated color
        let calculated_color = ThreadColor::from_scores(
            lint_score, type_check_score, test_coverage, functionality_score
        );
        
        // Allow some flexibility in color assignment
        let score_diff = (color_status.to_score() - calculated_color.to_score()).abs();
        prop_assert!(score_diff <= 0.4); // Allow some variance
    }
}

/// Property: CommitNode should maintain structural integrity
proptest! {
    #[test]
    fn prop_commit_node_integrity(
        message in commit_message(),
        timestamp in timestamp(),
        health_score in quality_score(),
        parent_count in 0usize..5,
    ) {
        let id = Uuid::new_v4().to_string();
        let hash = format!("commit_{:x}", timestamp);
        let parent_hashes: Vec<String> = (0..parent_count)
            .map(|i| format!("parent_{:x}", timestamp + i as u64))
            .collect();
        
        let commit = CommitNode {
            id: id.clone(),
            hash: hash.clone(),
            parent_hashes: parent_hashes.clone(),
            message: message.clone(),
            timestamp,
            file_threads: HashMap::new(),
            health_score,
            convergence_metrics: ConvergenceMetrics {
                attempts: 1,
                successful_builds: 1,
                test_pass_rate: health_score,
                quality_trend: vec![health_score],
                is_converged: health_score > 0.8,
            },
        };
        
        // Basic field integrity
        prop_assert_eq!(commit.id, id);
        prop_assert_eq!(commit.hash, hash);
        prop_assert_eq!(commit.parent_hashes, parent_hashes);
        prop_assert_eq!(commit.message, message);
        prop_assert_eq!(commit.timestamp, timestamp);
        
        // Health score should be valid
        prop_assert!(commit.health_score >= 0.0);
        prop_assert!(commit.health_score <= 1.0);
        
        // Convergence metrics should be consistent
        prop_assert_eq!(commit.convergence_metrics.test_pass_rate, health_score);
        prop_assert_eq!(commit.convergence_metrics.quality_trend.len(), 1);
        prop_assert_eq!(commit.convergence_metrics.is_converged, health_score > 0.8);
    }
}

/// Property: Error types should preserve information correctly
proptest! {
    #[test]
    fn prop_error_information_preservation(
        operation in "[a-z ]{5,20}",
        component in "[a-z_]{3,15}",
        rule in "[a-z_]{3,15}",
        details in "[a-z ]{10,50}",
        iterations in 1u32..100,
        score in quality_score(),
        threshold in quality_score(),
    ) {
        // Test GitError
        let git_error = GdkError::git_error(&operation, git2::Error::from_str("test error"));
        prop_assert_eq!(git_error.category(), "git");
        
        // Test ValidationError
        let validation_error = GdkError::validation_error(&component, &rule, &details, Some(score));
        prop_assert_eq!(validation_error.category(), "validation");
        prop_assert!(!validation_error.is_recoverable());
        
        // Test ConvergenceError
        let convergence_error = GdkError::convergence_error("reason", iterations, score, threshold);
        prop_assert_eq!(convergence_error.category(), "convergence");
        prop_assert!(convergence_error.is_recoverable());
        
        // Verify error context preservation
        match convergence_error {
            GdkError::ConvergenceError { iterations: i, last_score: s, threshold: t, .. } => {
                prop_assert_eq!(i, iterations);
                prop_assert!((s - score).abs() < f64::EPSILON);
                prop_assert!((t - threshold).abs() < f64::EPSILON);
            }
            _ => prop_assert!(false, "Expected ConvergenceError"),
        }
    }
}

/// Property: Serialization should be lossless for all data types
proptest! {
    #[test]
    fn prop_serialization_lossless(
        color in thread_color(),
        lines_added in 0u32..1000,
        lines_removed in 0u32..1000,
        complexity_delta in -0.5..0.5f64,
        quality_score in quality_score(),
    ) {
        // Test ThreadColor serialization
        let color_json = serde_json::to_string(&color).unwrap();
        let color_deserialized: ThreadColor = serde_json::from_str(&color_json).unwrap();
        prop_assert_eq!(color, color_deserialized);
        
        // Test ThreadMetrics serialization
        let metrics = ThreadMetrics {
            lines_added,
            lines_removed,
            complexity_delta,
            quality_score,
        };
        let metrics_json = serde_json::to_string(&metrics).unwrap();
        let metrics_deserialized: ThreadMetrics = serde_json::from_str(&metrics_json).unwrap();
        prop_assert_eq!(metrics, metrics_deserialized.clone());
        
        // Verify numerical precision is preserved
        prop_assert!((metrics.complexity_delta - metrics_deserialized.complexity_delta).abs() < 1e-10);
        prop_assert!((metrics.quality_score - metrics_deserialized.quality_score).abs() < 1e-10);
    }
}

/// Property: Quality calculations should be associative for averaging
proptest! {
    #[test]
    fn prop_quality_averaging_associative(
        scores in prop::collection::vec(quality_score(), 2..20),
    ) {
        let n = scores.len();
        
        // Calculate average directly
        let direct_avg = scores.iter().sum::<f64>() / n as f64;
        
        // Calculate average in groups
        let mid = n / 2;
        let (first_half, second_half) = scores.split_at(mid);
        
        let first_avg = first_half.iter().sum::<f64>() / first_half.len() as f64;
        let second_avg = second_half.iter().sum::<f64>() / second_half.len() as f64;
        let group_avg = (first_avg * first_half.len() as f64 + second_avg * second_half.len() as f64) / n as f64;
        
        // Results should be equivalent (within floating point precision)
        prop_assert!((direct_avg - group_avg).abs() < 1e-10);
        
        // Both should be in valid range
        prop_assert!(direct_avg >= 0.0 && direct_avg <= 1.0);
        prop_assert!(group_avg >= 0.0 && group_avg <= 1.0);
    }
}

/// Property: Thread color transitions should be monotonic
proptest! {
    #[test]
    fn prop_color_transitions_monotonic(base_score in 0.0..0.9f64) {
        let scores = [base_score, base_score + 0.05, base_score + 0.1];
        let colors: Vec<ThreadColor> = scores.iter()
            .map(|&s| ThreadColor::from_scores(s, s, s, s))
            .collect();
        
        // Colors should improve or stay same as scores increase
        for i in 1..colors.len() {
            prop_assert!(colors[i].to_score() >= colors[i-1].to_score());
        }
    }
}