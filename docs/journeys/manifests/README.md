# Journey Manifests

This directory contains journey manifests documenting GDK's key user-facing flows as part of the [phenotype-infra journey-traceability standard](https://github.com/kooshapari/phenotype-infra/blob/main/docs/governance/journey-traceability-standard.md).

## What Are Journey Manifests

A journey manifest is a structured record of a user-facing workflow. It captures:

- **Flow name and description**: What the user does and why
- **Keyframes**: Terminal screenshots (VHS recordings) at critical decision points
- **Traceability links**: References to the FRs and source files that implement each step
- **Metadata**: Persona, frequency, success criteria

Manifests live here as `.json` files and are verified in CI via `phenotype-journey verify`.

## Key User Flows in GDK

GDK's primary user-facing flows are:

| Flow | Description | CLI Entry Point |
|------|-------------|-----------------|
| `gdk spiral` | Run infinite monkey convergence on a branch | `src/bin/main.rs` `Commands::Spiral` |
| `gdk visualize` | Render commit tree as ASCII/SVG/HTML | `src/bin/main.rs` `Commands::Visualize` |
| `gdk checkpoint` | Create a named revert point | `src/bin/main.rs` `Commands::Checkpoint` |
| `gdk revert` | Revert to the last checkpoint | `src/bin/main.rs` `Commands::Revert` |
| `gdk status` | Inspect convergence status for an agent | `src/bin/main.rs` `Commands::Status` |
| `gdk suggest` | Get AI-recommended next action | `src/bin/main.rs` `Commands::Suggest` |

## Manifest File Format

Each manifest is a JSON file named after the flow (e.g., `spiral-flow.json`):

```json
{
  "flow_id": "spiral-flow",
  "flow_name": "Infinite Monkey Spiral",
  "description": "Run the convergence algorithm until quality threshold is met or max attempts reached.",
  "persona": "AI Agent",
  "frequency": "per-feature",
  "steps": [
    {
      "step": 1,
      "action": "gdk spiral --agent-id my-agent --branch-name feature-x",
      "keyframe": "keyframes/spiral-start.png",
      "frs": ["FR-004", "FR-005"],
      "source_files": ["src/agent.rs", "src/convergence.rs"]
    }
  ],
  "success_criteria": "Commit converges (is_converged=true) or max_attempts exhausted"
}
```

## Status

- [x] Identify key user-facing flows (6 flows documented above)
- [ ] Record VHS tapes for each flow
- [ ] Author manifests in `docs/journeys/manifests/`
- [ ] Run `phenotype-journey verify` in CI
