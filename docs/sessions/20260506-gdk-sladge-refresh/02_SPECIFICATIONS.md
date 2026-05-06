# Specifications

## Requirement

Current GDK branch evidence must disclose LLM-heavy/agent-oriented authorship
and runtime ownership with the Sladge badge near the README badge block.

## Acceptance Criteria

- `README.md` contains `https://sladge.net/badge.svg`.
- The change is prepared in an isolated worktree because canonical GDK has
  unrelated local changes.
- Validation records diff hygiene and badge-presence proof.

## ARUs

- Assumption: Badge-only documentation work does not require broad Rust builds.
- Risk: The README is already over the repository's line-count target.
- Mitigation: Keep the README edit minimal and document the pre-existing size
  issue instead of combining this governance repair with a large decomposition.
