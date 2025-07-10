use crate::{FileThread, ThreadColor, ThreadMetrics, ThreadState};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadManager {
    pub active_threads: HashMap<String, FileThread>,
    pub thread_history: Vec<ThreadSnapshot>,
    pub color_rules: ColorRules,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadSnapshot {
    pub snapshot_id: Uuid,
    pub timestamp: u64,
    pub commit_hash: String,
    pub threads: HashMap<String, FileThread>,
    pub overall_health: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorRules {
    pub green_threshold: f64,
    pub light_green_threshold: f64,
    pub yellow_threshold: f64,
    pub orange_threshold: f64,
    pub weight_lint: f64,
    pub weight_type_check: f64,
    pub weight_test_coverage: f64,
    pub weight_functionality: f64,
}

impl Default for ColorRules {
    fn default() -> Self {
        Self {
            green_threshold: 0.9,
            light_green_threshold: 0.7,
            yellow_threshold: 0.5,
            orange_threshold: 0.3,
            weight_lint: 0.25,
            weight_type_check: 0.25,
            weight_test_coverage: 0.25,
            weight_functionality: 0.25,
        }
    }
}

impl ThreadManager {
    pub fn new() -> Self {
        Self {
            active_threads: HashMap::new(),
            thread_history: Vec::new(),
            color_rules: ColorRules::default(),
        }
    }

    pub fn create_thread(&mut self, file_path: &str, commit_hash: &str) -> Result<&FileThread> {
        let thread = FileThread {
            file_path: file_path.to_string(),
            thread_id: Uuid::new_v4(),
            color_status: ThreadColor::Red,
            lint_score: 0.0,
            type_check_score: 0.0,
            test_coverage: 0.0,
            functionality_score: 0.0,
            history: vec![ThreadState {
                commit_hash: commit_hash.to_string(),
                diff_content: String::new(),
                metrics: ThreadMetrics {
                    lines_added: 0,
                    lines_removed: 0,
                    complexity_delta: 0.0,
                    quality_score: 0.0,
                },
                timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
            }],
        };

        self.active_threads.insert(file_path.to_string(), thread);
        Ok(self.active_threads.get(file_path).unwrap())
    }

    pub fn update_thread_quality(
        &mut self,
        file_path: &str,
        lint: f64,
        type_check: f64,
        test_coverage: f64,
        functionality: f64,
    ) -> Result<()> {
        let color_status =
            self.calculate_thread_color(lint, type_check, test_coverage, functionality);

        let thread = self
            .active_threads
            .get_mut(file_path)
            .ok_or_else(|| anyhow!("Thread not found for file: {}", file_path))?;

        thread.lint_score = lint;
        thread.type_check_score = type_check;
        thread.test_coverage = test_coverage;
        thread.functionality_score = functionality;
        thread.color_status = color_status;

        Ok(())
    }

    pub fn add_thread_state(
        &mut self,
        file_path: &str,
        commit_hash: &str,
        diff_content: &str,
        metrics: ThreadMetrics,
    ) -> Result<()> {
        let thread = self
            .active_threads
            .get_mut(file_path)
            .ok_or_else(|| anyhow!("Thread not found for file: {}", file_path))?;

        let state = ThreadState {
            commit_hash: commit_hash.to_string(),
            diff_content: diff_content.to_string(),
            metrics,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
        };

        thread.history.push(state);

        // Keep only the last 50 states to prevent unbounded growth
        if thread.history.len() > 50 {
            thread.history.drain(0..(thread.history.len() - 50));
        }

        Ok(())
    }

    pub fn calculate_thread_color(
        &self,
        lint: f64,
        type_check: f64,
        test_coverage: f64,
        functionality: f64,
    ) -> ThreadColor {
        let weighted_score = lint * self.color_rules.weight_lint
            + type_check * self.color_rules.weight_type_check
            + test_coverage * self.color_rules.weight_test_coverage
            + functionality * self.color_rules.weight_functionality;

        match weighted_score {
            x if x >= self.color_rules.green_threshold => ThreadColor::Green,
            x if x >= self.color_rules.light_green_threshold => ThreadColor::LightGreen,
            x if x >= self.color_rules.yellow_threshold => ThreadColor::Yellow,
            x if x >= self.color_rules.orange_threshold => ThreadColor::Orange,
            _ => ThreadColor::Red,
        }
    }

    pub fn get_thread_health_trend(&self, file_path: &str, window_size: usize) -> Result<Vec<f64>> {
        let thread = self
            .active_threads
            .get(file_path)
            .ok_or_else(|| anyhow!("Thread not found for file: {}", file_path))?;

        let trend: Vec<f64> = thread
            .history
            .iter()
            .rev()
            .take(window_size)
            .map(|state| state.metrics.quality_score)
            .collect();

        Ok(trend)
    }

    pub fn get_overall_health(&self) -> f64 {
        if self.active_threads.is_empty() {
            return 0.0;
        }

        let total_quality: f64 = self
            .active_threads
            .values()
            .map(|thread| {
                let latest_state = thread.history.last();
                latest_state.map_or(0.0, |state| state.metrics.quality_score)
            })
            .sum();

        total_quality / self.active_threads.len() as f64
    }

    pub fn get_color_distribution(&self) -> HashMap<ThreadColor, usize> {
        let mut distribution = HashMap::new();
        distribution.insert(ThreadColor::Red, 0);
        distribution.insert(ThreadColor::Orange, 0);
        distribution.insert(ThreadColor::Yellow, 0);
        distribution.insert(ThreadColor::LightGreen, 0);
        distribution.insert(ThreadColor::Green, 0);

        for thread in self.active_threads.values() {
            *distribution.get_mut(&thread.color_status).unwrap() += 1;
        }

        distribution
    }

    pub fn create_snapshot(&mut self, commit_hash: &str) -> Result<Uuid> {
        let snapshot_id = Uuid::new_v4();
        let snapshot = ThreadSnapshot {
            snapshot_id,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
            commit_hash: commit_hash.to_string(),
            threads: self.active_threads.clone(),
            overall_health: self.get_overall_health(),
        };

        self.thread_history.push(snapshot);

        // Keep only the last 100 snapshots
        if self.thread_history.len() > 100 {
            self.thread_history
                .drain(0..(self.thread_history.len() - 100));
        }

        Ok(snapshot_id)
    }

    pub fn restore_from_snapshot(&mut self, snapshot_id: Uuid) -> Result<()> {
        let snapshot = self
            .thread_history
            .iter()
            .find(|s| s.snapshot_id == snapshot_id)
            .ok_or_else(|| anyhow!("Snapshot not found: {}", snapshot_id))?;

        self.active_threads = snapshot.threads.clone();
        Ok(())
    }

    pub fn get_threads_by_color(&self, color: ThreadColor) -> Vec<&FileThread> {
        self.active_threads
            .values()
            .filter(|thread| thread.color_status == color)
            .collect()
    }

    pub fn analyze_thread_convergence(&self, file_path: &str, window_size: usize) -> Result<bool> {
        let thread = self
            .active_threads
            .get(file_path)
            .ok_or_else(|| anyhow!("Thread not found for file: {}", file_path))?;

        if thread.history.len() < window_size {
            return Ok(false);
        }

        let recent_scores: Vec<f64> = thread
            .history
            .iter()
            .rev()
            .take(window_size)
            .map(|state| state.metrics.quality_score)
            .collect();

        // Check for convergence: all recent scores above threshold and trend is stable/improving
        let threshold = 0.8;
        let all_above_threshold = recent_scores.iter().all(|&score| score >= threshold);

        if !all_above_threshold {
            return Ok(false);
        }

        // Check trend stability (variance should be low)
        let mean = recent_scores.iter().sum::<f64>() / recent_scores.len() as f64;
        let variance = recent_scores
            .iter()
            .map(|&score| (score - mean).powi(2))
            .sum::<f64>()
            / recent_scores.len() as f64;

        Ok(variance < 0.01) // Low variance indicates convergence
    }

    pub fn get_thread_statistics(&self) -> ThreadStatistics {
        let total_threads = self.active_threads.len();
        let color_dist = self.get_color_distribution();
        let overall_health = self.get_overall_health();

        let quality_scores: Vec<f64> = self
            .active_threads
            .values()
            .filter_map(|thread| thread.history.last())
            .map(|state| state.metrics.quality_score)
            .collect();

        let avg_quality = if !quality_scores.is_empty() {
            quality_scores.iter().sum::<f64>() / quality_scores.len() as f64
        } else {
            0.0
        };

        ThreadStatistics {
            total_threads,
            red_threads: *color_dist.get(&ThreadColor::Red).unwrap_or(&0),
            orange_threads: *color_dist.get(&ThreadColor::Orange).unwrap_or(&0),
            yellow_threads: *color_dist.get(&ThreadColor::Yellow).unwrap_or(&0),
            light_green_threads: *color_dist.get(&ThreadColor::LightGreen).unwrap_or(&0),
            green_threads: *color_dist.get(&ThreadColor::Green).unwrap_or(&0),
            overall_health,
            average_quality: avg_quality,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadStatistics {
    pub total_threads: usize,
    pub red_threads: usize,
    pub orange_threads: usize,
    pub yellow_threads: usize,
    pub light_green_threads: usize,
    pub green_threads: usize,
    pub overall_health: f64,
    pub average_quality: f64,
}

impl Default for ThreadManager {
    fn default() -> Self {
        Self::new()
    }
}
