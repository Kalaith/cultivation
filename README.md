# Heavenly Mandate: Disciples of the Outer World

Cultivation/base‑management RPG built in Rust with Macroquad. You guide a fallen sect’s survivors, build facilities, raise disciples, and navigate a living world of factions and events.

## Features
- Base building with Feng Shui considerations
- Outer/Inner disciple stratification
- Cultivation laws and breakthroughs
- World map missions and outcomes
- Faction/world simulation framework
- Data‑driven content via JSON in assets/data
- AI scheduling with needs, cooldowns, and debug overlay

## Requirements
- Rust (stable toolchain)
- Cargo

## Quick Start
- Build and run:
  - `cargo run`

## Controls
- F9: Toggle AI debug overlay

## Project Structure
- src/: Game code
- assets/data/: JSON content (buildings, laws, missions, tuning, etc.)
- assets/images/: Artwork

## Save/Load
- Desktop builds save to savegame.json in the project directory.

## Data Tuning
- AI scheduling parameters live in assets/data/ai_scheduler.json.

## Status
Actively in development. See design notes in gdd.md and GAME_DEVELOPMENT_GUIDE.md.
