# Journey Traceability

## Standard

GDK follows the [phenotype-infra journey-traceability standard](https://github.com/kooshapari/phenotype-infra/blob/main/docs/governance/journey-traceability-standard.md).

## What Is Traced

Journey traceability links user-facing CLI flows to:

1. **Functional requirements (FRs)** in `FUNCTIONAL_REQUIREMENTS.md`
2. **Source implementation** in `src/`
3. **Behavioral evidence** in tests and recordings

## GDK User Journey Matrix

| Journey | FRs | Source Files | Test Coverage |
|---------|-----|--------------|---------------|
| Init Agent | FR-003, FR-007 | `src/bin/main.rs`, `src/agent.rs`, `src/core.rs` | `tests/smoke_test.rs` |
| Create Checkpoint | FR-005, FR-007 | `src/bin/main.rs`, `src/agent.rs`, `src/core.rs`, `src/git.rs` | `tests/integration_tests.rs` |
| Spiral Convergence | FR-002, FR-004, FR-005 | `src/agent.rs`, `src/convergence.rs`, `src/core.rs`, `src/bin/main.rs` | `tests/integration_tests.rs` |
| Revert to Checkpoint | FR-005, FR-007 | `src/bin/main.rs`, `src/agent.rs`, `src/core.rs` | `tests/integration_tests.rs` |
| Thread Visualization | FR-006 | `src/visualization.rs`, `src/bin/main.rs`, `src/lib.rs` | `tests/unit_tests.rs` |
| Quality / Status Reporting | FR-002, FR-007 | `src/bin/main.rs`, `src/convergence.rs`, `src/threads.rs` | `tests/unit_tests.rs` |

## Keyframes (Evidence Requirements)

For each journey, capture VHS evidence at decision points:

- **Entry state** (before command execution)
- **Action state** (immediately after command)
- **Result state** (success, failure, or retry)
- **Exit state** (post-state and artifacts)

Examples:

- `init-agent.entry.png`
- `init-agent.action.png`
- `init-agent.result.png`

## Verification and Governance

- Keep manifest records under `docs/journeys/manifests/` and link them from this matrix.
- Run `phenotype-journey verify` in CI.
- Ensure every user-facing command in `src/bin/main.rs` has at least one corresponding journey and one executable/test trace.
- Update `FUNCTIONAL_REQUIREMENTS.md` whenever new FRs or tests are introduced.
