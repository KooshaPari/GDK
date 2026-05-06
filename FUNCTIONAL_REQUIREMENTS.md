# Functional Requirements

## Overview

GDK (Git Workflow Deep Knowledge) is an enterprise git workflow system for AI agents featuring thread-based quality tracking, infinite monkey theorem convergence, spiral branching with intelligent checkpoints, multi-format tree visualization, and multi-agent session management.

## Requirements

| ID | Title | Description | Priority | Status |
|----|-------|-------------|----------|--------|
| FR-001 | Thread-Based Quality Tracking | Per-file quality tracking across lint, typecheck, test coverage, and functionality dimensions with color-coded status (Red to Green). ThreadManager maintains active threads per file, calculates weighted scores, tracks color transitions over history, and produces color distribution statistics. | High | Implemented |
| FR-002 | Convergence Analysis | Mathematical analysis engine that detects when a workflow has achieved stable, high-quality output. Uses five weighted factors: quality stability (30%), thread health ratio (25%), test pass consistency (20%), build success rate (15%), and trend improvement (10%). Configurable thresholds and variance analysis for convergence detection and prediction. | High | Implemented |
| FR-003 | Agent Session Management | Multi-agent workflow controller that tracks isolated agent sessions with distinct state. Maintains session ID, commit tracking, revert point stacks, convergence history, and spiral attempt counters. Logs all agent actions with type, timestamp, before/after commits, and success status. Provides statistical analysis including success rates and convergence state. | High | Implemented |
| FR-004 | Infinite Monkey Convergence Workflow | Iterative workflow engine that executes the infinite monkey theorem for AI agents. Attempts solution approaches, evaluates quality against a configurable threshold (default 0.8), automatically reverts to the last checkpoint on failure, and converges when all quality threads meet thresholds or max attempts are reached. Spiral attempt tracking with configurable limits per session. | High | Implemented |
| FR-005 | Spiral Branching with Checkpoints | Checkpoint creation that captures named revert points with full file state snapshots and metadata. Branch creation for experimental changes. Auto-revert on quality failure back to the last checkpoint. Supports merge on successful convergence. RevertPoint captures commit hash, branch name, file states, dependencies, and convergence state metadata. | High | Implemented |
| FR-006 | Tree Visualization | Multi-format commit tree rendering with ASCII (Simple, Unicode, Organic styles), SVG export, and HTML dashboard generation. Shows health scores, thread colors, timestamps, and spiral indicators. Configurable message truncation, depth tracking, and merge/spiral node classification. Exports interactive HTML dashboards. | Medium | Implemented |
| FR-007 | Git Integration and Workflow Orchestration | GitWorkflowManager orchestrates all git operations via the git2 crate. Tracks commit graph with parent-child relationships. Provides commit node creation with quality metrics, revert point management, convergence analysis, thread color updates, and CI/CD validation. Bridges native git operations with the quality tracking system. | High | Implemented |

## Test Traceability

| FR | Test File | Test Name | Status |
|----|-----------|-----------|--------|
| FR-001 | `tests/unit_tests.rs` | `test_thread_color_exhaustive` | Pass |
| FR-001 | `tests/unit_tests.rs` | `test_thread_color_boundaries` | Pass |
| FR-001 | `tests/unit_tests.rs` | `prop_thread_color_score_consistency` | Pass |
| FR-002 | `tests/unit_tests.rs` | `test_convergence_metrics` | Pass |
| FR-003 | `tests/smoke_test.rs` | `smoke_test_loads` | Pass |
| FR-004 | `tests/integration_tests.rs` | (agent spiral/infinite monkey workflow integration tests) | Pass |
| FR-005 | `tests/integration_tests.rs` | (checkpoint/revert integration tests) | Pass |
| FR-006 | `tests/smoke_test.rs` | `smoke_test_loads` | Pass |
| FR-007 | `tests/smoke_test.rs` | `smoke_test_loads` | Pass |

## Thread Color System

| Color | Range | Meaning |
|-------|-------|---------|
| Red | 0.0–0.3 | Critical issues, broken code |
| Orange | 0.3–0.5 | Major issues, needs attention |
| Yellow | 0.5–0.7 | Minor issues, acceptable |
| Light Green | 0.7–0.9 | Good quality, minor improvements |
| Green | 0.9–1.0 | Excellent, production ready |

## Quality Dimensions

Each file thread tracks four quality dimensions (equal 0.25 weight):

| Dimension | Score | Description |
|-----------|-------|-------------|
| Lint | 0.0–1.0 | Code style, syntax, best practices |
| Type Check | 0.0–1.0 | Compilation, type safety |
| Test Coverage | 0.0–1.0 | Test coverage percentage |
| Functionality | 0.0–1.0 | Runtime behavior, correctness |

## Convergence Algorithm

The confidence score is calculated as:

```
confidence = 0.30 * quality_stability
           + 0.25 * thread_health_ratio
           + 0.20 * test_pass_consistency
           + 0.15 * build_success_rate
           + 0.10 * trend_improvement
```

Convergence requires: `confidence >= threshold` AND `quality_stability > 0.8` AND `thread_health_ratio >= min_green_threads_ratio`.
