# 🌳 GDK - Git Workflow Deep Knowledge

**Deep git workflow system for AI agents with infinite monkey theorem convergence**

GDK transforms git into a powerful state management engine for AI agents, enabling intelligent branching, quality-based threading, and convergence algorithms. Each commit becomes a decision point where agents can branch out, spiral forward, or snap back to working states.

## 🎯 Core Features

- **📊 Thread-Based Quality Tracking**: Each file has N "threads" (lint, typecheck, tests) with Red→Green color coding
- **🔄 Infinite Monkey Convergence**: Agents iterate until quality thresholds are met
- **🌿 Spiral Branching**: Create checkpoints, try risky changes, auto-revert on failure
- **🎨 Tree Visualization**: ASCII, SVG, and HTML views of your decision tree
- **⚡ Agent Workflow Management**: Multi-agent session tracking with statistics

## 🖼️ Visualizations

### ASCII Tree View
<img src="/KooshaPari/GDK/raw/main/screenshots/demo_tree.txt" alt="ASCII Tree Visualization" style="max-width: 100%;">

*Beautiful Unicode tree showing commit hierarchy with color-coded health indicators*

### SVG Visualization
<img src="/KooshaPari/GDK/raw/main/screenshots/demo_tree.svg" alt="SVG Tree Visualization" style="max-width: 100%;">

*Scalable vector graphics with precise node positioning and quality metrics*

### HTML Interactive View
<img src="/KooshaPari/GDK/raw/main/screenshots/demo_tree.html" alt="HTML Interactive Tree" style="max-width: 100%;">

*Interactive web visualization with detailed commit information and branching structure*

## 🧠 How It Works

### 1. **Commit = Decision Point**
Every commit represents a state where agents can:
- Branch out (try different approaches)
- Snap back (revert to working state)  
- Spiral forward (iterate until convergence)

### 2. **Quality Threading**
```rust
// Each file tracks multiple quality dimensions
struct FileThread {
    lint_score: f64,      // 🔴 Red → 🟢 Green
    typecheck_score: f64, // Code correctness
    test_score: f64,      // Test coverage
    // ... more threads
}
```

### 3. **Infinite Monkey Algorithm**
```rust
// Agent tries approaches until success
loop {
    attempt_solution();
    if quality_score > threshold { break; }
    git_revert_to_checkpoint();
}
```

### 4. **Spiral Branching**
- Agent creates checkpoint: `git commit`
- Tries risky change on new branch
- If fails: snaps back to checkpoint
- If succeeds: merges and continues

## 🚀 Quick Start

### Installation
```bash
git clone https://github.com/KooshaPari/GDK.git
cd GDK
cargo build --release
```

### Basic Usage
```bash
# Initialize GDK workflow
cargo run --bin gdk-cli init

# Create a checkpoint
cargo run --bin gdk-cli checkpoint "Starting new feature"

# Spiral branch (try risky changes)
cargo run --bin gdk-cli spiral "experimental-algorithm"

# View quality status
cargo run --bin gdk-cli status

# Generate tree visualization
cargo run --bin gdk-cli visualize --format html

# Get AI agent recommendations
cargo run --bin gdk-cli suggest
```

### Generate Demo Visualizations
```bash
# Create sample tree with branching commits
cargo run --bin generate_demo

# Creates: demo_tree.txt, demo_tree.svg, demo_tree.html
```

## 🏗️ Architecture

### Core Components

- **`GitWorkflowManager`**: Main workflow orchestration
- **`ThreadManager`**: File-level quality tracking
- **`ConvergenceAnalyzer`**: Mathematical convergence detection
- **`AgentWorkflowController`**: Multi-agent session management
- **`ValidationSuite`**: Rust-specific quality validators
- **`TreeVisualizer`**: Multi-format tree generation

### Data Structures

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitNode {
    pub id: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub threads: Vec<FileThread>,
    pub parent_ids: Vec<String>,
    pub convergence_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileThread {
    pub file_path: String,
    pub thread_type: ThreadType,
    pub color: ThreadColor,
    pub score: f64,
    pub history: Vec<QualityPoint>,
}
```

## 🎨 Thread Color System

| Color | Range | Meaning |
|-------|-------|---------|
| 🔴 **Red** | 0.0-0.2 | Critical issues, broken code |
| 🟠 **Orange** | 0.2-0.4 | Major issues, needs attention |
| 🟡 **Yellow** | 0.4-0.6 | Minor issues, acceptable |
| 🟢 **Light Green** | 0.6-0.8 | Good quality, minor improvements |
| 💚 **Green** | 0.8-1.0 | Excellent quality, production ready |

## 🔧 Configuration

### Quality Thresholds
```rust
const CONVERGENCE_THRESHOLD: f64 = 0.8;
const MIN_SPIRAL_ITERATIONS: usize = 3;
const MAX_SPIRAL_ITERATIONS: usize = 50;
```

### Validation Commands
```rust
// Default Rust validators
"cargo check"     // Compilation check
"cargo clippy"    // Linting
"cargo test"      // Test execution
"cargo fmt"       // Code formatting
"cargo audit"     // Security audit
```

## 🤖 Agent Integration

### For AI Agents
```rust
use gdk::agent::AgentWorkflowController;

let mut controller = AgentWorkflowController::new();

// Start agent session
controller.start_agent_session("agent-1", "feature-implementation").await?;

// Create quality checkpoint
controller.create_checkpoint("agent-1", "Initial implementation").await?;

// Attempt solution with auto-revert
let result = controller.spiral_branch("agent-1", "risky-optimization").await?;

// Get recommendations
let suggestions = controller.get_agent_recommendations("agent-1").await?;
```

### Workflow Commands
```bash
# Agent workflow management
gdk-cli agent start <agent-id> <task>
gdk-cli agent checkpoint <agent-id> <message>
gdk-cli agent spiral <agent-id> <branch-name>
gdk-cli agent revert <agent-id> <checkpoint-id>
gdk-cli agent suggest <agent-id>
```

## 📊 Statistics & Analytics

### Repository Health
- **Average Quality Score**: Overall codebase health
- **Convergence Rate**: How quickly agents reach stable states
- **Thread Distribution**: Quality breakdown across dimensions
- **Agent Success Rate**: Percentage of successful convergences

### Export Options
```bash
# Generate statistics report
gdk-cli stats --format json > repo_health.json

# Export visualization
gdk-cli visualize --format svg --output tree.svg
gdk-cli visualize --format html --output tree.html
```

## 🔄 Convergence Algorithm

The infinite monkey theorem implementation:

1. **Initialize**: Set quality thresholds and iteration limits
2. **Attempt**: Try solution approach
3. **Evaluate**: Calculate thread scores across all dimensions
4. **Decide**: If quality ≥ threshold, commit; else revert
5. **Iterate**: Repeat until convergence or max iterations
6. **Spiral**: Branch to new approach if stuck

## 🌟 Use Cases

### For AI Agents
- **Code Generation**: Iterate until compilation + tests pass
- **Refactoring**: Preserve functionality while improving quality
- **Bug Fixes**: Revert unsuccessful attempts automatically
- **Feature Development**: Branch strategies with quality gates

### For Development Teams
- **Quality Assurance**: Visual quality tracking across commits
- **Code Reviews**: Thread-based quality insights
- **Technical Debt**: Identify quality degradation patterns
- **Release Planning**: Convergence metrics for readiness

## 🔗 Integration

### GitHub Actions
```yaml
- name: GDK Quality Check
  run: |
    cargo run --bin gdk-cli status
    cargo run --bin gdk-cli visualize --format html
```

### Pre-commit Hooks
```bash
#!/bin/bash
# .git/hooks/pre-commit
cargo run --bin gdk-cli validate
```

## 🤝 Contributing

1. Fork the repository
2. Create feature branch: `git checkout -b feature/amazing-feature`
3. Use GDK workflow: `cargo run --bin gdk-cli init`
4. Commit changes: `cargo run --bin gdk-cli checkpoint "Add amazing feature"`
5. Push to branch: `git push origin feature/amazing-feature`
6. Open Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Infinite Monkey Theorem**: For the theoretical foundation
- **Git**: For the robust version control system
- **Rust Community**: For the excellent ecosystem
- **AI Agents**: For being the inspiration behind this system

---

**Built with ❤️ for AI agents who need deep git workflow management**