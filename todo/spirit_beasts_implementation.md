# Spirit Beasts Implementation Plan

## Scope
Add Spirit Beasts as recruitable, trainable companions that start at **Mortal tier**. They **cannot equip standard disciple gear**; only **beast-only equipment** is allowed.

---

## Phase 1: Data & Schemas
1) **Data models**
- `SpiritBeast` core fields: id, name, species, tier, element, stats, loyalty, hunger, growth, traits, abilities.
- `BeastTier`: Mortal → Body → Qi → Foundation (match MVP cap), with future tiers gated.
- `BeastEquipmentSlot`: collar, harness, talisman, relic (beast-only).
- `BeastEquipmentItem` data: slot, tier req, stats, tags (`beast_only: true`).

2) **Data files**
- `assets/data/spirit_beasts.json`: species templates, tier caps, base stats, trait pools, growth rates.
- `assets/data/beast_equipment.json`: beast-only items.
- `assets/data/beast_training.json`: training actions, costs, outcomes.

3) **Validation**
- Reject equipping non-beast items to beast slots.
- Enforce `beast_only` tag and slot compatibility.

---

## Phase 2: Game State + Save/Load
1) Add `spirit_beasts: Vec<SpiritBeast>` to `GameData` or `GameState`.
2) Persist beasts in save files (id, tier, growth, loyalty, equipment, injuries).
3) Migration for existing saves (empty list default).

---

## Phase 3: Recruitment Systems
1) **Recruitment sources**
- Mission outcome: “Taming Hunt” (new mission type).
- World event: “Injured Beast” (choice-driven).
- Rare drop: “Spirit Egg” item that hatches after N ticks.

2) **Recruitment flow**
- Success check: tier vs disciple skill + item bonus.
- On success: add `SpiritBeast` at Mortal tier with base stats.
- On fail: possible injury or aggression event.

---

## Phase 4: Training & Progression
1) **Training facility**
- Add `BeastTrainingYard` building (future unlock via tech).
- Training actions: loyalty, growth, elemental attunement.

2) **Progression rules**
- Tier starts at Mortal.
- Promotion requires growth, loyalty threshold, and event/ritual.

---

## Phase 5: Combat & Missions
1) Add beast participation to mission simulation:
- Provide additive stats/abilities to mission outcomes.
- Special beast abilities apply modifiers (e.g., dodge, crit, damage).

2) Ensure beasts never use disciple equipment; only beast-only items.

---

## Phase 6: UI & UX
1) Add Beast Roster screen:
- List beasts, tier, loyalty, hunger, status.

2) Beast detail view:
- Stats, abilities, equipment slots (beast-only).

3) Mission assignment:
- Optional beast slot in missions.

---

## Phase 7: Art & Assets
- Beast portraits (512x512) and icons (128x128).
- Beast equipment icons (256x256).

---

## Acceptance Criteria
- Recruit beast via mission/event/egg.
- All beasts start at Mortal tier.
- Beast equipment is restricted to beast-only items and slots.
- Save/load preserves beasts and equipment.
- UI allows viewing and assigning beasts to missions.
