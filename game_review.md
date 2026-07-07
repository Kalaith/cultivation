# Heavenly Mandate — Design Review

*Senior design / production review. Prepared 2026-07-07. Based on the GDD, PRD, build plan, source code (~18–19k lines of Rust), the JSON content set, the owner's own `feedback.md`, and captured screenshots of the intro, sect base, and world map screens.*

> **Bottom line up front:** This is not a prototype. It is a broad, genuinely-built systems game that has out-run its own core loop, its content, and — most damagingly — its readability. The engineering is ahead of the design. The single highest-value work is **not** another system; it is choosing what the game *is to the player*, making the sect's rise *legible and dramatic*, and fixing the UI so the player can actually read what's happening. Recommendation: **Continue, but redesign the framing and freeze the system count.**

---

# 1. Project Overview

## Project Name
**Heavenly Mandate** (full working title: *Heavenly Mandate: Disciples of the Outer World*).

## Genre
Cultivation / progression-fantasy **sect-management sim** — a systems-driven 2D colony sim (base building + logistics) fused with an RPG roster (disciples, cultivation, breakthroughs) and a light 4X-ish world layer (factions, diplomacy, trade, events). Closest existing reference point: *Amazing Cultivation Simulator* by way of *RimWorld*, with the presentation ambitions of a wuxia ink painting.

## Core Concept
- **What the player does:** You are the last elder of a fallen sect. You rebuild ruined facilities, recruit and grow disciples, guide their cultivation toward breakthroughs, send them on world-map missions, and manage relations/trade with seven surrounding factions — while seasons, world events, and faction AI churn around you.
- **The fantasy:** *From ashes to heaven.* Start with one ruined hall and 50 spirit stones; end (in the full vision) shattering mountains with a thought. The power-growth curve **is** the product.
- **What makes it different:** Two things stand out. (1) The **Feng Shui / Wu Xing spatial layer** — element placement and room harmony actually feed a scoring system, which is a mechanic almost nobody in this niche ships. (2) The **indirect, emergent-story framing** — disciples have needs, talents, bloodlines, fate traits, and *hidden per-disciple bottlenecks*, so the sect's history is meant to write itself.
- **Target player:** Xianxia/progression-fantasy readers (Cradle, Renegade Immortal) who also enjoy optimization/management games. Secondarily, idle/incremental fans — which, per the owner's own instinct, may be the *better* target (see §10).

## Current State
**Feature-complete-but-unpolished, tipping into content-expansion — with a readability crisis.**

Why: The state machine has 12 real screens; cultivation, crafting, missions, factions, economy, world simulation, AI need-scheduling, seasons, tech, tribulations, spirit beasts, and versioned save/load are all *implemented*, not stubbed. That is far past "prototype." But three things hold it below "polished":
1. **Content is thin** relative to the systems: 3 cultivation Laws, 2 spirit beasts, ~2 map nodes of data, 5 herbs, 10 missions, 14 buildings. The machine is big; the fuel tank is small.
2. **The UI is hard to read** (owner's #1 complaint, confirmed in screenshots) — low-contrast beige, raw code-ish log text, overlapping elements.
3. **The core loop has no spine** — many systems, no single compelling reason-per-minute to keep clicking.

> ⚠️ **Documentation-vs-reality gap:** `build-plan.md` marks Phases 9–13 "100% complete," but Spirit Beasts, the professions/jobs system, and the UI overhaul are only partially or not implemented. Treat the build plan as aspirational, not as ground truth.

---

# 2. Core Gameplay Analysis

## Main Gameplay Loop

The *intended* loop, as built:

> **Repair a facility → Recruit/assign disciples → They auto-work & cultivate (needs sim) → Send a squad on a world mission → Spend rewards on repairs/tech/crafting → Attempt a breakthrough (maybe a tribulation) → Repeat, bigger.**

Under the hood there is a real tick engine: a needs-based scheduler assigns disciples to Rest/Eat/Cultivate/Work every frame; cultivation, income, herb growth, missions, and world-sim update on 60-tick cadences; seasons and salaries on longer ones. So the loop genuinely *runs itself* — a strong foundation for either a management game or an idle game.

**Evaluation:**

| Question | Verdict | Notes |
|---|---|---|
| Is the loop clear? | ❌ Weak | The screenshots show the *tutorial* teaching step 1 ("Restore the Sect Hall"), but nothing communicates the *rhythm* — why do I return to this screen, what am I optimizing? |
| Is it satisfying? | ⚠️ Partial | Repairing and dispatching feel good (owner agrees). Breakthroughs *should* be the payoff but are buried in a modal. |
| Meaningful decisions? | ⚠️ Thin | Decisions exist (who to promote, which mission, where to place elements) but their **consequences aren't surfaced**, so they don't *feel* meaningful. |
| Enough variety? | ❌ No | 10 missions and 3 Laws exhaust quickly; every crafting building shares one UI/mechanic behind different art. |
| Long-term motivation? | ❌ Not yet | The power fantasy is described in the GDD but not *dramatized* in play. Progression happens; it isn't *felt*. |

**The core problem:** the loop is **mechanically complete but emotionally flat.** Every ingredient of a great progression game is present in code, but the game never *tells the player they are winning*. Numbers go up in panels the player can barely read.

---

# 3. Existing Systems Review

The game has an unusually large number of implemented systems. I'm reviewing the ones that matter for the decision; minor systems are folded in.

---

## Cultivation, Disciples & Breakthroughs
### Purpose
The heart of the game — the progression fantasy engine. Disciples are both your labor force and your heroes.
### Current Implementation
Deep. Disciples carry realm, rank (Outer/Inner/Leader), talent tier (Dim Spark → Heaven-Sent), Body/Mind/Spirit attributes, a four-need sim (hunger/rest/qi/morale), bloodlines, fate traits with modifier pipelines, equipment slots, injury/permadeath, and **procedurally-generated hidden bottlenecks** (`engine/bottleneck.rs`) gating breakthroughs behind concealed objectives. Tribulations trigger at higher realms.
### Strengths
This is the project's crown jewel. The hidden-bottleneck idea plus fate traits is a genuine **emergent-story generator** — the exact thing that makes *RimWorld*/*Dwarf Fortress* memorable. It's real, and it's rare in this genre.
### Weaknesses
- The emergent stories **never reach the player**. There's no "chronicle," no character-driven event pop, no memorial that lands emotionally.
- Per the owner: the hidden bottleneck is currently *visible* on the disciple screen — which **destroys the entire mechanic**. A hidden fate test the player can just read is neither hidden nor a test.
- MVP caps at Foundation Establishment with only 3 Laws, so the "exponential power growth" pillar is asserted, not experienced.
### Improvement Ideas
- **Gate bottleneck visibility** behind a building/tech (Divination Hall, "Read Fate") and reveal it as *hints* (dreams, rumors, log omens), never a checklist. — **Impact: High, Cost: Small.**
- **Surface disciple arcs** as first-class dramatic moments: a named disciple hitting a wall, breaking through, dying to tribulation, becoming a legend. The `moments` overlay system already exists — feed it disciple stories. — **Impact: Game-changing, Cost: Medium.**
- Add 3–4 more Laws with *distinct* mechanical identities (not just element reskins). — **Impact: High, Cost: Medium.**

---

## User Interface & Readability
### Purpose
The lens onto everything. In a systems game with no direct control, the UI *is* the game.
### Current Implementation
12 bespoke screens with a cohesive ink-wash wuxia art direction (the backgrounds and the world map are genuinely attractive). But: a beige/parchment palette with low text contrast, all-caps headers, a log that shows raw enum/code text, and overlapping elements. A full 7-phase `ui_overhaul_plan.md` exists but is largely unimplemented.
### Strengths
The *aesthetic direction is right* — the ink map and mist-shrouded base have real mood. This is a solvable polish problem, not a redesign-from-zero.
### Weaknesses
This is, bluntly, **the #1 thing standing between this project and being enjoyable.** From the screenshots: on the world map, "Threat 8 | Corruption 0" is orange-on-beige and nearly invisible; the sect-base log ("Sect Annals") mixes genuine flavor with mechanical noise; panels crowd each other. The owner's own review leads with it, twice.
### Improvement Ideas
- **Contrast pass first:** darken text, add a subtle scrim behind text over busy backgrounds, retire pure-beige-on-beige. — **Impact: Game-changing, Cost: Small.**
- **Humanize the log:** map every logged event to authored prose ("Elder Lin's qi surged past the barrier" not `Breakthrough(id=4)`), and *tier* it (dramatic moments loud, background sim quiet/collapsible). — **Impact: High, Cost: Medium.**
- **Establish one information hierarchy:** what is the player's eye supposed to land on each screen? Right now everything has equal weight. — **Impact: High, Cost: Medium.**

---

## Sect Base, Buildings & Feng Shui / Wu Xing
### Purpose
The spatial/optimization layer and the game's most distinctive mechanic.
### Current Implementation
20×20 grid, 14 building types with Ruined→Constructing→Active states and level upgrades; a real element-propagation model (generation/destruction cycles, radius-2 Qi decay, per-tile auspiciousness scoring) with an 'F' overlay.
### Strengths
The Feng Shui system is the **strongest structural differentiator** in the design. Almost nothing in this niche ships a working spatial-element puzzle.
### Weaknesses
- It is **under-taught and under-rewarded.** If a player can't see what good placement *did for them*, a deep system becomes invisible busywork — which violates the game's own "Mastery, not Gatekeeping" pillar (it's currently *neither* — no gate, but no legible mastery payoff either).
- All crafting buildings share one mechanic behind different art, so "14 buildings" is less variety than it sounds.
### Improvement Ideas
- **Make Feng Shui legible and celebrated:** when a room becomes "Auspicious," say so loudly and show the concrete bonus ("+15% breakthrough chance in this hall"). — **Impact: High, Cost: Small.**
- Give 3–4 buildings genuinely distinct interactions rather than shared crafting UI. — **Impact: Medium, Cost: Medium.**

---

## World Map, Missions & Combat
### Purpose
The outward-facing risk/reward engine and pacing driver.
### Current Implementation
Handsome ink-map with faction/threat nodes; 5 mission types, squad assignment, danger levels, trait-modified success rolls, injury risk, varied rewards, dedicated resolution screen. "Combat" is abstracted: missions resolve as success rolls; the only *interactive* combat is wave-based tribulations.
### Strengths
Dispatch-and-resolve is the loop the owner already likes, and abstracting combat as rolls is the **correct scope decision** — do not build real-time combat.
### Weaknesses
- 10 authored missions is a few sessions of variety at most.
- Mission outcomes are rolls the player can't influence in the moment, so they read as slot-machine rather than decision.
### Improvement Ideas
- **Proceduralize missions** from a template + modifier grammar so the board never runs dry. — **Impact: High, Cost: Medium.**
- Add *pre-mission* levers (bring a pill, pick a formation, accept higher danger for higher reward) so the decision lives *before* the roll. — **Impact: Medium, Cost: Small.**

---

## Factions, Diplomacy, Trade & World Simulation
### Purpose
Living-world backdrop; a source of pressure and opportunity.
### Current Implementation
Genuinely substantial: 7 factions with AI actions, relation drift, territory/war, a supply/demand economy with elasticity pricing and trade routes, and a 12-type world-event system with triggers and effects — all ticking independently in `world_sim.rs`.
### Strengths
An impressive amount of simulation for a solo project; it *can* produce the "living world" pillar.
### Weaknesses
**This is where I'd challenge the project hardest.** This is a large, expensive surface that is mostly **backdrop the player doesn't act on** and can barely see. Several event *effects* are defined but not fully wired. For a game whose actual fun is "watch my sect rise," a full 4X-lite economy/diplomacy sim is **scope the game may not need** — it competes for the same UI/attention budget that readability and disciple-drama desperately need.
### Improvement Ideas
- **Don't expand this. Possibly demote it.** Keep factions/events as *flavor and pressure* (raids, caravans, omens in the log) and defer deep diplomacy/economy interaction until the core loop sings. — **Impact: High (via focus), Cost: Small (mostly *stop* work).**

---

## Crafting (Alchemy / Blacksmith / Talisman)
### Purpose
Production chains feeding cultivation.
### Current Implementation
Recipe difficulty, 5 quality tiers, success formula (building level + Mind + element synergy), crit success, material loss on failure; 18 recipes across three stations that share one mechanic.
### Strengths
Clean, data-driven, extensible. The element-synergy hook ties crafting to Feng Shui nicely.
### Weaknesses
Three "systems" that are one system in three costumes; 18 recipes is thin; outputs' impact on the power curve isn't dramatized.
### Improvement Ideas
- Make **pills the visible accelerant** of breakthroughs (close the crafting→cultivation loop loudly). — **Impact: Medium, Cost: Small.** Hold Talisman/Artifact depth for later.

---

## Onboarding: Intro & Tutorial
### Purpose
Give the player a reason to care and a first step.
### Current Implementation
A **new visual-novel intro** (most recent commit) — "The Long Road Home," multi-beat, fade transitions, the patriarch returning to his ruined sect. Plus an in-base "First Decrees 1/5" step tutorial.
### Strengths
The intro **directly answers the owner's own request** and is the right instinct — it frames the fantasy before the systems. Good.
### Weaknesses
The tutorial teaches *clicks*, not the *loop* or the *fantasy*. It doesn't yet "highlight where to go next" (owner's note).
### Improvement Ideas
- Extend the intro's tone *into* the first 10 minutes: the tutorial's first goals should each deliver a small power-fantasy beat, with clear next-step highlighting. — **Impact: High, Cost: Small.**

---

# 4. Similar Games & Lessons

## Amazing Cultivation Simulator
The nearest sibling — cultivation + colony sim + Feng Shui.
- **Does better:** legible Qi/formation feedback and a *staggering* content depth of techniques and items; the sim is dense enough that optimization is endlessly engaging.
- **Adapt:** its commitment to *showing* the cultivation payoff; the way rooms visibly become power engines.
- **Don't copy:** its notorious UI opacity and brutal onboarding — Heavenly Mandate is *already* fighting that battle and must win it, not import it.

## RimWorld / Dwarf Fortress
The emergent-story gold standard.
- **Does better:** turns simulation into *narrative* — every colonist is a story the game narrates to you.
- **Adapt:** the **story-log / chronicle** and dramatic event framing. Heavenly Mandate already has fate traits, hidden bottlenecks, and a `moments` overlay — it is *one narration layer away* from this and isn't using it.
- **Don't copy:** unbounded system sprawl. That's the trap this project is already near.

## NGU Idle / Cultivation-themed idle games (e.g. *Idle cultivation* mobile titles)
The owner's own stated instinct ("a really good idle game").
- **Does better:** the dopamine of numbers rising while away; prestige loops; *readable, celebratory* progress feedback.
- **Adapt:** offline/idle progression, milestone celebrations, and prestige — a natural fit for the tick engine that already runs the sim unattended.
- **Don't copy:** shallow, purely-numeric play — Heavenly Mandate's disciples give it *soul* an idle game usually lacks. That soul is the moat.

## Songs of Syx / Kingdoms & management sims
- **Lesson:** they succeed by making a *legible growth graph* the player feels. Heavenly Mandate has the growth; it lacks the graph.

**Cross-cutting lesson:** every comparison points the same way — **the systems are competitive; the *communication* of progress is not.** Fix communication before adding systems.

---

# 5. Feature Improvement List

## Critical Improvements
| Priority | Feature | Description | Player Benefit | Dev Cost |
|---|---|---|---|---|
| Critical | **UI contrast & readability pass** | Kill beige-on-beige, add text scrims, fix overlaps, establish per-screen visual hierarchy | The game becomes *playable*; removes the #1 barrier | Small |
| Critical | **Humanized, tiered event log** | Map enum log lines to authored prose; separate dramatic moments from background noise | Turns a debug feed into the sect's living story | Medium |
| Critical | **Re-hide the disciple bottleneck** | Gate it behind a building/tech; reveal via hints, not a label | Restores the game's best emergent mechanic | Small |
| Critical | **Dramatize progression** | Feed disciple breakthroughs/deaths/legends into the `moments` overlay + a chronicle | Makes the power fantasy *felt*, not just computed | Medium |

## High Value Improvements
| Priority | Feature | Description | Player Benefit | Dev Cost |
|---|---|---|---|---|
| High | **Procedural missions** | Template + modifier grammar so the board never empties | Long-term variety without hand-authoring | Medium |
| High | **Legible Feng Shui payoff** | Announce "Auspicious" rooms and show concrete bonuses | Makes the signature mechanic worth learning | Small |
| High | **Idle / offline progression + milestones** | Compute away-time gains; celebrate thresholds on return | Serves the strongest audience fit; huge retention lever | Medium |
| High | **Tutorial that teaches the loop** | Each first-session goal delivers a power beat + next-step highlight | Players understand *why*, not just *what* | Small |
| High | **2–3 more distinct Laws** | Mechanically different paths, not element reskins | Build variety and replay | Medium |

## Nice To Have
| Priority | Feature | Description | Player Benefit | Dev Cost |
|---|---|---|---|---|
| Medium | Pre-mission loadout levers | Pills/formations/risk toggles before the roll | Decisions before outcomes | Small |
| Medium | Building specialization | Give a few buildings unique interactions | Real variety behind the art | Medium |
| Medium | Spirit-beast content pass | Flesh out beyond 2 beasts once core loop is solid | Depth for invested players | Medium |
| Low | Sect Chronicle screen | Browsable history of legends & fallen | Long-tail attachment | Small |

## Avoid / Do Not Add
| Feature | Why avoid |
|---|---|
| **Real-time / tactical combat** | Abstracted rolls are the correct scope; real combat is a second game |
| **Deeper diplomacy/economy 4X systems** | Already over-built vs. player payoff; steals focus from readability & drama |
| **Multiplayer** | Antithetical to a single-sect progression fantasy; enormous cost, no fit |
| **The full professions/jobs system (now)** | Well-designed doc, but more surface before the core loop is fun = wrong order |
| **More parallel systems of any kind** | The game's problem is *too many quiet systems*, not too few |

---

# 6. Missing Gameplay Elements

## A legible "power graph" / progress dashboard
- **Why expected:** every progression game shows you your ascent.
- **Needs it?** **Yes — critical.** The rise is the product and it's currently invisible.
- **Implementation:** a persistent header stat (Sect Power / Prestige) plus a milestone ladder; celebrate each rung.
- **Priority: Critical.**

## Dramatic narration of emergent events
- **Why expected:** the fate/bottleneck systems *promise* stories.
- **Needs it?** **Yes.** It's the payoff for systems already built.
- **Implementation:** route disciple lifecycle events through `moments` + a chronicle log.
- **Priority: Critical.**

## Offline / idle progression
- **Why expected:** the owner wants an idle game; the sim already ticks unattended.
- **Needs it?** **Yes, if idle is the chosen direction** (see §10).
- **Implementation:** timestamp on save, compute elapsed ticks on load, present a "while you were away" summary.
- **Priority: High.**

## Prestige / New-Game-Plus
- **Why expected:** progression games need a top-end reset to sustain play.
- **Needs it?** **Eventually** — after the first-hour loop is fun. Not now.
- **Priority: Medium (deferred).**

## Sound / music
- **Why expected:** mood is half of a wuxia game; screenshots suggest a silent build.
- **Needs it?** **Yes** for polish; ink-wash visuals starve without audio.
- **Priority: Medium.**

## NOT missing (resist the pull)
Character customization, item enchant/gem/set depth, quest-taxonomy sprawl, currency taxonomy — the GDD lists these as "not yet specified." **Keep them unspecified.** They are depth for a game that hasn't yet earned its first fun hour.

---

# 7. Content & Replayability Analysis

**Current reasons to keep playing:** repairing the sect, recruiting, and dispatching missions — a real but shallow well. The world-sim provides *ambient* change but not *player-facing* reasons.

| Lever | State | Move |
|---|---|---|
| Variety | ❌ Thin (3 Laws, 10 missions, 2 beasts) | Proceduralize missions; add Laws |
| Progression | ⚠️ Exists, invisible | Add a legible power graph & milestones |
| Unlocks | ✅ Tech tree present | Tie unlocks to visible power beats |
| Randomness | ✅ Fate/bottleneck/events | *Narrate* it — that's where the value is |
| Player choice | ⚠️ Present, low-consequence | Surface consequences; add pre-mission levers |
| Different strategies | ⚠️ Latent (element builds) | Reward Feng Shui / Law specialization visibly |
| Emergent gameplay | ✅✅ **The moat** | Make it *reach the player* |
| Long-term goals | ❌ Undefended | Milestone ladder now; prestige later |

**Verdict:** replayability potential is **high but locked** — the emergent-story machinery exists in code and never surfaces. Unlocking narration and adding procedural missions buys more replay value than any new system would.

---

# 8. Player Experience Review

## First 10 Minutes
The new visual-novel intro sets mood well (a real improvement). Then the player lands on the sect base, is told "Restore the Sect Hall," and… squints. They grasp *click this building*, but not *why they'll come back*, and they struggle to read panels. **Fix:** carry the intro's tone into tutorial goals, each delivering a tiny power beat, with explicit next-step highlighting.

## First Hour
The hook is weak. A player who pushes through repairs a few buildings, runs a few missions, and sees numbers change in hard-to-read panels. The *promised* hook — a disciple you're rooting for hitting a wall and breaking through — exists in the simulation but is never staged for the player. **The first hour must contain at least one authored, dramatized disciple triumph.**

## Long-Term
Today: little holds a player past the content ceiling (3 Laws, 10 missions, Foundation cap). With procedural missions, a milestone ladder, narrated legends, and (if chosen) idle/prestige, long-term engagement becomes real — and cheaply, because the sim already runs.

---

# 9. Development Roadmap

## Phase 1 — Make It Fun (readability + payoff)
- **Goals:** the game becomes legible and its progression becomes *felt*.
- **Features:** UI contrast/hierarchy pass; humanized tiered log; re-hidden bottleneck; disciple moments dramatized; a persistent Sect Power stat + first milestones; tutorial that teaches the loop.
- **Why first:** none of the existing systems pay off until the player can *read* them and *feel* progress. This is the entire difference between "impressive tech demo" and "game."

## Phase 2 — Add Depth (variety that reuses the engine)
- **Goals:** stop the first-hour content wall.
- **Features:** procedural mission grammar; 2–3 distinct Laws; legible Feng Shui payoffs; pre-mission loadout levers.
- **Why this order:** depth is only worth adding once the payoff is visible; otherwise you're decorating an unread room.

## Phase 3 — Add Content & the Idle/Prestige Direction
- **Goals:** long-tail retention; commit to the audience.
- **Features:** offline/idle progression + "while you were away"; prestige/NG+; spirit-beast content pass; audio.
- **Why:** retention systems only pay off on a loop that's already fun and legible.

## Phase 4 — Polish
- **Goals:** ship quality.
- **Features:** animation/feedback (ripples, brush strokes per the UI plan), sound design pass, balance, save-migration hardening, WASM perf.
- **Why last:** polish multiplies value only once the thing being polished is worth playing.

**Ordering principle:** *communicate before you complicate.* Every phase makes the previously-built systems more legible before adding new ones.

---

# 10. Final Assessment

## Strongest Idea
The **emergent-disciple engine** — fate traits + hidden bottlenecks + permadeath + the `moments` overlay — sitting on top of a **working Feng Shui spatial layer**. Together these are a genuine, rare, defensible hook. Almost no cultivation game ships either well; this one has *both* in code.

## Biggest Risk
**Death by breadth.** The project keeps adding quiet systems (economy, diplomacy, world-sim, professions design) while the player still can't read the screen or feel the sect rising. If that pattern continues, it becomes an impressive, unenjoyable pile — a common and fatal failure mode for solo-dev systems games. The build plan already over-reports completion, which masks the risk.

## Missing Ingredient
**Legibility of progress.** One thing above all: the player must be able to *read the UI* and *feel the sect getting stronger*. Everything the game needs to be great already exists — it just never reaches the player's eyes.

## Unique Selling Point
*"A wuxia sect that writes its own legends."* An ink-painting sect-sim where disciples you grow to love break through impossible walls, fall to heavenly tribulation, and become legends the game *tells you about* — with a Feng Shui layer that turns your halls into engines of ascension. That's a pitch no competitor cleanly owns.

## Recommendation
**Continue development — but redesign the framing and freeze the system count.**

Concretely:
1. **Stop building new systems.** The engineering is already ahead of the design.
2. **Spend the next cycle entirely on Phase 1** (readability + dramatized progression + re-hidden bottleneck). This is small-to-medium work with game-changing impact because it *activates systems already built*.
3. **Pick the audience.** The owner's instinct — "a really good idle game" — is well-founded and the tick engine supports it for free. Leaning idle/progression-fantasy (with the emergent disciple soul as the differentiator) is a **more focused, more shippable** target than the full *Amazing Cultivation Simulator* colony sim. Recommend committing to it.
4. **Demote, don't delete, the 4X-lite layer.** Keep factions/economy/events as pressure and flavor; do not deepen them.

This project does **not** need to be reduced to a prototype or archived — that would waste a large, working, differentiated codebase. It needs *focus and communication*, not more scope. The path to a polished, playable experience is short and mostly already paid for; it runs through the player's eyes, not through new features.
