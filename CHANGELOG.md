# Changelog

All notable changes to CPOS are documented here. Components are versioned independently:

| Component | Version file |
| --- | --- |
| Terminal app | `Cargo.toml` |
| VS Code extension | `extensions/vscode/package.json` |
| Browser companion | `extensions/chrome/manifest.json` |

Format loosely follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added
- Community docs: `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`, `ARCHITECTURE.md`
- GitHub Actions CI (`cargo test`, VS Code extension compile)
- Issue and pull request templates

### Changed
- Landing page redesign with optimized WebP screenshots and ecosystem section

## Terminal app — 0.1.0

### Added
- TUI with Dashboard, Problems, Contests, Analytics, and Recommend tabs
- Local problem browser with search, rating filter, and platform switch
- Codeforces sync (problems, submissions, rating, contests)
- CSES progress sync via browser companion
- Recommendation engine (30 unsolved problems targeting weak tags)
- Localhost capture server on port `27121`
- Sample test runner and browser submit autofill

## VS Code extension — 0.3.7

### Added
- CPOS side panel: run samples, submit, open problem
- Browser capture server on port `27122`
- Auto file creation in open workspace folder
- Per-file sample storage and inline pass/fail results
- Configurable language, compile commands, and save location
- Sync with TUI over localhost

## Browser companion — 0.6.1

### Added
- Sample capture from Codeforces and CSES problem pages
- Submit form autofill when submitting from CPOS
- CSES solved/attempted progress scraping
- Localhost-only communication (`127.0.0.1:27121` / `27122`)
- Published on Chrome Web Store

---

When cutting a release, add a dated section under `[Unreleased]` and link the GitHub Release tag.
