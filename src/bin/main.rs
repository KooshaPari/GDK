use anyhow::Result;
use clap::{Parser, Subcommand};
use gdk::{agent::AgentWorkflowController, core::GitWorkflowManager, visualization::*};
use std::fs::File;
use std::io::Write;
use tracing::{info, Level};

#[derive(Parser)]
#[command(name = "gdk")]
#[command(about = "Git Workflow Deep Knowledge system for AI agents")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value = ".")]
    repo_path: String,

    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[arg(short, long)]
        agent_id: String,
    },
    Commit {
        #[arg(short, long)]
        agent_id: String,
        #[arg(short, long)]
        message: String,
    },
    Spiral {
        #[arg(short, long)]
        agent_id: String,
        #[arg(short, long, default_value = "100")]
        max_attempts: u32,
        #[arg(short, long, default_value = "0.8")]
        target_convergence: f64,
    },
    Revert {
        #[arg(short, long)]
        agent_id: String,
    },
    Checkpoint {
        #[arg(short, long)]
        agent_id: String,
        #[arg(short, long)]
        reason: String,
    },
    Status {
        #[arg(short, long)]
        agent_id: String,
    },
    Stats {
        #[arg(short, long)]
        agent_id: String,
    },
    Suggest {
        #[arg(short, long)]
        agent_id: String,
    },
    Visualize {
        #[arg(short, long, default_value = "ascii")]
        format: String,
        #[arg(short, long)]
        output: Option<String>,
        #[arg(long)]
        show_health: bool,
        #[arg(long)]
        show_threads: bool,
        #[arg(long)]
        show_timestamps: bool,
        #[arg(long, default_value = "unicode")]
        style: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let level = if cli.verbose {
        Level::DEBUG
    } else {
        Level::INFO
    };
    tracing_subscriber::fmt().with_max_level(level).init();

    let workflow = GitWorkflowManager::new(&cli.repo_path)?;
    let mut controller = AgentWorkflowController::new(workflow);

    match cli.command {
        Commands::Init { agent_id } => {
            let session_id = controller.start_agent_session(&agent_id).await?;
            info!(
                "Initialized agent session {} with ID: {}",
                agent_id, session_id
            );
            println!("Agent session initialized: {session_id}");
        }

        Commands::Commit { agent_id, message } => {
            let commit_node = controller.validate_and_commit(&agent_id, &message).await?;
            info!("Created commit: {}", commit_node.hash);
            println!("Commit created: {}", commit_node.hash);
            println!("Health score: {:.2}", commit_node.health_score);
            println!(
                "Convergence: {}",
                commit_node.convergence_metrics.is_converged
            );
        }

        Commands::Spiral {
            agent_id,
            max_attempts,
            target_convergence,
        } => {
            info!(
                "Starting infinite monkey spiral for agent {} (max: {}, target: {})",
                agent_id, max_attempts, target_convergence
            );

            // Set max attempts in session
            if let Some(session) = controller.active_sessions.get_mut(&agent_id) {
                session.max_spiral_attempts = max_attempts;
            }

            match controller
                .execute_infinite_monkey_workflow(&agent_id, target_convergence)
                .await
            {
                Ok(commit_node) => {
                    println!("🎉 CONVERGENCE ACHIEVED!");
                    println!("Final commit: {}", commit_node.hash);
                    println!("Health score: {:.2}", commit_node.health_score);
                    println!(
                        "Attempts: {}",
                        controller
                            .active_sessions
                            .get(&agent_id)
                            .unwrap()
                            .spiral_attempts
                    );
                }
                Err(e) => {
                    println!("❌ Failed to converge: {e}");
                    if let Some(session) = controller.active_sessions.get(&agent_id) {
                        println!("Attempts made: {}", session.spiral_attempts);
                    }
                }
            }
        }

        Commands::Revert { agent_id } => {
            controller.revert_to_last_checkpoint(&agent_id).await?;
            info!("Reverted agent {} to last checkpoint", agent_id);
            println!("Reverted to last checkpoint");
        }

        Commands::Checkpoint { agent_id, reason } => {
            let revert_point = controller
                .create_spiral_checkpoint(&agent_id, &reason)
                .await?;
            info!(
                "Created checkpoint for agent {}: {}",
                agent_id, revert_point.commit_hash
            );
            println!("Checkpoint created at commit: {}", revert_point.commit_hash);
        }

        Commands::Status { agent_id } => {
            let convergence = controller.get_convergence_status(&agent_id).await?;
            println!("=== Agent Status: {agent_id} ===");
            println!("Converged: {}", convergence.is_converged);
            println!("Attempts: {}", convergence.attempts);
            println!("Test pass rate: {:.2}%", convergence.test_pass_rate * 100.0);
            println!("Successful builds: {}", convergence.successful_builds);

            if !convergence.quality_trend.is_empty() {
                let latest_quality = convergence.quality_trend.last().unwrap();
                println!("Latest quality: {latest_quality:.2}");
            }
        }

        Commands::Stats { agent_id } => {
            let stats = controller.get_agent_statistics(&agent_id)?;
            println!("=== Agent Statistics: {} ===", stats.agent_id);
            println!("Total actions: {}", stats.total_actions);
            println!("Success rate: {:.2}%", stats.success_rate * 100.0);
            println!("Spiral attempts: {}", stats.spiral_attempts);
            println!("Revert points used: {}", stats.revert_points_used);
            println!(
                "Current convergence: {}",
                stats.convergence_state.is_converged
            );
        }

        Commands::Suggest { agent_id } => {
            let suggestion = controller.suggest_next_action(&agent_id).await?;
            println!("💡 Suggested action: {suggestion}");
        }

        Commands::Visualize {
            format,
            output,
            show_health,
            show_threads,
            show_timestamps,
            style,
        } => {
            let commits = &controller.workflow.commit_history;

            if commits.is_empty() {
                println!("⚠️  No commits found. Create some commits first using 'gdk commit' or 'gdk spiral'.");
                return Ok(());
            }

            let ascii_style = match style.as_str() {
                "simple" => AsciiStyle::Simple,
                "unicode" => AsciiStyle::Unicode,
                "organic" => AsciiStyle::Organic,
                _ => AsciiStyle::Unicode,
            };

            let config = VisualizationConfig {
                show_health_scores: show_health,
                show_thread_colors: show_threads,
                show_timestamps,
                ascii_style,
                ..Default::default()
            };

            let tree_output = match format.as_str() {
                "ascii" | "txt" => export_tree_ascii(commits, Some(config))?,
                "svg" => export_tree_svg(commits, Some(config))?,
                "html" => export_tree_html(commits, Some(config))?,
                _ => {
                    println!("❌ Unsupported format: {format}. Use 'ascii', 'svg', or 'html'");
                    return Ok(());
                }
            };

            match output {
                Some(filename) => {
                    let mut file = File::create(&filename)?;
                    file.write_all(tree_output.as_bytes())?;
                    println!("🌳 Tree visualization saved to: {filename}");

                    if format == "html" {
                        println!("🌐 Open {filename} in your browser to view the interactive tree");
                    }
                }
                None => {
                    if format == "ascii" || format == "txt" {
                        println!("{tree_output}");
                    } else {
                        println!("📄 {} output:", format.to_uppercase());
                        println!("{tree_output}");
                    }
                }
            }
        }
    }

    Ok(())
}
