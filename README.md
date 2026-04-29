# GDK — Git Workflow Deep Knowledge

[![AI Slop Inside](https://sladge.net/badge.svg)](https://sladge.net)

GDK is an experimental Rust library and CLI that treats a git repository as a
state-management surface for AI agents: it tracks per-file "quality threads"
(lint, typecheck, tests, security, performance, docs), creates checkpoints,
and explores branches with a convergence loop that snaps back when quality
regresses.

**Status:** Early / exploratory. The crate version `1.0.0` declared in
`Cargo.toml` does not reflect production readiness — recent commits are
baseline org hygiene (MIT LICENSE, monthly SBOM workflow, cargo-deny
baseline, phenotype-tooling CI adoption, `.editorconfig` / `.gitattributes`
templates, Dependabot bumps). Treat the public API, quality scoring, and
convergence behaviour as subject to change.

## What it does

- Models each tracked file as a set of "threads" (lint, typecheck, tests,
  security, performance, docs) with a numeric score per thread.
- Provides checkpoint and spiral-branch primitives: snapshot state, try a
  change, revert when quality regresses below a threshold.
- Generates ASCII / SVG / HTML visualisations of the resulting decision
  tree.
- Tracks concurrent agent sessions for multi-agent workflows.

## Stack

- Rust 2021 edition
- `git2` (libgit2 bindings), `tokio`, `async-trait`, `futures`
- `petgraph` for tree modelling, `rayon` + `dashmap` for parallelism
- `serde` / `serde_json`, `tracing`, `thiserror`, `anyhow`

Source layout (`src/`):

```
agent.rs            convergence.rs      core.rs
errors.rs           git.rs              lib.rs
performance.rs      quality_metrics.rs  threads.rs
validation.rs       visualization.rs    bin/
```

## Build and test

```bash
git clone https://github.com/KooshaPari/GDK.git
cd GDK
cargo build --release
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --check
```

The CLI binary lives under `src/bin/` and is built as part of `cargo build`.

## Usage sketch

The CLI exposes agent-session, checkpoint, spiral, and status subcommands
(see `src/bin/` for the current surface). Example shape:

```bash
gdk-cli init       --agent-id <id>
gdk-cli checkpoint --agent-id <id> --message "<msg>"
gdk-cli spiral     --agent-id <id> --branch-name "<branch>"
gdk-cli status     --agent-id <id>
```

Quality scores produced by GDK are heuristic and depend on the configured
thread checks; they are not certifications and should not be reported as
such.

## Limitations and honest caveats

- Not enterprise- or production-certified. Earlier marketing copy in this
  README has been removed.
- "Quality scores" are computed by whichever thread checks you configure;
  their meaning is bounded by what those checks measure.
- Parallel processing, NUMA placement, and adaptive batching are
  best-effort; no published benchmarks accompany this repository.
- Visualisation outputs (SVG / HTML) require the corresponding renderer
  modules; demo artefacts referenced in older copy may not be present.

## Documentation

- `SPEC.md` — design notes
- `PLAN.md` — work plan
- `FUNCTIONAL_REQUIREMENTS.md` — FR scaffolding (in progress)
- `INSTALL.md` — installation notes
- `ENTERPRISE.md`, `SECURITY.md` — operational notes
- `AGENTS.md`, `CONTRIBUTING.md`, `CHANGELOG.md`
- `docs/` — additional documentation

## License

Dual-licensed under MIT or Apache-2.0 at your option. See `LICENSE-MIT` and
`LICENSE-APACHE`.
