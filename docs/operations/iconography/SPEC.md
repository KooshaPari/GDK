# Iconography Standard

Implements the [phenotype-infra iconography standard](https://github.com/kooshapari/phenotype-infra/blob/main/docs/governance/iconography-standard.md).

## GDK-Specific Icons

GDK uses icons primarily in terminal output (ASCII art) and HTML dashboard visualizations. The icon set maps to the thread color system and quality dimensions.

## Color Emoji (Terminal)

GDK's terminal UI uses emoji to represent thread colors and quality states. These are rendered by the `ThreadColor` `Display` impl in `src/lib.rs`:

| Icon | Name | Color | Score Range |
|------|------|-------|-------------|
| `🔴` | Red | Critical | 0.0–0.3 |
| `🟠` | Orange | Warning | 0.3–0.5 |
| `🟡` | Yellow | Caution | 0.5–0.7 |
| `🟢` | Light Green | Good | 0.7–0.9 |
| `💚` | Green | Excellent | 0.9–1.0 |

## Quality Dimension Icons

These emoji appear in HTML dashboards and SVG exports:

| Icon | Dimension | Source |
|------|-----------|--------|
| `🔍` | Lint | CLI output |
| `🛠️` | TypeCheck | CLI output |
| `🧪` | Test | CLI output |
| `🔒` | Security | CLI output |
| `⚡` | Performance | CLI output |
| `📝` | Docs | CLI output |

## SVG Icon Requirements

For HTML dashboard and SVG export icons (from `src/visualization.rs`):

- **Size**: 24×24 viewBox
- **Color**: `currentColor` (inherits from CSS)
- **Role**: `role="img"` with `aria-label`
- **Style**: Fluent (stroke) preferred for consistency with the terminal aesthetic
- **File naming**: `icon-<name>.svg` stored in `docs/operations/iconography/icons/`

## ASCII Art Icons

For ASCII tree rendering in `TreeVisualizer` (`src/visualization.rs`):

| ASCII Style | Characters | Use |
|-------------|------------|-----|
| Unicode (default) | `├──`, `│`, `└` | Box-drawing for tree branches |
| Simple | `|-`, `|`, `\` | Basic ASCII fallback |
| Organic | `+-`, `:`, `` ` `` | Tree-branch aesthetic |

## SVG Icon Catalog

Place new icons under `docs/operations/iconography/icons/`. Each icon must be a self-contained SVG:

```svg
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" role="img" aria-label="Lint">
  <circle cx="11" cy="11" r="8"/>
  <path d="m21 21-4.35-4.35"/>
</svg>
```

## SVG Icon Naming Convention

| Icon | Filename | Purpose |
|------|----------|---------|
| Lint | `icon-lint.svg` | Represents code style and static analysis |
| TypeCheck | `icon-typecheck.svg` | Represents type system checks |
| Test | `icon-test.svg` | Represents test execution |
| Security | `icon-security.svg` | Represents security scanning |
| Performance | `icon-performance.svg` | Represents benchmark results |
| Docs | `icon-docs.svg` | Represents documentation coverage |
| Commit | `icon-commit.svg` | Represents git commit nodes |
| Branch | `icon-branch.svg` | Represents git branches |
| Merge | `icon-merge.svg` | Represents merge commits |
| Checkpoint | `icon-checkpoint.svg` | Represents named revert points |

## Style Policy

- All icons: Fluent (stroke) style unless a filled variant is required
- Color inheritance via `currentColor` for theming
- Consistent 2px stroke weight
- 24×24 viewBox, no hardcoded width/height attributes
- Accessibility: every icon must have `role="img"` and an `aria-label`
