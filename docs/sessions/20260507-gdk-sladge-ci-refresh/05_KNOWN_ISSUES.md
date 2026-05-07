# Known Issues

## Superseded Branch

The older `docs/gdk-sladge-current` branch at `6a8c9bf` diverged from current
canonical head and should be treated as stale evidence after this refresh.

## Oversized README

`README.md` is pre-existing over the 500-line governance target. This
badge-only update does not decompose the README.

## Pre-Existing Rust Gates

`cargo fmt --all --check` reports formatting drift across benches, source, and
tests outside the README/worklog change.

`cargo test --workspace --offline` compiles but fails existing integration test
`test_file_thread_creation` at `tests/integration_tests.rs:275` because
`commit.file_threads` is empty.
