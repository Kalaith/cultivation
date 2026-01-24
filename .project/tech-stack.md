# Heavenly Mandate - Tech Stack

> **Document Location:** `.project/tech-stack.md`
>
> This document outlines the technology choices and rationale for the project, aligned with the requirements in `prd.md`.

---

## Stack Overview

```
┌─────────────────────────────────────────────────┐
│                   Frontend                       │
│  Rust + Macroquad + macroquad-toolkit (Immediate Mode UI)      │
├─────────────────────────────────────────────────┤
│                    Backend                       │
│  All logic contained within the Rust application       │
├─────────────────────────────────────────────────┤
│                   Data Layer                     │
│  JSON Files + serde_json                          │
├─────────────────────────────────────────────────┤
│                Infrastructure                    │
│  GitHub Actions (CI/CD) + Itch.io/Steam (Hosting)       │
└─────────────────────────────────────────────────┘
```

---

## Core Technologies

### Language & Runtime

| Technology | Version | Purpose |
|------------|---------|---------|
| Rust | 2021 Edition | Primary language for all game logic and rendering. |
| WASM | | Target binary format for WebGL deployment. |

**Rationale:**
- **Rust:** Provides memory safety and high performance, which are crucial for game development. Its strong type system helps prevent common bugs in complex, systems-driven games.
- **WASM:** Ensures the game is accessible via web browsers, as specified in the PRD.

---

### Framework

| Technology | Version | Purpose |
|------------|---------|---------|
| Macroquad | 0.4 | Thin layer for rendering, input, and audio. |

**Rationale:**
- Macroquad is simple, lightweight, and has excellent cross-platform support for both native (Windows) and web (WebGL) targets. This directly maps to the PRD's platform requirements.
- Its "immediate mode" philosophy aligns with our goal to keep the rendering layer thin and the game logic separate and explicit. We will avoid using it for state management or complex scene graphs.

---

### Database

| Technology | Version | Purpose |
|------------|---------|---------|
| JSON Files | N/A | Primary store for game definitions and save files. |
| serde / serde_json | 1.0 | Serialization/deserialization of Rust structs to/from JSON. |

**Rationale:**
- JSON is human-readable and easy to edit, making it ideal for defining game data (missions, buildings, traits) during development.
- Using `serde` for serializing the entire game state to a single JSON file provides a robust, ironman-friendly save system as required by the PRD.

---

## Dependencies

### Production Dependencies

| Package | Version | Purpose |
|---------|---------|---------|
| `macroquad` | "0.4" | Rendering, input, audio. |
| `macroquad-toolkit` | local path | Immediate-mode UI components. |
| `serde` | "1.0" | Data serialization/deserialization framework. (features: `derive`) |
| `serde_json` | "1.0" | JSON implementation for serde. |
| `rand` | "0.8" | Random number generation for procedural elements. |

### Development Dependencies

| Package | Version | Purpose |
|---------|---------|---------|
| `cargo-watch` | latest | Automatically recompile on code changes. |

---

## Build & Tooling

### Build System

| Tool | Version | Purpose |
|------|---------|---------|
| Cargo | (Bundled with Rust) | Build system and package manager. |

### Development Tools

| Tool | Purpose |
|------|---------|
| `rustfmt` | Code formatting to maintain a consistent style. |
| `clippy` | Code linting to catch common errors and improve idioms. |
| `rustc` | The Rust compiler provides core type checking. |

### Build Commands

```bash
# Run in debug mode for development
cargo run

# Create a release build for Windows
cargo build --release

# Create a release build for WebGL/WASM
cargo build --release --target wasm32-unknown-unknown

# Run all tests
cargo test

# Lint the codebase
cargo clippy -- -D warnings
```

---

## Architecture Patterns

### Code Organization

```
cultivation/
├── .project/           # Project documentation
├── assets/
│   ├── data/           # Game data (JSON files)
│   └── images/         # Sprites and icons
├── src/
│   ├── main.rs         # Entry point, window config, main loop
│   ├── game.rs         # Top-level game struct and state transition logic
│   ├── state/          # Game states (SectBase, WorldMap, etc.)
│   ├── engine/         # Core, stateless game logic
│   ├── data/           # Data structures (structs) and loaders
│   ├── ui/             # Reusable immediate-mode UI functions
│   └── save/           # Save/load system
├── Cargo.toml
├── publish.ps1         # Build & deploy script
└── index.html          # WebGL host page
```

### Design Patterns Used

| Pattern | Where Used | Purpose |
|---------|------------|---------|
| State Machine | `game.rs`, `state/` | To manage the active game screen (e.g., Base, Map) and enforce explicit transitions. |
| Immediate-Mode UI | `ui/`, all draw calls | To ensure UI is a simple function of state, returning user intent without modifying state itself. |
| Data-Oriented | `assets/data/`, `data/` | To define game entities (missions, traits) in JSON, separating data from code for easier tuning. |

---

## Environment Configuration

### Required Environment Variables
- None. The application is self-contained.

### Configuration Files
| File | Purpose |
|------|---------|
| `Cargo.toml` | Project manifest, dependencies, and build profiles. |
| `assets/data/*.json` | Defines game data like missions, buildings, and traits. |
| `save.json` | The player's single save file (generated at runtime). |

---

## External Services
- None for MVP.

---

## Security Considerations

### Authentication
- None. This is a single-player, offline game.

### Data Protection
- Save data is stored locally on the user's machine. While not encrypted, the serialized format will discourage trivial manual editing, supporting the ironman design goal.

### Dependencies
- We will use `cargo audit` periodically to scan for vulnerabilities in third-party crates. Dependencies will be kept up-to-date.

---

## Performance Targets

| Metric | Target | Measurement |
|--------|--------|-------------|
| Framerate | Stable 60 FPS | In-game FPS counter during development builds. |
| CPU/Memory Usage | Low when idle | System monitoring tools (Task Manager, etc.). |
| Web Page Load | < 5 seconds | Browser developer tools network analysis. |

---

## Decision Log

| Date | Decision | Rationale | Alternatives Considered |
|------|----------|-----------|------------------------|
| 2026-01-24 | Use Rust + Macroquad | Aligns with the PRD's requirement for a cross-platform (Windows/WebGL), 2D, systems-driven game. Macroquad is lightweight and avoids engine overhead, fitting the "simplicity is a feature" philosophy. | Bevy (more complex, ECS-first), Fyrox (full-featured 3D engine, overkill for this project). |
| 2026-01-24 | Use JSON for data | Human-readable and easy to modify, which is critical for a data-driven game where balancing and content iteration are frequent. `serde` provides robust, low-effort integration with Rust structs. | SQLite (better for complex queries but overkill for MVP), custom binary format (less portable and harder to debug). |

---

*Last updated: 2026-01-24*