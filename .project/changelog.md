# Heavenly Mandate - Changelog

> **Document Location:** `.project/changelog.md`
>
> All notable changes to this project will be documented in this file.
> The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

---

## [Unreleased]

### Added
- Phase 7 (Deep Systems) and Phase 8 (Polish) to `build-plan.md` to cover missing GDD features.
- Compilation fix in `game.rs` for `Game::new` and imports.

### Changed
- Updated `build-plan.md` progress summary.

### Fixed
- Unclosed delimiter error in `src/game.rs`.

---

## [0.1.0] - 2026-01-24

### Added
- Initialized the project's documentation framework in the `.project/` directory.
- Created `prd.md` to define the product requirements for the MVP.
- Created `tech-stack.md` to document all technology and architecture decisions.
- Created `build-plan.md` to establish a phased task list for MVP development.
- Created this `changelog.md` to track project history.

---

## Version Guidelines

### Version Format: `MAJOR.MINOR.PATCH`

- **MAJOR**: Breaking changes or significant milestones.
- **MINOR**: New features, completed phases.
- **PATCH**: Bug fixes, small improvements.

### Change Types

| Type | Description |
|------|-------------|
| **Added** | New features or capabilities. |
| **Changed** | Changes to existing functionality. |
| **Deprecated** | Features marked for removal. |
| **Removed** | Features that were removed. |
| **Fixed** | Bug fixes. |
| **Security** | Security-related changes. |

---
*Last updated: 2026-01-24*