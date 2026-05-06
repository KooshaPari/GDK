# DAG WBS

## Work Breakdown

1. Confirm canonical dirty state and active branch.
2. Create isolated worktree from active `ci/pin-trufflehog` branch.
3. Add Sladge badge to README badge block.
4. Record governance evidence in `docs/worklogs/GOVERNANCE.md`.
5. Validate diff hygiene and README badge presence.
6. Commit isolated downstream work with required trailer.
7. Update projects-landing governance/tasks ledgers.

## Dependencies

The projects-landing ledger update depends on the downstream GDK commit hash
and validation result.
