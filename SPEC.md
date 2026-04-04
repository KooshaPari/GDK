# SPEC.md - GDK (Git Workflow Deep Knowledge)

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                        GDK Core Architecture                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐      │
│  │   CLI Layer  │    │  Agent API   │    │  Git Hooks   │      │
│  │   (clap)     │    │  (async)     │    │  Integration │      │
│  └──────┬───────┘    └──────┬───────┘    └──────┬───────┘      │
│         │                   │                   │              │
│         └───────────────────┼───────────────────┘              │
│                             │                                    │
│  ┌──────────────────────────┴──────────────────────────┐        │
│  │            GitWorkflowManager (Core)                │        │
│  │  • Session Management  • Branch Operations          │        │
│  │  • Commit Tracking     • Quality Integration         │        │
│  └──────────────────────────┬──────────────────────────┘        │
│                             │                                    │
│  ┌──────────────┐  ┌────────┴────────┐  ┌──────────────┐      │
│  │   Thread     │  │   Convergence    │  │   Quality    │      │
│  │   Manager    │  │   Analyzer       │  │   Metrics    │      │
│  │              │  │                  │  │              │      │
│  │ • File-level │  │ • Math detection │  │ • Scoring    │      │
│  │   quality    │  │ • Auto-revert    │  │ • Threading  │      │
│  │ • N threads  │  │ • Convergence    │  │ • Validation │      │
│  │   per file   │  │   tracking       │  │ • Reporting  │      │
│  └──────────────┘  └──────────────────┘  └──────────────┘      │
│                             │                                    │
│  ┌──────────────────────────┴──────────────────────────┐        │
│  │              Validation Suite (Rust-specific)         │        │
│  │   cargo check • cargo clippy • cargo test • audit    │        │
│  └──────────────────────────────────────────────────────┘        │
│                             │                                    │
│  ┌──────────────────────────┴──────────────────────────┐        │
│  │              Visualization Engine                     │        │
│  │   ASCII Trees • SVG Export • HTML Dashboards         │        │
│  └──────────────────────────────────────────────────────┘        │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

## Component Breakdown

### 1. Core Engine (`src/core.rs`)
- **GitWorkflowManager**: Central orchestration for all git operations
- **Session Management**: Track agent sessions and their git contexts
- **Commit Graph**: Track parent-child relationships between commits
- **Checkpoint System**: Create named restore points for safe experimentation

### 2. Thread Manager (`src/threads.rs`)
- **FileThread**: Per-file quality tracking across multiple dimensions
  - Thread types: Lint, TypeCheck, Test, Security, Performance, Docs
  - Color coding: Red (0.0-0.2) → Orange → Yellow → Light Green → Green (0.8-1.0)
  - History tracking with QualityPoint snapshots
- **ThreadAggregation**: Roll up file-level scores to commit-level quality

### 3. Convergence Engine (`src/convergence.rs`)
- **Infinite Monkey Algorithm**: Automated iteration until quality threshold
- **Spiral Branching**: Create checkpoint → Try risky change → Auto-revert on failure
- **Convergence Detection**: Mathematical detection of quality plateau
- **Quality Thresholds**: Configurable minimum scores per dimension

### 4. Agent Workflow Controller (`src/agent.rs`)
- **Multi-Agent Support**: Concurrent agent session tracking
- **Agent Metrics**: Success rates, convergence times, iteration counts
- **Recommendation Engine**: AI-powered next steps with confidence scoring
- **Swarm Coordination**: Multi-agent deployment and monitoring

### 5. Validation Suite (`src/validation.rs`)
- **Rust Validators**: Native integration with cargo ecosystem
- **Quality Metrics**: Complexity, coverage, security, performance analysis
- **Extensible Architecture**: Plugin system for custom validators
- **Parallel Execution**: Concurrent validation for performance

### 6. Visualization (`src/visualization.rs`)
- **TreeRenderer**: ASCII art commit trees with quality indicators
- **SVG Export**: Scalable vector graphics for documentation
- **HTML Dashboards**: Interactive web-based visualizations
- **Quality Tables**: Thread-by-thread score breakdowns

## Data Models

### CommitNode
```rust
pub struct CommitNode {
    pub id: String,                    // SHA or UUID
    pub message: String,               // Commit message
    pub timestamp: DateTime<Utc>,    // When created
    pub threads: Vec<FileThread>,    // Quality threads
    pub parent_ids: Vec<String>,     // Parent commit(s)
    pub convergence_score: f64,      // Overall quality (0.0-1.0)
    pub author: String,              // Agent or user ID
    pub checkpoint_name: Option<String>, // Named checkpoint
}
```

### FileThread
```rust
pub struct FileThread {
    pub file_path: String,           // Relative path
    pub thread_type: ThreadType,     // Lint, Test, Security, etc.
    pub color: ThreadColor,          // Red, Orange, Yellow, Green, DarkGreen
    pub score: f64,                  // 0.0-1.0
    pub history: Vec<QualityPoint>,  // Score over time
    pub last_updated: DateTime<Utc>,
}

pub enum ThreadType {
    Lint,        // Code style and static analysis
    TypeCheck,   // Type system correctness
    Test,        // Test coverage and results
    Security,    // Vulnerability scanning
    Performance, // Benchmark results
    Docs,        // Documentation completeness
}
```

### AgentSession
```rust
pub struct AgentSession {
    pub agent_id: String,              // Unique agent identifier
    pub current_branch: String,        // Active git branch
    pub checkpoints: Vec<Checkpoint>,  // Named restore points
    pub convergence_stats: ConvergenceStats,
    pub created_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
}

pub struct ConvergenceStats {
    pub total_iterations: u32,
    pub successful_convergences: u32,
    pub average_convergence_time: Duration,
    pub quality_improvements: Vec<f64>,
}
```

### QualityThresholds
```rust
pub struct QualityThresholds {
    pub overall: f64,                  // Minimum overall score
    pub lint: f64,                     // Minimum lint score
    pub typecheck: f64,                // Minimum type check score
    pub test: f64,                     // Minimum test score
    pub security: f64,                 // Minimum security score
    pub performance: f64,              // Minimum performance score
    pub max_iterations: usize,         // Infinite monkey limit
}
```

## Thread Color System

| Color | Range | Meaning | Visual |
|-------|-------|---------|--------|
| 🔴 Red | 0.0-0.2 | Critical issues, broken code | Blocker |
| 🟠 Orange | 0.2-0.4 | Major issues, needs attention | Warning |
| 🟡 Yellow | 0.4-0.6 | Minor issues, acceptable | Caution |
| 🟢 Light Green | 0.6-0.8 | Good quality, minor improvements | Pass |
| 💚 Green | 0.8-1.0 | Excellent, production ready | Excellent |

## Performance Specifications

### Convergence Algorithm
- **Max Iterations**: 50 (configurable)
- **Convergence Threshold**: 0.8 default (0.95 enterprise)
- **Time per Iteration**: <2 seconds (typical)
- **Success Rate Target**: >90%

### Quality Thread Processing
- **Parallel Validation**: Enabled via Rayon
- **File Watch Interval**: 100ms
- **Memory per Thread**: <1MB
- **Max Threads Tracked**: 10,000 per commit

### Visualization Generation
- **ASCII Tree**: <100ms for 100 commits
- **SVG Export**: <500ms for 100 commits
- **HTML Dashboard**: <1s generation time

## Integration Points

### Git Integration
- Uses `git2` crate for native git operations
- Supports standard git workflows (merge, rebase, cherry-pick)
- Compatible with GitHub, GitLab, Bitbucket

### CI/CD Integration
- GitHub Actions via `gdk-cli status --fail-on-threshold`
- Pre-commit hooks for quality gates
- Docker deployment ready

### AI Agent Integration
- MCP (Model Context Protocol) support
- JSON API for programmatic access
- Webhook notifications on convergence

## Security Model

- **Memory Safe**: Rust implementation prevents common vulnerabilities
- **Audit Trail**: All agent actions logged with timestamps
- **Sandboxed Validation**: Isolated execution for untrusted code
- **No External Calls**: Self-contained, no network dependencies in core

## Extensibility

### Custom Validators
```rust
pub trait Validator {
    fn name(&self) -> &str;
    fn validate(&self, files: &[PathBuf]) -> ValidationResult;
    fn thread_type(&self) -> ThreadType;
}
```

### Custom Visualization
```rust
pub trait Visualizer {
    fn render(&self, tree: &CommitTree) -> Result<String, VizError>;
    fn format(&self) -> VisualFormat;
}
```
