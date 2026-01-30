# UI Update Plan – Sect Management Game

## High-level goals
Your UI should:
- Read clearly at a glance
- Reinforce the cultivation fantasy
- Scale as systems are added
- Avoid looking like a spreadsheet

Right now it succeeds at #1 only partially.

---

## Phase 1 – Information Hierarchy (MOST IMPORTANT)
**Goal:** Fix what the eye sees first.

### Current problems
- Everything has equal visual weight
- Mission list is visually dense
- Event log competes with core gameplay
- No strong “focus area”

### Step 1. Define UI Zones (lock these)
```
┌───────────────┬──────────────────────────────┬───────────────┐
│ Navigation    │ Primary Content               │ Event Log     │
│ (Left Rail)   │ (Map / Missions / Detail)     │ (Right Rail)  │
└───────────────┴──────────────────────────────┴───────────────┘
```

### Step 2. Pick ONE Primary Focus Per Screen
For the Sect Base screen:
- Primary focus = Mission Board

### Action
- Reduce opacity/contrast of Event Log by ~20–30%
- Reduce saturation/brightness of left menu
- Increase contrast on **selected mission only**

---

## Phase 2 – Typography Cleanup (Week 1)
**Goal:** Improve readability without redesign.

### Current issue
- Fonts feel “debuggy”
- Too many sizes look similar
- All caps everywhere reduces readability

### Recommended font roles (no brand changes needed)
| Role | Style |
|------|-------|
| Title (SECT MANAGEMENT) | All caps, wide tracking |
| Section headers | Small caps or bold |
| Body text | Normal case |
| Flavor text | Slightly lighter / italic |

### Immediate fixes
- Reduce ALL CAPS usage outside headers
- Increase line height on mission text
- Shorten mission titles visually (truncate + tooltip)

---

## Phase 3 – Mission Board Redesign (Week 1)
**Goal:** Make missions scannable and decision-oriented.

### Current issues
- Missions look identical
- No scanability
- No sense of risk or reward at a glance

### New mission card structure
Each mission row becomes:
```
[ ICON ]  Mission Name
         Location • Risk • Duration
         Rewards (icons only)
```

### Visual rules
- Only selected mission is fully bright
- Others are muted
- Hover lifts contrast slightly

### Add indicators
- Risk icon (skull / lightning / yin mark)
- Element icon (small stamp)
- Time icon

---

## Phase 4 – Event Log Rework (Week 2)
**Goal:** Reduce noise while keeping value.

### Problems
- Too verbose
- Same weight as player actions
- Feels like a debug stream

### Solution: Tiered Event Visibility
- **Tier 1 – Important (always visible):** wars, major faction changes, disasters, breakthroughs
- **Tier 2 – Background (collapsed/muted):** proposals, minor actions, seasonal changes

### UI changes
- Add event icons
- Fade old events
- Group repeated actions

Example:
```
Crimson Fang Sect proposed alliance (x3)
```

---

## Phase 5 – Color & Contrast Pass (Week 2)
**Goal:** Match ink-wash palette without over-styling.

### Rules
- UI panels ≠ map
- UI uses darker parchment / charcoal
- Map stays light

### Recommended adjustments
- Replace flat dark panels with subtle gradient + paper grain
- Increase contrast between background panel and text
- Reduce yellow usage to highlights only

---

## Phase 6 – Interaction Feedback (Week 2–3)
**Goal:** Make UI feel alive without clutter.

Add:
- Soft ink ripple on click
- Brush stroke highlight on selection
- Slight delay + sound on mission acceptance
- Scroll inertia (subtle)

---

## Phase 7 – Long-term UI Scalability (Week 3)
**Goal:** Future-proof as systems expand.

Add early:
- Collapsible panels
- Contextual sidebars
- Tooltips everywhere
- Icon-first, text-second design

---

## Visual Priority Ladder (pin this)
When the screen is busy, priority should be:
1) Player decision (selected mission)
2) Current resource constraints
3) Immediate threats
4) Background world state
5) Historical log

If something violates this, dim it.

---

## What NOT to do
- Do not add more panels
- Do not add more text
- Do not animate everything
- Do not over-style

Restraint fits your theme.

---

## Suggested Implementation Order (2–3 weeks total)
**Week 1**
- Typography cleanup
- Mission list redesign
- Event log fading

**Week 2**
- Iconography pass
- Hover/selection feedback
- Risk/reward indicators

**Week 3**
- Polish: spacing, sound, transitions

You’ll get ~80% improvement by end of Week 1.

---

## Final framing
Your UI should feel like:
> A sect ledger maintained by an overworked elder who values clarity, ritual, and tradition more than modern convenience.

Right now it feels like:
> A log window with ambitions.
