# Changelog

All notable changes to CPOS are documented here. Components are versioned independently:

| Component | Current version | Version file |
| --- | --- | --- |
| Terminal app | 0.1.0 | `Cargo.toml` |
| VS Code extension | 0.3.13 | `extensions/vscode/package.json` |
| Browser companion | 0.6.6 | `extensions/chrome/manifest.json` |

Format loosely follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Terminal app
- Opening a problem with **`o`** or Enter now prefers your **active project directory** when VS Code has synced a solution path, a recent session points outside the default `~/cpos` tree, or the shell working directory looks like a project.

### Added
- Community documentation: `CONTRIBUTING.md`, `SECURITY.md`, `ARCHITECTURE.md`
- GitHub Actions CI (`cargo test`, VS Code extension compile)
- Issue and pull request templates
- Terminal app: `plain` theme (neutral grayscale palette)

### Changed
- Landing page redesign with optimized WebP screenshots

---

## Browser companion — 0.6.6

### Fixed
- **Codeforces submit:** reliable autofill by setting the source textarea first, then language and problem fields, then activating the submit control—without resetting the Ace editor
- **Language selection:** numeric program type id when available, with ranked fallback on compiler display names when Codeforces updates ids
- **Retries:** up to eight injection attempts while the submit page loads
- **Concurrency:** background worker is the sole submit handler; avoids conflicting fills from the isolated content-script world

## Browser companion — 0.6.3 – 0.6.5

### Added
- Capture of Codeforces **sub-test-case block metadata** (`test-example-line-*`) for aligned input/output highlighting in the VS Code panel

### Fixed
- Codeforces submit uses main-world injection and extended page-load timing
- CSES submit behavior unchanged

## Browser companion — 0.6.2

### Fixed
- Codeforces submit on contest pages (`submittedProblemIndex`) and problemset pages (`submittedProblemCode`)

## Browser companion — 0.6.1

### Added
- Sample capture from Codeforces and CSES problem pages
- Submit form autofill when submitting from CPOS
- CSES solved and attempted progress scraping
- Localhost-only communication (`127.0.0.1:27121` / `27122`)
- Initial Chrome Web Store release

---

## VS Code extension — 0.3.13

### Added
- **Test-case panel:** per-block striping, gutter labels, and linked highlight between input blocks and expected output lines when block metadata is present
- **Resizable columns** between input and expected output; ratio persisted in panel state
- Wider default width for multi-line sample input

### Fixed
- Duplicate text rendering on striped input rows (decoration layer shows stripes only; editable text remains in the textarea)

## VS Code extension — 0.3.10 – 0.3.12

### Added
- Resizable input and output layout in the test panel (0.3.10)

### Fixed
- Visual artifact on striped input rows (0.3.12)

## VS Code extension — 0.3.9

### Fixed
- Submit coordination with the browser companion (localhost queue and submit URL)

## VS Code extension — 0.3.8

### Added
- Panel themes: **CPOS**, **Midnight**, **Amber**, **Paper**, and **Native** (follows the active VS Code theme); selection persisted via the theme control in the panel header

### Changed
- Refreshed panel visual design: improved contrast, card layout, branded header

## VS Code extension — 0.3.7

### Added
- CPOS side panel: run samples, submit, open problem
- Browser capture server on port `27122`
- Automatic file creation in the open workspace folder
- Per-file sample storage and inline pass/fail results
- Configurable language, compile commands, and save location
- Localhost sync with the terminal app

---

## Terminal app — 0.1.0

### Added
- TUI with Dashboard, Problems, Contests, Analytics, and Recommend tabs
- Local problem browser with search, rating filter, and platform filter
- Codeforces sync (problems, submissions, rating, contests)
- CSES progress sync via the browser companion
- Recommendation engine (30 unsolved problems targeting weak topics)
- Localhost capture server on port `27121`
- Local sample test runner; submit via the browser companion

---

When cutting a release, add a dated section, bump the version in the component manifest or `Cargo.toml`, and publish a GitHub release.
