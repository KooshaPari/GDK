use crate::{CommitNode, ConvergenceMetrics, FileThread, ThreadColor, ThreadMetrics};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use colored::{Color, Colorize};
use petgraph::graph::{DiGraph, NodeIndex};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Write as FmtWrite;
use std::io::Write;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeNode {
    pub commit_hash: String,
    pub short_hash: String,
    pub message: String,
    pub timestamp: u64,
    pub health_score: f64,
    pub thread_colors: HashMap<String, ThreadColor>,
    pub parent_hashes: Vec<String>,
    pub children: Vec<String>,
    pub depth: usize,
    pub is_merge: bool,
    pub is_spiral: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeVisualization {
    pub nodes: HashMap<String, TreeNode>,
    pub root_nodes: Vec<String>,
    pub max_depth: usize,
    pub total_commits: usize,
}

#[derive(Debug, Clone)]
pub struct VisualizationConfig {
    pub show_health_scores: bool,
    pub show_thread_colors: bool,
    pub show_timestamps: bool,
    pub max_message_length: usize,
    pub ascii_style: AsciiStyle,
    pub show_spiral_indicators: bool,
}

#[derive(Debug, Clone)]
pub enum AsciiStyle {
    Simple,  // Basic ASCII characters
    Unicode, // Unicode box drawing
    Organic, // Tree-like organic appearance
}

impl Default for VisualizationConfig {
    fn default() -> Self {
        Self {
            show_health_scores: true,
            show_thread_colors: true,
            show_timestamps: false,
            max_message_length: 50,
            ascii_style: AsciiStyle::Unicode,
            show_spiral_indicators: true,
        }
    }
}

pub struct TreeVisualizer {
    config: VisualizationConfig,
}

impl TreeVisualizer {
    pub fn new(config: VisualizationConfig) -> Self {
        Self { config }
    }

    pub fn create_tree_visualization(&self, commits: &[CommitNode]) -> Result<TreeVisualization> {
        let mut nodes = HashMap::new();
        let mut graph = DiGraph::new();
        let mut node_indices = HashMap::new();

        // Create nodes
        for commit in commits {
            let short_hash = if commit.hash.len() >= 8 {
                commit.hash[..8].to_string()
            } else {
                commit.hash.clone()
            };

            let thread_colors: HashMap<String, ThreadColor> = commit
                .file_threads
                .iter()
                .map(|(path, thread)| (path.clone(), thread.color_status.clone()))
                .collect();

            let is_spiral = commit.message.contains("spiral") || commit.message.contains("attempt");
            let is_merge = commit.parent_hashes.len() > 1;

            let tree_node = TreeNode {
                commit_hash: commit.hash.clone(),
                short_hash,
                message: self.truncate_message(&commit.message),
                timestamp: commit.timestamp,
                health_score: commit.health_score,
                thread_colors,
                parent_hashes: commit.parent_hashes.clone(),
                children: Vec::new(),
                depth: 0,
                is_merge,
                is_spiral,
            };

            let index = graph.add_node(commit.hash.clone());
            node_indices.insert(commit.hash.clone(), index);
            nodes.insert(commit.hash.clone(), tree_node);
        }

        // Create edges and update children
        for commit in commits {
            for parent_hash in &commit.parent_hashes {
                if let (Some(&parent_idx), Some(&child_idx)) = (
                    node_indices.get(parent_hash),
                    node_indices.get(&commit.hash),
                ) {
                    graph.add_edge(parent_idx, child_idx, ());

                    if let Some(parent_node) = nodes.get_mut(parent_hash) {
                        parent_node.children.push(commit.hash.clone());
                    }
                }
            }
        }

        // Find root nodes and calculate depths
        let mut root_nodes = Vec::new();
        self.calculate_depths(&mut nodes, &graph, &node_indices, &mut root_nodes)?;

        let max_depth = nodes.values().map(|n| n.depth).max().unwrap_or(0);

        Ok(TreeVisualization {
            nodes,
            root_nodes,
            max_depth,
            total_commits: commits.len(),
        })
    }

    pub fn render_ascii_tree(&self, tree: &TreeVisualization) -> Result<String> {
        let mut output = String::new();

        // Header
        writeln!(output, "🌳 GDK Git Workflow Tree Visualization")?;
        writeln!(
            output,
            "📊 Total commits: {} | Max depth: {}",
            tree.total_commits, tree.max_depth
        )?;
        writeln!(output, "{}", "═".repeat(80))?;

        // Legend
        if self.config.show_thread_colors {
            writeln!(
                output,
                "Thread Colors: {} {} {} {} {}",
                "🔴 Red".color(Color::Red),
                "🟠 Orange".color(Color::Yellow),
                "🟡 Yellow".color(Color::Yellow),
                "🟢 Light Green".color(Color::Green),
                "💚 Green".color(Color::Green)
            )?;
            writeln!(output)?;
        }

        // Render tree starting from root nodes
        for root_hash in &tree.root_nodes {
            self.render_node_recursive(tree, root_hash, &mut output, 0, Vec::new(), true)?;
        }

        // Statistics section
        writeln!(output, "\n{}", "═".repeat(80))?;
        writeln!(output, "📈 Repository Statistics:")?;

        let health_stats = self.calculate_health_statistics(tree);
        writeln!(output, "Average Health: {:.2}", health_stats.average_health)?;
        writeln!(
            output,
            "Healthy Commits (>0.8): {}/{}",
            health_stats.healthy_commits, tree.total_commits
        )?;

        Ok(output)
    }

    fn render_node_recursive(
        &self,
        tree: &TreeVisualization,
        node_hash: &str,
        output: &mut String,
        _depth: usize,
        prefix: Vec<bool>,
        is_last: bool,
    ) -> Result<()> {
        let node = tree
            .nodes
            .get(node_hash)
            .ok_or_else(|| anyhow!("Node not found: {}", node_hash))?;

        // Draw tree structure
        for (i, &is_continuation) in prefix.iter().enumerate() {
            if i == prefix.len() - 1 {
                let symbol = match self.config.ascii_style {
                    AsciiStyle::Simple => {
                        if is_last {
                            "`-- "
                        } else {
                            "|-- "
                        }
                    }
                    AsciiStyle::Unicode => {
                        if is_last {
                            "└── "
                        } else {
                            "├── "
                        }
                    }
                    AsciiStyle::Organic => {
                        if is_last {
                            "🌿 "
                        } else {
                            "🌱 "
                        }
                    }
                };
                write!(output, "{symbol}")?;
            } else {
                let symbol = match self.config.ascii_style {
                    AsciiStyle::Simple => {
                        if is_continuation {
                            "|   "
                        } else {
                            "    "
                        }
                    }
                    AsciiStyle::Unicode => {
                        if is_continuation {
                            "│   "
                        } else {
                            "    "
                        }
                    }
                    AsciiStyle::Organic => {
                        if is_continuation {
                            "🌳 "
                        } else {
                            "   "
                        }
                    }
                };
                write!(output, "{symbol}")?;
            }
        }

        // Node content
        let node_display = self.format_node_display(node)?;
        writeln!(output, "{node_display}")?;

        // Recursively render children
        let children = &node.children;
        for (i, child_hash) in children.iter().enumerate() {
            let mut new_prefix = prefix.clone();
            new_prefix.push(i < children.len() - 1);

            self.render_node_recursive(
                tree,
                child_hash,
                output,
                _depth + 1,
                new_prefix,
                i == children.len() - 1,
            )?;
        }

        Ok(())
    }

    fn format_node_display(&self, node: &TreeNode) -> Result<String> {
        let mut display = String::new();

        // Hash and indicators
        write!(display, "{}", node.short_hash.clone().color(Color::Cyan))?;

        if node.is_spiral && self.config.show_spiral_indicators {
            write!(display, " 🌀")?;
        }
        if node.is_merge {
            write!(display, " 🔀")?;
        }

        // Health score with color
        if self.config.show_health_scores {
            let health_color = match node.health_score {
                x if x >= 0.8 => Color::Green,
                x if x >= 0.5 => Color::Yellow,
                _ => Color::Red,
            };
            write!(
                display,
                " ({:.2})",
                node.health_score.to_string().color(health_color)
            )?;
        }

        // Thread colors
        if self.config.show_thread_colors && !node.thread_colors.is_empty() {
            write!(display, " [")?;
            for (i, (_, color)) in node.thread_colors.iter().enumerate() {
                if i > 0 {
                    write!(display, " ")?;
                }
                let symbol = match color {
                    ThreadColor::Red => "🔴",
                    ThreadColor::Orange => "🟠",
                    ThreadColor::Yellow => "🟡",
                    ThreadColor::LightGreen => "🟢",
                    ThreadColor::Green => "💚",
                };
                write!(display, "{symbol}")?;
            }
            write!(display, "]")?;
        }

        // Message
        write!(display, " {}", node.message.clone().color(Color::White))?;

        // Timestamp
        if self.config.show_timestamps {
            let dt = DateTime::from_timestamp(node.timestamp as i64, 0).unwrap_or_else(Utc::now);
            write!(
                display,
                " ({})",
                dt.format("%Y-%m-%d %H:%M").to_string().color(Color::Black)
            )?;
        }

        Ok(display)
    }

    fn calculate_depths(
        &self,
        nodes: &mut HashMap<String, TreeNode>,
        graph: &DiGraph<String, ()>,
        node_indices: &HashMap<String, NodeIndex>,
        root_nodes: &mut Vec<String>,
    ) -> Result<()> {
        // Find root nodes (nodes with no incoming edges)
        for (hash, &index) in node_indices {
            let incoming_edges = graph
                .edges_directed(index, petgraph::Direction::Incoming)
                .count();
            if incoming_edges == 0 {
                root_nodes.push(hash.clone());
            }
        }

        // Calculate depths using BFS from root nodes
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();

        for root_hash in root_nodes.iter() {
            queue.push_back((root_hash.clone(), 0));
        }

        while let Some((hash, depth)) = queue.pop_front() {
            if visited.contains(&hash) {
                continue;
            }
            visited.insert(hash.clone());

            if let Some(node) = nodes.get_mut(&hash) {
                node.depth = depth;

                for child_hash in &node.children.clone() {
                    if !visited.contains(child_hash) {
                        queue.push_back((child_hash.clone(), depth + 1));
                    }
                }
            }
        }

        Ok(())
    }

    fn truncate_message(&self, message: &str) -> String {
        if message.len() <= self.config.max_message_length {
            message.to_string()
        } else {
            format!("{}...", &message[..self.config.max_message_length - 3])
        }
    }

    fn calculate_health_statistics(&self, tree: &TreeVisualization) -> HealthStatistics {
        let total = tree.nodes.len() as f64;
        let sum: f64 = tree.nodes.values().map(|n| n.health_score).sum();
        let healthy_count = tree.nodes.values().filter(|n| n.health_score > 0.8).count();

        HealthStatistics {
            average_health: if total > 0.0 { sum / total } else { 0.0 },
            healthy_commits: healthy_count,
            total_commits: tree.nodes.len(),
        }
    }
}

#[derive(Debug)]
pub struct HealthStatistics {
    pub average_health: f64,
    pub healthy_commits: usize,
    pub total_commits: usize,
}

// Export functions for ASCII format
pub fn export_tree_ascii(
    commits: &[CommitNode],
    config: Option<VisualizationConfig>,
) -> Result<String> {
    let config = config.unwrap_or_default();
    let visualizer = TreeVisualizer::new(config);
    let tree = visualizer.create_tree_visualization(commits)?;
    visualizer.render_ascii_tree(&tree)
}

// Simplified SVG export
pub fn export_tree_svg(
    commits: &[CommitNode],
    _config: Option<VisualizationConfig>,
) -> Result<String> {
    let mut svg = String::new();
    writeln!(
        svg,
        "<svg width='800' height='600' xmlns='http://www.w3.org/2000/svg'>"
    )?;
    writeln!(svg, "<rect width='100%' height='100%' fill='white'/>")?;

    for (i, commit) in commits.iter().enumerate() {
        let y = 50 + i * 40;
        let color = if commit.health_score >= 0.8 {
            "#00aa00"
        } else if commit.health_score >= 0.5 {
            "#aaaa00"
        } else {
            "#aa0000"
        };
        let short_hash = if commit.hash.len() >= 8 {
            &commit.hash[..8]
        } else {
            &commit.hash
        };

        writeln!(
            svg,
            "<circle cx='50' cy='{y}' r='15' fill='{color}' stroke='black'/>"
        )?;
        writeln!(
            svg,
            "<text x='80' y='{}' font-family='monospace' font-size='12'>{} ({:.2}) {}</text>",
            y + 5,
            short_hash,
            commit.health_score,
            commit.message
        )?;
    }

    writeln!(svg, "</svg>")?;
    Ok(svg)
}

// Simplified HTML export
pub fn export_tree_html(
    commits: &[CommitNode],
    _config: Option<VisualizationConfig>,
) -> Result<String> {
    let mut html = String::new();

    writeln!(html, "<!DOCTYPE html>")?;
    writeln!(
        html,
        "<html><head><title>GDK Tree Visualization</title></head><body>"
    )?;
    writeln!(html, "<h1>🌳 GDK Git Workflow Tree</h1>")?;
    writeln!(html, "<p>Total commits: {}</p>", commits.len())?;

    for commit in commits {
        let color = if commit.health_score >= 0.8 {
            "green"
        } else if commit.health_score >= 0.5 {
            "orange"
        } else {
            "red"
        };
        let short_hash = if commit.hash.len() >= 8 {
            &commit.hash[..8]
        } else {
            &commit.hash
        };

        writeln!(
            html,
            "<div style='margin: 10px; padding: 5px; border-left: 4px solid {color};'>"
        )?;
        writeln!(
            html,
            "<strong>{}</strong> ({:.2}) {}",
            short_hash, commit.health_score, commit.message
        )?;
        writeln!(html, "</div>")?;
    }

    writeln!(html, "</body></html>")?;
    Ok(html)
}

pub fn save_visualization<W: Write>(
    commits: &[CommitNode],
    format: &str,
    writer: &mut W,
    config: Option<VisualizationConfig>,
) -> Result<()> {
    let output = match format.to_lowercase().as_str() {
        "ascii" | "txt" => export_tree_ascii(commits, config)?,
        "svg" => export_tree_svg(commits, config)?,
        "html" => export_tree_html(commits, config)?,
        _ => {
            return Err(anyhow!(
                "Unsupported format: {}. Use 'ascii', 'svg', or 'html'",
                format
            ))
        }
    };

    write!(writer, "{output}")?;
    Ok(())
}

// Generate sample tree data for testing
pub fn generate_sample_tree(num_commits: usize, num_branches: usize) -> Vec<CommitNode> {
    let mut commits = Vec::new();
    let mut commit_counter = 1;

    // Create main trunk commits
    let trunk_commits = num_commits / 2;
    for i in 0..trunk_commits {
        let parent_hashes = if i > 0 {
            vec![format!("commit_{}", i)]
        } else {
            vec![]
        };

        let commit = create_sample_commit(
            format!("commit_{commit_counter}"),
            format!("Main trunk commit {commit_counter}"),
            parent_hashes,
            0.7 + (i as f64 * 0.1) % 0.3,
        );
        commits.push(commit);
        commit_counter += 1;
    }

    // Create branch commits
    for branch in 1..=num_branches {
        let branch_point = trunk_commits / 2;
        let branch_commits = (num_commits - trunk_commits) / num_branches;

        for i in 0..branch_commits {
            let parent_hash = if i == 0 {
                format!("commit_{branch_point}")
            } else {
                format!("branch_{branch}_{i}")
            };

            let commit = create_sample_commit(
                format!("branch_{}_{}", branch, i + 1),
                format!("Feature branch {} commit {}", branch, i + 1),
                vec![parent_hash],
                0.5 + (i as f64 * 0.2) % 0.4,
            );
            commits.push(commit);
            commit_counter += 1;
        }
    }

    commits
}

fn create_sample_commit(
    hash: String,
    message: String,
    parent_hashes: Vec<String>,
    health_score: f64,
) -> CommitNode {
    let mut file_threads = HashMap::new();

    // Add some sample file threads
    for i in 1..=3 {
        let file_path = format!("src/file_{i}.rs");
        let thread = FileThread {
            file_path: file_path.clone(),
            thread_id: uuid::Uuid::new_v4(),
            color_status: ThreadColor::from_scores(
                health_score,
                health_score,
                health_score,
                health_score,
            ),
            lint_score: health_score,
            type_check_score: health_score,
            test_coverage: health_score,
            functionality_score: health_score,
            history: vec![crate::ThreadState {
                commit_hash: hash.clone(),
                diff_content: format!("Sample diff for {file_path}"),
                metrics: ThreadMetrics {
                    lines_added: 10,
                    lines_removed: 5,
                    complexity_delta: 0.1,
                    quality_score: health_score,
                },
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            }],
        };
        file_threads.insert(file_path, thread);
    }

    CommitNode {
        id: uuid::Uuid::new_v4().to_string(),
        hash,
        parent_hashes,
        message,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        file_threads,
        health_score,
        convergence_metrics: ConvergenceMetrics {
            attempts: 1,
            successful_builds: if health_score > 0.7 { 1 } else { 0 },
            test_pass_rate: health_score,
            quality_trend: vec![health_score],
            is_converged: health_score > 0.8,
        },
    }
}
