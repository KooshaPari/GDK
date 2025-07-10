//! Performance benchmarks for GDK operations
//!
//! These benchmarks measure:
//! - ThreadColor calculation performance
//! - Convergence analysis speed
//! - Serialization/deserialization throughput
//! - Large repository handling
//! - Memory usage patterns

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use gdk::{ThreadColor, ConvergenceMetrics, CommitNode, FileThread};
use std::collections::HashMap;
use uuid::Uuid;

/// Benchmark ThreadColor calculations for different input patterns
fn bench_thread_color_calculation(c: &mut Criterion) {
    let mut group = c.benchmark_group("thread_color_calculation");
    
    // Test single calculation
    group.bench_function("single_calculation", |b| {
        b.iter(|| {
            ThreadColor::from_scores(
                black_box(0.85),
                black_box(0.92),
                black_box(0.78),
                black_box(0.88),
            )
        })
    });
    
    // Test batch calculations with different sizes
    for size in [100, 1000, 10000] {
        group.bench_with_input(BenchmarkId::new("batch_calculation", size), &size, |b, &size| {
            let scores: Vec<(f64, f64, f64, f64)> = (0..size)
                .map(|i| {
                    let base = (i as f64) / (size as f64);
                    (base, base + 0.1, base + 0.05, base + 0.15)
                })
                .collect();
            
            b.iter(|| {
                for &(lint, type_check, test, func) in &scores {
                    black_box(ThreadColor::from_scores(
                        black_box(lint % 1.0),
                        black_box(type_check % 1.0),
                        black_box(test % 1.0),
                        black_box(func % 1.0),
                    ));
                }
            })
        });
    }
    
    group.finish();
}

/// Benchmark convergence analysis with different history sizes
fn bench_convergence_analysis(c: &mut Criterion) {
    let mut group = c.benchmark_group("convergence_analysis");
    
    for history_size in [10, 50, 100, 500] {
        group.bench_with_input(
            BenchmarkId::new("analyze_trend", history_size),
            &history_size,
            |b, &size| {
                // Create realistic quality trend data
                let quality_trend: Vec<f64> = (0..size)
                    .map(|i| {
                        let progress = (i as f64) / (size as f64);
                        // Simulate improving quality with some noise
                        0.5 + (progress * 0.4) + (0.1 * (i as f64 * 0.1).sin())
                    })
                    .collect();
                
                b.iter(|| {
                    // Simulate convergence analysis logic
                    let recent_commits = quality_trend.iter().rev().take(10);
                    let recent_avg: f64 = recent_commits.clone().sum::<f64>() / recent_commits.count().max(1) as f64;
                    
                    let is_converged = if quality_trend.len() >= 3 {
                        recent_avg > 0.8 && quality_trend.windows(2).all(|w| w[0] <= w[1])
                    } else {
                        false
                    };
                    
                    black_box(ConvergenceMetrics {
                        attempts: size as u32,
                        successful_builds: quality_trend.iter().filter(|&&q| q > 0.7).count() as u32,
                        test_pass_rate: recent_avg,
                        quality_trend: quality_trend.clone(),
                        is_converged,
                    })
                })
            }
        );
    }
    
    group.finish();
}

/// Benchmark serialization performance for different data sizes
fn bench_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialization");
    
    // Create test data of different sizes
    for thread_count in [1, 10, 50, 100] {
        let mut file_threads = HashMap::new();
        
        for i in 0..thread_count {
            let thread = FileThread {
                file_path: format!("src/file_{}.rs", i),
                thread_id: Uuid::new_v4(),
                color_status: ThreadColor::Green,
                lint_score: 0.95,
                type_check_score: 0.98,
                test_coverage: 0.85,
                functionality_score: 0.92,
                history: vec![], // Empty for benchmark simplicity
            };
            file_threads.insert(format!("src/file_{}.rs", i), thread);
        }
        
        let commit = CommitNode {
            id: Uuid::new_v4().to_string(),
            hash: "abc123def456".to_string(),
            parent_hashes: vec!["parent123".to_string()],
            message: "Benchmark commit".to_string(),
            timestamp: 1234567890,
            file_threads,
            health_score: 0.92,
            convergence_metrics: ConvergenceMetrics {
                attempts: 5,
                successful_builds: 5,
                test_pass_rate: 1.0,
                quality_trend: vec![0.8, 0.85, 0.9, 0.92, 0.92],
                is_converged: true,
            },
        };
        
        // Benchmark JSON serialization
        group.bench_with_input(
            BenchmarkId::new("json_serialize", thread_count),
            &commit,
            |b, commit| {
                b.iter(|| {
                    black_box(serde_json::to_string(commit).unwrap())
                })
            }
        );
        
        // Benchmark JSON deserialization
        let json = serde_json::to_string(&commit).unwrap();
        group.bench_with_input(
            BenchmarkId::new("json_deserialize", thread_count),
            &json,
            |b, json| {
                b.iter(|| {
                    black_box(serde_json::from_str::<CommitNode>(json).unwrap())
                })
            }
        );
    }
    
    group.finish();
}

/// Benchmark color score conversion performance
fn bench_color_score_conversion(c: &mut Criterion) {
    let mut group = c.benchmark_group("color_score_conversion");
    
    let colors = vec![
        ThreadColor::Red,
        ThreadColor::Orange,
        ThreadColor::Yellow,
        ThreadColor::LightGreen,
        ThreadColor::Green,
    ];
    
    group.bench_function("to_score_conversion", |b| {
        b.iter(|| {
            for color in &colors {
                black_box(color.to_score());
            }
        })
    });
    
    group.bench_function("from_scores_conversion", |b| {
        b.iter(|| {
            for i in 0..colors.len() {
                let score = (i as f64) / (colors.len() - 1) as f64;
                black_box(ThreadColor::from_scores(score, score, score, score));
            }
        })
    });
    
    group.finish();
}

/// Benchmark large file thread collection operations
fn bench_large_collections(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_collections");
    
    for size in [100, 500, 1000, 5000] {
        // Create large collection of file threads
        let mut file_threads = HashMap::new();
        for i in 0..size {
            let thread = FileThread {
                file_path: format!("src/module_{}/file_{}.rs", i / 100, i % 100),
                thread_id: Uuid::new_v4(),
                color_status: ThreadColor::from_scores(
                    0.8 + (i as f64 % 100.0) / 500.0,
                    0.9,
                    0.85,
                    0.88,
                ),
                lint_score: 0.8 + (i as f64 % 100.0) / 500.0,
                type_check_score: 0.9,
                test_coverage: 0.85,
                functionality_score: 0.88,
                history: vec![],
            };
            file_threads.insert(thread.file_path.clone(), thread);
        }
        
        // Benchmark health score calculation
        group.bench_with_input(
            BenchmarkId::new("health_score_calculation", size),
            &file_threads,
            |b, threads| {
                b.iter(|| {
                    let health_score = threads
                        .values()
                        .map(|t| t.functionality_score)
                        .sum::<f64>()
                        / threads.len().max(1) as f64;
                    black_box(health_score)
                })
            }
        );
        
        // Benchmark thread color distribution analysis
        group.bench_with_input(
            BenchmarkId::new("color_distribution", size),
            &file_threads,
            |b, threads| {
                b.iter(|| {
                    let mut color_counts = HashMap::new();
                    for thread in threads.values() {
                        *color_counts.entry(thread.color_status.clone()).or_insert(0) += 1;
                    }
                    black_box(color_counts)
                })
            }
        );
    }
    
    group.finish();
}

/// Benchmark memory allocation patterns
fn bench_memory_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_patterns");
    
    // Benchmark UUID generation (used frequently in GDK)
    group.bench_function("uuid_generation", |b| {
        b.iter(|| {
            black_box(Uuid::new_v4())
        })
    });
    
    // Benchmark HashMap creation and insertion
    group.bench_function("hashmap_creation", |b| {
        b.iter(|| {
            let mut map = HashMap::new();
            for i in 0..100 {
                map.insert(format!("key_{}", i), i);
            }
            black_box(map)
        })
    });
    
    // Benchmark string formatting (used in error messages)
    group.bench_function("string_formatting", |b| {
        b.iter(|| {
            for i in 0..100 {
                black_box(format!("Operation {} failed with code {}", 
                    black_box(format!("test_{}", i)), 
                    black_box(i % 10)
                ));
            }
        })
    });
    
    group.finish();
}

/// Benchmark thread color display formatting
fn bench_color_display(c: &mut Criterion) {
    let mut group = c.benchmark_group("color_display");
    
    let colors = vec![
        ThreadColor::Red,
        ThreadColor::Orange,
        ThreadColor::Yellow,
        ThreadColor::LightGreen,
        ThreadColor::Green,
    ];
    
    group.bench_function("display_formatting", |b| {
        b.iter(|| {
            for color in &colors {
                black_box(format!("{}", color));
            }
        })
    });
    
    group.bench_function("debug_formatting", |b| {
        b.iter(|| {
            for color in &colors {
                black_box(format!("{:?}", color));
            }
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_thread_color_calculation,
    bench_convergence_analysis,
    bench_serialization,
    bench_color_score_conversion,
    bench_large_collections,
    bench_memory_patterns,
    bench_color_display
);

criterion_main!(benches);