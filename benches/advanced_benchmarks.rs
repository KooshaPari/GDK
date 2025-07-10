//! Advanced performance benchmarks for GDK system
//!
//! These benchmarks measure performance across various scenarios:
//! - Large repository handling (10k+ commits)
//! - Concurrent multi-agent operations
//! - Memory usage patterns and optimization
//! - Streaming vs batch processing comparison
//! - Cache effectiveness and hit ratios

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use gdk::performance::{ParallelCommitProcessor, ConcurrentThreadManager, StreamingAnalyzer};
use gdk::{CommitNode, FileThread, ThreadColor};
use std::collections::HashMap;
use uuid::Uuid;

/// Generate realistic test data for benchmarking
fn generate_test_commits(count: usize) -> Vec<CommitNode> {
    (0..count)
        .map(|i| {
            let mut file_threads = HashMap::new();
            
            // Generate 5-20 files per commit (realistic range)
            let file_count = 5 + (i % 16);
            for j in 0..file_count {
                let file_path = format!("src/module_{}/file_{}.rs", i % 10, j);
                let thread = FileThread {
                    file_path: file_path.clone(),
                    thread_id: Uuid::new_v4(),
                    color_status: ThreadColor::Green,
                    lint_score: 0.8 + (i as f64 % 0.2),
                    type_check_score: 0.9 + (i as f64 % 0.1),
                    test_coverage: 0.7 + (i as f64 % 0.3),
                    functionality_score: 0.85 + (i as f64 % 0.15),
                    history: Vec::new(),
                };
                file_threads.insert(file_path, thread);
            }

            CommitNode {
                id: format!("commit-{}", i),
                hash: format!("abc123def456_{:08x}", i),
                parent_hashes: if i == 0 { 
                    Vec::new() 
                } else { 
                    vec![format!("abc123def456_{:08x}", i - 1)]
                },
                message: format!("Commit message {}", i),
                timestamp: 1640995200 + (i as u64 * 3600), // One hour apart
                file_threads,
                health_score: 0.8 + (i as f64 % 0.2),
                convergence_metrics: gdk::ConvergenceMetrics {
                    attempts: 1,
                    successful_builds: 1,
                    test_pass_rate: 0.85,
                    quality_trend: vec![0.8, 0.82, 0.85],
                    is_converged: true,
                },
            }
        })
        .collect()
}

/// Benchmark parallel commit processing at various scales
fn bench_parallel_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel_processing");
    
    for size in [100, 500, 1000, 5000, 10000].iter() {
        let commits = generate_test_commits(*size);
        let processor = ParallelCommitProcessor::new();
        
        group.bench_with_input(
            BenchmarkId::new("parallel_commit_analysis", size),
            size,
            |b, _| {
                b.to_async(tokio::runtime::Runtime::new().unwrap())
                    .iter(|| async {
                        let result = processor
                            .process_commits_parallel(&commits, |commit| {
                                // Simulate realistic processing work
                                let health_sum: f64 = commit
                                    .file_threads
                                    .values()
                                    .map(|t| {
                                        (t.lint_score + t.type_check_score + 
                                         t.test_coverage + t.functionality_score) / 4.0
                                    })
                                    .sum();
                                
                                Ok(black_box(health_sum / commit.file_threads.len() as f64))
                            })
                            .await;
                        
                        black_box(result)
                    });
            },
        );
    }
    group.finish();
}

/// Benchmark sequential vs parallel processing comparison
fn bench_sequential_vs_parallel(c: &mut Criterion) {
    let mut group = c.benchmark_group("sequential_vs_parallel");
    let commits = generate_test_commits(1000);
    
    // Sequential processing
    group.bench_function("sequential_processing", |b| {
        b.iter(|| {
            let results: Vec<f64> = commits
                .iter()
                .map(|commit| {
                    let health_sum: f64 = commit
                        .file_threads
                        .values()
                        .map(|t| {
                            (t.lint_score + t.type_check_score + 
                             t.test_coverage + t.functionality_score) / 4.0
                        })
                        .sum();
                    health_sum / commit.file_threads.len() as f64
                })
                .collect();
            
            black_box(results)
        });
    });
    
    // Parallel processing
    let processor = ParallelCommitProcessor::new();
    group.bench_function("parallel_processing", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                let result = processor
                    .process_commits_parallel(&commits, |commit| {
                        let health_sum: f64 = commit
                            .file_threads
                            .values()
                            .map(|t| {
                                (t.lint_score + t.type_check_score + 
                                 t.test_coverage + t.functionality_score) / 4.0
                            })
                            .sum();
                        
                        Ok(health_sum / commit.file_threads.len() as f64)
                    })
                    .await;
                
                black_box(result)
            });
    });
    
    group.finish();
}

/// Benchmark concurrent thread management
fn bench_concurrent_thread_management(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_thread_management");
    let manager = ConcurrentThreadManager::new();
    
    // Generate thread updates
    let updates: Vec<_> = (0..1000)
        .map(|i| gdk::performance::ThreadUpdate {
            file_path: format!("src/file_{}.rs", i % 100),
            lint_score: 0.8 + (i as f64 % 0.2),
            type_check_score: 0.9,
            test_coverage: 0.7,
            functionality_score: 0.85,
        })
        .collect();
    
    group.bench_function("batch_thread_updates", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                let result = manager.update_threads_batch(&updates).await;
                black_box(result)
            });
    });
    
    // Benchmark cached thread access
    group.bench_function("cached_thread_access", |b| {
        b.iter(|| {
            let mut results = Vec::new();
            for i in 0..100 {
                let file_path = format!("src/file_{}.rs", i);
                let color = manager.get_thread_cached(&file_path);
                results.push(color);
            }
            black_box(results)
        });
    });
    
    group.finish();
}

/// Benchmark streaming vs batch analysis
fn bench_streaming_vs_batch(c: &mut Criterion) {
    let mut group = c.benchmark_group("streaming_vs_batch");
    let commits = generate_test_commits(5000);
    
    // Streaming analysis (constant memory)
    group.bench_function("streaming_analysis", |b| {
        b.iter(|| {
            let mut analyzer = StreamingAnalyzer::new(50);
            let mut results = Vec::new();
            
            for commit in &commits {
                let result = analyzer.process_commit_streaming(commit).unwrap();
                results.push(result);
            }
            
            black_box(results)
        });
    });
    
    // Batch analysis (loads all into memory)
    group.bench_function("batch_analysis", |b| {
        b.iter(|| {
            // Simulate batch processing by computing statistics on full dataset
            let health_scores: Vec<f64> = commits
                .iter()
                .map(|c| c.health_score)
                .collect();
            
            let avg = health_scores.iter().sum::<f64>() / health_scores.len() as f64;
            let variance = health_scores
                .iter()
                .map(|&x| (x - avg).powi(2))
                .sum::<f64>() / health_scores.len() as f64;
            
            black_box((avg, variance))
        });
    });
    
    group.finish();
}

/// Benchmark memory usage patterns
fn bench_memory_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_patterns");
    
    // Small vector optimization benchmark
    group.bench_function("smallvec_optimization", |b| {
        b.iter(|| {
            let mut results = Vec::new();
            
            for i in 0..1000 {
                // SmallVec for small collections (on stack)
                let mut small_vec: smallvec::SmallVec<[u32; 8]> = smallvec::SmallVec::new();
                for j in 0..5 {
                    small_vec.push(i * j);
                }
                results.push(small_vec.len());
            }
            
            black_box(results)
        });
    });
    
    // Regular Vec benchmark for comparison
    group.bench_function("regular_vec", |b| {
        b.iter(|| {
            let mut results = Vec::new();
            
            for i in 0..1000 {
                let mut vec = Vec::new();
                for j in 0..5 {
                    vec.push(i * j);
                }
                results.push(vec.len());
            }
            
            black_box(results)
        });
    });
    
    group.finish();
}

/// Benchmark cache effectiveness
fn bench_cache_effectiveness(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_effectiveness");
    let processor = ParallelCommitProcessor::new();
    let commits = generate_test_commits(1000);
    
    // Cold cache performance
    group.bench_function("cold_cache", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                processor.reset(); // Clear cache
                let result = processor
                    .process_commits_parallel(&commits[..100], |commit| {
                        Ok(commit.health_score)
                    })
                    .await;
                black_box(result)
            });
    });
    
    // Warm cache performance (process same commits multiple times)
    group.bench_function("warm_cache", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                // Don't reset cache - simulate repeated access patterns
                let result = processor
                    .process_commits_parallel(&commits[..100], |commit| {
                        Ok(commit.health_score)
                    })
                    .await;
                black_box(result)
            });
    });
    
    group.finish();
}

/// Benchmark JSON serialization performance
fn bench_serialization_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialization");
    let commits = generate_test_commits(100);
    
    // Standard serde_json
    group.bench_function("serde_json", |b| {
        b.iter(|| {
            let json = serde_json::to_string(&commits).unwrap();
            let _deserialized: Vec<CommitNode> = serde_json::from_str(&json).unwrap();
            black_box(json.len())
        });
    });
    
    // SIMD JSON (if available)
    #[cfg(feature = "simd")]
    group.bench_function("simd_json", |b| {
        b.iter(|| {
            let json = serde_json::to_string(&commits).unwrap();
            let mut json_bytes = json.into_bytes();
            let parsed = simd_json::from_slice::<Vec<CommitNode>>(&mut json_bytes).unwrap();
            black_box(parsed.len())
        });
    });
    
    group.finish();
}

/// Benchmark concurrent access patterns
fn bench_concurrent_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_access");
    let manager = ConcurrentThreadManager::new();
    
    // Simulate multiple agents accessing threads concurrently
    group.bench_function("multi_agent_access", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                let tasks = (0..10).map(|agent_id| {
                    let manager = &manager;
                    async move {
                        let mut results = Vec::new();
                        for i in 0..100 {
                            let file_path = format!("agent_{}_file_{}.rs", agent_id, i % 20);
                            let color = manager.get_thread_cached(&file_path);
                            results.push(color);
                        }
                        results
                    }
                });
                
                let all_results = futures::future::join_all(tasks).await;
                black_box(all_results.len())
            });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_parallel_processing,
    bench_sequential_vs_parallel,
    bench_concurrent_thread_management,
    bench_streaming_vs_batch,
    bench_memory_patterns,
    bench_cache_effectiveness,
    bench_serialization_performance,
    bench_concurrent_access
);

criterion_main!(benches);