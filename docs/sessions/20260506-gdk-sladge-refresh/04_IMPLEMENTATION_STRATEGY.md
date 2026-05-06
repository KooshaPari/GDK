# Implementation Strategy

## Approach

Apply the badge in the existing README badge block so the disclosure is visible
near the repository identity and existing status badges.

## Isolation

Use `GDK-wtrees/gdk-sladge-current` on branch `docs/gdk-sladge-current` because
canonical GDK has unrelated local modifications.

## Validation

Run targeted documentation validation: `git diff --check`, README badge search,
and `git status`. Broad Cargo validation is deferred for this badge-only lane
because the current machine has already hit disk exhaustion on build caches.
