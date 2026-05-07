# Specifications

## Acceptance Criteria

- The README shows the Sladge badge near the existing status badges.
- The refresh starts from current canonical `ci/pin-trufflehog`.
- Governance evidence records the superseded prepared branch.
- Validation records whitespace, badge presence, and Rust quality-gate results
  or exact blockers.

## ARUs

- Assumption: This is a documentation/governance disclosure only.
- Risk: The older `6a8c9bf` prepared branch may be selected accidentally unless
  `projects-landing` is updated after this refresh.
- Uncertainty: Broad Rust validation may expose pre-existing issues unrelated to
  this README/session-doc change.
