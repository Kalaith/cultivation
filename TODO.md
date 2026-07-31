# TODO — Heavenly Mandate

## Readability & feedback
- Contrast pass on the UI: retire beige-on-beige text, add scrims over busy backgrounds, fix overlapping panels and all-caps body text.
- Establish one information hierarchy per screen so the eye lands on the pending decision rather than the log.
- Humanize the Sect Annals log — authored prose per event instead of raw enum text, tiered into dramatic and background lines.
- Announce Feng Shui payoffs: say when a room becomes Auspicious and show the concrete bonus it grants.
- Add a persistent Sect Power / prestige readout plus a milestone ladder so the sect's rise is legible.
- Route disciple breakthroughs, deaths and legends through the `moments` overlay and a browsable chronicle.
- Make the tutorial teach the loop, not the clicks, and highlight where to go next.

## Cultivation & disciples
- Re-hide the breakthrough bottleneck — `state/roster/details.rs` prints `player_description` outright; gate it behind a building or tech and reveal it as hints.
- Add 2–3 cultivation Laws with distinct mechanics; `assets/data/laws.json` still holds only the three seed paths.

## Content & variety
- Proceduralize missions from a template plus modifier grammar; the authored board runs dry quickly.
- Add pre-mission levers (bring a pill, pick a formation, accept higher danger) so the decision precedes the roll.
- Spirit beast content pass: two entries in `assets/data/spirit_beasts.json`, no beast roster screen, and no beast slot on missions.
- Expand `assets/data/map_nodes.json` beyond its two nodes.
- Give a few buildings genuinely distinct interactions instead of the shared crafting UI.

## Sect economy & base
- Sect storage capacity: cap held resources, let the Sect Hall and new Treasury/Warehouse buildings raise it, and show usage in the header.

## World map
- Colour regions by controlling faction, and draw trade routes shaded by safety.
- Mark nodes that have missions available and allow dispatching from the map.

## Retention & polish
- Offline/idle progression — timestamp saves, compute elapsed ticks on load, present a "while you were away" summary.
- Prestige / New Game+ once the first hour is fun.
- Audio: music and SFX; the build is silent.

## Engineering
- Integration tests for roster details, roster modals, faction milestones, mission assignment and mission resolution.
- Validate world-map commands for scouting, travel, trade, tribulation and site actions before they mutate campaign state.
- Centralize cultivation, crafting, herb, beast and building tuning into data fixtures with boundary tests.
- Split sect-base map rendering from simulation updates so building details, crafting and panels can share derived state safely.

## Deliberately deferred
- The full professions/jobs system (alchemist, healer, artificer, formation master, talisman master, beast tamer tiers) — designed, but it adds surface before the core loop is fun.
- Real-time or tactical combat, and deeper 4X diplomacy/economy — abstracted rolls and ambient factions are the intended scope.
